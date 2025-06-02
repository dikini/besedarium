//! # Global Protocol Types for Enhanced MPST System
//!
//! This module provides Global Protocol Types that incorporate `CommMetadata` and
//! `ActionIOType` concepts as specified in `docs/duality.md`. These types represent
//! the choreography of multi-party protocols with explicit channel management.
//!
//! ## Module Navigation
//!
//! **Core Protocol Framework:**
//! - [`crate::protocol::foundation`] - Protocol foundation types and traits
//! - [`crate::protocol::local`] - Local endpoint types for protocol participants
//! - [`crate::protocol::projection`] - Global-to-local protocol projection
//! - [`crate::protocol::duality`] - Protocol duality verification and dual generation
//!
//! **Macro System:**
//! - [`crate::macros`] - Protocol construction macros and derivation helpers
//!
//! ## Key Components
//!
//! - **TChanSend/TChanRecv**: Channel-based message sending and receiving
//! - **TChanChoice**: Protocol branching and choice constructions
//! - **TChanPar**: Parallel composition of protocol branches
//! - **TChanEnd/TChanStart**: Protocol lifecycle management
//! - **Type Aliases**: Convenience types for common patterns
//! - **Builder Functions**: Easy construction of complex protocols
//!
//! ## Integration Test Examples
//!
//! Working examples of global protocol construction and usage can be found in:
//! - [`tests/client_server_integration.rs`] - Complete client-server protocol implementations
//! - [`tests/integration_common.rs`] - Shared protocol patterns and test utilities
//!
//! Key test functions demonstrating global protocol usage:
//! - `test_simple_client_server_protocol()` - Basic global protocol construction
//! - `test_choice_protocol()` - Branching and choice in global protocols
//! - `test_parallel_composition()` - Parallel protocol construction patterns
//!
//! ## Quick Start Example
//!
//! ```rust
//! # use besedarium::protocol::global::*;
//! # use besedarium::protocol::foundation::*;
//! # use besedarium::define_role;
//! #
//! # // Define example roles
//! # define_role!(RoleA);
//! # define_role!(RoleB);
//! #
//! // Simple request-response global protocol
//! type GlobalProtocol = SimpleChannelSend<
//!     RoleA,        // Sender role
//!     RoleB,        // Receiver role  
//!     String,       // Message type
//!     SimpleChannelRecv<RoleB, RoleA, i32, SimpleChannelEnd>
//! >;
//! ```

mod implementations;
mod protocols;

// Re-export all protocol types and make implementations available
pub use protocols::{
    TChanChoice, TChanEnd, TChanOffer, TChanPar, TChanRecv, TChanSend, TChanStart,
};
// Implementations are included through module system (impl blocks can't be re-exported)

use crate::protocol::foundation::{BiDirectionalAction, DefaultChan, RequestLbl, ResponseLbl};

// ============================================================================
// Type Aliases for Common Patterns
// ============================================================================

/// Convenience type alias for simple send with default channel
pub type SimpleChannelSend<S, R, Msg, P> =
    TChanSend<S, R, DefaultChan, RequestLbl, Msg, P, BiDirectionalAction>;

/// Convenience type alias for simple receive with default channel  
pub type SimpleChannelRecv<R, S, Msg, P> =
    TChanRecv<R, S, DefaultChan, ResponseLbl, Msg, P, BiDirectionalAction>;

/// Convenience type alias for simple choice with default channel
pub type SimpleChannelChoice<R, Left, Right> =
    TChanChoice<R, DefaultChan, RequestLbl, Left, Right, BiDirectionalAction>;

/// Convenience type alias for simple offer with default channel
pub type SimpleChannelOffer<R, Left, Right> =
    TChanOffer<R, DefaultChan, RequestLbl, Left, Right, BiDirectionalAction>;

/// Convenience type alias for simple parallel composition with default channel  
pub type SimpleChannelPar<Left, Right, IsDisjoint> =
    TChanPar<DefaultChan, RequestLbl, Left, Right, IsDisjoint, BiDirectionalAction>;

/// Convenience type alias for simple termination with default channel
pub type SimpleChannelEnd = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;

/// Convenience type alias for simple start with default channel
pub type SimpleChannelStart<Start> =
    TChanStart<DefaultChan, RequestLbl, Start, BiDirectionalAction>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests;
