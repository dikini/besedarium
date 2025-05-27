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
//!
//! # Usage Example
//!
//! ```rust
//! use besedarium::runtime::{ProtocolState, TypedChannel, RuntimeError};
//! use besedarium::protocol::foundation::CommMetadata;
//!
//! // Create a protocol state machine
//! let state = ProtocolState::new("session-1", Box::new(Alice));
//!
//! // Create typed channels for communication
//! let metadata = CommMetadata::new();
//! let (ch1, ch2) = TypedChannel::new(metadata);
//! ```

pub mod state;
pub mod channel;
pub mod error;
pub mod session;

// Re-export main types
pub use state::{ProtocolState, StateTransition, ExecutionContext};
pub use channel::{TypedChannel, ChannelMessage, ChannelConfig};
pub use error::{RuntimeError, ProtocolViolation, CommunicationError};
pub use session::{Session, SessionManager};
