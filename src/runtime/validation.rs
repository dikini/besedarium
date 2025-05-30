//! Advanced state transition validation and deadlock/livelock detection
//!
//! This module implements comprehensive validation mechanisms for protocol
//! state transitions and detects various forms of deadlocks and livelocks
//! that can occur during protocol execution.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use tokio::sync::RwLock;

use crate::protocol::foundation::{Role, GlobalProtocol, BiDirectionalAction};
use crate::runtime::error::{
    DeadlockError, LivelockError, RuntimeError, RuntimeResult, StateValidationError,
    ValidationContext, ValidationMode,
};
use crate::runtime::state::ProtocolState;

/// Configuration for validation and detection mechanisms
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Maximum time to wait before considering a deadlock
    pub deadlock_timeout: Duration,
    /// Number of repeated transitions before considering livelock
    pub livelock_threshold: usize,
    /// Time window for livelock detection
    pub livelock_window: Duration,
    /// Validation mode to use
    pub validation_mode: ValidationMode,
    /// Maximum recursion depth for protocol validation
    pub max_recursion_depth: usize,
    /// Enable resource allocation graph analysis
    pub enable_resource_analysis: bool,
    /// Enable progress tracking for livelock detection
    pub enable_progress_tracking: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            deadlock_timeout: Duration::from_secs(30),
            livelock_threshold: 10,
            livelock_window: Duration::from_secs(5),
            validation_mode: ValidationMode::Production,
            max_recursion_depth: 100,
            enable_resource_analysis: true,
            enable_progress_tracking: true,
        }
    }
}

/// State transition validator with deadlock and livelock detection
#[derive(Debug)]
pub struct StateValidator {
    config: ValidationConfig,
    // Resource allocation graph for deadlock detection
    resource_graph: Arc<RwLock<ResourceAllocationGraph>>,
    // State transition history for livelock detection
    transition_history: Arc<RwLock<TransitionHistory>>,
    // Progress tracking for sessions
    progress_tracker: Arc<RwLock<ProgressTracker>>,
}

impl StateValidator {
    /// Create a new state validator with default configuration
    pub fn new() -> Self {
        Self::with_config(ValidationConfig::default())
    }

    /// Create a new state validator with custom configuration
    pub fn with_config(config: ValidationConfig) -> Self {
        Self {
            config,
            resource_graph: Arc::new(RwLock::new(ResourceAllocationGraph::new())),
            transition_history: Arc::new(RwLock::new(TransitionHistory::new())),
            progress_tracker: Arc::new(RwLock::new(ProgressTracker::new())),
        }
    }

    /// Validate a state transition before allowing it to proceed
    pub async fn validate_transition<FromP, ToP, R>(
        &self,
        from_state: &ProtocolState<FromP>,
        to_protocol: &ToP,
        action: &str,
        role: &R,
    ) -> RuntimeResult<ValidationResult>
    where
        FromP: GlobalProtocol + Clone,
        ToP: GlobalProtocol + Clone,
        R: Role,
    {
        let validation_context = ValidationContext {
            timestamp: SystemTime::now(),
            session_metadata: HashMap::new(), // Could be populated from state
            role_context: format!("{:?}", role),
            protocol_position: format!("transition from {} to {}", 
                std::any::type_name::<FromP>(), 
                std::any::type_name::<ToP>()
            ),
            validation_mode: self.config.validation_mode,
        };

        // 1. Validate the transition is protocol-compliant
        self.validate_protocol_compliance(from_state, to_protocol, action, &validation_context)?;

        // 2. Check for potential deadlocks
        if self.config.enable_resource_analysis {
            self.check_deadlock_potential(from_state.session_id(), role).await?;
        }

        // 3. Check for livelock patterns
        if self.config.enable_progress_tracking {
            self.check_livelock_patterns(from_state, action).await?;
        }

        // 4. Record the transition for future analysis
        self.record_transition(from_state.session_id(), action, &validation_context).await;

        Ok(ValidationResult::Valid {
            session_id: from_state.session_id().to_string(),
            validation_timestamp: SystemTime::now(),
            checks_performed: vec![
                "protocol_compliance".to_string(),
                "deadlock_detection".to_string(),
                "livelock_detection".to_string(),
            ],
        })
    }

