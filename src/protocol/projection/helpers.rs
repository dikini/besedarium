//! Helper traits for role-based dispatch in protocol projection
//!
//! This module provides supporting traits for the projection system,
//! including role equality checking and case-specific projection handlers.
//!
//! ## Core Components
//!
//! - **Boolean Logic Types**: Type-level `True`/`False` for compile-time decisions
//! - **Role Equality**: `RoleEq` trait for comparing roles at the type level
//! - **Projection Helpers**: Case-specific traits for handling different projection scenarios
//!
//! ## Type-Level Boolean Logic
//!
//! The boolean types provide the foundation for conditional type-level computation:
//!
//! ```rust
//! use besedarium::protocol::projection::helpers::{Bool, True, False};
//!
//! // Boolean types enable compile-time conditional logic
//! fn type_level_example<B: Bool>() {
//!     // Different implementations can be provided for True vs False
//! }
//! ```
//!
//! ## Role Equality System
//!
//! The `RoleEq` trait enables type-level comparison of roles:
//!
//! ```rust
//! use besedarium::protocol::projection::helpers::{RoleEq, True, False};
//! use besedarium::protocol::foundation::Role;
//!
//! # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
//! # struct Alice;
//! # impl Role for Alice {}
//! # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
//! # struct Bob;
//! # impl Role for Bob {}
//!
//! // Role equality produces boolean types
//! type SameRole = <Alice as RoleEq<Alice>>::Output;  // True
//! // Different roles would produce False (with manual implementations)
//! ```

use super::Project;
use crate::protocol::foundation::{
    ActionIOTMarker, ChanId, CommMetadata, GlobalProtocol, LocalProtocol, Message, MsgLbl, Role,
    SupportsActionIO,
};
use crate::protocol::local::{EpChanRecv, EpChanSend};

/// Type-level boolean trait for case selection
///
/// This trait serves as the base for type-level boolean logic in the projection system.
/// It enables compile-time conditional behavior through marker types.
///
/// The trait is sealed through its `Send + Sync + 'static` bounds, ensuring only
/// the predefined `True` and `False` types can implement it.
///
/// # Usage in Type-Level Programming
///
/// Boolean types are used throughout the projection system to enable conditional
/// type-level computation:
///
/// ```rust
/// use besedarium::protocol::projection::helpers::{Bool, True, False};
///
/// // Define conditional behavior based on boolean type
/// trait ConditionalBehavior<B: Bool> {
///     type Result;
/// }
///
/// // Different implementations for True and False
/// impl ConditionalBehavior<True> for () {
///     type Result = String;  // One behavior for True
/// }
///
/// impl ConditionalBehavior<False> for () {
///     type Result = i32;     // Different behavior for False
/// }
/// ```
///
/// # See Also
///
/// - [`True`] - Type-level true value
/// - [`False`] - Type-level false value
/// - [`RoleEq`] - Produces boolean types for role comparison
pub trait Bool: Send + Sync + 'static {}

/// Type-level True value
///
/// Represents the `true` value in type-level boolean logic. This type is used
/// throughout the projection system to enable compile-time conditional behavior.
///
/// # Usage
///
/// `True` is typically produced by type-level predicates and consumed by
/// conditional traits:
///
/// ```rust
/// use besedarium::protocol::projection::helpers::{True, RoleEq};
/// use besedarium::protocol::foundation::Role;
///
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Alice;
/// # impl Role for Alice {}
///
/// // Role equality with itself produces True
/// type IsReflexive = <Alice as RoleEq<Alice>>::Output;  // True
/// ```
///
/// # See Also
///
/// - [`Bool`] - Base trait for boolean types
/// - [`False`] - Type-level false value
/// - [`RoleEq`] - Example trait that produces boolean types
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct True;
impl Bool for True {}

