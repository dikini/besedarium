// filepath: /home/dikini/Projects/besedarium/src/protocol/transforms/projection.rs
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
            TChoice as GlobalTChoice, TEnd as GlobalTEnd, TPar as GlobalTPar, TRecv as GlobalTRecv,
            TSend as GlobalTSend, TSession as GlobalTSession,
        },
        local::RoleEq,
        local::{EpChoice, EpEnd, EpSession},
    },
    types::{Bool, ProtocolLabel, SessionType},
    Disjoint, Role,
};

// Import helper projection traits from other modules
use super::parallel::ProjectPar; // For TPar projection
use super::recv::ProjectRecvCase;
use super::send::ProjectSendCase; // For TSend projection // For TRecv projection

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

// ProjectRole for TEnd<IO, Lbl>
impl<Me, IO, Lbl> ProjectRole<Me, IO, GlobalTEnd<IO, Lbl>> for ()
where
    Me: Role,
    IO: SessionType,
    Lbl: ProtocolLabel,
{
    type Out = EpEnd<IO, Lbl, Me>; // EpEnd<IO, Label, Role>
}

// ProjectRole for TSend<IO, Lbl, RSender, P, G>
impl<Me, IO, Lbl, RSender, P, G> ProjectRole<Me, IO, GlobalTSend<IO, Lbl, RSender, P, G>> for ()
where
    Me: Role,
    IO: SessionType,
    Lbl: ProtocolLabel,
    RSender: Role,
    P: Send + 'static,
    G: GlobalTSession<IO> + ProjectRole<Me, IO, G>,
    <G as ProjectRole<Me, IO, G>>::Out: EpSession<IO, Me>,
    Me: RoleEq<RSender>, // Required for case selection
    <Me as RoleEq<RSender>>::Output: Bool,
    (): ProjectSendCase<Me, IO, Lbl, RSender, P, G, <Me as RoleEq<RSender>>::Output>,
{
    // Delegate to ProjectSendCase trait for role-based case analysis
    type Out = <() as ProjectSendCase<
        Me,
        IO,
        Lbl,
        RSender,
        P,
        G,
        <Me as RoleEq<RSender>>::Output,
    >>::Output;
}

// ProjectRole for TRecv<IO, Lbl, RReceiver, P, G>
impl<Me, IO, Lbl, RReceiver, P, G> ProjectRole<Me, IO, GlobalTRecv<IO, Lbl, RReceiver, P, G>> for ()
where
    Me: Role,
    IO: SessionType,
    Lbl: ProtocolLabel,
    RReceiver: Role,
    P: Send + 'static,
    G: GlobalTSession<IO> + ProjectRole<Me, IO, G>,
    <G as ProjectRole<Me, IO, G>>::Out: EpSession<IO, Me>,
    Me: RoleEq<RReceiver>, // Required for case selection
    <Me as RoleEq<RReceiver>>::Output: Bool,
    (): ProjectRecvCase<Me, IO, Lbl, RReceiver, P, G, <Me as RoleEq<RReceiver>>::Output>,
{
    // Delegate to ProjectRecvCase trait for role-based case analysis
    type Out = <() as ProjectRecvCase<
        Me,
        IO,
        Lbl,
        RReceiver,
        P,
        G,
        <Me as RoleEq<RReceiver>>::Output,
    >>::Output;
}

// ProjectRole for TPar<IO, Lbl, G1, G2, IsDisjointFlag>
impl<Me, IO, Lbl, G1, G2, IsDisjointFlag>
    ProjectRole<Me, IO, GlobalTPar<IO, Lbl, G1, G2, IsDisjointFlag>> for ()
where
    Me: Role,
    IO: SessionType,
    Lbl: ProtocolLabel,
    G1: GlobalTSession<IO> + ProjectRole<Me, IO, G1>, // Global protocol G1
    G2: GlobalTSession<IO> + ProjectRole<Me, IO, G2>, // Global protocol G2
    IsDisjointFlag: Disjoint<G1, G2>, // Marker ensuring G1 and G2 are disjoint for parallel composition
    <G1 as ProjectRole<Me, IO, G1>>::Out: EpSession<IO, Me>,
    <G2 as ProjectRole<Me, IO, G2>>::Out: EpSession<IO, Me>,
    // Use ProjectPar to project the parallel branches together
    (): ProjectPar<
        Me,
        IO,
        Lbl,
        <G1 as ProjectRole<Me, IO, G1>>::Out,
        <G2 as ProjectRole<Me, IO, G2>>::Out,
    >,
{
    type Out = <() as ProjectPar<
        Me,
        IO,
        Lbl,
        <G1 as ProjectRole<Me, IO, G1>>::Out,
        <G2 as ProjectRole<Me, IO, G2>>::Out,
    >>::Out;
}

// ProjectRole for TChoice<IO, Lbl, LeftBranch, RightBranch> (Binary Choice)
impl<Me, IO, Lbl, LeftBranch, RightBranch>
    ProjectRole<Me, IO, GlobalTChoice<IO, Lbl, LeftBranch, RightBranch>> for ()
where
    Me: Role,
    IO: SessionType,
    Lbl: ProtocolLabel,
    LeftBranch: GlobalTSession<IO> + ProjectRole<Me, IO, LeftBranch>,
    RightBranch: GlobalTSession<IO> + ProjectRole<Me, IO, RightBranch>,
    <LeftBranch as ProjectRole<Me, IO, LeftBranch>>::Out: EpSession<IO, Me>,
    <RightBranch as ProjectRole<Me, IO, RightBranch>>::Out: EpSession<IO, Me>,
{
    type Out = EpChoice<
        IO,
        Lbl,
        Me, // The role 'Me' is making a local choice
        <LeftBranch as ProjectRole<Me, IO, LeftBranch>>::Out,
        <RightBranch as ProjectRole<Me, IO, RightBranch>>::Out,
    >;
}
