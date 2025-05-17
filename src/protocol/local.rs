//! # Local Protocol Types
//!
//! This module contains the local protocol session types that model
//! the behavior of individual participants in a communication protocol.
//! These types represent the endpoint-level view of interactions.
//!
//! Key components:
//!
//! - `EpSession`: Core trait for all local session types
//! - `EpSend`: Endpoint sending operation
//! - `EpRecv`: Endpoint receiving operation
//! - `EpChoice`: Endpoint protocol choice
//! - `EpPar`: Endpoint parallel composition
//! - `EpEnd`: Endpoint protocol termination
//! - `EpSkip`: No-op type for roles not involved in a branch
//!
//! Local protocols are derived from global protocols through projection
//! onto specific roles. They describe the sequence of operations that
//! an individual participant must perform.

use crate::sealed;
use crate::types;
use core::marker::PhantomData;

/// Concrete roles for protocols and tests
pub struct TClient;
pub struct TServer;
pub struct TBroker;
pub struct TWorker;

/// Placeholder parameter for protocol handlers
/// Never actually used at runtime, just for type-level protocol descriptors
pub struct Void;

/// Marker trait for protocol participants (roles).
///
/// Implement this trait for each participant in your protocol.
pub trait Role {}
impl Role for TClient {}
impl Role for TServer {}
impl Role for TBroker {}
impl Role for TWorker {}
impl Role for Void {}

/// Type-level equality for roles.
///
/// Used to determine if two roles are the same at compile time (for projection).
pub trait RoleEq<R> {
    type Output;
}

/// Trait for all local (endpoint) session types.
///
/// - `IO`: Protocol marker type.
/// - `R`: Role being projected.
pub trait EpSession<IO, R>: sealed::Sealed {}

/// Endpoint type for sending a message in a local protocol.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this interaction (for traceability and debugging).
/// - `R`: Role performing the send.
/// - `H`: Message type being sent.
/// - `T`: Continuation after sending.
pub struct EpSend<IO, Lbl: types::ProtocolLabel, R, H, T>(PhantomData<(IO, Lbl, R, H, T)>);
impl<IO, Lbl: types::ProtocolLabel, R, H, T> EpSession<IO, R> for EpSend<IO, Lbl, R, H, T> {}
impl<IO, Lbl: types::ProtocolLabel, R, H, T> sealed::Sealed for EpSend<IO, Lbl, R, H, T> {}

/// Endpoint type for receiving a message in a local protocol.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this interaction (for traceability and debugging).
/// - `R`: Role performing the receive.
/// - `H`: Message type being received.
/// - `T`: Continuation after receiving.
pub struct EpRecv<IO, Lbl: types::ProtocolLabel, R, H, T>(PhantomData<(IO, Lbl, R, H, T)>);
impl<IO, Lbl: types::ProtocolLabel, R, H, T> EpSession<IO, R> for EpRecv<IO, Lbl, R, H, T> {}
impl<IO, Lbl: types::ProtocolLabel, R, H, T> sealed::Sealed for EpRecv<IO, Lbl, R, H, T> {}

/// Endpoint type for protocol termination in a local protocol.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this endpoint (for traceability and debugging).
/// - `R`: Role for which the protocol ends.
pub struct EpEnd<IO, Lbl: types::ProtocolLabel, R>(PhantomData<(IO, Lbl, R)>);
impl<IO, Lbl: types::ProtocolLabel, R> EpSession<IO, R> for EpEnd<IO, Lbl, R> {}
impl<IO, Lbl: types::ProtocolLabel, R> sealed::Sealed for EpEnd<IO, Lbl, R> {}

/// Endpoint type for local protocol branching (choice/offer).
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this choice (for traceability and debugging).
/// - `Me`: The role being projected.
/// - `L`, `R`: The two local protocol branches.
pub struct EpChoice<IO, Lbl: types::ProtocolLabel, Me, L, R>(PhantomData<(IO, Lbl, Me, L, R)>);
impl<IO, Lbl: types::ProtocolLabel, Me, L, R> EpSession<IO, Me> for EpChoice<IO, Lbl, Me, L, R> {}
impl<IO, Lbl: types::ProtocolLabel, Me, L, R> sealed::Sealed for EpChoice<IO, Lbl, Me, L, R> {}

/// Endpoint type for local protocol parallel composition.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this parallel composition (for traceability and debugging).
/// - `Me`: The role being projected.
/// - `L`, `R`: The two local protocol branches.
pub struct EpPar<IO, Lbl: types::ProtocolLabel, Me, L, R>(PhantomData<(IO, Lbl, Me, L, R)>);
impl<IO, Lbl: types::ProtocolLabel, Me, L, R> EpSession<IO, Me> for EpPar<IO, Lbl, Me, L, R> {}
impl<IO, Lbl: types::ProtocolLabel, Me, L, R> sealed::Sealed for EpPar<IO, Lbl, Me, L, R> {}

/// No-op endpoint type for roles uninvolved in a protocol branch.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this skip operation (for traceability and debugging).
/// - `R`: Role that is skipping this branch.
///
/// Used to improve type-level precision for projections.
pub struct EpSkip<IO, Lbl: types::ProtocolLabel, R>(PhantomData<(IO, Lbl, R)>);
impl<IO, Lbl: types::ProtocolLabel, R> EpSession<IO, R> for EpSkip<IO, Lbl, R> {}
impl<IO, Lbl: types::ProtocolLabel, R> sealed::Sealed for EpSkip<IO, Lbl, R> {}

/// Type-level marker types for dispatch
pub struct IsEpSkipType;
pub struct IsNotEpSkipType;