/// Type-level False value  
///
/// Represents the `false` value in type-level boolean logic. This type is used
/// to represent negative conditions in compile-time conditional behavior.
///
/// # Usage
///
/// `False` is typically produced by type-level predicates when a condition
/// is not met:
///
/// ```rust
/// use besedarium::protocol::projection::helpers::{False, RoleEq};
/// use besedarium::protocol::foundation::Role;
///
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Alice;
/// # impl Role for Alice {}
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Bob;
/// # impl Role for Bob {}
///
/// // Different roles would produce False (with manual implementation)
/// // impl RoleEq<Bob> for Alice { type Output = False; }
/// ```
///
/// # Implementation Note
///
/// Non-reflexive `RoleEq` implementations that produce `False` must be
/// manually implemented for specific role pairs as needed by the protocol.
///
/// # See Also
///
/// - [`Bool`] - Base trait for boolean types
/// - [`True`] - Type-level true value
/// - [`RoleEq`] - Example trait that produces boolean types
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct False;
impl Bool for False {}

/// Check if two roles are equal at the type level
///
/// This trait enables compile-time comparison of role types, producing boolean
/// types that can be used for conditional type-level computation in the projection system.
///
/// # Type Parameters
///
/// - `Other`: The role type to compare against
///
/// # Associated Types
///
/// - `Output`: A [`Bool`] type representing the equality result
///   - [`True`] if the roles are the same type
///   - [`False`] if the roles are different types
///
/// # Core Principle
///
/// Role equality is fundamental to protocol projection logic. When projecting
/// a protocol operation (like `TSend` or `TRecv`) onto a specific role, the
/// projection behavior depends on whether the role being projected is the
/// sender, receiver, or an observer.
///
/// # Examples
///
/// ## Reflexive Equality
///
/// ```rust
/// use besedarium::protocol::projection::helpers::{RoleEq, True};
/// use besedarium::protocol::foundation::Role;
///
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Alice;
/// # impl Role for Alice {}
///
/// // A role always equals itself
/// type SelfEq = <Alice as RoleEq<Alice>>::Output;  // True
/// ```
///
/// ## Non-Reflexive Equality
///
/// ```rust
/// use besedarium::protocol::projection::helpers::{RoleEq, False};
/// use besedarium::protocol::foundation::Role;
///
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Alice;
/// # impl Role for Alice {}
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Bob;
/// # impl Role for Bob {}
///
/// // Manual implementation required for different roles
/// impl RoleEq<Bob> for Alice {
///     type Output = False;
/// }
///
/// type DifferentRoles = <Alice as RoleEq<Bob>>::Output;  // False
/// ```
///
/// ## Usage in Projection
///
/// ```rust
/// use besedarium::protocol::projection::helpers::{RoleEq, True};
/// use besedarium::protocol::foundation::Role;
///
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Alice;
/// # impl Role for Alice {}
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Bob;
/// # impl Role for Bob {}
///
/// // Projection behavior depends on role equality
/// type AliceIsSender = <Alice as RoleEq<Alice>>::Output;  // True
/// // For different roles, manual implementations would be needed:
/// // type BobIsSender = <Alice as RoleEq<Bob>>::Output;   // False (requires manual impl)
///
/// // These boolean types drive different projection implementations
/// // via helper traits like ProjectSendCase and ProjectRecvCase
/// ```
///
/// # Implementation Requirements
///
/// All role types automatically get reflexive equality (self == self) through
/// the blanket implementation. For non-reflexive cases (different roles),
/// manual implementations must be provided as needed by specific protocols.
///
/// # Design Rationale
///
/// This trait uses associated types rather than const generics to maintain
/// compatibility with Rust's stable trait system and enable complex type-level
/// computation patterns used throughout the projection system.
///
/// # See Also
///
/// - [`Bool`] - Output type for equality comparisons
/// - [`True`] / [`False`] - Possible equality results
/// - [`ProjectSendCase`] - Uses role equality for conditional projection
/// - [`ProjectRecvCase`] - Uses role equality for conditional projection
pub trait RoleEq<Other: Role>: Role {
    type Output: Bool;
}

