//! # Global Protocol Types for Enhanced MPST System
//!
//! This module provides Global Protocol Types that incorporate `CommMetadata` and
//! `ActionIOType` concepts as specified in `docs/duality.md`. These types represent
//! the choreography of multi-party protocols with explicit channel management.
//!
//! ## Key Components
//!
//! - **TChanSend/TChanRecv**: Channel-based message sending and receiving
//! - **TChanChoice**: Protocol branching and choice constructions
//! - **TChanPar**: Parallel composition of protocol branches
//! - **TChanEnd/TChanStart**: Protocol lifecycle management
//! - **Type Aliases**: Convenience types for common patterns
//! - **Builder Functions**: Easy construction of complex protocols

mod protocols;
mod implementations;

// Re-export all protocol types and make implementations available
pub use protocols::{
    TChanSend, TChanRecv, TChanChoice, TChanOffer, 
    TChanPar, TChanEnd, TChanStart
};
// Implementations are included through module system (impl blocks can't be re-exported)

use crate::protocol::foundation::{
    BiDirectionalAction, DefaultChan, RequestLbl, ResponseLbl,
};

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
