//! Runtime error types for session type execution
//!
//! This module defines comprehensive error types for all runtime failures
//! that can occur during protocol execution, including protocol violations,
//! communication errors, and system-level failures.

use std::time::{Duration, SystemTime};
use thiserror::Error;

/// Error severity levels for runtime errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Default)]
pub enum ErrorSeverity {
    /// Low severity - warnings or recoverable issues
    Low,
    /// Medium severity - errors that may affect functionality but allow continuation
    #[default]
    Medium,
    /// High severity - critical errors that require immediate attention
    High,
    /// Critical severity - fatal errors that require shutdown or termination
    Critical,
}


impl std::fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSeverity::Low => write!(f, "LOW"),
            ErrorSeverity::Medium => write!(f, "MEDIUM"),
            ErrorSeverity::High => write!(f, "HIGH"),
            ErrorSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Error categories for classification and handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Protocol-related errors (violations, state transitions)
    Protocol,
    /// Communication and networking errors
    Communication,
    /// Data serialization/deserialization errors
    Serialization,
    /// Timeout and timing-related errors
    Timeout,
    /// Configuration and setup errors
    Configuration,
    /// System-level errors (I/O, resources)
    System,
    /// Execution and runtime errors
    Execution,
    /// Session management errors
    Session,
    /// Validation and verification errors
    Validation,
    /// Unknown or unclassified errors
    Unknown,
}

impl std::fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCategory::Protocol => write!(f, "Protocol"),
            ErrorCategory::Communication => write!(f, "Communication"),
            ErrorCategory::Serialization => write!(f, "Serialization"),
            ErrorCategory::Timeout => write!(f, "Timeout"),
            ErrorCategory::Configuration => write!(f, "Configuration"),
            ErrorCategory::System => write!(f, "System"),
            ErrorCategory::Execution => write!(f, "Execution"),
            ErrorCategory::Session => write!(f, "Session"),
            ErrorCategory::Validation => write!(f, "Validation"),
            ErrorCategory::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Recovery suggestions for runtime errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoverySuggestion {
    /// Retry the operation with the same parameters
    Retry,
    /// Retry with exponential backoff
    RetryWithBackoff,
    /// Retry with different parameters
    RetryWithDifferentParams(String),
    /// Restart the session or connection
    RestartSession,
    /// Check configuration and retry
    CheckConfiguration,
    /// Check network connectivity
    CheckNetwork,
    /// Terminate gracefully and report error
    Terminate,
    /// Custom recovery action
    Custom(String),
    /// No recovery possible
    None,
}

impl std::fmt::Display for RecoverySuggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecoverySuggestion::Retry => write!(f, "Retry the operation"),
            RecoverySuggestion::RetryWithBackoff => write!(f, "Retry with exponential backoff"),
            RecoverySuggestion::RetryWithDifferentParams(params) => {
                write!(f, "Retry with different parameters: {}", params)
            }
            RecoverySuggestion::RestartSession => write!(f, "Restart the session or connection"),
            RecoverySuggestion::CheckConfiguration => write!(f, "Check configuration and retry"),
            RecoverySuggestion::CheckNetwork => write!(f, "Check network connectivity"),
            RecoverySuggestion::Terminate => write!(f, "Terminate gracefully and report error"),
            RecoverySuggestion::Custom(action) => write!(f, "{}", action),
            RecoverySuggestion::None => write!(f, "No recovery action available"),
        }
    }
}

