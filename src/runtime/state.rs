//! Protocol state machine for tracking and validating protocol execution
//!
//! This module implements the core state machine that tracks protocol execution
//! progress, validates state transitions, and provides async support for
//! protocol operations.

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use tokio::sync::RwLock;

use crate::protocol::foundation::{GlobalProtocol, Role};
use crate::runtime::error::{
    runtime_error, ErrorContext, ErrorSeverity, ProtocolViolation, RecoverySuggestion, RuntimeError, RuntimeResult,
};
use crate::runtime::validation::{StateValidator, ValidationConfig, ValidationResult};

/// Core protocol state machine tracking execution progress
#[derive(Debug, Clone)]
pub struct ProtocolState<P> {
    session_id: String,
    current_protocol: P,
    is_complete: bool,
    step_count: usize,
    start_time: Instant,
    last_activity: SystemTime,
    execution_context: ExecutionContext,
    validator: Option<Arc<StateValidator>>,
    _protocol: PhantomData<P>,
}

impl<P> ProtocolState<P>
where
    P: GlobalProtocol + Clone,
{
    /// Create a new protocol state machine
    pub fn new<R>(session_id: impl Into<String>, role: R, protocol: P) -> Self
    where
        R: Role,
    {
        let session_id_str = session_id.into();
        let role_str = format!("{:?}", role);
        Self {
            session_id: session_id_str.clone(),
            current_protocol: protocol,
            is_complete: false,
            step_count: 0,
            start_time: Instant::now(),
            last_activity: SystemTime::now(),
            execution_context: ExecutionContext::new(session_id_str, role_str),
            validator: None,
            _protocol: PhantomData,
        }
    }

    /// Create a new protocol state machine with validation enabled
    pub fn with_validation<R>(
        session_id: impl Into<String>,
        role: R,
        protocol: P,
        validator: Arc<StateValidator>,
    ) -> Self
    where
        R: Role,
    {
        let session_id_str = session_id.into();
        let role_str = format!("{:?}", role);
        Self {
            session_id: session_id_str.clone(),
            current_protocol: protocol,
            is_complete: false,
            step_count: 0,
            start_time: Instant::now(),
            last_activity: SystemTime::now(),
            execution_context: ExecutionContext::new(session_id_str, role_str),
            validator: Some(validator),
            _protocol: PhantomData,
        }
    }

    /// Enable validation with custom configuration
    pub fn enable_validation(&mut self, config: ValidationConfig) {
        self.validator = Some(Arc::new(StateValidator::with_config(config)));
    }

    /// Get the current session ID
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Get the current protocol
    pub fn current_protocol(&self) -> &P {
        &self.current_protocol
    }

    /// Check if protocol execution is complete
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    /// Get execution step count
    pub fn step_count(&self) -> usize {
        self.step_count
    }

    /// Get elapsed execution time
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get the execution context
    pub fn context(&self) -> &ExecutionContext {
        &self.execution_context
    }

    /// Update activity timestamp
    pub fn update_activity(&mut self) {
        self.last_activity = SystemTime::now();
        self.step_count += 1;
    }

    /// Mark protocol as complete
    pub fn mark_complete(&mut self) -> RuntimeResult<()> {
        if self.is_complete {
            return Err(runtime_error(RuntimeError::Protocol {
                violation: ProtocolViolation::SessionTerminated {
                    session_id: self.session_id.clone(),
                },
                severity: ErrorSeverity::Critical,
                context: ErrorContext::new()
                    .with_session_id(&self.session_id)
                    .with_component("state_machine")
                    .with_operation("mark_complete"),
                recovery_suggestion: RecoverySuggestion::RestartSession,
            }));
        }

        self.is_complete = true;
        self.update_activity();
        Ok(())
    }

    /// Transition to a new protocol state
    pub fn transition<NewP>(self, new_protocol: NewP) -> RuntimeResult<ProtocolState<NewP>>
    where
        NewP: GlobalProtocol + Clone,
    {
        if self.is_complete {
            return Err(runtime_error(RuntimeError::Protocol {
                violation: ProtocolViolation::SessionTerminated {
                    session_id: self.session_id.clone(),
                },
                severity: ErrorSeverity::Critical,
                context: ErrorContext::new()
                    .with_session_id(&self.session_id)
                    .with_component("state_machine")
                    .with_operation("transition"),
                recovery_suggestion: RecoverySuggestion::RestartSession,
            }));
        }

        Ok(ProtocolState {
            session_id: self.session_id,
            current_protocol: new_protocol,
            is_complete: false,
            step_count: self.step_count + 1,
            start_time: self.start_time,
            last_activity: SystemTime::now(),
            execution_context: self.execution_context,
            validator: self.validator.clone(),
            _protocol: PhantomData,
        })
    }

    /// Transition to a new protocol state with validation
    pub async fn validated_transition<NewP, R>(
        self,
        new_protocol: NewP,
        action: &str,
        role: &R,
    ) -> RuntimeResult<ProtocolState<NewP>>
    where
        NewP: GlobalProtocol + Clone,
        R: Role,
    {
        if self.is_complete {
            return Err(runtime_error(RuntimeError::Protocol {
                violation: ProtocolViolation::SessionTerminated {
                    session_id: self.session_id.clone(),
                },
                severity: ErrorSeverity::Critical,
                context: ErrorContext::new()
                    .with_session_id(&self.session_id)
                    .with_component("state_machine")
                    .with_operation("transition"),
                recovery_suggestion: RecoverySuggestion::RestartSession,
            }));
        }

        // Perform validation if validator is available
        if let Some(validator) = &self.validator {
            match validator
                .validate_transition(&self, &new_protocol, action, role)
                .await
            {
                Ok(ValidationResult::Valid { .. }) => {
                    // Validation passed, proceed with transition
                }
                Ok(ValidationResult::Warning { warnings, .. }) => {
                    // Log warnings but continue
                    for warning in warnings {
                        eprintln!("VALIDATION WARNING: {}", warning);
                    }
                }
                Ok(ValidationResult::Invalid { errors, .. }) => {
                    // Validation failed, log all errors and return the first one
                    let error_vec: Vec<_> = errors.into_iter().collect();
                    
                    // Log all validation errors for comprehensive debugging
                    for error in &error_vec {
                        eprintln!("VALIDATION ERROR: {}", error);
                    }
                    
                    if let Some(first_error) = error_vec.into_iter().next() {
                        return Err(runtime_error(RuntimeError::StateValidation {
                            error: first_error,
                            severity: ErrorSeverity::High,
                            context: ErrorContext::new()
                                .with_session_id(&self.session_id)
                                .with_component("state_machine")
                                .with_operation("validation"),
                            recovery_suggestion: RecoverySuggestion::CheckConfiguration,
                        }));
                    }
                }
                Err(e) => {
                    // Validation system error
                    return Err(e);
                }
            }
        }

        Ok(ProtocolState {
            session_id: self.session_id,
            current_protocol: new_protocol,
            is_complete: false,
            step_count: self.step_count + 1,
            start_time: self.start_time,
            last_activity: SystemTime::now(),
            execution_context: self.execution_context,
            validator: self.validator.clone(),
            _protocol: PhantomData,
        })
    }
}

