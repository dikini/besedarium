//! Runtime error types for session type execution
//!
//! This module defines comprehensive error types for all runtime failures
//! that can occur during protocol execution, including protocol violations,
//! communication errors, and system-level failures.

use std::fmt;
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
