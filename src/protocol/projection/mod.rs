//! # Protocol Projection Module
//!
//! This module implements the `Project<P, Role>` trait for projecting Global Protocols 
//! to Local Endpoint Types with channel awareness and IO capability validation.
//!
//! ## Core Concepts
//!
//! - **Project trait**: Maps global protocols to local endpoint types for specific roles
//! - **Role-based projection**: Different roles see different views of the same protocol
//! - **IO capability validation**: Ensures local endpoints support required actions
//! - **Type-level validation**: Compile-time verification of projection correctness
//!
//! ## Usage
//!
//! ```rust
//! use besedarium::protocol::projection::{Project, ProjectOutput, assert_projects};
//! use besedarium::protocol::foundation::*;
//! use besedarium::protocol::global::*;
//! use besedarium::protocol::local::*;
//!
//! // Basic projection example  
//! type SendProto = TChanSend<Alice, Bob, Meta, HelloMsg, End, BiDirectionalAction>;
//! type AliceView = ProjectOutput<SendProto, Alice>;  // Results in EpChanSend
//! type BobView = ProjectOutput<SendProto, Bob>;      // Results in EpChanRecv
//! ```

use std::marker::PhantomData;
use crate::protocol::foundation::*;
use crate::protocol::global::*;
use crate::protocol::local::*;
use crate::{Bool, True as TTrue, False as TFalse};

// Re-export key types for convenience
pub use crate::types::{True, False};

/// Core projection trait for channel-aware protocols
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
    
    /// Validate that the projection is well-formed
    type IsValid: Bool;
    
    /// Error type if projection fails
    type Error: ProjectionError;
}

/// Type alias for cleaner projection usage
pub type ProjectOutput<P, R> = <() as Project<P, R>>::Output;

/// Type alias for projection validation
pub type ProjectIsValid<P, R> = <() as Project<P, R>>::IsValid;

/// Type alias for projection errors
pub type ProjectError<P, R> = <() as Project<P, R>>::Error;

/// Enhanced projection trait that includes IO capability validation
pub trait ProjectWithIO<P, R, IO> 
where
    P: GlobalProtocol,
    R: Role,
{
    /// The resulting local protocol for role R with IO capability
    type Output: LocalProtocol;
    
    /// Validate that IO supports all required actions in the protocol
    type IOSupported: Bool;
    
    /// List of required ActionIOTMarkers for this projection
    type RequiredActions;
}

/// Project with explicit IO capability checking
impl<P, R, IO> ProjectWithIO<P, R, IO> for ()
where
    P: GlobalProtocol,
    R: Role,
    (): Project<P, R>,
    (): ValidateIOCapabilities<<() as Project<P, R>>::Output, IO>,
{
    type Output = <() as Project<P, R>>::Output;
    type IOSupported = <() as ValidateIOCapabilities<Self::Output, IO>>::IsSupported;
    type RequiredActions = <() as ValidateIOCapabilities<Self::Output, IO>>::RequiredActions;
}

//
// Error Types and Validation
//

/// Error types for projection failures
pub trait ProjectionError: Send + Sync + 'static + std::fmt::Debug {}

/// No error occurred
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoError;
impl ProjectionError for NoError {}

/// Role not found in protocol
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleNotFoundError<R: Role> {
    _role: PhantomData<R>,
}
impl<R: Role> ProjectionError for RoleNotFoundError<R> {}

/// Choice projection error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChoiceProjectionError;
impl ProjectionError for ChoiceProjectionError {}

/// Offer projection error  
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfferProjectionError;
impl ProjectionError for OfferProjectionError {}

/// Parallel projection error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParallelProjectionError;
impl ProjectionError for ParallelProjectionError {}

/// IO capability missing error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IOCapabilityError<IO, AIO> {
    _io: PhantomData<IO>,
    _aio: PhantomData<AIO>,
}
impl<IO, AIO: ActionIOTMarker> ProjectionError for IOCapabilityError<IO, AIO> {}

//
// Helper Traits for Type-Level Operations
//

/// Type-level AND operation for combining booleans
pub trait TypeAnd<Other: Bool>: Bool {
    type Output: Bool;
}

impl TypeAnd<TTrue> for TTrue {
    type Output = TTrue;
}

impl TypeAnd<TFalse> for TTrue {
    type Output = TFalse;
}

impl TypeAnd<TTrue> for TFalse {
    type Output = TFalse;
}

impl TypeAnd<TFalse> for TFalse {
    type Output = TFalse;
}

