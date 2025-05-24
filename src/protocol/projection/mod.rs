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

use crate::protocol::foundation::{
    ActionIOTMarker, ChanId, CommMetadata, GlobalProtocol, LocalProtocol, Message, MsgLbl, Role,
    SupportsActionIO,
};
use crate::protocol::global::{TChanChoice, TChanEnd, TChanPar, TChanRecv, TChanSend, TChanStart};
use crate::protocol::local::{EpChanChoice, EpChanEnd, EpChanPar, EpChanStart};
use std::fmt;
use std::fmt::Debug;

// ============================================================================
// Module Structure
// ============================================================================

pub mod helpers;

// Re-export essential helper types for external use
pub use helpers::{Bool, False, ProjectRecvCase, ProjectSendCase, RoleEq, True};

// ============================================================================
// Projection Error Types and Validation
// ============================================================================

/// Error type for projection validation and runtime failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionError {
    /// Role not involved in the protocol step
    RoleNotInvolved { role: String, protocol_step: String },
    /// Invalid projection due to type constraints
    InvalidProjection {
        reason: String,
        protocol_type: String,
        target_role: String,
    },
    /// Action I/O capability mismatch
    ActionIOCapabilityMismatch {
        required_capability: String,
        actual_capability: String,
    },
    /// Invalid channel or message metadata
    InvalidMetadata { description: String },
}

impl fmt::Display for ProjectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectionError::RoleNotInvolved {
                role,
                protocol_step,
            } => {
                write!(
                    f,
                    "Role '{}' is not involved in protocol step: {}",
                    role, protocol_step
                )
            }
            ProjectionError::InvalidProjection {
                reason,
                protocol_type,
                target_role,
            } => {
                write!(
                    f,
                    "Invalid projection of '{}' to role '{}': {}",
                    protocol_type, target_role, reason
                )
            }
            ProjectionError::ActionIOCapabilityMismatch {
                required_capability,
                actual_capability,
            } => {
                write!(
                    f,
                    "Action I/O capability mismatch: required '{}', found '{}'",
                    required_capability, actual_capability
                )
            }
            ProjectionError::InvalidMetadata { description } => {
                write!(f, "Invalid metadata: {}", description)
            }
        }
    }
}

impl std::error::Error for ProjectionError {}

/// Trait for validating projection constraints at compile-time
pub trait ValidateProjection<P, R>
where
    P: GlobalProtocol,
    R: Role,
{
    /// Type-level validation result (True if valid, False if invalid)
    type IsValid: Bool;

    /// Validation error message (if any)
    type ErrorType: Send + Sync + 'static;
}

/// Helper trait for runtime projection validation
pub trait ProjectionValidator {
    /// Validate that a role is appropriately involved in a protocol step
    fn validate_role_involvement(role: &str, protocol_step: &str) -> Result<(), ProjectionError>;

    /// Validate action I/O capabilities
    fn validate_action_io(required: &str, actual: &str) -> Result<(), ProjectionError>;

    /// Validate metadata consistency
    fn validate_metadata(description: &str) -> Result<(), ProjectionError>;
}

/// Default implementation of projection validation
pub struct DefaultProjectionValidator;

impl ProjectionValidator for DefaultProjectionValidator {
    fn validate_role_involvement(role: &str, protocol_step: &str) -> Result<(), ProjectionError> {
        // Basic validation - can be extended with more sophisticated checks
        if role.is_empty() {
            return Err(ProjectionError::RoleNotInvolved {
                role: role.to_string(),
                protocol_step: protocol_step.to_string(),
            });
        }
        Ok(())
    }

    fn validate_action_io(required: &str, actual: &str) -> Result<(), ProjectionError> {
        if required != actual {
            return Err(ProjectionError::ActionIOCapabilityMismatch {
                required_capability: required.to_string(),
                actual_capability: actual.to_string(),
            });
        }
        Ok(())
    }