/// Enhanced error context with structured information
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Timestamp when the error occurred
    pub timestamp: SystemTime,
    /// Session identifier if applicable
    pub session_id: Option<String>,
    /// Component or module where the error occurred
    pub component: Option<String>,
    /// Operation being performed when error occurred
    pub operation: Option<String>,
    /// Additional metadata as key-value pairs
    pub metadata: std::collections::HashMap<String, String>,
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
            session_id: None,
            component: None,
            operation: None,
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl ErrorContext {
    /// Create a new error context with current timestamp
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the session ID
    pub fn with_session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    /// Set the component name
    pub fn with_component(mut self, component: impl Into<String>) -> Self {
        self.component = Some(component.into());
        self
    }

    /// Set the operation name
    pub fn with_operation(mut self, operation: impl Into<String>) -> Self {
        self.operation = Some(operation.into());
        self
    }

    /// Add metadata key-value pair
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Main runtime error type encompassing all possible runtime failures
#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("[{severity}] Protocol violation: {violation}")]
    Protocol {
        violation: ProtocolViolation,
        severity: ErrorSeverity,
        context: ErrorContext,
        recovery_suggestion: RecoverySuggestion,
    },

    #[error("[{severity}] Communication error: {error}")]
    Communication {
        error: CommunicationError,
        severity: ErrorSeverity,
        context: ErrorContext,
        recovery_suggestion: RecoverySuggestion,
    },

    #[error("[{severity}] Serialization error: {message}")]
    Serialization {
        message: String,
        severity: ErrorSeverity,
        context: ErrorContext,
        recovery_suggestion: RecoverySuggestion,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    },

    #[error("[{severity}] Timeout error: operation timed out after {duration_ms}ms")]
    Timeout {
        duration_ms: u64,
        severity: ErrorSeverity,
        context: ErrorContext,
        recovery_suggestion: RecoverySuggestion,
    },

    #[error("[{severity}] Configuration error: {message}")]
    Configuration {
        message: String,
        severity: ErrorSeverity,
        context: ErrorContext,
        recovery_suggestion: RecoverySuggestion,
    },

    #[error("[{severity}] System error: {message}")]
    System {
        message: String,
        severity: ErrorSeverity,
        context: ErrorContext,
        recovery_suggestion: RecoverySuggestion,
    },

    #[error("[{severity}] Execution error: {message}")]
    Execution {
        message: String,
        severity: ErrorSeverity,
        context: ErrorContext,
        recovery_suggestion: RecoverySuggestion,
    },

    #[error("[{severity}] Session already exists: {session_id}")]
    SessionAlreadyExists {
        session_id: String,
        severity: ErrorSeverity,
        context: ErrorContext,
        recovery_suggestion: RecoverySuggestion,
    },

    #[error("[{severity}] Deadlock detected: {error}")]
    Deadlock {
        error: DeadlockError,
        severity: ErrorSeverity,
        context: ErrorContext,
        recovery_suggestion: RecoverySuggestion,
    },

    #[error("[{severity}] Livelock detected: {error}")]
    Livelock {
        error: LivelockError,
        severity: ErrorSeverity,
        context: ErrorContext,
        recovery_suggestion: RecoverySuggestion,
    },

    #[error("[{severity}] State validation failed: {error}")]
    StateValidation {
        error: StateValidationError,
        severity: ErrorSeverity,
        context: ErrorContext,
        recovery_suggestion: RecoverySuggestion,
    },

    #[error("[{severity}] Unknown error: {message}")]
    Unknown {
        message: String,
        severity: ErrorSeverity,
        context: ErrorContext,
        recovery_suggestion: RecoverySuggestion,
    },
}

impl RuntimeError {
    /// Get the error category
    pub fn category(&self) -> ErrorCategory {
        match self {
            RuntimeError::Protocol { .. } => ErrorCategory::Protocol,
            RuntimeError::Communication { .. } => ErrorCategory::Communication,
            RuntimeError::Serialization { .. } => ErrorCategory::Serialization,
            RuntimeError::Timeout { .. } => ErrorCategory::Timeout,
            RuntimeError::Configuration { .. } => ErrorCategory::Configuration,
            RuntimeError::System { .. } => ErrorCategory::System,
            RuntimeError::Execution { .. } => ErrorCategory::Execution,
            RuntimeError::SessionAlreadyExists { .. } => ErrorCategory::Session,
            RuntimeError::Deadlock { .. } => ErrorCategory::Protocol,
            RuntimeError::Livelock { .. } => ErrorCategory::Protocol,
            RuntimeError::StateValidation { .. } => ErrorCategory::Validation,
            RuntimeError::Unknown { .. } => ErrorCategory::Unknown,
        }
    }