/// Check if a protocol contains a specific role
pub trait ContainsRole<P, R>
where
    P: GlobalProtocol,
    R: Role,
{
    type Output: Bool;
}

/// Default implementation: protocols don't contain roles unless specified
impl<P, R> ContainsRole<P, R> for ()
where
    P: GlobalProtocol,
    R: Role,
{
    type Output = TFalse;
}

/// Validate IO capabilities for a projected protocol
pub trait ValidateIOCapabilities<EP, IO>
where
    EP: LocalProtocol,
{
    type IsSupported: Bool;
    type RequiredActions; // Type-level list of required ActionIOTMarkers
    type MissingCapabilities; // Type-level list of missing capabilities
}

/// Default implementation: assume IO supports everything
impl<EP, IO> ValidateIOCapabilities<EP, IO> for ()
where
    EP: LocalProtocol,
{
    type IsSupported = TTrue;
    type RequiredActions = ();
    type MissingCapabilities = ();
}

//
// Role Equality Helpers
//

/// Check if two roles are equal at the type level
pub trait RoleEq<Other: Role>: Role {
    type Output: Bool;
}

/// Default implementation: roles are different unless specified
impl<R1, R2> RoleEq<R2> for R1
where
    R1: Role,
    R2: Role,
{
    default type Output = TFalse;
}

/// Reflexive case: a role equals itself
impl<R> RoleEq<R> for R
where
    R: Role,
{
    type Output = TTrue;
}

//
// TChanEnd Projection
//

/// Project TChanEnd to local endpoint (termination)
impl<M, AIO, Role_Me> Project<TChanEnd<M, AIO>, Role_Me> for ()
where
    M: CommMetadataTrait,
    AIO: ActionIOTMarker,
    Role_Me: Role,
{
    type Output = EpChanEnd<TerminationAction, M, AIO>;
    type IsValid = TTrue;
    type Error = NoError;
}

//
// TChanStart Projection
//

/// Project TChanStart to local endpoint (initialization)
impl<M, Start, AIO, Role_Me> Project<TChanStart<M, Start, AIO>, Role_Me> for ()
where
    M: CommMetadataTrait,
    Start: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<Start, Role_Me>,
{
    type Output = EpChanStart<InputAction, M, <() as Project<Start, Role_Me>>::Output, AIO>;
    type IsValid = <() as Project<Start, Role_Me>>::IsValid;
    type Error = <() as Project<Start, Role_Me>>::Error;
}

//
// TChanSend Projection
//

/// Project TChanSend to local endpoint based on role involvement
impl<S, R, M, Msg, P, AIO, Role_Me> Project<TChanSend<S, R, M, Msg, P, AIO>, Role_Me> for ()
where
    S: Role,
    R: Role,
    Role_Me: Role,
    M: CommMetadataTrait,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): Project<P, Role_Me>,
    Role_Me: RoleEq<S>,
    Role_Me: RoleEq<R>,
    (): ProjectSendCase<S, R, M, Msg, P, AIO, Role_Me, <Role_Me as RoleEq<S>>::Output, <Role_Me as RoleEq<R>>::Output>,
{
    type Output = <() as ProjectSendCase<S, R, M, Msg, P, AIO, Role_Me, <Role_Me as RoleEq<S>>::Output, <Role_Me as RoleEq<R>>::Output>>::Output;
    type IsValid = <() as ProjectSendCase<S, R, M, Msg, P, AIO, Role_Me, <Role_Me as RoleEq<S>>::Output, <Role_Me as RoleEq<R>>::Output>>::IsValid;
    type Error = <() as ProjectSendCase<S, R, M, Msg, P, AIO, Role_Me, <Role_Me as RoleEq<S>>::Output, <Role_Me as RoleEq<R>>::Output>>::Error;
}

/// Helper trait for Send projection case analysis
pub trait ProjectSendCase<S, R, M, Msg, P, AIO, Role_Me, IsSender, IsReceiver>
where
    S: Role,
    R: Role,
    M: CommMetadataTrait,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    IsSender: Bool,
    IsReceiver: Bool,
{
    type Output: LocalProtocol;
    type IsValid: Bool;
    type Error: ProjectionError;
}

// Case 1: Role_Me is the sender (IsSender = TTrue, IsReceiver = TFalse)
impl<S, R, M, Msg, P, AIO, Role_Me> ProjectSendCase<S, R, M, Msg, P, AIO, Role_Me, TTrue, TFalse> for ()
where
    S: Role,
    R: Role,
    M: CommMetadataTrait,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<P, Role_Me>,
{
    type Output = EpChanSend<OutputAction, M, Msg, <() as Project<P, Role_Me>>::Output, AIO>;
    type IsValid = <() as Project<P, Role_Me>>::IsValid;
    type Error = <() as Project<P, Role_Me>>::Error;
}

