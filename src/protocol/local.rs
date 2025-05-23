//! # Local Protocol Types
//!
//! This module contains the local protocol session types that model
//! the behavior of individual participants in a communication protocol.
//! These types represent the endpoint-level view of interactions.
//!
//! Key components:
//!
//! - `EpSession`: Core trait for all local session types
//! - `EpStart`: Protocol entry point
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
//!
//! # Protocol Label Invariant
//! All local endpoint combinators (EpSend, EpRecv, EpEnd, EpChoice, EpPar, EpSkip)
//! must have a label parameter and implement the GetProtocolLabel trait.
//! This enables type-level extraction and reasoning about protocol structure and
//! label preservation throughout all protocol transformations.
//endregion

use crate::{
    protocol::{
        EpChoice as BaseEpChoice, EpEnd as BaseEpEnd, EpOffer as BaseEpOffer, EpPar as BaseEpPar,
        EpRec as BaseEpRec, EpRecv as BaseEpRecv, EpSend as BaseEpSend, EpSkip as BaseEpSkip,
        Message, ProtocolLabel, RecvKindMarker, RoleMarker, SendKindMarker, SessionIO, TSession,
    },
    sealed, // Added crate::sealed
    types::{self, Bool, False, NoLabel, True}, // Corrected paths for types
};
use core::marker::PhantomData;

/// Core trait for all local session types.
///
/// - `IO`: Protocol marker type (e.g., Http, Mqtt).
/// - `Me`: The role this endpoint belongs to.
/// - Implemented by all local protocol combinators.
pub trait EpSession<IO, Me>: sealed::Sealed {}

/// Local protocol entry point.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this start point.
/// - `Me`: The role this endpoint belongs to.
/// - `S`: Continuation local protocol.
pub struct EpStart<IO, Lbl, Me, S> {
    _io: PhantomData<IO>,
    _lbl: PhantomData<Lbl>,
    _me: PhantomData<Me>,
    _s: PhantomData<S>,
}

impl<IO, Lbl, Me, S> sealed::Sealed for EpStart<IO, Lbl, Me, S> {}
impl<IO, Lbl, Me, S> EpSession<IO, Me> for EpStart<IO, Lbl, Me, S> {}

/// Endpoint sending operation.
///
/// - `IO`: Protocol marker type.
/// - `M`: Communication metadata (`CommMetadata`).
/// - `RMe`: The role this endpoint belongs to (sender).
/// - `RPeer`: The peer role (receiver).
/// - `Msg`: Message type being sent.
/// - `G`: Continuation local protocol.
pub struct EpSend<IO, M, RMe, RPeer, Msg, G> {
    _io: PhantomData<IO>,
    _m: PhantomData<M>,
    _me: PhantomData<RMe>,
    _peer: PhantomData<RPeer>,
    _msg: PhantomData<Msg>,
    _g: PhantomData<G>,
}

impl<IO, M, RMe, RPeer, Msg, G> sealed::Sealed for EpSend<IO, M, RMe, RPeer, Msg, G> {}
impl<IO, M, RMe, RPeer, Msg, G> EpSession<IO, RMe> for EpSend<IO, M, RMe, RPeer, Msg, G> {}

/// Endpoint receiving operation.
///
/// - `IO`: Protocol marker type.
/// - `M`: Communication metadata (`CommMetadata`).
/// - `RMe`: The role this endpoint belongs to (receiver).
/// - `RPeer`: The peer role (sender).
/// - `Msg`: Message type being received.
/// - `G`: Continuation local protocol.
pub struct EpRecv<IO, M, RMe, RPeer, Msg, G> {
    _io: PhantomData<IO>,
    _m: PhantomData<M>,
    _me: PhantomData<RMe>,
    _peer: PhantomData<RPeer>,
    _msg: PhantomData<Msg>,
    _g: PhantomData<G>,
}

impl<IO, M, RMe, RPeer, Msg, G> sealed::Sealed for EpRecv<IO, M, RMe, RPeer, Msg, G> {}
impl<IO, M, RMe, RPeer, Msg, G> EpSession<IO, RMe> for EpRecv<IO, M, RMe, RPeer, Msg, G> {}

/// Endpoint protocol choice.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this choice point.
/// - `Me`: The role this endpoint belongs to.
/// - `L`: Local protocol for the left branch.
/// - `R`: Local protocol for the right branch.
pub struct EpChoice<IO, Lbl, Me, L, R> {
    _io: PhantomData<IO>,
    _lbl: PhantomData<Lbl>,
    _me: PhantomData<Me>,
    _l: PhantomData<L>,
    _r: PhantomData<R>,
}

impl<IO, Lbl, Me, L, R> sealed::Sealed for EpChoice<IO, Lbl, Me, L, R> {}
impl<IO, Lbl, Me, L, R> EpSession<IO, Me> for EpChoice<IO, Lbl, Me, L, R> {}