    /// Get the error severity
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            RuntimeError::Protocol { severity, .. } => *severity,
            RuntimeError::Communication { severity, .. } => *severity,
            RuntimeError::Serialization { severity, .. } => *severity,
            RuntimeError::Timeout { severity, .. } => *severity,
            RuntimeError::Configuration { severity, .. } => *severity,
            RuntimeError::System { severity, .. } => *severity,
            RuntimeError::Execution { severity, .. } => *severity,
            RuntimeError::SessionAlreadyExists { severity, .. } => *severity,
            RuntimeError::Deadlock { severity, .. } => *severity,
            RuntimeError::Livelock { severity, .. } => *severity,
            RuntimeError::StateValidation { severity, .. } => *severity,
            RuntimeError::Unknown { severity, .. } => *severity,
        }
    }

    /// Get the error context
    pub fn context(&self) -> &ErrorContext {
        match self {
            RuntimeError::Protocol { context, .. } => context,
            RuntimeError::Communication { context, .. } => context,
            RuntimeError::Serialization { context, .. } => context,
            RuntimeError::Timeout { context, .. } => context,
            RuntimeError::Configuration { context, .. } => context,
            RuntimeError::System { context, .. } => context,
            RuntimeError::Execution { context, .. } => context,
            RuntimeError::SessionAlreadyExists { context, .. } => context,
            RuntimeError::Deadlock { context, .. } => context,
            RuntimeError::Livelock { context, .. } => context,
            RuntimeError::StateValidation { context, .. } => context,
            RuntimeError::Unknown { context, .. } => context,
        }
    }

    /// Get the recovery suggestion
    pub fn recovery_suggestion(&self) -> &RecoverySuggestion {
        match self {
            RuntimeError::Protocol {
                recovery_suggestion,
                ..
            } => recovery_suggestion,
            RuntimeError::Communication {
                recovery_suggestion,
                ..
            } => recovery_suggestion,
            RuntimeError::Serialization {
                recovery_suggestion,
                ..
            } => recovery_suggestion,
            RuntimeError::Timeout {
                recovery_suggestion,
                ..
            } => recovery_suggestion,
            RuntimeError::Configuration {
                recovery_suggestion,
                ..
            } => recovery_suggestion,
            RuntimeError::System {
                recovery_suggestion,
                ..
            } => recovery_suggestion,
            RuntimeError::Execution {
                recovery_suggestion,
                ..
            } => recovery_suggestion,
            RuntimeError::SessionAlreadyExists {
                recovery_suggestion,
                ..
            } => recovery_suggestion,
            RuntimeError::Deadlock {
                recovery_suggestion,
                ..
            } => recovery_suggestion,
            RuntimeError::Livelock {
                recovery_suggestion,
                ..
            } => recovery_suggestion,
            RuntimeError::StateValidation {
                recovery_suggestion,
                ..
            } => recovery_suggestion,
            RuntimeError::Unknown {
                recovery_suggestion,
                ..
            } => recovery_suggestion,
        }
    }

    /// Create a formatted diagnostic report
    pub fn diagnostic_report(&self) -> String {
        let mut report = String::new();
        report.push_str("Runtime Error Report\n");
        report.push_str("====================\n\n");
        report.push_str(&format!("Category: {}\n", self.category()));
        report.push_str(&format!("Severity: {}\n", self.severity()));
        report.push_str(&format!("Error: {}\n\n", self));

        let context = self.context();
        if let Some(session_id) = &context.session_id {
            report.push_str(&format!("Session ID: {}\n", session_id));
        }
        if let Some(component) = &context.component {
            report.push_str(&format!("Component: {}\n", component));
        }
        if let Some(operation) = &context.operation {
            report.push_str(&format!("Operation: {}\n", operation));
        }

        if !context.metadata.is_empty() {
            report.push_str("\nMetadata:\n");
            for (key, value) in &context.metadata {
                report.push_str(&format!("  {}: {}\n", key, value));
            }
        }

        report.push_str(&format!(
            "\nRecovery Suggestion: {}\n",
            self.recovery_suggestion()
        ));

        if let Ok(elapsed) = context.timestamp.elapsed() {
            report.push_str(&format!("Time since error: {:?}\n", elapsed));
        }

        report
    }
}