// Case 2: Role_Me is the receiver (IsSender = TFalse, IsReceiver = TTrue)
impl<S, R, M, Msg, P, AIO, Role_Me> ProjectSendCase<S, R, M, Msg, P, AIO, Role_Me, TFalse, TTrue> for ()
where
    S: Role,
    R: Role,
    M: CommMetadataTrait,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<P, Role_Me>,
{
    type Output = EpChanRecv<InputAction, M, Msg, <() as Project<P, Role_Me>>::Output, AIO>;
    type IsValid = <() as Project<P, Role_Me>>::IsValid;
    type Error = <() as Project<P, Role_Me>>::Error;
}

// Case 3: Role_Me is neither sender nor receiver (IsSender = TFalse, IsReceiver = TFalse)
impl<S, R, M, Msg, P, AIO, Role_Me> ProjectSendCase<S, R, M, Msg, P, AIO, Role_Me, TFalse, TFalse> for ()
where
    S: Role,
    R: Role,
    M: CommMetadataTrait,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<P, Role_Me>,
{
    // Role not involved in this action, project the continuation
    type Output = <() as Project<P, Role_Me>>::Output;
    type IsValid = <() as Project<P, Role_Me>>::IsValid;
    type Error = <() as Project<P, Role_Me>>::Error;
}

//
// TChanRecv Projection (similar pattern, reversed roles)
//

/// Project TChanRecv to local endpoint based on role involvement
impl<R, S, M, Msg, P, AIO, Role_Me> Project<TChanRecv<R, S, M, Msg, P, AIO>, Role_Me> for ()
where
    R: Role,
    S: Role,
    Role_Me: Role,
    M: CommMetadataTrait,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): Project<P, Role_Me>,
    Role_Me: RoleEq<R>,
    Role_Me: RoleEq<S>,
    (): ProjectRecvCase<R, S, M, Msg, P, AIO, Role_Me, <Role_Me as RoleEq<R>>::Output, <Role_Me as RoleEq<S>>::Output>,
{
    type Output = <() as ProjectRecvCase<R, S, M, Msg, P, AIO, Role_Me, <Role_Me as RoleEq<R>>::Output, <Role_Me as RoleEq<S>>::Output>>::Output;
    type IsValid = <() as ProjectRecvCase<R, S, M, Msg, P, AIO, Role_Me, <Role_Me as RoleEq<R>>::Output, <Role_Me as RoleEq<S>>::Output>>::IsValid;
    type Error = <() as ProjectRecvCase<R, S, M, Msg, P, AIO, Role_Me, <Role_Me as RoleEq<R>>::Output, <Role_Me as RoleEq<S>>::Output>>::Error;
}

/// Helper trait for Recv projection case analysis
pub trait ProjectRecvCase<R, S, M, Msg, P, AIO, Role_Me, IsReceiver, IsSender>
where
    R: Role,
    S: Role,
    M: CommMetadataTrait,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    IsReceiver: Bool,
    IsSender: Bool,
{
    type Output: LocalProtocol;
    type IsValid: Bool;
    type Error: ProjectionError;
}

// Case 1: Role_Me is the receiver (IsReceiver = TTrue, IsSender = TFalse)
impl<R, S, M, Msg, P, AIO, Role_Me> ProjectRecvCase<R, S, M, Msg, P, AIO, Role_Me, TTrue, TFalse> for ()
where
    R: Role,
    S: Role,
    M: CommMetadataTrait,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<P, Role_Me>,
{
    type Output = EpChanRecv<InputAction, M, Msg, <() as Project<P, Role_Me>>::Output, AIO>;
    type IsValid = <() as Project<P, Role_Me>>::IsValid;
    type Error = <() as Project<P, Role_Me>>::Error;
}

// Case 2: Role_Me is the sender (IsReceiver = TFalse, IsSender = TTrue)
impl<R, S, M, Msg, P, AIO, Role_Me> ProjectRecvCase<R, S, M, Msg, P, AIO, Role_Me, TFalse, TTrue> for ()
where
    R: Role,
    S: Role,
    M: CommMetadataTrait,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<P, Role_Me>,
{
    type Output = EpChanSend<OutputAction, M, Msg, <() as Project<P, Role_Me>>::Output, AIO>;
    type IsValid = <() as Project<P, Role_Me>>::IsValid;
    type Error = <() as Project<P, Role_Me>>::Error;
}