    fn validate_metadata(description: &str) -> Result<(), ProjectionError> {
        if description.contains("invalid") {
            return Err(ProjectionError::InvalidMetadata {
                description: description.to_string(),
            });
        }
        Ok(())
    }
}

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
// Project Trait Implementations
// ============================================================================

/// Project TChanSend: Role-based dispatch to determine send vs recv vs continuation
impl<Me, S, R, C, L, Msg, P, AIO> Project<TChanSend<S, R, C, L, Msg, P, AIO>, Me> for ()
where
    Me: Role,
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    Me: RoleEq<S>,
    <Me as RoleEq<S>>::Output: Bool,
    (): ProjectSendCase<Me, S, R, C, L, Msg, P, AIO, <Me as RoleEq<S>>::Output>,
{
    type Output =
        <() as ProjectSendCase<Me, S, R, C, L, Msg, P, AIO, <Me as RoleEq<S>>::Output>>::Output;
}

/// Project TChanRecv: Similar to TChanSend but for receive operations
impl<Me, S, R, C, L, Msg, P, AIO> Project<TChanRecv<S, R, C, L, Msg, P, AIO>, Me> for ()
where
    Me: Role,
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    Me: RoleEq<R>,
    <Me as RoleEq<R>>::Output: Bool,
    (): ProjectRecvCase<Me, S, R, C, L, Msg, P, AIO, <Me as RoleEq<R>>::Output>,
{
    type Output =
        <() as ProjectRecvCase<Me, S, R, C, L, Msg, P, AIO, <Me as RoleEq<R>>::Output>>::Output;
}

/// Project TChanEnd: Always project to EpChanEnd
impl<Me, C, L, AIO> Project<TChanEnd<C, L, AIO>, Me> for ()
where
    Me: Role + SupportsActionIO<AIO>,
    C: ChanId,
    L: MsgLbl,
    AIO: ActionIOTMarker,
{
    type Output = EpChanEnd<Me, CommMetadata<C, L>, AIO>;
}

/// Project TChanStart: Project to EpChanStart with projected continuation
impl<Me, C, L, S, AIO> Project<TChanStart<C, L, S, AIO>, Me> for ()
where
    Me: Role + SupportsActionIO<AIO>,
    C: ChanId,
    L: MsgLbl,
    S: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): Project<S, Me>,
    <() as Project<S, Me>>::Output: LocalProtocol,
{
    type Output = EpChanStart<Me, CommMetadata<C, L>, <() as Project<S, Me>>::Output, AIO>;
}

/// Project TChanChoice: Project to EpChanChoice with projected branches
impl<Me, R, C, L, Left, Right, AIO> Project<TChanChoice<R, C, L, Left, Right, AIO>, Me> for ()
where
    Me: Role + SupportsActionIO<AIO>,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): Project<Left, Me>,
    (): Project<Right, Me>,
    <() as Project<Left, Me>>::Output: LocalProtocol,
    <() as Project<Right, Me>>::Output: LocalProtocol,
{
    type Output = EpChanChoice<
        Me,
        CommMetadata<C, L>,
        <() as Project<Left, Me>>::Output,
        <() as Project<Right, Me>>::Output,
        AIO,
    >;
}

/// Project TChanPar: Project to EpChanPar with projected branches
impl<Me, C, L, Left, Right, IsDisjoint, AIO>
    Project<TChanPar<C, L, Left, Right, IsDisjoint, AIO>, Me> for ()
where
    Me: Role + SupportsActionIO<AIO>,
    C: ChanId,
    L: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    IsDisjoint: Send + Sync + 'static + Debug,
    AIO: ActionIOTMarker,
    (): Project<Left, Me>,
    (): Project<Right, Me>,
    <() as Project<Left, Me>>::Output: LocalProtocol,
    <() as Project<Right, Me>>::Output: LocalProtocol,
{
    type Output = EpChanPar<
        Me,
        CommMetadata<C, L>,
        <() as Project<Left, Me>>::Output,
        <() as Project<Right, Me>>::Output,
        IsDisjoint,
        AIO,
    >;
}

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