impl From<ProtocolViolation> for RuntimeError {
    fn from(violation: ProtocolViolation) -> Self {
        let severity = match violation {
            ProtocolViolation::SessionTerminated { .. } => ErrorSeverity::Critical,
            ProtocolViolation::Deadlock { .. } => ErrorSeverity::Critical,
            ProtocolViolation::InvalidTransition { .. } => ErrorSeverity::High,
            ProtocolViolation::UnexpectedMessage { .. } => ErrorSeverity::High,
            ProtocolViolation::RecursionDepthExceeded { .. } => ErrorSeverity::High,
            _ => ErrorSeverity::Medium,
        };

        let recovery_suggestion = match violation {
            ProtocolViolation::SessionTerminated { .. } => RecoverySuggestion::RestartSession,
            ProtocolViolation::Deadlock { .. } => RecoverySuggestion::RestartSession,
            ProtocolViolation::InvalidTransition { .. } => RecoverySuggestion::CheckConfiguration,
            ProtocolViolation::UnexpectedMessage { .. } => RecoverySuggestion::Retry,
            ProtocolViolation::IncompleteProtocol { .. } => {
                RecoverySuggestion::Custom("Complete the protocol execution".to_string())
            }
            _ => RecoverySuggestion::Retry,
        };

        RuntimeError::Protocol {
            violation,
            severity,
            context: ErrorContext::new().with_component("protocol"),
            recovery_suggestion,
        }
    }
}

impl From<CommunicationError> for RuntimeError {
    fn from(error: CommunicationError) -> Self {
        let severity = match error {
            CommunicationError::ChannelClosed => ErrorSeverity::High,
            CommunicationError::ConnectionLost { .. } => ErrorSeverity::High,
            CommunicationError::AuthenticationFailed { .. } => ErrorSeverity::Critical,
            CommunicationError::ChannelTimeout { .. } => ErrorSeverity::Medium,
            _ => ErrorSeverity::Medium,
        };

        let recovery_suggestion = match error {
            CommunicationError::ChannelClosed => RecoverySuggestion::RestartSession,
            CommunicationError::ConnectionLost { .. } => RecoverySuggestion::CheckNetwork,
            CommunicationError::ConnectionRefused { .. } => RecoverySuggestion::CheckNetwork,
            CommunicationError::AuthenticationFailed { .. } => {
                RecoverySuggestion::CheckConfiguration
            }
            CommunicationError::ChannelTimeout { .. } => RecoverySuggestion::RetryWithBackoff,
            _ => RecoverySuggestion::Retry,
        };

        RuntimeError::Communication {
            error,
            severity,
            context: ErrorContext::new().with_component("communication"),
            recovery_suggestion,
        }
    }
}

impl From<DeadlockError> for RuntimeError {
    fn from(error: DeadlockError) -> Self {
        RuntimeError::Deadlock {
            error,
            severity: ErrorSeverity::Critical,
            context: ErrorContext::new().with_component("deadlock_detector"),
            recovery_suggestion: RecoverySuggestion::RestartSession,
        }
    }
}

impl From<LivelockError> for RuntimeError {
    fn from(error: LivelockError) -> Self {
        RuntimeError::Livelock {
            error,
            severity: ErrorSeverity::High,
            context: ErrorContext::new().with_component("livelock_detector"),
            recovery_suggestion: RecoverySuggestion::RestartSession,
        }
    }
}