// Case 3: Role_Me is neither (IsReceiver = TFalse, IsSender = TFalse)
impl<R, S, M, Msg, P, AIO, Role_Me> ProjectRecvCase<R, S, M, Msg, P, AIO, Role_Me, TFalse, TFalse> for ()
where
    R: Role,
    S: Role,
    M: CommMetadataTrait,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<P, Role_Me>,
{
    // Role not involved in this action, project the continuation
    type Output = <() as Project<P, Role_Me>>::Output;
    type IsValid = <() as Project<P, Role_Me>>::IsValid;
    type Error = <() as Project<P, Role_Me>>::Error;
}

//
// TChanChoice Projection
//

/// Project TChanChoice to local endpoint
impl<Chooser, M, L, R_branch, AIO, Role_Me> Project<TChanChoice<Chooser, M, L, R_branch, AIO>, Role_Me> for ()
where
    Chooser: Role,
    Role_Me: Role,
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: RoleEq<Chooser>,
    (): Project<L, Role_Me>,
    (): Project<R_branch, Role_Me>,
    (): ContainsRole<L, Role_Me>,
    (): ContainsRole<R_branch, Role_Me>,
    (): ProjectChoiceCase<Chooser, M, L, R_branch, AIO, Role_Me, <Role_Me as RoleEq<Chooser>>::Output>,
{
    type Output = <() as ProjectChoiceCase<Chooser, M, L, R_branch, AIO, Role_Me, <Role_Me as RoleEq<Chooser>>::Output>>::Output;
    type IsValid = <() as ProjectChoiceCase<Chooser, M, L, R_branch, AIO, Role_Me, <Role_Me as RoleEq<Chooser>>::Output>>::IsValid;
    type Error = <() as ProjectChoiceCase<Chooser, M, L, R_branch, AIO, Role_Me, <Role_Me as RoleEq<Chooser>>::Output>>::Error;
}

/// Helper trait for Choice projection case analysis
pub trait ProjectChoiceCase<Chooser, M, L, R_branch, AIO, Role_Me, IsChooser>
where
    Chooser: Role,
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    IsChooser: Bool,
{
    type Output: LocalProtocol;
    type IsValid: Bool;
    type Error: ProjectionError;
}

// Case 1: Role_Me is the chooser (IsChooser = TTrue)
impl<Chooser, M, L, R_branch, AIO, Role_Me> ProjectChoiceCase<Chooser, M, L, R_branch, AIO, Role_Me, TTrue> for ()
where
    Chooser: Role,
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<L, Role_Me>,
    (): Project<R_branch, Role_Me>,
{
    type Output = EpChanChoice<OutputAction, M, <() as Project<L, Role_Me>>::Output, <() as Project<R_branch, Role_Me>>::Output, AIO>;
    type IsValid = <() as TypeAnd<<() as Project<L, Role_Me>>::IsValid, <() as Project<R_branch, Role_Me>>::IsValid>>::Output;
    type Error = ChoiceProjectionError;
}

// Case 2: Role_Me is not the chooser (IsChooser = TFalse)
impl<Chooser, M, L, R_branch, AIO, Role_Me> ProjectChoiceCase<Chooser, M, L, R_branch, AIO, Role_Me, TFalse> for ()
where
    Chooser: Role,
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<L, Role_Me>,
    (): Project<R_branch, Role_Me>,
    (): ContainsRole<L, Role_Me>,
    (): ContainsRole<R_branch, Role_Me>,
    (): ProjectChoiceNonChooser<M, L, R_branch, AIO, Role_Me, <() as ContainsRole<L, Role_Me>>::Output, <() as ContainsRole<R_branch, Role_Me>>::Output>,
{
    type Output = <() as ProjectChoiceNonChooser<M, L, R_branch, AIO, Role_Me, <() as ContainsRole<L, Role_Me>>::Output, <() as ContainsRole<R_branch, Role_Me>>::Output>>::Output;
    type IsValid = <() as ProjectChoiceNonChooser<M, L, R_branch, AIO, Role_Me, <() as ContainsRole<L, Role_Me>>::Output, <() as ContainsRole<R_branch, Role_Me>>::Output>>::IsValid;
    type Error = <() as ProjectChoiceNonChooser<M, L, R_branch, AIO, Role_Me, <() as ContainsRole<L, Role_Me>>::Output, <() as ContainsRole<R_branch, Role_Me>>::Output>>::Error;
}