/// Endpoint parallel composition.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this parallel composition.
/// - `Me`: The role this endpoint belongs to.
/// - `L`: Left local protocol branch.
/// - `R`: Right local protocol branch.
pub struct EpPar<IO, Lbl, Me, L, R> {
    _io: PhantomData<IO>,
    _lbl: PhantomData<Lbl>,
    _me: PhantomData<Me>,
    _l: PhantomData<L>,
    _r: PhantomData<R>,
}

impl<IO, Lbl, Me, L, R> sealed::Sealed for EpPar<IO, Lbl, Me, L, R> {}
impl<IO, Lbl, Me, L, R> EpSession<IO, Me> for EpPar<IO, Lbl, Me, L, R> {}

/// Endpoint protocol termination.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this end point.
/// - `Me`: The role this endpoint belongs to.
pub struct EpEnd<IO, Lbl, Me> {
    _io: PhantomData<IO>,
    _lbl: PhantomData<Lbl>,
    _me: PhantomData<Me>,
}

impl<IO, Lbl, Me> sealed::Sealed for EpEnd<IO, Lbl, Me> {}
impl<IO, Lbl, Me> EpSession<IO, Me> for EpEnd<IO, Lbl, Me> {}

/// No-op type for roles not involved in a branch or action.
///
/// - `IO`: Protocol marker type.
/// - `Me`: The role this endpoint belongs to.
pub struct EpSkip<IO, Me> {
    _io: PhantomData<IO>,
    _me: PhantomData<Me>,
}

impl<IO, Me> sealed::Sealed for EpSkip<IO, Me> {}
impl<IO, Me> EpSession<IO, Me> for EpSkip<IO, Me> {}

/// Local recursion point.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this recursion point.
/// - `Me`: The role this endpoint belongs to.
/// - `S`: Local protocol body.
pub struct EpRec<IO, Lbl, Me, S> {
    _io: PhantomData<IO>,
    _lbl: PhantomData<Lbl>,
    _me: PhantomData<Me>,
    _s: PhantomData<S>,
}

impl<IO, Lbl, Me, S> sealed::Sealed for EpRec<IO, Lbl, Me, S> {}
impl<IO, Lbl, Me, S> EpSession<IO, Me> for EpRec<IO, Lbl, Me, S> {}

/// Local continue to a recursion point.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label of the `EpRec` to continue to.
/// - `Me`: The role this endpoint belongs to.
pub struct EpContinue<IO, Lbl, Me> {
    _io: PhantomData<IO>,
    _lbl: PhantomData<Lbl>,
    _me: PhantomData<Me>,
}

impl<IO, Lbl, Me> sealed::Sealed for EpContinue<IO, Lbl, Me> {}
impl<IO, Lbl, Me> EpSession<IO, Me> for EpContinue<IO, Lbl, Me> {}

/// Represents a role in a protocol.
///
/// This trait is used to mark types that can represent roles.
pub trait Role: sealed::Sealed + core::fmt::Debug + Send + Sync + 'static {}

/// Represents a silent action for a role (internal computation).
///
/// - `IO`: Protocol marker type.
/// - `Me`: The role this endpoint belongs to.
pub struct EpSilent<IO, Me> {
    _io: PhantomData<IO>,
    _me: PhantomData<Me>,
}

impl<IO, Me> sealed::Sealed for EpSilent<IO, Me> {}
impl<IO, Me> EpSession<IO, Me> for EpSilent<IO, Me> {}

/// --- GetProtocolLabel ---

/// Extracts the protocol label from an endpoint type.
pub trait GetProtocolLabel {
    type Output: ProtocolLabel;
}

// Implementations for endpoint types
impl<IO: SessionIO, M: SendKindMarker + ProtocolLabel, RMe: RoleMarker, RPeer: RoleMarker, MsgBody: Message, G: TSession<IO = IO>> GetProtocolLabel
    for EpSend<IO, M, RMe, RPeer, MsgBody, G>
{
    type Output = M;
}

impl<IO: SessionIO, M: RecvKindMarker + ProtocolLabel, RMe: RoleMarker, RPeer: RoleMarker, MsgBody: Message, G: TSession<IO = IO>> GetProtocolLabel
    for EpRecv<IO, M, RMe, RPeer, MsgBody, G>
{
    type Output = M;
}

impl<IO: SessionIO, Me: RoleMarker> GetProtocolLabel for EpSkip<IO, Me> {
    type Output = NoLabel; // Use crate::types::NoLabel, already imported as NoLabel
}

/// Implements the protocol label invariant for EpSkip.
/// See: Protocol Label Invariant in project documentation.
impl<IO, Me> crate::protocol::transforms::GetProtocolLabel
    for EpSkip<IO, Me>