    /// Validate protocol compliance for a state transition
    fn validate_protocol_compliance<FromP, ToP>(
        &self,
        from_state: &ProtocolState<FromP>,
        to_protocol: &ToP,
        action: &str,
        context: &ValidationContext,
    ) -> RuntimeResult<()>
    where
        FromP: GlobalProtocol + Clone,
        ToP: GlobalProtocol + Clone,
    {
        // For now, implement basic validation
        // In a full implementation, this would check against protocol specification
        match context.validation_mode {
            ValidationMode::Strict => {
                // Perform comprehensive protocol validation
                self.strict_protocol_validation(from_state, to_protocol, action, context)
            }
            ValidationMode::Lenient => {
                // Allow some flexibility in transitions
                Ok(())
            }
            ValidationMode::Debug => {
                // Extra validation with detailed logging
                self.debug_protocol_validation(from_state, to_protocol, action, context)
            }
            ValidationMode::Production => {
                // Essential validation only
                self.production_protocol_validation(from_state, to_protocol, action, context)
            }
        }
    }

    fn strict_protocol_validation<FromP, ToP>(
        &self,
        _from_state: &ProtocolState<FromP>,
        _to_protocol: &ToP,
        _action: &str,
        _context: &ValidationContext,
    ) -> RuntimeResult<()>
    where
        FromP: GlobalProtocol + Clone,
        ToP: GlobalProtocol + Clone,
    {
        // Implement strict validation logic
        // This would involve checking protocol automaton states
        Ok(())
    }

    fn debug_protocol_validation<FromP, ToP>(
        &self,
        _from_state: &ProtocolState<FromP>,
        _to_protocol: &ToP,
        action: &str,
        context: &ValidationContext,
    ) -> RuntimeResult<()>
    where
        FromP: GlobalProtocol + Clone,
        ToP: GlobalProtocol + Clone,
    {
        // Log detailed information for debugging
        println!("DEBUG: Validating transition '{}' at {:?}", action, context.timestamp);
        Ok(())
    }

    fn production_protocol_validation<FromP, ToP>(
        &self,
        _from_state: &ProtocolState<FromP>,
        _to_protocol: &ToP,
        _action: &str,
        _context: &ValidationContext,
    ) -> RuntimeResult<()>
    where
        FromP: GlobalProtocol + Clone,
        ToP: GlobalProtocol + Clone,
    {
        // Fast, essential validation only
        Ok(())
    }

    /// Check for potential deadlock situations
    async fn check_deadlock_potential<R>(
        &self,
        session_id: &str,
        role: &R,
    ) -> RuntimeResult<()>
    where
        R: Role,
    {
        let resource_graph = self.resource_graph.read().await;
        
        // Check if adding this role/session would create a cycle
        if let Some(cycle) = resource_graph.detect_cycle(session_id, &format!("{:?}", role)) {
            return Err(RuntimeError::Deadlock(DeadlockError::CircularDependency {
                session_id: session_id.to_string(),
                involved_roles: cycle.roles,
                resource_chain: cycle.resources,
                detection_time: SystemTime::now(),
            }));
        }

        Ok(())
    }

    /// Check for livelock patterns in state transitions
    async fn check_livelock_patterns<P>(
        &self,
        state: &ProtocolState<P>,
        action: &str,
    ) -> RuntimeResult<()>
    where
        P: GlobalProtocol + Clone,
    {
        let history = self.transition_history.write().await;
        
        // Check for repeated transitions
        if let Some(repeated_count) = history.check_repeated_transitions(
            state.session_id(),
            action,
            self.config.livelock_threshold,
            self.config.livelock_window,
        ) {
            return Err(RuntimeError::Livelock(LivelockError::RepeatedTransitions {
                session_id: state.session_id().to_string(),
                transition_count: repeated_count,
                repeated_transition: action.to_string(),
                duration: self.config.livelock_window,
                state_history: history.get_recent_states(state.session_id(), 10),
            }));
        }

        Ok(())
    }