impl From<StateValidationError> for RuntimeError {
    fn from(error: StateValidationError) -> Self {
        let severity = match error {
            StateValidationError::ProtocolSpecViolation { .. } => ErrorSeverity::High,
            StateValidationError::RoleValidationFailed { .. } => ErrorSeverity::High,
            StateValidationError::ProtocolConsistency { .. } => ErrorSeverity::Medium,
            _ => ErrorSeverity::Medium,
        };

        RuntimeError::StateValidation {
            error,
            severity,
            context: ErrorContext::new().with_component("state_validator"),
            recovery_suggestion: RecoverySuggestion::CheckConfiguration,
        }
    }
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

    #[error(
        "Unexpected message type: expected '{expected}', got '{actual}' in state '{current_state}'"
    )]
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
    ChoiceNotAvailable {
        choice: String,
        current_state: String,
    },

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
    #[error(
        "Channel {operation} failed on channel '{channel_id}' in session '{session_id}': {details}"
    )]
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

    #[error(
        "Protocol deadlock in session '{session_id}': roles {roles:?} are in mutual wait state"
    )]
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

    #[error(
        "Protocol livelock detected: roles are active but making no progress towards completion"
    )]
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
#[derive(Default)]
pub enum ValidationMode {
    /// Strict validation - all protocol rules must be followed exactly
    Strict,
    /// Lenient validation - some minor violations may be warnings
    Lenient,
    /// Debug validation - extensive checking with detailed reporting
    Debug,
    /// Production validation - optimized for performance with essential checks
    #[default]
    Production,
}


/// Convenience type alias for runtime results
/// Uses boxed RuntimeError to reduce stack size of large error variants
pub type RuntimeResult<T> = Result<T, Box<RuntimeError>>;

/// Helper function to box RuntimeError for easier error handling
pub fn runtime_error(error: RuntimeError) -> Box<RuntimeError> {
    Box::new(error)
}

/// Extension trait for converting standard errors to runtime errors
pub trait IntoRuntimeError {
    fn into_runtime_error(self) -> RuntimeError;
}

impl IntoRuntimeError for std::io::Error {
    fn into_runtime_error(self) -> RuntimeError {
        RuntimeError::System {
            message: self.to_string(),
            severity: ErrorSeverity::Medium,
            context: ErrorContext::new().with_component("io"),
            recovery_suggestion: RecoverySuggestion::Retry,
        }
    }
}

impl IntoRuntimeError for serde_json::Error {
    fn into_runtime_error(self) -> RuntimeError {
        RuntimeError::Serialization {
            message: self.to_string(),
            severity: ErrorSeverity::Medium,
            context: ErrorContext::new().with_component("serde_json"),
            recovery_suggestion: RecoverySuggestion::Retry,
            source: Some(Box::new(self)),
        }
    }
}

impl IntoRuntimeError for tokio::time::error::Elapsed {
    fn into_runtime_error(self) -> RuntimeError {
        RuntimeError::Timeout {
            duration_ms: 0,
            severity: ErrorSeverity::Medium,
            context: ErrorContext::new().with_component("tokio_timeout"),
            recovery_suggestion: RecoverySuggestion::RetryWithBackoff,
        }
    }
}

/// Create a protocol violation error
pub fn protocol_violation(violation: ProtocolViolation) -> RuntimeError {
    RuntimeError::from(violation)
}

/// Create a communication error
pub fn communication_error(error: CommunicationError) -> RuntimeError {
    RuntimeError::from(error)
}

/// Create a configuration error
pub fn configuration_error(message: impl Into<String>) -> RuntimeError {
    RuntimeError::Configuration {
        message: message.into(),
        severity: ErrorSeverity::Medium,
        context: ErrorContext::new().with_component("configuration"),
        recovery_suggestion: RecoverySuggestion::CheckConfiguration,
    }
}

