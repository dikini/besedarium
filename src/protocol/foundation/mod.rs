//! # Foundation Types for Enhanced MPST System
//!
//! This module provides the foundational trait definitions and core infrastructure 
//! for the Besedarium MPST library as specified in `docs/duality.md`.
//!
//! ## Key Components
//!
//! - **Foundation Traits**: Basic traits for roles, messages, and protocol types
//! - **CommMetadata**: Communication metadata for precise channel and message identification
//! - **Channel and Message Labels**: Type-safe identifiers for communication channels and messages
//! - **Action I/O Types**: Markers for different I/O capabilities (Input, Output, BiDirectional)
//! - **SupportsActionIO**: Trait for verifying I/O capability compatibility

use std::fmt::Debug;
use std::hash::Hash;

// ============================================================================
// Task 1.1.1a: Foundation Trait Definitions
// ============================================================================

/// Fundamental trait for role identification in protocols
pub trait Role: Send + Sync + 'static + Debug + Clone + PartialEq + Eq + Hash {}

/// Trait for messages that can be exchanged in protocols  
pub trait Message: Send + Sync + 'static + Debug + Clone {}

/// Marker trait for Global Protocol types
pub trait GlobalProtocol: Send + Sync + 'static + Debug {}

/// Marker trait for Local Endpoint Protocol types  
pub trait LocalProtocol: Send + Sync + 'static + Debug {}

// ============================================================================
// Task 1.1.1c: Channel and Message Label Traits
// ============================================================================

/// Trait for channel identifiers
pub trait ChanId: Send + Sync + 'static + Debug + Clone + PartialEq + Eq + Hash {}

/// Trait for message labels within channels  
pub trait MsgLbl: Send + Sync + 'static + Debug + Clone + PartialEq + Eq + Hash {}

// Example concrete channel types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct DefaultChan;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct HandshakeChan;

impl ChanId for DefaultChan {}
impl ChanId for HandshakeChan {}

// Example concrete message label types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct RequestLbl;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ResponseLbl;

impl MsgLbl for RequestLbl {}
impl MsgLbl for ResponseLbl {}

// ============================================================================
// Task 1.1.1b: CommMetadata Implementation
// ============================================================================

/// Communication metadata for precise channel and message identification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommMetadata<C: ChanId, L: MsgLbl> {
    pub chan_id: C,
    pub msg_lbl: L,
}

impl<C: ChanId, L: MsgLbl> CommMetadata<C, L> {
    pub fn new(chan_id: C, msg_lbl: L) -> Self {
        Self { chan_id, msg_lbl }
    }
}

// ============================================================================
// Task 1.1.1d: ActionIOTMarker System
// ============================================================================

/// Marker trait for Action I/O Types - what I/O capability an action requires
pub trait ActionIOTMarker: Send + Sync + 'static + Debug + Clone + PartialEq + Eq {}

/// Standard Action I/O Types
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputAction;

#[derive(Debug, Clone, PartialEq, Eq)] 
pub struct OutputAction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BiDirectionalAction;

impl ActionIOTMarker for InputAction {}
impl ActionIOTMarker for OutputAction {}
impl ActionIOTMarker for BiDirectionalAction {}

// ============================================================================
// Task 1.1.1e: SupportsActionIO Trait
// ============================================================================

/// Trait to verify IO capability compatibility
pub trait SupportsActionIO<AIO: ActionIOTMarker> {
    /// Returns true if this IO capability can handle the specified action type
    fn supports_action_io() -> bool {
        true // Default implementation assumes support
    }
}

// Example implementation: TCP-based session I/O that supports all actions
#[derive(Debug)]
pub struct TcpOnlySessionIO;

impl SupportsActionIO<InputAction> for TcpOnlySessionIO {}
impl SupportsActionIO<OutputAction> for TcpOnlySessionIO {}
impl SupportsActionIO<BiDirectionalAction> for TcpOnlySessionIO {}

// Example implementation: HTTP-based session I/O that only supports output and bidirectional
#[derive(Debug)]
pub struct HttpOnlySessionIO;

impl SupportsActionIO<OutputAction> for HttpOnlySessionIO {}
impl SupportsActionIO<BiDirectionalAction> for HttpOnlySessionIO {}
// Note: HttpOnlySessionIO doesn't support InputAction

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests;