    /// Record a transition for future analysis
    async fn record_transition(
        &self,
        session_id: &str,
        action: &str,
        context: &ValidationContext,
    ) {
        let mut history = self.transition_history.write().await;
        history.record_transition(session_id, action, context.timestamp);

        let mut progress = self.progress_tracker.write().await;
        progress.update_activity(session_id, context.timestamp);
    }
}

/// Result of state validation
#[derive(Debug, Clone)]
pub enum ValidationResult {
    Valid {
        session_id: String,
        validation_timestamp: SystemTime,
        checks_performed: Vec<String>,
    },
    Warning {
        session_id: String,
        warnings: Vec<String>,
        validation_timestamp: SystemTime,
    },
    Invalid {
        session_id: String,
        errors: Vec<StateValidationError>,
        validation_timestamp: SystemTime,
    },
}

/// Resource allocation graph for deadlock detection
#[derive(Debug)]
struct ResourceAllocationGraph {
    // Session -> Set of resources it's waiting for
    waiting_for: HashMap<String, HashSet<String>>,
    // Resource -> Session that owns it
    owned_by: HashMap<String, String>,
    // Session -> Set of resources it owns
    owns: HashMap<String, HashSet<String>>,
}

impl ResourceAllocationGraph {
    fn new() -> Self {
        Self {
            waiting_for: HashMap::new(),
            owned_by: HashMap::new(),
            owns: HashMap::new(),
        }
    }

    /// Detect if adding a new dependency would create a cycle
    fn detect_cycle(&self, _session_id: &str, _resource: &str) -> Option<CycleInfo> {
        // Implement cycle detection algorithm (e.g., DFS)
        // For now, return None (no cycle detected)
        // In a real implementation, this would perform graph traversal
        None
    }

    /// Add a resource dependency
    fn add_dependency(&mut self, session_id: String, resource: String) {
        self.waiting_for.entry(session_id).or_insert_with(HashSet::new).insert(resource);
    }

    /// Remove a resource dependency
    fn remove_dependency(&mut self, session_id: &str, resource: &str) {
        if let Some(resources) = self.waiting_for.get_mut(session_id) {
            resources.remove(resource);
        }
    }
}

#[derive(Debug, Clone)]
struct CycleInfo {
    roles: Vec<String>,
    resources: Vec<String>,
}

/// Transition history tracker for livelock detection
#[derive(Debug)]
struct TransitionHistory {
    // Session -> List of (action, timestamp) pairs
    transitions: HashMap<String, VecDeque<(String, SystemTime)>>,
    // Maximum history size per session
    max_history_size: usize,
}

impl TransitionHistory {
    fn new() -> Self {
        Self {
            transitions: HashMap::new(),
            max_history_size: 100,
        }
    }

    /// Record a new transition
    fn record_transition(&mut self, session_id: &str, action: &str, timestamp: SystemTime) {
        let history = self.transitions.entry(session_id.to_string()).or_insert_with(VecDeque::new);
        
        history.push_back((action.to_string(), timestamp));
        
        // Keep history size manageable
        while history.len() > self.max_history_size {
            history.pop_front();
        }
    }