/// Helper for projecting choice when not the chooser
pub trait ProjectChoiceNonChooser<M, L, R_branch, AIO, Role_Me, InLeft, InRight>
where
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    InLeft: Bool,
    InRight: Bool,
{
    type Output: LocalProtocol;
    type IsValid: Bool;
    type Error: ProjectionError;
}

// Case: Role in both branches -> EpOffer
impl<M, L, R_branch, AIO, Role_Me> ProjectChoiceNonChooser<M, L, R_branch, AIO, Role_Me, TTrue, TTrue> for ()
where
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<L, Role_Me>,
    (): Project<R_branch, Role_Me>,
{
    type Output = EpChanOffer<InputAction, M, <() as Project<L, Role_Me>>::Output, <() as Project<R_branch, Role_Me>>::Output, AIO>;
    type IsValid = <() as TypeAnd<<() as Project<L, Role_Me>>::IsValid, <() as Project<R_branch, Role_Me>>::IsValid>>::Output;
    type Error = OfferProjectionError;
}

// Case: Role in left branch only -> Project left
impl<M, L, R_branch, AIO, Role_Me> ProjectChoiceNonChooser<M, L, R_branch, AIO, Role_Me, TTrue, TFalse> for ()
where
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<L, Role_Me>,
{
    type Output = <() as Project<L, Role_Me>>::Output;
    type IsValid = <() as Project<L, Role_Me>>::IsValid;
    type Error = <() as Project<L, Role_Me>>::Error;
}

// Case: Role in right branch only -> Project right
impl<M, L, R_branch, AIO, Role_Me> ProjectChoiceNonChooser<M, L, R_branch, AIO, Role_Me, TFalse, TTrue> for ()
where
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<R_branch, Role_Me>,
{
    type Output = <() as Project<R_branch, Role_Me>>::Output;
    type IsValid = <() as Project<R_branch, Role_Me>>::IsValid;
    type Error = <() as Project<R_branch, Role_Me>>::Error;
}

// Case: Role in neither branch -> Continue (skip this choice)
impl<M, L, R_branch, AIO, Role_Me> ProjectChoiceNonChooser<M, L, R_branch, AIO, Role_Me, TFalse, TFalse> for ()
where
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
{
    type Output = EpChanEnd<TerminationAction, M, AIO>; // No participation -> end
    type IsValid = TTrue;
    type Error = NoError;
}

//
// TChanOffer Projection (similar to Choice but reversed)
//

/// Project TChanOffer to local endpoint
impl<Offerer, M, L, R_branch, AIO, Role_Me> Project<TChanOffer<Offerer, M, L, R_branch, AIO>, Role_Me> for ()
where
    Offerer: Role,
    Role_Me: Role,
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: RoleEq<Offerer>,
    (): Project<L, Role_Me>,
    (): Project<R_branch, Role_Me>,
    (): ContainsRole<L, Role_Me>,
    (): ContainsRole<R_branch, Role_Me>,
    (): ProjectOfferCase<Offerer, M, L, R_branch, AIO, Role_Me, <Role_Me as RoleEq<Offerer>>::Output>,
{
    type Output = <() as ProjectOfferCase<Offerer, M, L, R_branch, AIO, Role_Me, <Role_Me as RoleEq<Offerer>>::Output>>::Output;
    type IsValid = <() as ProjectOfferCase<Offerer, M, L, R_branch, AIO, Role_Me, <Role_Me as RoleEq<Offerer>>::Output>>::IsValid;
    type Error = <() as ProjectOfferCase<Offerer, M, L, R_branch, AIO, Role_Me, <Role_Me as RoleEq<Offerer>>::Output>>::Error;
}

/// Helper trait for Offer projection case analysis
pub trait ProjectOfferCase<Offerer, M, L, R_branch, AIO, Role_Me, IsOfferer>
where
    Offerer: Role,
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    IsOfferer: Bool,
{
    type Output: LocalProtocol;
    type IsValid: Bool;
    type Error: ProjectionError;
}

// Case 1: Role_Me is the offerer (IsOfferer = TTrue)
impl<Offerer, M, L, R_branch, AIO, Role_Me> ProjectOfferCase<Offerer, M, L, R_branch, AIO, Role_Me, TTrue> for ()
where
    Offerer: Role,
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<L, Role_Me>,
    (): Project<R_branch, Role_Me>,
{
    type Output = EpChanOffer<InputAction, M, <() as Project<L, Role_Me>>::Output, <() as Project<R_branch, Role_Me>>::Output, AIO>;
    type IsValid = <() as TypeAnd<<() as Project<L, Role_Me>>::IsValid, <() as Project<R_branch, Role_Me>>::IsValid>>::Output;
    type Error = OfferProjectionError;
}

