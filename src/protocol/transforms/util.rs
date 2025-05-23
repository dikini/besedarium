//! Shared helpers and traits for protocol transformations
//!
//! Contains helpers such as `ContainsRole`, `NotContainsRole`, `GetProtocolLabel`, etc.

use crate::protocol::global::{
    TChanOffer, TEnd, TChanPar, TChanRecv, TChanSend, GlobalProtocol
};
use crate::RoleEq;
use crate::types::{self, ActionIOTMarker, Bool, BoolOr, ProtocolLabel, RoleMarker, SessionType, SupportsActionIO};

/// Returns a type-level boolean indicating whether the role is present.
pub trait ContainsRole<R> {
    type Output: Bool;
}

/// Helper trait to check if a role is NOT present in a protocol branch.
pub trait NotContainsRole<R> {}

/// Extracts the protocol label from a protocol or endpoint type.
pub trait GetProtocolLabel {
    type Label: ProtocolLabel;
}

/// Extracts the label from a local endpoint type.
pub trait GetLocalLabel {
    type Label: ProtocolLabel;
}

// Base case: TEnd doesn't contain any role
impl<IO: SessionType, Lbl: ProtocolLabel, R> ContainsRole<R> for TEnd<IO, Lbl> {
    type Output = types::False;
}
impl<IO: SessionType, Lbl: ProtocolLabel, R> NotContainsRole<R> for TEnd<IO, Lbl> {}

// TChanSend contains the role if the sender matches, or the continuation contains the role
impl<Snd, Rcv, M, Msg, G, AIO, IO, RoleT> ContainsRole<RoleT>
    for TChanSend<Snd, Rcv, M, Msg, G, AIO, IO>
where
    Snd: RoleMarker,
    Rcv: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static, 
    Msg: core::fmt::Debug + Send + Sync + 'static,
    G: GlobalProtocol + ContainsRole<RoleT>,
    AIO: ActionIOTMarker,
    IO: SessionType + SupportsActionIO<AIO>,
    Snd: RoleEq<RoleT>,
    <Snd as RoleEq<RoleT>>::Output: Bool,
    <G as ContainsRole<RoleT>>::Output: Bool,
    <Snd as RoleEq<RoleT>>::Output: BoolOr<<G as ContainsRole<RoleT>>::Output>,
{
    type Output = <<Snd as RoleEq<RoleT>>::Output as BoolOr<<G as ContainsRole<RoleT>>::Output>>::Output;
}

// TChanRecv contains the role if the receiver matches, or the continuation contains the role
impl<Rcv, Snd, M, Msg, G, AIO, IO, RoleT> ContainsRole<RoleT>
    for TChanRecv<Rcv, Snd, M, Msg, G, AIO, IO>
where
    Rcv: RoleMarker,
    Snd: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static,
    Msg: core::fmt::Debug + Send + Sync + 'static,
    G: GlobalProtocol + ContainsRole<RoleT>,
    AIO: ActionIOTMarker,
    IO: SessionType + SupportsActionIO<AIO>,
    Rcv: RoleEq<RoleT>,
    <Rcv as RoleEq<RoleT>>::Output: Bool,
    <G as ContainsRole<RoleT>>::Output: Bool,
    <Rcv as RoleEq<RoleT>>::Output: BoolOr<<G as ContainsRole<RoleT>>::Output>,
{
    type Output = <<Rcv as RoleEq<RoleT>>::Output as BoolOr<<G as ContainsRole<RoleT>>::Output>>::Output;
}

// TChanOffer contains the role if either branch contains it, or if the offerer/chooser is the role.
// For simplicity, we check branches. A more precise check might involve ROfferer and RChooser.
impl<ROfferer, RChooser, M, L, R, AIO, IO, RoleT> ContainsRole<RoleT>
    for TChanOffer<ROfferer, RChooser, M, L, R, AIO, IO>
where
    ROfferer: RoleMarker,
    RChooser: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static,
    L: GlobalProtocol + ContainsRole<RoleT>,
    R: GlobalProtocol + ContainsRole<RoleT>,
    AIO: ActionIOTMarker,
    IO: SessionType + SupportsActionIO<AIO>,
    <L as ContainsRole<RoleT>>::Output: Bool,
    <R as ContainsRole<RoleT>>::Output: Bool,
    <L as ContainsRole<RoleT>>::Output: BoolOr<<R as ContainsRole<RoleT>>::Output>,
    // Optionally, check if ROfferer or RChooser is RoleT
    // ROfferer: RoleEq<RoleT>,
    // RChooser: RoleEq<RoleT>,
    // ... and combine with BoolOr
{
    type Output = <<L as ContainsRole<RoleT>>::Output as BoolOr<<R as ContainsRole<RoleT>>::Output>>::Output;
}

// TChanPar contains the role if either branch contains it
impl<M, L, R, IsDisjoint, IO, RoleT> ContainsRole<RoleT>
    for TChanPar<M, L, R, IsDisjoint, IO>
where
    M: core::fmt::Debug + Send + Sync + 'static,
    L: GlobalProtocol + ContainsRole<RoleT>,
    R: GlobalProtocol + ContainsRole<RoleT>,
    IsDisjoint: core::fmt::Debug + Send + Sync + 'static, // Kept as per TChanPar definition, ideally types::Bool
    IO: SessionType,
    <L as ContainsRole<RoleT>>::Output: Bool,
    <R as ContainsRole<RoleT>>::Output: Bool,
    <L as ContainsRole<RoleT>>::Output: BoolOr<<R as ContainsRole<RoleT>>::Output>,
{
    type Output = <<L as ContainsRole<RoleT>>::Output as BoolOr<<R as ContainsRole<RoleT>>::Output>>::Output;
}

// TODO: Add ContainsRole for TChanRec and TChanContinue
// TChanRec<RecLbl, S, IO>
// TChanContinue<RecLbl, IO>

// Example for TChanRec:
// impl<RecLbl: ProtocolLabel, S: GlobalProtocol + ContainsRole<RoleT>, IO: SessionType, RoleT> ContainsRole<RoleT>
//     for TChanRec<RecLbl, S, IO>
// where
//     <S as ContainsRole<RoleT>>::Output: Bool,
// {
//     type Output = <S as ContainsRole<RoleT>>::Output;
// }

// TChanContinue does not directly contain roles other than through its context in TChanRec.
// Its ContainsRole might be considered False or depend on a more complex analysis.
// For now, let's assume it doesn't introduce new roles for simplicity in projection rules.
// impl<RecLbl: ProtocolLabel, IO: SessionType, RoleT> ContainsRole<RoleT> for TChanContinue<RecLbl, IO> {
// type Output = types::False;
// }
