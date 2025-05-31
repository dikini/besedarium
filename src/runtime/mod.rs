//! Runtime components for session type execution
//!
//! This module provides the runtime infrastructure for executing session-typed
//! protocols, including state management, communication, and error handling.
//!
//! # Core Components
//!
//! - [`state`] - Protocol state machine for tracking execution progress
//! - [`channel`] - Typed channel communication with async support
//! - [`error`] - Comprehensive error types for runtime failures
//! - [`session`] - Session lifecycle management for protocols
//! - [`validation`] - State validation and consistency checks

pub mod channel;
pub mod error;
pub mod session;
pub mod state;
pub mod validation;

// Re-export common types for convenience
pub use channel::{ChannelConfig, TypedChannel};
pub use error::{
    CommunicationError, DeadlockError, LivelockError, ProtocolViolation, RuntimeError,
    RuntimeResult, StateValidationError, ValidationMode,
};
pub use state::{ExecutionContext, ProtocolState, StateTransition};
pub use session::{Session, SessionConfig, SessionManager};
pub use validation::{StateValidator, ValidationConfig, ValidationResult};