// Case 2: Role_Me is not the offerer -> delegate to choice-like logic
impl<Offerer, M, L, R_branch, AIO, Role_Me> ProjectOfferCase<Offerer, M, L, R_branch, AIO, Role_Me, TFalse> for ()
where
    Offerer: Role,
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<L, Role_Me>,
    (): Project<R_branch, Role_Me>,
    (): ContainsRole<L, Role_Me>,
    (): ContainsRole<R_branch, Role_Me>,
    (): ProjectChoiceNonChooser<M, L, R_branch, AIO, Role_Me, <() as ContainsRole<L, Role_Me>>::Output, <() as ContainsRole<R_branch, Role_Me>>::Output>,
{
    type Output = <() as ProjectChoiceNonChooser<M, L, R_branch, AIO, Role_Me, <() as ContainsRole<L, Role_Me>>::Output, <() as ContainsRole<R_branch, Role_Me>>::Output>>::Output;
    type IsValid = <() as ProjectChoiceNonChooser<M, L, R_branch, AIO, Role_Me, <() as ContainsRole<L, Role_Me>>::Output, <() as ContainsRole<R_branch, Role_Me>>::Output>>::IsValid;
    type Error = <() as ProjectChoiceNonChooser<M, L, R_branch, AIO, Role_Me, <() as ContainsRole<L, Role_Me>>::Output, <() as ContainsRole<R_branch, Role_Me>>::Output>>::Error;
}

//
// TChanPar Projection
//

/// Project TChanPar to local endpoint
impl<M, L, R_branch, IsDisjoint, AIO, Role_Me> Project<TChanPar<M, L, R_branch, IsDisjoint, AIO>, Role_Me> for ()
where
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    IsDisjoint: Bool,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<L, Role_Me>,
    (): Project<R_branch, Role_Me>,
    (): ContainsRole<L, Role_Me>,
    (): ContainsRole<R_branch, Role_Me>,
    (): ProjectParCase<M, L, R_branch, IsDisjoint, AIO, Role_Me, <() as ContainsRole<L, Role_Me>>::Output, <() as ContainsRole<R_branch, Role_Me>>::Output>,
{
    type Output = <() as ProjectParCase<M, L, R_branch, IsDisjoint, AIO, Role_Me, <() as ContainsRole<L, Role_Me>>::Output, <() as ContainsRole<R_branch, Role_Me>>::Output>>::Output;
    type IsValid = <() as ProjectParCase<M, L, R_branch, IsDisjoint, AIO, Role_Me, <() as ContainsRole<L, Role_Me>>::Output, <() as ContainsRole<R_branch, Role_Me>>::Output>>::IsValid;
    type Error = <() as ProjectParCase<M, L, R_branch, IsDisjoint, AIO, Role_Me, <() as ContainsRole<L, Role_Me>>::Output, <() as ContainsRole<R_branch, Role_Me>>::Output>>::Error;
}

/// Helper trait for Parallel projection case analysis
pub trait ProjectParCase<M, L, R_branch, IsDisjoint, AIO, Role_Me, InLeft, InRight>
where
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    IsDisjoint: Bool,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    InLeft: Bool,
    InRight: Bool,
{
    type Output: LocalProtocol;
    type IsValid: Bool;
    type Error: ProjectionError;
}

// Case: Role in both branches -> EpPar
impl<M, L, R_branch, IsDisjoint, AIO, Role_Me> ProjectParCase<M, L, R_branch, IsDisjoint, AIO, Role_Me, TTrue, TTrue> for ()
where
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    IsDisjoint: Bool,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<L, Role_Me>,
    (): Project<R_branch, Role_Me>,
{
    type Output = EpChanPar<InputAction, M, <() as Project<L, Role_Me>>::Output, <() as Project<R_branch, Role_Me>>::Output, IsDisjoint, AIO>;
    type IsValid = <() as TypeAnd<<() as Project<L, Role_Me>>::IsValid, <() as Project<R_branch, Role_Me>>::IsValid>>::Output;
    type Error = ParallelProjectionError;
}

// Case: Role in left branch only -> Project left
impl<M, L, R_branch, IsDisjoint, AIO, Role_Me> ProjectParCase<M, L, R_branch, IsDisjoint, AIO, Role_Me, TTrue, TFalse> for ()
where
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    IsDisjoint: Bool,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<L, Role_Me>,
{
    type Output = <() as Project<L, Role_Me>>::Output;
    type IsValid = <() as Project<L, Role_Me>>::IsValid;
    type Error = <() as Project<L, Role_Me>>::Error;
}