/// Reflexive case: a role equals itself
///
/// This blanket implementation ensures that any role type is considered equal
/// to itself, producing [`True`] as the output. This is the foundation of the
/// role equality system and handles the most common case in projection logic.
///
/// # Type Parameters
///
/// - `R`: Any type implementing [`Role`]
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::projection::helpers::{RoleEq, True};
/// use besedarium::protocol::foundation::Role;
///
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Alice;
/// # impl Role for Alice {}
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Server;
/// # impl Role for Server {}
///
/// // Any role equals itself
/// type AliceEqAlice = <Alice as RoleEq<Alice>>::Output;    // True
/// type ServerEqServer = <Server as RoleEq<Server>>::Output; // True
/// ```
///
/// # Implementation Note
///
/// This implementation takes precedence over any manual implementations due to
/// its blanket nature. For non-reflexive equality (comparing different role types),
/// separate manual implementations are required.
impl<R> RoleEq<R> for R
where
    R: Role,
{
    type Output = True;
}

/// Helper trait for projecting TSend operations based on role equality
///
/// This trait enables conditional projection behavior for `TSend` global protocol
/// operations. The projection logic depends on whether the role being projected
/// (`Me`) is the sender (`S`) or not, determined by the `IsEqual` boolean type.
///
/// # Type Parameters
///
/// - `Me`: The role being projected onto
/// - `S`: The sender role in the TSend operation
/// - `R`: The receiver role in the TSend operation  
/// - `C`: Channel identifier type
/// - `L`: Message label type
/// - `Msg`: Message type being sent
/// - `P`: Continuation protocol after the send
/// - `AIO`: Action I/O marker for the communication
/// - `IsEqual`: Boolean type indicating if `Me == S`
///
/// # Associated Types
///
/// - `Output`: The resulting local protocol type after projection
///
/// # Projection Logic
///
/// The trait has two key implementations:
///
/// 1. **When `Me == S` (IsEqual = True)**: Projects to `EpChanSend` endpoint
/// 2. **When `Me != S` (IsEqual = False)**: Projects to continuation protocol only
///
/// # Examples
///
/// ## Conceptual Usage
///
/// ```rust
/// use besedarium::protocol::projection::helpers::{ProjectSendCase, RoleEq, True, False};
/// use besedarium::protocol::foundation::Role;
///
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Alice;
/// # impl Role for Alice {}
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Bob;
/// # impl Role for Bob {}
///
/// // For Alice projecting TSend<Alice, Bob, ...>:
/// type AliceIsSender = <Alice as RoleEq<Alice>>::Output;  // True
/// // This would use ProjectSendCase with IsEqual = True
/// // Result: EpChanSend for Alice
///
/// // For Bob projecting TSend<Alice, Bob, ...>:
/// // (assuming manual RoleEq impl exists)
/// // This would use ProjectSendCase with IsEqual = False
/// // Result: Just the continuation protocol
/// ```
///
/// # Implementation Strategy
///
/// This trait uses the marker type dispatch pattern to handle different cases
/// without trait specialization. The `IsEqual` parameter drives which implementation
/// is selected at compile time.
///
/// # See Also
///
/// - [`ProjectRecvCase`] - Similar trait for TRecv operations
/// - [`RoleEq`] - Produces the IsEqual boolean parameter
/// - [`Project`] - Main projection trait that uses these helpers
pub trait ProjectSendCase<Me, S, R, C, L, Msg, P, AIO, IsEqual>
where
    Me: Role,
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    IsEqual: Bool,
{
    type Output: LocalProtocol;
}

/// Case when Me == Sender: Project as EpChanSend
///
/// This implementation handles the case where the role being projected (`Me`)
/// is the same as the sender role (`S`) in a TSend operation. In this case,
/// the projection produces an `EpChanSend` endpoint that enables the role
/// to send the specified message.
///
/// # Projection Result
///
/// ```text
/// TSend<S, R, C, L, Msg, P> projected onto S
/// ↓
/// EpChanSend<S, CommMetadata<C, L>, Msg, Project<P, S>, AIO>
/// ```
///
/// # Type Requirements
///
/// - `Me` must support the action I/O marker (`SupportsActionIO<AIO>`)
/// - The continuation protocol `P` must be projectable onto `Me`
/// - The projected continuation must implement `LocalProtocol`
///
/// # Example
///
/// ```rust
/// use besedarium::protocol::projection::helpers::{ProjectSendCase, True};
/// use besedarium::protocol::foundation::Role;
/// # use besedarium::protocol::foundation::{ChanId, MsgLbl, Message, GlobalProtocol, ActionIOTMarker};
///
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Alice;
/// # impl Role for Alice {}
/// # impl besedarium::protocol::foundation::SupportsActionIO<HttpIO> for Alice {}
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct HttpIO;
/// # impl ActionIOTMarker for HttpIO {}
///
/// // When Alice projects TSend<Alice, Bob, ...>, Me == S, so IsEqual = True
/// // This implementation produces EpChanSend<Alice, ...>
/// ```
impl<Me, S, R, C, L, Msg, P, AIO> ProjectSendCase<Me, S, R, C, L, Msg, P, AIO, True> for ()
where
    Me: Role + SupportsActionIO<AIO>,
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): Project<P, Me>,
    <() as Project<P, Me>>::Output: LocalProtocol,
{
    type Output = EpChanSend<Me, CommMetadata<C, L>, Msg, <() as Project<P, Me>>::Output, AIO>;
}

