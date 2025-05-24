//! # Protocol Projection Module
//!
//! This module implements the `Project<P, R>` trait for projecting Global Protocols
//! to Local Endpoint Types. The implementation follows the foundation established
//! in Tasks 1.1.1-1.1.4 and provides compile-time projection with role-based dispatch.
//!
//! ## Core Concepts
//!
//! - **Project trait**: Maps global protocols to local endpoint types for specific roles
//! - **Role-based projection**: Different roles see different views of the same protocol
//! - **Type-level validation**: Compile-time verification of projection correctness
//! - **Helper traits**: Modular dispatch system for complex projection cases
//!
//! ## Usage
//!
//! ```rust
//! use besedarium::protocol::projection::{Project, ProjectOutput};
//! use besedarium::protocol::foundation::*;
//! use besedarium::protocol::global::*;
//! use besedarium::protocol::local::*;
//!
//! // Example: Alice sends to Bob, project to Alice's view  
//! # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
//! # struct Alice;
//! # impl Role for Alice {}
//! # impl SupportsActionIO<BiDirectionalAction> for Alice {}
//! # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
//! # struct Bob;
//! # impl Role for Bob {}
//! # #[derive(Debug, Clone)]
//! # struct HelloMsg;
//! # impl Message for HelloMsg {}
//! type SendProto = TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>, BiDirectionalAction>;
//! type AliceView = ProjectOutput<SendProto, Alice>;  // Results in EpChanSend
//! ```

use crate::protocol::foundation::{GlobalProtocol, LocalProtocol, Role};

// ============================================================================
// Module Structure
// ============================================================================

pub mod errors;
pub mod helpers;
pub mod implementations;

// Re-export essential types for external use
pub use errors::{DefaultProjectionValidator, ProjectionError, ProjectionValidator, ValidateProjection};
pub use helpers::{Bool, False, ProjectRecvCase, ProjectSendCase, RoleEq, True};

// ============================================================================
// Core Projection Trait
// ============================================================================

// ============================================================================
// Core Projection Trait
// ============================================================================

/// Core projection trait for mapping global protocols to local endpoint types
///
/// Projects a Global Protocol `P` to a Local Endpoint Type for role `R`.
/// This trait provides compile-time protocol projection with validation.
pub trait Project<P, R>
where
    P: GlobalProtocol,
    R: Role,
{
    /// The resulting local protocol for role R
    type Output: LocalProtocol;
}

/// Type alias for cleaner projection usage
pub type ProjectOutput<P, R> = <() as Project<P, R>>::Output;

// ============================================================================
// Non-Reflexive RoleEq Implementations (Required for Boolean Logic)
// ============================================================================

// Note: Non-reflexive RoleEq implementations need to be added for specific
// role pairs as needed. For now, the reflexive case (R == R -> True) is handled
// by the blanket implementation above.

// Example implementations would look like:
// impl RoleEq<Bob> for Alice { type Output = False; }
// impl RoleEq<Alice> for Bob { type Output = False; }

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests;
