//! # Local Endpoint Types for Enhanced MPST System
//!
//! This module provides Local Endpoint Types that represent the projected view
//! of protocols from individual role perspectives. These types use extensible
//! metadata patterns to enable downstream implementations to enhance metadata
//! while maintaining type safety and specification compliance.
//!
//! ## Module Navigation
//!
//! **Core Protocol Framework:**
//! - [`crate::protocol::foundation`] - Protocol foundation types and traits
//! - [`crate::protocol::global`] - Global protocol types for choreography specification
//! - [`crate::protocol::projection`] - Global-to-local protocol projection algorithms
//! - [`crate::protocol::duality`] - Protocol duality verification and dual generation
//!
//! **Macro System:**
//! - [`crate::macros`] - Protocol construction macros and endpoint derivation helpers
//!
//! ## Key Design Features
//!
//! - **Extensible Metadata**: Uses `CommMetadataTrait` to allow downstream extensions
//! - **IO Capability Integration**: All endpoints verify required I/O capabilities  
//! - **Type Safety**: Compile-time verification of channel and message compatibility
//! - **Specification Compliance**: Follows the patterns established in `docs/duality.md`
//!
//! ## Core Local Endpoint Types
//!
//! - **EpChanSend/EpChanRecv**: Local endpoint message sending and receiving
//! - **EpChanChoice/EpChanOffer**: Local endpoint protocol branching and choice handling
//! - **EpChanPar**: Local endpoint parallel composition
//! - **EpChanEnd/EpChanStart**: Local endpoint protocol lifecycle management
//! - **Type Aliases**: Convenience types for common patterns
//!
//! ## Integration Test Examples
//!
//! Working examples of local endpoint usage and projection can be found in:
//! - [`tests/client_server_integration.rs`] - Complete local endpoint implementations
//! - [`tests/integration_common.rs`] - Shared endpoint patterns and utilities
//!
//! Key test functions demonstrating local endpoint usage:
//! - `test_client_endpoint_projection()` - Client-side endpoint construction and execution
//! - `test_server_endpoint_projection()` - Server-side endpoint construction and execution
//! - `test_endpoint_duality_verification()` - Verifying dual endpoint compatibility
//! - `test_parallel_endpoint_composition()` - Parallel endpoint execution patterns
//!
//! ## Quick Start Example
//!
//! ```rust
//! # use besedarium::protocol::local::*;
//! # use besedarium::protocol::foundation::*;
//! #
//! # // Define example IO capability types
//! # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
//! # struct SendCapability;
//! # impl SupportsActionIO<BiDirectionalAction> for SendCapability {}
//! #
//! # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
//! # struct ReceiveCapability;
//! # impl SupportsActionIO<BiDirectionalAction> for ReceiveCapability {}
//! #
//! # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
//! # struct EndCapability;
//! # impl SupportsActionIO<BiDirectionalAction> for EndCapability {}
//! #
//! // Simple client endpoint: send request, receive response
//! type ClientEndpoint = SimpleEpSend<
//!     SendCapability,   // IO capability
//!     String,          // Request message type
//!     SimpleEpRecv<ReceiveCapability, i32, SimpleEpEnd<EndCapability>>
//! >;
//! ```

mod endpoints;
mod implementations;

// Re-export all endpoint types and make implementations available
pub use endpoints::{
    EpChanChoice, EpChanEnd, EpChanOffer, EpChanPar, EpChanRecv, EpChanSend, EpChanStart,
};
// Implementations are included through module system (impl blocks can't be re-exported)

use crate::protocol::foundation::{
    BiDirectionalAction, CommMetadata, DefaultChan, RequestLbl, ResponseLbl,
};

// ============================================================================
// Type Aliases for Common Patterns
// ============================================================================

/// Convenience type alias for simple send with default CommMetadata
pub type SimpleEpSend<IO, Msg, P> =
    EpChanSend<IO, CommMetadata<DefaultChan, RequestLbl>, Msg, P, BiDirectionalAction>;

/// Convenience type alias for simple receive with default CommMetadata
pub type SimpleEpRecv<IO, Msg, P> =
    EpChanRecv<IO, CommMetadata<DefaultChan, ResponseLbl>, Msg, P, BiDirectionalAction>;

/// Convenience type alias for simple offer with default CommMetadata
pub type SimpleEpOffer<IO, Left, Right> =
    EpChanOffer<IO, CommMetadata<DefaultChan, RequestLbl>, Left, Right, BiDirectionalAction>;

/// Convenience type alias for simple choice with default CommMetadata
pub type SimpleEpChoice<IO, Left, Right> =
    EpChanChoice<IO, CommMetadata<DefaultChan, RequestLbl>, Left, Right, BiDirectionalAction>;

/// Convenience type alias for simple parallel with default CommMetadata
pub type SimpleEpPar<IO, Left, Right, IsDisjoint> = EpChanPar<
    IO,
    CommMetadata<DefaultChan, RequestLbl>,
    Left,
    Right,
    IsDisjoint,
    BiDirectionalAction,
>;

/// Convenience type alias for simple termination with default CommMetadata
pub type SimpleEpEnd<IO> =
    EpChanEnd<IO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;

/// Convenience type alias for simple start with default CommMetadata
pub type SimpleEpStart<IO, Start> =
    EpChanStart<IO, CommMetadata<DefaultChan, RequestLbl>, Start, BiDirectionalAction>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests;