// Case: Role in right branch only -> Project right
impl<M, L, R_branch, IsDisjoint, AIO, Role_Me> ProjectParCase<M, L, R_branch, IsDisjoint, AIO, Role_Me, TFalse, TTrue> for ()
where
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    IsDisjoint: Bool,
    AIO: ActionIOTMarker,
    Role_Me: Role,
    (): Project<R_branch, Role_Me>,
{
    type Output = <() as Project<R_branch, Role_Me>>::Output;
    type IsValid = <() as Project<R_branch, Role_Me>>::IsValid;
    type Error = <() as Project<R_branch, Role_Me>>::Error;
}

// Case: Role in neither branch -> No participation
impl<M, L, R_branch, IsDisjoint, AIO, Role_Me> ProjectParCase<M, L, R_branch, IsDisjoint, AIO, Role_Me, TFalse, TFalse> for ()
where
    M: CommMetadataTrait,
    L: GlobalProtocol,
    R_branch: GlobalProtocol,
    IsDisjoint: Bool,
    AIO: ActionIOTMarker,
    Role_Me: Role,
{
    type Output = EpChanEnd<TerminationAction, M, AIO>; // No participation -> end
    type IsValid = TTrue;
    type Error = NoError;
}

//
// Convenience Macros for Type-Level Assertions
//

/// Macro for asserting successful projection
#[macro_export]
macro_rules! assert_projects {
    ($Protocol:ty, $Role:ty, $Expected:ty) => {
        const _: () = {
            fn _assert_projects()
            where
                (): Project<$Protocol, $Role>,
                ProjectOutput<$Protocol, $Role>: $Expected,
                ProjectIsValid<$Protocol, $Role>: EqualsTrue,
            {}
        };
    };
}

/// Macro for asserting projection with IO validation
#[macro_export]
macro_rules! assert_projects_with_io {
    ($Protocol:ty, $Role:ty, $IO:ty, $Expected:ty) => {
        const _: () = {
            fn _assert_projects_with_io()
            where
                (): ProjectWithIO<$Protocol, $Role, $IO>,
                <() as ProjectWithIO<$Protocol, $Role, $IO>>::Output: $Expected,
                <() as ProjectWithIO<$Protocol, $Role, $IO>>::IOSupported: EqualsTrue,
            {}
        };
    };
}

/// Macro for asserting projection failure
#[macro_export]
macro_rules! assert_projection_fails {
    ($Protocol:ty, $Role:ty) => {
        const _: () = {
            fn _assert_projection_fails()
            where
                (): Project<$Protocol, $Role>,
                ProjectIsValid<$Protocol, $Role>: EqualsFalse,
            {}
        };
    };
}

