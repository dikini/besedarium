//! Runtime error types for session type execution
//!
//! This module defines comprehensive error types for all runtime failures
//! that can occur during protocol execution, including protocol violations,
//! communication errors, and system-level failures.

use std::time::{Duration, SystemTime};
use thiserror::Error;

/// Main runtime error type encompassing all possible runtime failures
#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Protocol violation: {0}")]
    Protocol(#[from] ProtocolViolation),
    
    #[error("Communication error: {0}")]
    Communication(#[from] CommunicationError),
    
    #[error("Serialization error: {message}")]
    Serialization {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    },
    
    #[error("Timeout error: operation timed out after {duration_ms}ms")]
    Timeout { duration_ms: u64 },
    
    #[error("Configuration error: {message}")]
    Configuration { message: String },
    
    #[error("System error: {message}")]
    System { message: String },
    
    #[error("Deadlock detected: {0}")]
    Deadlock(#[from] DeadlockError),
    
    #[error("Livelock detected: {0}")]
    Livelock(#[from] LivelockError),
    
    #[error("State validation failed: {0}")]
    StateValidation(#[from] StateValidationError),
    
    #[error("Unknown error: {message}")]
    Unknown { message: String },
}

/// Protocol-specific violations that occur during execution
#[derive(Error, Debug)]
pub enum ProtocolViolation {
    #[error("Invalid state transition: attempted to move from state '{current_state}' by action '{action_taken}' to invalid next state. Expected one of: {expected_actions_or_states}")]
    InvalidTransition {
        current_state: String,
        action_taken: String, // e.g., "receive MessageX", "perform ActionY"
        expected_actions_or_states: String, // e.g., "receive MessageZ", "transition to StateA"
    },

    #[error("Unexpected message type: expected '{expected}', got '{actual}' in state '{current_state}'")]
    UnexpectedMessage {
        expected: String,
        actual: String,
        current_state: String,
    },

    #[error("Action '{action}' not allowed in current state '{current_state}'")]
    ActionNotAllowedInState {
        action: String,
        current_state: String,
    },

    #[error("Protocol deadlock detected in session '{session_id}'")]
    Deadlock { session_id: String },
    
    #[error("Choice '{choice}' not available in current protocol state '{current_state}'")]
    ChoiceNotAvailable { choice: String, current_state: String },

    #[error("Role mismatch: expected role '{expected}', got '{actual}' for current action")]
    RoleMismatch { expected: String, actual: String },
    
    #[error("Protocol not complete: {remaining} steps remaining")]
    IncompleteProtocol { remaining: usize },
    
    #[error("Recursive protocol depth exceeded: {depth} (max: {max_depth})")]
    RecursionDepthExceeded { depth: usize, max_depth: usize },
    
    #[error("Protocol session '{session_id}' already terminated")]
    SessionTerminated { session_id: String },
    
    #[error("Invalid protocol state: {details}")]
    InvalidState { details: String },
}

/// Communication-related errors for channel operations
#[derive(Error, Debug)]
pub enum CommunicationError {
    #[error("Channel closed unexpectedly")]
    ChannelClosed,
    
    #[error("Channel full: capacity {capacity} exceeded")]
    ChannelFull { capacity: usize },
    
    #[error("Channel receive timeout after {timeout_ms}ms")]
    ReceiveTimeout { timeout_ms: u64 },
    
    #[error("Channel send timeout after {timeout_ms}ms")]
    SendTimeout { timeout_ms: u64 },
    
    #[error("Network error: {message}")]
    Network { message: String },
    
    #[error("Connection refused to {address}")]
    ConnectionRefused { address: String },
    
    #[error("Connection lost to {address}")]
    ConnectionLost { address: String },
    
    #[error("Authentication failed for session '{session_id}'")]
    AuthenticationFailed { session_id: String },
    
    #[error("Message encoding failed: {details}")]
    EncodingFailed { details: String },
    
    #[error("Message decoding failed: {details}")]
    DecodingFailed { details: String },
    
    /// Enhanced communication error with detailed context
    #[error("Channel {operation} failed on channel '{channel_id}' in session '{session_id}': {details}")]
    ChannelOperationFailed {
        channel_id: String,
        operation: ChannelOperation,
        peer_role: Option<String>,
        session_id: String,
        details: String,
        #[source]
        underlying_error: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    
    /// Enhanced timeout error with context
    #[error("Channel {operation} timeout after {timeout_ms}ms on channel '{channel_id}' in session '{session_id}'")]
    ChannelTimeout {
        channel_id: String,
        operation: ChannelOperation,
        peer_role: Option<String>,
        session_id: String,
        timeout_ms: u64,
    },
    
    /// Enhanced deserialization error with type information
    #[error("Message deserialization failed on channel '{channel_id}': expected '{expected_type}', got data of length {actual_data_length} bytes")]
    DeserializationFailed {
        channel_id: String,
        expected_type: String,
        actual_data_length: usize,
        raw_data_preview: Option<String>, // First few bytes as hex, for debugging
        session_id: String,
        underlying_error: String,
    },
    
    /// Enhanced serialization error
    #[error("Message serialization failed on channel '{channel_id}' for type '{message_type}' in session '{session_id}': {underlying_error}")]
    SerializationFailed {
        channel_id: String,
        message_type: String,
        session_id: String,
        underlying_error: String,
    },
}

/// Channel operation types for error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelOperation {
    Send,
    Receive,
    Close,
    Connect,
}

impl std::fmt::Display for ChannelOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelOperation::Send => write!(f, "send"),
            ChannelOperation::Receive => write!(f, "receive"),
            ChannelOperation::Close => write!(f, "close"),
            ChannelOperation::Connect => write!(f, "connect"),
        }
    }
}

/// Deadlock detection and reporting
#[derive(Error, Debug, Clone)]
pub enum DeadlockError {
    #[error("Circular dependency detected in session '{session_id}': {involved_roles:?} are waiting for each other")]
    CircularDependency {
        session_id: String,
        involved_roles: Vec<String>,
        resource_chain: Vec<String>,
        detection_time: SystemTime,
    },
    
    #[error("Resource deadlock detected: {resource_count} resources involved across {session_count} sessions")]
    ResourceDeadlock {
        session_count: usize,
        resource_count: usize,
        waiting_graph: String, // Serialized representation of the wait graph
        detection_algorithm: String,
    },
    
    #[error("Protocol deadlock in session '{session_id}': roles {roles:?} are in mutual wait state")]
    ProtocolDeadlock {
        session_id: String,
        roles: Vec<String>,
        current_states: Vec<String>,
        expected_actions: Vec<String>,
        wait_duration: Duration,
    },
}

/// Livelock detection and reporting
#[derive(Error, Debug, Clone)]
pub enum LivelockError {
    #[error("Repeated state transitions without progress in session '{session_id}': {transition_count} identical transitions in {duration:?}")]
    RepeatedTransitions {
        session_id: String,
        transition_count: usize,
        repeated_transition: String,
        duration: Duration,
        state_history: Vec<String>,
    },
    
    #[error("Protocol livelock detected: roles are active but making no progress towards completion")]
    ProtocolLivelock {
        session_id: String,
        involved_roles: Vec<String>,
        activity_count: usize,
        progress_metric: f64, // Measure of actual progress (0.0 = no progress, 1.0 = full progress)
        detection_threshold: f64,
    },
    
    #[error("Resource contention livelock: {contention_count} attempts to acquire resource '{resource_name}' without success")]
    ResourceContention {
        resource_name: String,
        contention_count: usize,
        involved_sessions: Vec<String>,
        average_wait_time: Duration,
    },
}

/// State validation errors with detailed context
#[derive(Error, Debug, Clone)]
pub enum StateValidationError {
    #[error("Invalid state transition in session '{session_id}': cannot transition from '{from_state}' to '{to_state}' via action '{action}'")]
    InvalidTransition {
        session_id: String,
        from_state: String,
        to_state: String,
        action: String,
        allowed_transitions: Vec<String>,
        validation_context: ValidationContext,
    },
    
    #[error("Protocol specification violation: {violation_type} in session '{session_id}'")]
    ProtocolSpecViolation {
        session_id: String,
        violation_type: String,
        current_state: String,
        protocol_constraints: Vec<String>,
        suggested_actions: Vec<String>,
    },
    
    #[error("Role validation failed: role '{role}' not authorized for action '{action}' in current state")]
    RoleValidationFailed {
        role: String,
        action: String,
        current_state: String,
        authorized_roles: Vec<String>,
    },
    
    #[error("Message type validation failed: expected {expected_types:?}, received '{actual_type}' in state '{current_state}'")]
    MessageTypeValidation {
        expected_types: Vec<String>,
        actual_type: String,
        current_state: String,
        message_context: String,
    },
    
    #[error("Protocol consistency check failed: {inconsistency_details}")]
    ProtocolConsistency {
        inconsistency_details: String,
        affected_states: Vec<String>,
        repair_suggestions: Vec<String>,
    },
}

/// Context information for state validation
#[derive(Debug, Clone)]
pub struct ValidationContext {
    pub timestamp: SystemTime,
    pub session_metadata: std::collections::HashMap<String, String>,
    pub role_context: String,
    pub protocol_position: String,
    pub validation_mode: ValidationMode,
}

/// Different validation modes for different scenarios
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationMode {
    /// Strict validation - all protocol rules must be followed exactly
    Strict,
    /// Lenient validation - some minor violations may be warnings
    Lenient,
    /// Debug validation - extensive checking with detailed reporting
    Debug,
    /// Production validation - optimized for performance with essential checks
    Production,
}

impl Default for ValidationMode {
    fn default() -> Self {
        ValidationMode::Production
    }
}

/// Convenience type alias for runtime results
pub type RuntimeResult<T> = Result<T, RuntimeError>;

/// Extension trait for converting standard errors to runtime errors
pub trait IntoRuntimeError {
    fn into_runtime_error(self) -> RuntimeError;
}

impl IntoRuntimeError for std::io::Error {
    fn into_runtime_error(self) -> RuntimeError {
        RuntimeError::System {
            message: self.to_string(),
        }
    }
}

impl IntoRuntimeError for serde_json::Error {
    fn into_runtime_error(self) -> RuntimeError {
        RuntimeError::Serialization {
            message: self.to_string(),
            source: Some(Box::new(self)),
        }
    }
}

impl IntoRuntimeError for tokio::time::error::Elapsed {
    fn into_runtime_error(self) -> RuntimeError {
        RuntimeError::Timeout { duration_ms: 0 }
    }
}

/// Create a protocol violation error
pub fn protocol_violation(violation: ProtocolViolation) -> RuntimeError {
    RuntimeError::Protocol(violation)
}

/// Create a communication error
pub fn communication_error(error: CommunicationError) -> RuntimeError {
    RuntimeError::Communication(error)
}

/// Create a configuration error
pub fn configuration_error(message: impl Into<String>) -> RuntimeError {
    RuntimeError::Configuration {
        message: message.into(),
    }
}

/// Create a system error
pub fn system_error(message: impl Into<String>) -> RuntimeError {
    RuntimeError::System {
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = ProtocolViolation::InvalidTransition {
            current_state: "Ready".to_string(),
            action_taken: "ProcessData".to_string(),
            expected_actions_or_states: "ReceiveAck or Timeout".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Invalid state transition: attempted to move from state 'Ready' by action 'ProcessData' to invalid next state. Expected one of: ReceiveAck or Timeout"
        );

        let error = ProtocolViolation::UnexpectedMessage {
            expected: "Ack".to_string(),
            actual: "Nack".to_string(),
            current_state: "WaitingForAck".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Unexpected message type: expected 'Ack', got 'Nack' in state 'WaitingForAck'"
        );

        let error = ProtocolViolation::ActionNotAllowedInState {
            action: "SendPayment".to_string(),
            current_state: "OrderPending".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Action 'SendPayment' not allowed in current state 'OrderPending'"
        );

        let error = ProtocolViolation::ChoiceNotAvailable {
            choice: "Retry".to_string(),
            current_state: "Failed".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Choice 'Retry' not available in current protocol state 'Failed'"
        );
    }

    #[test]
    fn test_runtime_error_from_protocol_violation() {
        let violation = ProtocolViolation::Deadlock {
            session_id: "test-session".to_string(),
        };
        let runtime_error = RuntimeError::Protocol(violation);
        assert!(matches!(runtime_error, RuntimeError::Protocol(_)));
    }

    #[test]
    fn test_error_convenience_functions() {
        let error = configuration_error("Invalid timeout");
        assert!(matches!(error, RuntimeError::Configuration { .. }));
        
        let error = system_error("File not found");
        assert!(matches!(error, RuntimeError::System { .. }));
    }
}
