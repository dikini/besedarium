//! Projection traits and implementations
//!
//! Contains the core projection trait `ProjectRole` that maps global protocols
//! to local (endpoint) protocols for specific roles. This module serves as the main
//! entry point for the projection system.
//!
//! This module implements the role projection mechanism that maps global protocols
//! to local (endpoint) protocols for specific roles. The core trait is `ProjectRole`,
//! which recursively projects a global protocol into a local endpoint protocol from
//! the perspective of a specific role.
//!
//! The implementation uses helper traits that are defined in other modules:
//! - `ProjectSendCase` (in send.rs): For projecting send operations
//! - `ProjectRecvCase` (in recv.rs): For projecting receive operations
//!
//! The code is organized across multiple modules for better modularity and maintainability.

use crate::{
    protocol::{
        global::{
            TChanOffer, TChanPar, TChanRecv, TChanSend, TChanRec, TChanContinue, TSession as GlobalTSession, TStart as GlobalTStart, TEnd as GlobalTEnd,
        },
        local::{EpChoice, EpSession, EpRec, EpContinue, Role},
    },
    types::{Bool, ProtocolLabel, SessionType, True, False, CommMetadata}, 
    Disjoint, 
    RoleEq, // Import RoleEq from crate root as suggested
};

// Import helper projection traits from other modules
use super::end::ProjectEndCase; // For TEnd projection
use super::parallel::ProjectPar; // For TPar projection
use super::recv::ProjectRecvCase; // For TRecv projection
use super::send::ProjectSendCase; // For TSend projection
use super::start::ProjectStartCase; // For TStart projection

/// General projection trait for a role 'Me' in a global protocol 'Global'.
///
/// This trait recursively projects a global protocol (a type implementing `TSession`)
/// into a local endpoint protocol (a type implementing `EpSession`) for a specific role.
///
/// # Type Parameters
///
/// * `Me`: The role being projected
/// * `IO`: The session type marker (e.g., Http, Mqtt)
/// * `Global`: The global protocol to project
///
/// # Associated Types
///
/// * `Out`: The resulting local endpoint protocol for role `Me`
pub trait ProjectRole<Me, IO, Global>
where
    Me: Role,
    IO: SessionType, // IO is a marker like Http, Mqtt, etc.
    Global: GlobalTSession<IO>,
{
    /// The resulting local endpoint protocol for role `Me`
    type Out: EpSession<IO, Me>; // The result is a local endpoint session for Me over IO
}

// --- ProjectRole Implementations ---

// ProjectRole for TStart<IO, Lbl, S>
impl<Me, IO, Lbl, S> ProjectRole<Me, IO, GlobalTStart<IO, Lbl, S>> for ()
where
    Me: Role,
    IO: SessionType,
    Lbl: ProtocolLabel,
    S: GlobalTSession<IO>, // Removed ProjectRole<Me, IO, S> bound here, will be in ProjectStartCase
    // <S as ProjectRole<Me, IO, S>>::Out: EpSession<IO, Me>, // Moved to ProjectStartCase
    (): ProjectStartCase<Me, IO, Lbl, S>, // S needs ProjectRole bound inside ProjectStartCase
{
    // Delegate to ProjectStartCase trait for start projection
    type Out = <() as ProjectStartCase<Me, IO, Lbl, S>>::Out;
}

// ProjectRole for TEnd<IO, Lbl>
impl<Me, IO, Lbl> ProjectRole<Me, IO, GlobalTEnd<IO, Lbl>> for ()
where
    Me: Role,
    IO: SessionType,
    Lbl: ProtocolLabel,
    (): ProjectEndCase<Me, IO, Lbl>,
{
    // Delegate to ProjectEndCase trait for end projection
    type Out = <() as ProjectEndCase<Me, IO, Lbl>>::Out;
}

// ProjectRole for TChanSend<SndR, RcvR, M, Msg, G, AIO, IOSess>
impl<SndR, RcvR, M, Msg, GProto, AIO, IOSess, Me> ProjectRole<Me, IOSess, TChanSend<SndR, RcvR, M, Msg, GProto, AIO, IOSess>> for ()
where
    Me: Role,
    SndR: RoleMarker,
    RcvR: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static,
    Msg: core::fmt::Debug + Send + Sync + 'static,
    GProto: GlobalProtocol + GlobalTSession<IOSess>,
    AIO: ActionIOTMarker,
    IOSess: SessionType + SupportsActionIO<AIO>,
    GProto: ProjectRole<Me, IOSess, GProto>, // Recursive projection for GProto
    <GProto as ProjectRole<Me, IOSess, GProto>>::Out: EpSession<IOSess, Me>,
    Me: RoleEq<SndR>, // Required for case selection
    <Me as RoleEq<SndR>>::Output: Bool,
    (): ProjectSendCase<Me, IOSess, M, SndR, Msg, GProto, <Me as RoleEq<SndR>>::Output, AIO>, // Added AIO
{
    type Out = <() as ProjectSendCase<Me, IOSess, M, SndR, Msg, GProto, <Me as RoleEq<SndR>>::Output, AIO>>::Output;
}