/// Create a configuration error with custom context
pub fn configuration_error_with_context(
    message: impl Into<String>,
    context: ErrorContext,
    severity: ErrorSeverity,
) -> RuntimeError {
    RuntimeError::Configuration {
        message: message.into(),
        severity,
        context,
        recovery_suggestion: RecoverySuggestion::CheckConfiguration,
    }
}

/// Create a system error
pub fn system_error(message: impl Into<String>) -> RuntimeError {
    RuntimeError::System {
        message: message.into(),
        severity: ErrorSeverity::Medium,
        context: ErrorContext::new().with_component("system"),
        recovery_suggestion: RecoverySuggestion::Terminate,
    }
}

/// Create a system error with custom context
pub fn system_error_with_context(
    message: impl Into<String>,
    context: ErrorContext,
    severity: ErrorSeverity,
) -> RuntimeError {
    RuntimeError::System {
        message: message.into(),
        severity,
        context,
        recovery_suggestion: RecoverySuggestion::Terminate,
    }
}

/// Create a timeout error
pub fn timeout_error(duration_ms: u64) -> RuntimeError {
    RuntimeError::Timeout {
        duration_ms,
        severity: ErrorSeverity::Medium,
        context: ErrorContext::new().with_component("timeout"),
        recovery_suggestion: RecoverySuggestion::RetryWithBackoff,
    }
}

/// Create a timeout error with custom context
pub fn timeout_error_with_context(
    duration_ms: u64,
    context: ErrorContext,
    severity: ErrorSeverity,
) -> RuntimeError {
    RuntimeError::Timeout {
        duration_ms,
        severity,
        context,
        recovery_suggestion: RecoverySuggestion::RetryWithBackoff,
    }
}

/// Create a serialization error
pub fn serialization_error(
    message: impl Into<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
) -> RuntimeError {
    RuntimeError::Serialization {
        message: message.into(),
        severity: ErrorSeverity::Medium,
        context: ErrorContext::new().with_component("serialization"),
        recovery_suggestion: RecoverySuggestion::Retry,
        source,
    }
}

/// Create a serialization error with custom context
pub fn serialization_error_with_context(
    message: impl Into<String>,
    context: ErrorContext,
    severity: ErrorSeverity,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
) -> RuntimeError {
    RuntimeError::Serialization {
        message: message.into(),
        severity,
        context,
        recovery_suggestion: RecoverySuggestion::Retry,
        source,
    }
}

/// Create an execution error
pub fn execution_error(message: impl Into<String>) -> RuntimeError {
    RuntimeError::Execution {
        message: message.into(),
        severity: ErrorSeverity::Medium,
        context: ErrorContext::new().with_component("execution"),
        recovery_suggestion: RecoverySuggestion::Retry,
    }
}

