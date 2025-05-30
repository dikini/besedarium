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

pub mod state;
pub mod channel;
pub mod error;
pub mod session;
pub mod validation;

// Re-export common types for convenience
pub use state::{ProtocolState, ExecutionContext, StateTransition};
pub use channel::{TypedChannel, ChannelConfig};
pub use error::{RuntimeError, RuntimeResult, ProtocolViolation, CommunicationError, 
               DeadlockError, LivelockError, StateValidationError, ValidationMode};
// pub use session::{Session, SessionConfig, SessionManager}; // TODO: Implement session module
pub use validation::{StateValidator, ValidationConfig, ValidationResult};