/// Represents a valid state transition in protocol execution
pub trait StateTransition<From, To> {
    type Error;

    /// Attempt to transition from one protocol state to another
    fn transition(from: ProtocolState<From>) -> Result<ProtocolState<To>, Self::Error>;
}

/// Execution context for protocol operations
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    session_id: String,
    role: String, // Store role as string for simplicity
    start_time: Instant,
    metadata: HashMap<String, String>,
    recursion_depth: usize,
    max_recursion_depth: usize,
}

impl ExecutionContext {
    pub fn new(session_id: String, role: String) -> Self {
        Self {
            session_id,
            role,
            start_time: Instant::now(),
            metadata: HashMap::new(),
            recursion_depth: 0,
            max_recursion_depth: 100, // Default max recursion depth
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn role(&self) -> &str {
        &self.role
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }

    pub fn recursion_depth(&self) -> usize {
        self.recursion_depth
    }

    pub fn max_recursion_depth(&self) -> usize {
        self.max_recursion_depth
    }

    pub fn set_max_recursion_depth(&mut self, depth: usize) {
        self.max_recursion_depth = depth;
    }

    pub fn enter_recursion(&mut self) -> RuntimeResult<()> {
        self.recursion_depth += 1;
        if self.recursion_depth > self.max_recursion_depth {
            Err(runtime_error(RuntimeError::Protocol {
                violation: ProtocolViolation::RecursionDepthExceeded {
                    depth: self.recursion_depth,
                    max_depth: self.max_recursion_depth,
                },
                severity: ErrorSeverity::High,
                context: ErrorContext::new()
                    .with_session_id(&self.session_id)
                    .with_component("state_machine")
                    .with_operation("enter_recursion"),
                recovery_suggestion: RecoverySuggestion::CheckConfiguration,
            }))
        } else {
            Ok(())
        }
    }

    pub fn exit_recursion(&mut self) {
        if self.recursion_depth > 0 {
            self.recursion_depth -= 1;
        }
    }
}

/// Async state machine for managing protocol execution with async operations
#[derive(Debug)]
pub struct AsyncProtocolMachine<S> {
    state: S,
    channels: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
    context: ExecutionContext,
}

impl<S> AsyncProtocolMachine<S> {
    pub fn new(state: S, context: ExecutionContext) -> Self {
        Self {
            state,
            channels: HashMap::new(),
            context,
        }
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn context(&self) -> &ExecutionContext {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut ExecutionContext {
        &mut self.context
    }

    /// Transition to a new state
    pub fn transition<T>(self, new_state: T) -> AsyncProtocolMachine<T> {
        AsyncProtocolMachine {
            state: new_state,
            channels: self.channels,
            context: self.context,
        }
    }

    /// Add a channel to the machine
    pub fn add_channel(&mut self, name: String, channel: Box<dyn std::any::Any + Send + Sync>) {
        self.channels.insert(name, channel);
    }

    /// Get a channel by name (unsafe cast required)
    pub fn get_channel<T: 'static>(&self, name: &str) -> Option<&T> {
        self.channels.get(name)?.downcast_ref::<T>()
    }

    /// Remove a channel by name
    pub fn remove_channel(&mut self, name: &str) -> Option<Box<dyn std::any::Any + Send + Sync>> {
        self.channels.remove(name)
    }
}

/// State manager for tracking multiple concurrent protocol sessions
#[derive(Debug)]
pub struct StateManager {
    sessions: RwLock<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }

    /// Add a new protocol session
    pub async fn add_session<P>(&self, session_id: String, state: ProtocolState<P>)
    where
        P: GlobalProtocol + Clone + Send + Sync + 'static,
    {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id, Box::new(state));
    }

    /// Get a protocol session
    pub async fn get_session<P>(&self, session_id: &str) -> Option<ProtocolState<P>>
    where
        P: GlobalProtocol + Clone + Send + Sync + 'static,
    {
        let sessions = self.sessions.read().await;
        sessions
            .get(session_id)?
            .downcast_ref::<ProtocolState<P>>()
            .cloned()
    }

    /// Remove a protocol session
    pub async fn remove_session(&self, session_id: &str) -> bool {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id).is_some()
    }