/// Create a session already exists error
pub fn session_already_exists_error(session_id: impl Into<String>) -> RuntimeError {
    RuntimeError::SessionAlreadyExists {
        session_id: session_id.into(),
        severity: ErrorSeverity::Medium,
        context: ErrorContext::new().with_component("session_manager"),
        recovery_suggestion: RecoverySuggestion::Custom("Use a different session ID".to_string()),
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
        let runtime_error = RuntimeError::from(violation);
        assert!(matches!(runtime_error, RuntimeError::Protocol { .. }));
        assert_eq!(runtime_error.severity(), ErrorSeverity::Critical);
        assert_eq!(runtime_error.category(), ErrorCategory::Protocol);
    }

    #[test]
    fn test_error_convenience_functions() {
        let error = configuration_error("Invalid timeout");
        assert!(matches!(error, RuntimeError::Configuration { .. }));
        assert_eq!(error.severity(), ErrorSeverity::Medium);

        let error = system_error("File not found");
        assert!(matches!(error, RuntimeError::System { .. }));
        assert_eq!(error.severity(), ErrorSeverity::Medium);
    }

    #[test]
    fn test_error_context_builder() {
        let context = ErrorContext::new()
            .with_session_id("test-session")
            .with_component("test-component")
            .with_operation("test-operation")
            .with_metadata("key1", "value1")
            .with_metadata("key2", "value2");

        assert_eq!(context.session_id, Some("test-session".to_string()));
        assert_eq!(context.component, Some("test-component".to_string()));
        assert_eq!(context.operation, Some("test-operation".to_string()));
        assert_eq!(context.metadata.get("key1"), Some(&"value1".to_string()));
        assert_eq!(context.metadata.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_error_severity_ordering() {
        assert!(ErrorSeverity::Low < ErrorSeverity::Medium);
        assert!(ErrorSeverity::Medium < ErrorSeverity::High);
        assert!(ErrorSeverity::High < ErrorSeverity::Critical);
    }

    #[test]
    fn test_error_display_formatting() {
        let error = RuntimeError::Protocol {
            violation: ProtocolViolation::InvalidTransition {
                current_state: "Ready".to_string(),
                action_taken: "ProcessData".to_string(),
                expected_actions_or_states: "ReceiveAck or Timeout".to_string(),
            },
            severity: ErrorSeverity::High,
            context: ErrorContext::new().with_session_id("test-session"),
            recovery_suggestion: RecoverySuggestion::CheckConfiguration,
        };

        let display_str = format!("{}", error);
        assert!(display_str.contains("[HIGH]"));
        assert!(display_str.contains("Protocol violation"));
        assert!(display_str.contains("Invalid state transition"));
    }

    #[test]
    fn test_diagnostic_report() {
        let error = RuntimeError::Communication {
            error: CommunicationError::ChannelClosed,
            severity: ErrorSeverity::High,
            context: ErrorContext::new()
                .with_session_id("test-session")
                .with_component("channel")
                .with_operation("send")
                .with_metadata("channel_id", "ch1"),
            recovery_suggestion: RecoverySuggestion::RestartSession,
        };

        let report = error.diagnostic_report();
        assert!(report.contains("Runtime Error Report"));
        assert!(report.contains("Category: Communication"));
        assert!(report.contains("Severity: HIGH"));
        assert!(report.contains("Session ID: test-session"));
        assert!(report.contains("Component: channel"));
        assert!(report.contains("Operation: send"));
        assert!(report.contains("channel_id: ch1"));
        assert!(report.contains("Recovery Suggestion: Restart the session"));
    }

    #[test]
    fn test_recovery_suggestion_display() {
        assert_eq!(RecoverySuggestion::Retry.to_string(), "Retry the operation");
        assert_eq!(
            RecoverySuggestion::RetryWithBackoff.to_string(),
            "Retry with exponential backoff"
        );
        assert_eq!(
            RecoverySuggestion::RestartSession.to_string(),
            "Restart the session or connection"
        );
        assert_eq!(
            RecoverySuggestion::Custom("Custom action".to_string()).to_string(),
            "Custom action"
        );
    }

    #[test]
    fn test_enhanced_error_creation_functions() {
        let context = ErrorContext::new().with_session_id("test");

        let timeout_error = timeout_error_with_context(5000, context.clone(), ErrorSeverity::High);
        assert_eq!(timeout_error.severity(), ErrorSeverity::High);
        assert_eq!(timeout_error.context().session_id, Some("test".to_string()));

        let config_error = configuration_error_with_context(
            "Bad config",
            context.clone(),
            ErrorSeverity::Critical,
        );
        assert_eq!(config_error.severity(), ErrorSeverity::Critical);

        let serialization_error = serialization_error_with_context(
            "Failed to serialize",
            context.clone(),
            ErrorSeverity::Medium,
            None,
        );
        assert_eq!(serialization_error.severity(), ErrorSeverity::Medium);
    }

    #[test]
    fn test_error_chaining() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let runtime_error = io_error.into_runtime_error();

        assert!(matches!(runtime_error, RuntimeError::System { .. }));
        assert_eq!(runtime_error.category(), ErrorCategory::System);
    }
}