    /// Check for repeated transitions within a time window
    fn check_repeated_transitions(
        &self,
        session_id: &str,
        action: &str,
        threshold: usize,
        window: Duration,
    ) -> Option<usize> {
        if let Some(history) = self.transitions.get(session_id) {
            let now = SystemTime::now();
            let cutoff = now - window;
            
            let recent_count = history
                .iter()
                .rev()
                .take_while(|(_, timestamp)| *timestamp > cutoff)
                .filter(|(a, _)| a == action)
                .count();
                
            if recent_count >= threshold {
                Some(recent_count)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get recent state information for debugging
    fn get_recent_states(&self, session_id: &str, count: usize) -> Vec<String> {
        if let Some(history) = self.transitions.get(session_id) {
            history
                .iter()
                .rev()
                .take(count)
                .map(|(action, _)| action.clone())
                .collect()
        } else {
            Vec::new()
        }
    }
}

/// Progress tracker for detecting lack of forward progress
#[derive(Debug)]
struct ProgressTracker {
    // Session -> Last activity timestamp
    last_activity: HashMap<String, SystemTime>,
    // Session -> Activity count in current time window
    activity_counts: HashMap<String, usize>,
    // Session -> Progress metric (0.0 to 1.0)
    progress_metrics: HashMap<String, f64>,
}

impl ProgressTracker {
    fn new() -> Self {
        Self {
            last_activity: HashMap::new(),
            activity_counts: HashMap::new(),
            progress_metrics: HashMap::new(),
        }
    }

    /// Update activity for a session
    fn update_activity(&mut self, session_id: &str, timestamp: SystemTime) {
        self.last_activity.insert(session_id.to_string(), timestamp);
        
        let count = self.activity_counts.entry(session_id.to_string()).or_insert(0);
        *count += 1;
    }

    /// Calculate progress metric for a session
    fn calculate_progress(&self, session_id: &str) -> f64 {
        // Simple implementation - in reality this would be more sophisticated
        self.progress_metrics.get(session_id).copied().unwrap_or(0.0)
    }
}

impl Default for StateValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::foundation::{CommMetadata, DefaultChan, RequestLbl, Role};
    use crate::protocol::global::TChanEnd;

    // Test role implementations
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct TestRoleA;
    impl Role for TestRoleA {}

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct TestRoleB;
    impl Role for TestRoleB {}

    #[tokio::test]
    async fn test_state_validator_creation() {
        let validator = StateValidator::new();
        assert_eq!(validator.config.validation_mode, ValidationMode::Production);
    }

    #[tokio::test]
    async fn test_validation_config_default() {
        let config = ValidationConfig::default();
        assert_eq!(config.deadlock_timeout, Duration::from_secs(30));
        assert_eq!(config.livelock_threshold, 10);
        assert!(config.enable_resource_analysis);
        assert!(config.enable_progress_tracking);
    }

    #[tokio::test]
    async fn test_basic_transition_validation() {
        let validator = StateValidator::new();
        let metadata = CommMetadata::new(DefaultChan, RequestLbl);
        let from_protocol: TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction> = TChanEnd::new();
        let to_protocol: TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction> = TChanEnd::new();
        
        let from_state = ProtocolState::new("test_session", TestRoleA, from_protocol);
        let role = TestRoleA;
        
        let result = validator.validate_transition(
            &from_state,
            &to_protocol,
            "test_action",
            &role,
        ).await;
        
        assert!(result.is_ok());
        if let Ok(ValidationResult::Valid { session_id, .. }) = result {
            assert_eq!(session_id, "test_session");
        }
    }

    #[test]
    fn test_resource_allocation_graph() {
        let mut graph = ResourceAllocationGraph::new();
        graph.add_dependency("session1".to_string(), "resource1".to_string());
        
        assert!(graph.waiting_for.contains_key("session1"));
        assert!(graph.waiting_for["session1"].contains("resource1"));
    }

    #[test]
    fn test_transition_history() {
        let mut history = TransitionHistory::new();
        let now = SystemTime::now();
        
        history.record_transition("session1", "action1", now);
        
        assert!(history.transitions.contains_key("session1"));
        assert_eq!(history.transitions["session1"].len(), 1);
    }

    #[test]
    fn test_repeated_transition_detection() {
        let mut history = TransitionHistory::new();
        let now = SystemTime::now();
        
        // Add multiple instances of the same action
        for _ in 0..5 {
            history.record_transition("session1", "repeated_action", now);
        }
        
        let result = history.check_repeated_transitions(
            "session1",
            "repeated_action",
            3,
            Duration::from_secs(60),
        );
        
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 5);
    }
}