// ProjectRole for TChanRecv<RcvR, SndR, M, Msg, G, AIO, IOSess>
impl<RcvR, SndR, M, Msg, GProto, AIO, IOSess, Me> ProjectRole<Me, IOSess, TChanRecv<RcvR, SndR, M, Msg, GProto, AIO, IOSess>> for ()
where
    Me: Role,
    RcvR: RoleMarker,
    SndR: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static,
    Msg: core::fmt::Debug + Send + Sync + 'static,
    GProto: GlobalProtocol + GlobalTSession<IOSess>,
    AIO: ActionIOTMarker,
    IOSess: SessionType + SupportsActionIO<AIO>,
    GProto: ProjectRole<Me, IOSess, GProto>,
    <GProto as ProjectRole<Me, IOSess, GProto>>::Out: EpSession<IOSess, Me>,
    Me: RoleEq<RcvR>,
    <Me as RoleEq<RcvR>>::Output: Bool,
    (): ProjectRecvCase<Me, IOSess, M, RcvR, Msg, GProto, <Me as RoleEq<RcvR>>::Output, AIO>, // Added AIO
{
    type Out = <() as ProjectRecvCase<Me, IOSess, M, RcvR, Msg, GProto, <Me as RoleEq<RcvR>>::Output, AIO>>::Output;
}


// ProjectRole for TChanPar<M, L, R, IsDisjoint, IO>
impl<Me, IO, M, G1, G2, IsDisjointFlag>
    ProjectRole<Me, IO, TChanPar<M, G1, G2, IsDisjointFlag, IO>> for ()
where
    Me: Role,
    IO: SessionType,
    M: core::fmt::Debug + Send + Sync + 'static + ProtocolLabel, // Added ProtocolLabel bound to M
    G1: GlobalProtocol + GlobalTSession<IO> + ProjectRole<Me, IO, G1>,
    G2: GlobalProtocol + GlobalTSession<IO> + ProjectRole<Me, IO, G2>,
    IsDisjointFlag: core::fmt::Debug + Send + Sync + 'static, 
    <G1 as ProjectRole<Me, IO, G1>>::Out: EpSession<IO, Me>,
    <G2 as ProjectRole<Me, IO, G2>>::Out: EpSession<IO, Me>,
    (): ProjectPar<
        Me,
        IO,
        M, // M is now bounded by ProtocolLabel
        <G1 as ProjectRole<Me, IO, G1>>::Out,
        <G2 as ProjectRole<Me, IO, G2>>::Out,
    >,
{
    type Out = <() as ProjectPar<
        Me,
        IO,
        M,
        <G1 as ProjectRole<Me, IO, G1>>::Out,
        <G2 as ProjectRole<Me, IO, G2>>::Out,
    >>::Out;
}

// ProjectRole for TChanOffer<ROfferer, RChooser, M, L, R, AIO, IO> (Binary Choice)
impl<Me, IO, ROfferer, RChooser, M, LeftBranch, RightBranch, AIO>
    ProjectRole<Me, IO, TChanOffer<ROfferer, RChooser, M, LeftBranch, RightBranch, AIO, IO>> for ()
where
    Me: Role,
    IO: SessionType + SupportsActionIO<AIO>,
    AIO: ActionIOTMarker,
    ROfferer: RoleMarker,
    RChooser: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static, // CommMetadata for Choice
    LeftBranch: GlobalProtocol + GlobalTSession<IO> + ProjectRole<Me, IO, LeftBranch>,
    RightBranch: GlobalProtocol + GlobalTSession<IO> + ProjectRole<Me, IO, RightBranch>,
    <LeftBranch as ProjectRole<Me, IO, LeftBranch>>::Out: EpSession<IO, Me>,
    <RightBranch as ProjectRole<Me, IO, RightBranch>>::Out: EpSession<IO, Me>,
    // Additional bounds might be needed for ProjectChoiceCase or similar helper
{
    // This needs to be more sophisticated, likely involving a ProjectChoiceCase helper
    // For now, assuming a direct EpChoice if Me is RChooser, or projection of branches.
    // This is a simplification and likely incorrect for the general case.
    // The actual projection depends on whether Me is ROfferer, RChooser, or neither.
    // A helper trait like ProjectChoiceRole (similar to ProjectSendCase/ProjectRecvCase)
    // would be needed to dispatch based on Me's relation to ROfferer and RChooser.
    //
    // Placeholder:
    type Out = EpChoice<
        IO,
        M, // Using CommMetadata M as Lbl for EpChoice
        Me,
        <LeftBranch as ProjectRole<Me, IO, LeftBranch>>::Out,
        <RightBranch as ProjectRole<Me, IO, RightBranch>>::Out,
    >;
}

// ProjectRole for TChanRec<RecLbl, S, IO>
impl<Me, IO, RecLbl, S> ProjectRole<Me, IO, TChanRec<RecLbl, S, IO>> for ()
where
    Me: Role,
    IO: SessionType,
    RecLbl: ProtocolLabel,
    S: GlobalProtocol + GlobalTSession<IO> + ProjectRole<Me, IO, S>,
    <S as ProjectRole<Me, IO, S>>::Out: EpSession<IO, Me>,
{
    type Out = EpRec<IO, RecLbl, Me, <S as ProjectRole<Me, IO, S>>::Out>;
}

// ProjectRole for TChanContinue<RecLbl, IO>
impl<Me, IO, RecLbl> ProjectRole<Me, IO, TChanContinue<RecLbl, IO>> for ()
where
    Me: Role,
    IO: SessionType,
    RecLbl: ProtocolLabel,
{
    type Out = EpContinue<IO, RecLbl, Me>;
}

// Need to import RoleMarker, ActionIOTMarker, GlobalProtocol, SupportsActionIO
use crate::types::{RoleMarker, ActionIOTMarker, GlobalProtocol, SupportsActionIO};