    /// List all active session IDs
    pub async fn list_sessions(&self) -> Vec<String> {
        let sessions = self.sessions.read().await;
        sessions.keys().cloned().collect()
    }

    /// Get session count
    pub async fn session_count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::foundation::{BiDirectionalAction, DefaultChan, RequestLbl};
    use crate::TChanEnd;

    // Test roles for validation testing
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Alice;

    impl Role for Alice {}

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[allow(dead_code)]
    struct Bob;

    impl Role for Bob {}

    #[test]
    fn test_protocol_state_creation() {
        let protocol = TChanEnd::<DefaultChan, RequestLbl, BiDirectionalAction>::new();
        let state = ProtocolState::new("test-session", Alice, protocol);

        assert_eq!(state.session_id(), "test-session");
        assert!(!state.is_complete());
        assert_eq!(state.step_count(), 0);
    }

    #[test]
    fn test_protocol_state_completion() {
        let protocol = TChanEnd::<DefaultChan, RequestLbl, BiDirectionalAction>::new();
        let mut state = ProtocolState::new("test-session", Alice, protocol);

        assert!(state.mark_complete().is_ok());
        assert!(state.is_complete());
        assert_eq!(state.step_count(), 1);

        // Second completion should fail
        assert!(state.mark_complete().is_err());
    }

    #[test]
    fn test_execution_context() {
        let mut context = ExecutionContext::new("test".to_string(), "Alice".to_string());

        assert_eq!(context.session_id(), "test");
        assert_eq!(context.recursion_depth(), 0);

        assert!(context.enter_recursion().is_ok());
        assert_eq!(context.recursion_depth(), 1);

        context.exit_recursion();
        assert_eq!(context.recursion_depth(), 0);
    }

    #[test]
    fn test_recursion_depth_limit() {
        let mut context = ExecutionContext::new("test".to_string(), "Alice".to_string());
        context.set_max_recursion_depth(2);

        assert!(context.enter_recursion().is_ok()); // depth 1
        assert!(context.enter_recursion().is_ok()); // depth 2
        assert!(context.enter_recursion().is_err()); // depth 3 - should fail
    }

    #[tokio::test]
    async fn test_state_manager() {
        let manager = StateManager::new();
        let protocol = TChanEnd::<DefaultChan, RequestLbl, BiDirectionalAction>::new();
        let state = ProtocolState::new("test-session", Alice, protocol);

        assert_eq!(manager.session_count().await, 0);

        manager.add_session("test-session".to_string(), state).await;
        assert_eq!(manager.session_count().await, 1);

        let sessions = manager.list_sessions().await;
        assert_eq!(sessions, vec!["test-session"]);

        assert!(manager.remove_session("test-session").await);
        assert_eq!(manager.session_count().await, 0);
    }
}