where
    IO: SessionIO,
    Me: RoleMarker,
{
    type Label = NoLabel; // Use crate::types::NoLabel
}

/// Implements the protocol label invariant for EpSend.
impl<IO, M, RMe, RPeer, Msg, G> crate::protocol::transforms::GetProtocolLabel
    for EpSend<IO, M, RMe, RPeer, Msg, G>
where
    IO: SessionIO,
    M: SendKindMarker + types::ProtocolLabel, // M is the label
    RMe: RoleMarker,
    RPeer: RoleMarker,
    Msg: Message,
    G: EpSession<IO, RMe>, // G is a local session for RMe
{
    type Label = M;
}

/// Implements the protocol label invariant for EpRecv.
impl<IO, M, RMe, RPeer, Msg, G> crate::protocol::transforms::GetProtocolLabel
    for EpRecv<IO, M, RMe, RPeer, Msg, G>
where
    IO: SessionIO,
    M: RecvKindMarker + types::ProtocolLabel, // M is the label
    RMe: RoleMarker,
    RPeer: RoleMarker,
    Msg: Message,
    G: EpSession<IO, RMe>, // G is a local session for RMe
{
    type Label = M;
}

/// Implements the protocol label invariant for EpEnd.
impl<IO, Lbl, Me> crate::protocol::transforms::GetProtocolLabel
    for BaseEpEnd<IO, Lbl, Me> // Using BaseEpEnd as EpEnd is defined locally
where
    IO: SessionIO,
    Lbl: types::ProtocolLabel,
    Me: RoleMarker,
{
    type Label = Lbl;
}

/// Implements the protocol label invariant for EpChoice.
impl<IO, Lbl, Me, L, R> crate::protocol::transforms::GetProtocolLabel
    for BaseEpChoice<IO, Lbl, Me, L, R> // Using BaseEpChoice
where
    IO: SessionIO,
    Lbl: types::ProtocolLabel,
    Me: RoleMarker,
    L: EpSession<IO, Me>,
    R: EpSession<IO, Me>,
{
    type Label = Lbl;
}

/// Implements the protocol label invariant for EpPar.
impl<IO, Lbl, Me, L, R> crate::protocol::transforms::GetProtocolLabel
    for BaseEpPar<IO, Lbl, Me, L, R> // Using BaseEpPar
where
    IO: SessionIO,
    Lbl: types::ProtocolLabel,
    Me: RoleMarker,
    L: EpSession<IO, Me>,
    R: EpSession<IO, Me>,
{
    type Label = Lbl;
}

/// Implements the protocol label invariant for EpStart.
impl<IO, Lbl, Me, S> crate::protocol::transforms::GetProtocolLabel
    for EpStart<IO, Lbl, Me, S>
where
    IO: SessionIO,
    Lbl: types::ProtocolLabel,
    Me: RoleMarker,
    S: EpSession<IO, Me>,
{
    type Label = Lbl;
}

/// Type-level marker types for dispatch
pub struct IsEpSkipType;
pub struct IsNotEpSkipType;

/// Implementation marker trait for EpSkip dispatch
pub trait IsEpSkipTypeImpl<IO, Me: Role> {
    type TypeMarker;
}

// EpSkip maps to IsEpSkipType
impl<IO, Me: Role> IsEpSkipTypeImpl<IO, Me> for EpSkip<IO, Me>
where
    IO: SessionIO,
{
    type TypeMarker = IsEpSkipType;
}

// All other EpSession<IO, Me> types map to IsNotEpSkipType
impl<IO, M, RMe: Role, RPeer, Msg, G> IsEpSkipTypeImpl<IO, RMe>
    for EpSend<IO, M, RMe, RPeer, Msg, G>
where
    IO: SessionIO,
    M: SendKindMarker + types::ProtocolLabel,
    RPeer: RoleMarker,
    Msg: Message,
    G: EpSession<IO, RMe>,
{
    type TypeMarker = IsNotEpSkipType;
}

impl<IO, M, RMe: Role, RPeer, Msg, G> IsEpSkipTypeImpl<IO, RMe>
    for EpRecv<IO, M, RMe, RPeer, Msg, G>
where
    IO: SessionIO,
    M: RecvKindMarker + types::ProtocolLabel,
    RPeer: RoleMarker,
    Msg: Message,
    G: EpSession<IO, RMe>,
{
    type TypeMarker = IsNotEpSkipType;
}

impl<IO, Lbl, MeChoice: Role, L, R> IsEpSkipTypeImpl<IO, MeChoice>
    for EpChoice<IO, Lbl, MeChoice, L, R>
where
    IO: SessionIO,
    Lbl: types::ProtocolLabel,
    L: EpSession<IO, MeChoice>,
    R: EpSession<IO, MeChoice>,
{
    type TypeMarker = IsNotEpSkipType;
}