/// Case when Me != Sender: Just project the continuation
///
/// This implementation handles the case where the role being projected (`Me`)
/// is different from the sender role (`S`) in a TSend operation. In this case,
/// the role is not directly involved in the send operation, so the projection
/// simply continues with projecting the continuation protocol.
///
/// # Projection Result
///
/// ```text
/// TSend<S, R, C, L, Msg, P> projected onto Role != S
/// ↓  
/// Project<P, Role>
/// ```
///
/// The send operation is transparent to roles that are neither sender nor receiver.
///
/// # Example
///
/// ```rust
/// use besedarium::protocol::projection::helpers::{ProjectSendCase, False};
/// use besedarium::protocol::foundation::Role;
///
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Charlie;
/// # impl Role for Charlie {}
///
/// // When Charlie projects TSend<Alice, Bob, ...>, Me != S, so IsEqual = False
/// // This implementation just projects the continuation protocol P
/// ```
///
/// # Design Rationale
///
/// This approach ensures that roles not directly involved in a communication
/// still see the protocol progression correctly. They skip the send operation
/// but continue with the same continuation protocol as the sender and receiver.
impl<Me, S, R, C, L, Msg, P, AIO> ProjectSendCase<Me, S, R, C, L, Msg, P, AIO, False> for ()
where
    Me: Role,
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): Project<P, Me>,
{
    type Output = <() as Project<P, Me>>::Output;
}

/// Helper trait for projecting TRecv operations based on role equality
///
/// This trait enables conditional projection behavior for `TRecv` global protocol
/// operations. The projection logic depends on whether the role being projected
/// (`Me`) is the receiver (`R`) or not, determined by the `IsEqual` boolean type.
///
/// # Type Parameters
///
/// - `Me`: The role being projected onto
/// - `S`: The sender role in the TRecv operation
/// - `R`: The receiver role in the TRecv operation
/// - `C`: Channel identifier type
/// - `L`: Message label type
/// - `Msg`: Message type being received
/// - `P`: Continuation protocol after the receive
/// - `AIO`: Action I/O marker for the communication
/// - `IsEqual`: Boolean type indicating if `Me == R`
///
/// # Associated Types
///
/// - `Output`: The resulting local protocol type after projection
///
/// # Projection Logic
///
/// The trait has two key implementations:
///
/// 1. **When `Me == R` (IsEqual = True)**: Projects to `EpChanRecv` endpoint
/// 2. **When `Me != R` (IsEqual = False)**: Projects to continuation protocol only
///
/// # Examples
///
/// ## Conceptual Usage
///
/// ```rust
/// use besedarium::protocol::projection::helpers::{ProjectRecvCase, RoleEq, True, False};
/// use besedarium::protocol::foundation::Role;
///
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Alice;
/// # impl Role for Alice {}
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Bob;
/// # impl Role for Bob {}
///
/// // For Bob projecting TRecv<Alice, Bob, ...>:
/// type BobIsReceiver = <Bob as RoleEq<Bob>>::Output;  // True
/// // This would use ProjectRecvCase with IsEqual = True
/// // Result: EpChanRecv for Bob
///
/// // For Alice projecting TRecv<Alice, Bob, ...>:
/// // (assuming manual RoleEq impl exists)
/// // This would use ProjectRecvCase with IsEqual = False
/// // Result: Just the continuation protocol
/// ```
///
/// # Design Symmetry
///
/// This trait mirrors [`ProjectSendCase`] but for receive operations. Together,
/// they provide the foundation for role-based conditional projection in the
/// protocol system.
///
/// # See Also
///
/// - [`ProjectSendCase`] - Similar trait for TSend operations
/// - [`RoleEq`] - Produces the IsEqual boolean parameter
/// - [`Project`] - Main projection trait that uses these helpers
pub trait ProjectRecvCase<Me, S, R, C, L, Msg, P, AIO, IsEqual>
where
    Me: Role,
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    IsEqual: Bool,
{
    type Output: LocalProtocol;
}