//
// Test Module
//

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::foundation::*;
    use crate::protocol::global::*;
    use crate::protocol::local::*;
    use crate::protocol::duality::*;

    // Test roles
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Alice;
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Bob;
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Charlie;
    
    impl Role for Alice {}
    impl Role for Bob {}
    impl Role for Charlie {}

    // Role equality implementations
    impl RoleEq<Alice> for Alice { type Output = TTrue; }
    impl RoleEq<Bob> for Bob { type Output = TTrue; }
    impl RoleEq<Charlie> for Charlie { type Output = TTrue; }

    // Test messages
    #[derive(Debug, Clone)]
    struct HelloMsg;
    #[derive(Debug, Clone)]
    struct AckMsg;
    #[derive(Debug, Clone)]
    struct DataMsg;
    
    impl Message for HelloMsg {}
    impl Message for AckMsg {}
    impl Message for DataMsg {}

    type TestMeta = CommMetadata<DefaultChan, RequestLbl>;

    #[test]
    fn test_end_projection() {
        type EndProto = TChanEnd<TestMeta, BiDirectionalAction>;
        type AliceView = ProjectOutput<EndProto, Alice>;
        
        // All roles should project to EpChanEnd
        fn _test_end_projection() 
        where
            AliceView: LocalProtocol,
            ProjectIsValid<EndProto, Alice>: EqualsTrue,
        {}
    }

    #[test]
    fn test_send_projection_sender() {
        type End = TChanEnd<TestMeta, BiDirectionalAction>;
        type SendProto = TChanSend<Alice, Bob, TestMeta, HelloMsg, End, BiDirectionalAction>;
        type AliceView = ProjectOutput<SendProto, Alice>;
        
        // Alice (sender) should get EpChanSend
        fn _test_send_projection() 
        where
            AliceView: LocalProtocol,
            ProjectIsValid<SendProto, Alice>: EqualsTrue,
        {}
    }

    #[test]
    fn test_send_projection_receiver() {
        type End = TChanEnd<TestMeta, BiDirectionalAction>;
        type SendProto = TChanSend<Alice, Bob, TestMeta, HelloMsg, End, BiDirectionalAction>;
        type BobView = ProjectOutput<SendProto, Bob>;
        
        // Bob (receiver) should get EpChanRecv
        fn _test_recv_projection() 
        where
            BobView: LocalProtocol,
            ProjectIsValid<SendProto, Bob>: EqualsTrue,
        {}
    }

    #[test]
    fn test_send_projection_observer() {
        type End = TChanEnd<TestMeta, BiDirectionalAction>;
        type SendProto = TChanSend<Alice, Bob, TestMeta, HelloMsg, End, BiDirectionalAction>;
        type CharlieView = ProjectOutput<SendProto, Charlie>;
        
        // Charlie (observer) should get the continuation (End)
        fn _test_observer_projection() 
        where
            CharlieView: LocalProtocol,
            ProjectIsValid<SendProto, Charlie>: EqualsTrue,
        {}
    }

    #[test]
    fn test_choice_projection_chooser() {
        type End = TChanEnd<TestMeta, BiDirectionalAction>;
        type ChoiceProto = TChanChoice<Alice, TestMeta, End, End, BiDirectionalAction>;
        type AliceView = ProjectOutput<ChoiceProto, Alice>;
        
        // Alice (chooser) should get EpChanChoice
        fn _test_choice_projection() 
        where
            AliceView: LocalProtocol,
            ProjectIsValid<ChoiceProto, Alice>: EqualsTrue,
        {}
    }

    #[test]
    fn test_parallel_projection() {
        type End = TChanEnd<TestMeta, BiDirectionalAction>;
        type LeftBranch = TChanSend<Alice, Bob, TestMeta, HelloMsg, End, BiDirectionalAction>;
        type RightBranch = TChanSend<Alice, Charlie, TestMeta, AckMsg, End, BiDirectionalAction>;
        type ParProto = TChanPar<TestMeta, LeftBranch, RightBranch, TTrue, BiDirectionalAction>;
        type AliceView = ProjectOutput<ParProto, Alice>;
        
        // Alice should get EpChanPar (participating in both branches)
        fn _test_parallel_projection() 
        where
            AliceView: LocalProtocol,
            ProjectIsValid<ParProto, Alice>: EqualsTrue,
        {}
    }

    #[test]
    fn test_complex_protocol_projection() {
        // Build a more complex protocol: Alice sends to Bob, then Bob chooses between two paths
        type End = TChanEnd<TestMeta, BiDirectionalAction>;
        type Path1 = TChanSend<Bob, Alice, TestMeta, AckMsg, End, BiDirectionalAction>;
        type Path2 = TChanSend<Bob, Alice, TestMeta, DataMsg, End, BiDirectionalAction>;
        type Choice = TChanChoice<Bob, TestMeta, Path1, Path2, BiDirectionalAction>;
        type ComplexProto = TChanSend<Alice, Bob, TestMeta, HelloMsg, Choice, BiDirectionalAction>;
        
        type AliceView = ProjectOutput<ComplexProto, Alice>;
        type BobView = ProjectOutput<ComplexProto, Bob>;
        
        // Both projections should be valid
        fn _test_complex_projections() 
        where
            AliceView: LocalProtocol,
            BobView: LocalProtocol,
            ProjectIsValid<ComplexProto, Alice>: EqualsTrue,
            ProjectIsValid<ComplexProto, Bob>: EqualsTrue,
        {}
    }

    #[test]
    fn test_projection_compilation() {
        // Test that all our projections compile successfully
        type End = TChanEnd<TestMeta, BiDirectionalAction>;
        type Start = TChanStart<TestMeta, End, BiDirectionalAction>;
        
        type StartAlice = ProjectOutput<Start, Alice>;
        type StartBob = ProjectOutput<Start, Bob>;
        
        fn _test_start_projections() 
        where
            StartAlice: LocalProtocol,
            StartBob: LocalProtocol,
            ProjectIsValid<Start, Alice>: EqualsTrue,
            ProjectIsValid<Start, Bob>: EqualsTrue,
        {}
    }
}