impl<IO, Lbl, MePar: Role, L, R> IsEpSkipTypeImpl<IO, MePar>
    for EpPar<IO, Lbl, MePar, L, R>
where
    IO: SessionIO,
    Lbl: types::ProtocolLabel,
    L: EpSession<IO, MePar>,
    R: EpSession<IO, MePar>,
{
    type TypeMarker = IsNotEpSkipType;
}

impl<IO, Lbl, Me: Role> IsEpSkipTypeImpl<IO, Me> for EpEnd<IO, Lbl, Me>
where
    IO: SessionIO,
    Lbl: types::ProtocolLabel,
{
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
impl<IO, Me: Role> IsEpSkipVariant<IO, Me> for EpSkip<IO, Me>
where
    IO: SessionIO,
{
    type Output = types::True;
}

impl<IO, M, RMe: Role, RPeer, Msg, G> IsEpSkipVariant<IO, RMe>
    for EpSend<IO, M, RMe, RPeer, Msg, G>
where
    IO: SessionIO,
    M: SendKindMarker + types::ProtocolLabel,
    RPeer: RoleMarker,
    Msg: Message,
    G: EpSession<IO, RMe>,
{
    type Output = types::False;
}

impl<IO, M, RMe: Role, RPeer, Msg, G> IsEpSkipVariant<IO, RMe>
    for EpRecv<IO, M, RMe, RPeer, Msg, G>
where
    IO: SessionIO,
    M: RecvKindMarker + types::ProtocolLabel,
    RPeer: RoleMarker,
    Msg: Message,
    G: EpSession<IO, RMe>,
{
    type Output = types::False;
}

impl<IO, Lbl, MeChoice: Role, L, R> IsEpSkipVariant<IO, MeChoice>
    for EpChoice<IO, Lbl, MeChoice, L, R>
where
    IO: SessionIO,
    Lbl: types::ProtocolLabel,
    L: EpSession<IO, MeChoice>,
    R: EpSession<IO, MeChoice>,
{
    type Output = types::False;
}

impl<IO, Lbl, MePar: Role, L, R> IsEpSkipVariant<IO, MePar>
    for EpPar<IO, Lbl, MePar, L, R>
where
    IO: SessionIO,
    Lbl: types::ProtocolLabel,
    L: EpSession<IO, MePar>,
    R: EpSession<IO, MePar>,
{
    type Output = types::False;
}

impl<IO, Lbl, MeEnd: Role> IsEpSkipVariant<IO, MeEnd> for EpEnd<IO, Lbl, MeEnd>
where
    IO: SessionIO,
    Lbl: types::ProtocolLabel,
{
    type Output = types::False;
}

// Implementations for IsEpEndVariant
impl<IO, Lbl, Me: Role> IsEpEndVariant<IO, Me> for EpEnd<IO, Lbl, Me>
where
    IO: SessionIO,
    Lbl: types::ProtocolLabel,
{
    type Output = types::True;
}

impl<IO, M, RMe: Role, RPeer, Msg, G> IsEpEndVariant<IO, RMe>
    for EpSend<IO, M, RMe, RPeer, Msg, G>
where
    IO: SessionIO,
    M: SendKindMarker + types::ProtocolLabel,
    RPeer: RoleMarker,
    Msg: Message,
    G: EpSession<IO, RMe>,
{
    type Output = types::False;
}

impl<IO, M, RMe: Role, RPeer, Msg, G> IsEpEndVariant<IO, RMe>
    for EpRecv<IO, M, RMe, RPeer, Msg, G>
where
    IO: SessionIO,
    M: RecvKindMarker + types::ProtocolLabel,
    RPeer: RoleMarker,
    Msg: Message,
    G: EpSession<IO, RMe>,
{
    type Output = types::False;
}

impl<IO, Lbl, MeChoice: Role, L, R> IsEpEndVariant<IO, MeChoice>
    for EpChoice<IO, Lbl, MeChoice, L, R>
where
    IO: SessionIO,
    Lbl: types::ProtocolLabel,
    L: EpSession<IO, MeChoice>,
    R: EpSession<IO, MeChoice>,
{
    type Output = types::False;
}

impl<IO, Lbl, MePar: Role, L, R> IsEpEndVariant<IO, MePar>
    for EpPar<IO, Lbl, MePar, L, R>
where
    IO: SessionIO,
    Lbl: types::ProtocolLabel,
    L: EpSession<IO, MePar>,
    R: EpSession<IO, MePar>,
{
    type Output = types::False;
}

impl<IO, MeSkip: Role> IsEpEndVariant<IO, MeSkip> for EpSkip<IO, MeSkip>
where
    IO: SessionIO,
{
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