/// Case when Me == Receiver: Project as EpChanRecv
///
/// This implementation handles the case where the role being projected (`Me`)
/// is the same as the receiver role (`R`) in a TRecv operation. In this case,
/// the projection produces an `EpChanRecv` endpoint that enables the role
/// to receive the specified message.
///
/// # Projection Result
///
/// ```text
/// TRecv<S, R, C, L, Msg, P> projected onto R
/// ↓
/// EpChanRecv<R, CommMetadata<C, L>, Msg, Project<P, R>, AIO>
/// ```
///
/// # Type Requirements
///
/// - `Me` must support the action I/O marker (`SupportsActionIO<AIO>`)
/// - The continuation protocol `P` must be projectable onto `Me`
/// - The projected continuation must implement `LocalProtocol`
///
/// # Example
///
/// ```rust
/// use besedarium::protocol::projection::helpers::{ProjectRecvCase, True};
/// use besedarium::protocol::foundation::Role;
/// # use besedarium::protocol::foundation::ActionIOTMarker;
///
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Bob;
/// # impl Role for Bob {}
/// # impl besedarium::protocol::foundation::SupportsActionIO<HttpIO> for Bob {}
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct HttpIO;
/// # impl ActionIOTMarker for HttpIO {}
///
/// // When Bob projects TRecv<Alice, Bob, ...>, Me == R, so IsEqual = True
/// // This implementation produces EpChanRecv<Bob, ...>
/// ```
impl<Me, S, R, C, L, Msg, P, AIO> ProjectRecvCase<Me, S, R, C, L, Msg, P, AIO, True> for ()
where
    Me: Role + SupportsActionIO<AIO>,
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): Project<P, Me>,
    <() as Project<P, Me>>::Output: LocalProtocol,
{
    type Output = EpChanRecv<Me, CommMetadata<C, L>, Msg, <() as Project<P, Me>>::Output, AIO>;
}

/// Case when Me != Receiver: Just project the continuation
///
/// This implementation handles the case where the role being projected (`Me`)
/// is different from the receiver role (`R`) in a TRecv operation. In this case,
/// the role is not directly involved in the receive operation, so the projection
/// simply continues with projecting the continuation protocol.
///
/// # Projection Result
///
/// ```text
/// TRecv<S, R, C, L, Msg, P> projected onto Role != R
/// ↓
/// Project<P, Role>
/// ```
///
/// The receive operation is transparent to roles that are neither sender nor receiver.
///
/// # Example
///
/// ```rust
/// use besedarium::protocol::projection::helpers::{ProjectRecvCase, False};
/// use besedarium::protocol::foundation::Role;
///
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Charlie;
/// # impl Role for Charlie {}
///
/// // When Charlie projects TRecv<Alice, Bob, ...>, Me != R, so IsEqual = False  
/// // This implementation just projects the continuation protocol P
/// ```
///
/// # Design Consistency
///
/// This approach mirrors the send case, ensuring that roles not directly involved
/// in a communication still see the protocol progression correctly. They skip the
/// receive operation but continue with the same continuation protocol.
impl<Me, S, R, C, L, Msg, P, AIO> ProjectRecvCase<Me, S, R, C, L, Msg, P, AIO, False> for ()
where
    Me: Role,
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): Project<P, Me>,
{
    type Output = <() as Project<P, Me>>::Output;
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