/// Implementation marker trait for EpSkip dispatch
pub trait IsEpSkipTypeImpl<IO, Me: Role> {
    type TypeMarker;
}

// EpSkip maps to IsEpSkipType
impl<IO, Lbl: types::ProtocolLabel, Me: Role> IsEpSkipTypeImpl<IO, Me> for EpSkip<IO, Lbl, Me> {
    type TypeMarker = IsEpSkipType;
}

// All other EpSession<IO, Me> types map to IsNotEpSkipType
impl<IO, Lbl: types::ProtocolLabel, Me: Role, H, T> IsEpSkipTypeImpl<IO, Me> for EpSend<IO, Lbl, Me, H, T> {
    type TypeMarker = IsNotEpSkipType;
}
impl<IO, Lbl: types::ProtocolLabel, Me: Role, H, T> IsEpSkipTypeImpl<IO, Me> for EpRecv<IO, Lbl, Me, H, T> {
    type TypeMarker = IsNotEpSkipType;
}
impl<IO, Lbl: types::ProtocolLabel, MeChoice: Role, L, R> IsEpSkipTypeImpl<IO, MeChoice> for EpChoice<IO, Lbl, MeChoice, L, R> {
    type TypeMarker = IsNotEpSkipType;
}
impl<IO, Lbl: types::ProtocolLabel, MePar: Role, L, R> IsEpSkipTypeImpl<IO, MePar> for EpPar<IO, Lbl, MePar, L, R> {
    type TypeMarker = IsNotEpSkipType;
}
impl<IO, Lbl: types::ProtocolLabel, Me: Role> IsEpSkipTypeImpl<IO, Me> for EpEnd<IO, Lbl, Me> {
    type TypeMarker = IsNotEpSkipType;
}

/// Traits for checking if an endpoint type is a specific variant
///
/// Trait to check if a type is an EpSkip variant
pub trait IsEpSkipVariant<IO, Me: Role> {
    type Output: types::Bool;
}

/// Trait to check if a type is an EpEnd variant
pub trait IsEpEndVariant<IO, Me: Role> {
    type Output: types::Bool;
}

// Implementations for IsEpSkipVariant
impl<IO, Lbl: types::ProtocolLabel, Me: Role> IsEpSkipVariant<IO, Me> for EpSkip<IO, Lbl, Me> {
    type Output = types::True;
}
impl<IO, Lbl: types::ProtocolLabel, R, H, T, Me: Role> IsEpSkipVariant<IO, Me> for EpSend<IO, Lbl, R, H, T> {
    type Output = types::False;
}
impl<IO, Lbl: types::ProtocolLabel, R, H, T, Me: Role> IsEpSkipVariant<IO, Me> for EpRecv<IO, Lbl, R, H, T> {
    type Output = types::False;
}
impl<IO, Lbl: types::ProtocolLabel, MeChoice: Role, L, R, MeFilter: Role> IsEpSkipVariant<IO, MeFilter>
    for EpChoice<IO, Lbl, MeChoice, L, R>
{
    type Output = types::False;
}
impl<IO, Lbl: types::ProtocolLabel, MePar: Role, L, R, MeFilter: Role> IsEpSkipVariant<IO, MeFilter>
    for EpPar<IO, Lbl, MePar, L, R>
{
    type Output = types::False;
}
impl<IO, Lbl: types::ProtocolLabel, MeEnd: Role, MeFilter: Role> IsEpSkipVariant<IO, MeFilter> for EpEnd<IO, Lbl, MeEnd> {
    type Output = types::False;
}

// Implementations for IsEpEndVariant
impl<IO, Lbl: types::ProtocolLabel, Me: Role> IsEpEndVariant<IO, Me> for EpEnd<IO, Lbl, Me> {
    type Output = types::True;
}
impl<IO, Lbl: types::ProtocolLabel, R, H, T, Me: Role> IsEpEndVariant<IO, Me> for EpSend<IO, Lbl, R, H, T> {
    type Output = types::False;
}
impl<IO, Lbl: types::ProtocolLabel, R, H, T, Me: Role> IsEpEndVariant<IO, Me> for EpRecv<IO, Lbl, R, H, T> {
    type Output = types::False;
}
impl<IO, Lbl: types::ProtocolLabel, MeChoice: Role, L, R, MeFilter: Role> IsEpEndVariant<IO, MeFilter>
    for EpChoice<IO, Lbl, MeChoice, L, R>
{
    type Output = types::False;
}
impl<IO, Lbl: types::ProtocolLabel, MePar: Role, L, R, MeFilter: Role> IsEpEndVariant<IO, MeFilter>
    for EpPar<IO, Lbl, MePar, L, R>
{
    type Output = types::False;
}
impl<IO, Lbl: types::ProtocolLabel, MeSkip: Role, MeFilter: Role> IsEpEndVariant<IO, MeFilter> for EpSkip<IO, Lbl, MeSkip> {
    type Output = types::False;
}

/// IsSkip: True if T is EpSkip<IO, Me>, else False.
pub type IsSkip<T, IO, Me> = <T as IsEpSkipVariant<IO, Me>>::Output;

/// IsEnd: True if T is EpEnd<IO, Me>, else False.
pub type IsEnd<T, IO, Me> = <T as IsEpEndVariant<IO, Me>>::Output;

/// Public facade trait that routes to the implementation trait
pub trait GetEpSkipTypeMarker<IO, Me: Role> {
    type TypeMarker;
}

impl<IO, Me: Role, T> GetEpSkipTypeMarker<IO, Me> for T
where
    T: IsEpSkipTypeImpl<IO, Me>,
{
    type TypeMarker = <T as IsEpSkipTypeImpl<IO, Me>>::TypeMarker;
}
