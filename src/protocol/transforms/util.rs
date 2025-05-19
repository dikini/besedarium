//! Shared helpers and traits for protocol transformations
//!
//! Contains helpers such as `ContainsRole`, `NotContainsRole`, `GetProtocolLabel`, etc.

use crate::protocol::global::{TChoice, TEnd, TPar, TRecv, TSend};
use crate::protocol::local::RoleEq;
use crate::types;

/// Returns a type-level boolean indicating whether the role is present.
pub trait ContainsRole<R> {
    type Output: types::Bool;
}

/// Helper trait to check if a role is NOT present in a protocol branch.
pub trait NotContainsRole<R> {}

/// Extracts the protocol label from a protocol or endpoint type.
pub trait GetProtocolLabel {
    type Label: types::ProtocolLabel;
}

/// Extracts the label from a local endpoint type.
pub trait GetLocalLabel {
    type Label: types::ProtocolLabel;
}

// Base case: TEnd doesn't contain any role
impl<IO, Lbl: types::ProtocolLabel, R> ContainsRole<R> for TEnd<IO, Lbl> {
    type Output = types::False;
}
impl<IO, Lbl: types::ProtocolLabel, R> NotContainsRole<R> for TEnd<IO, Lbl> {}

// TSend contains the role if the sender matches, or the continuation contains the role
impl<IO, Lbl: types::ProtocolLabel, To, H, T, RoleT> ContainsRole<RoleT>
    for TSend<IO, Lbl, To, H, T>
where
    To: RoleEq<RoleT>,
    <To as RoleEq<RoleT>>::Output: types::Bool,
    T: ContainsRole<RoleT> + crate::protocol::global::TSession<IO>,
    <T as ContainsRole<RoleT>>::Output: types::Bool,
    <To as RoleEq<RoleT>>::Output: types::BoolOr<<T as ContainsRole<RoleT>>::Output>,
{
    type Output = types::Or<<To as RoleEq<RoleT>>::Output, <T as ContainsRole<RoleT>>::Output>;
}

// TRecv contains the role if the receiver matches, or the continuation contains the role
impl<IO, Lbl: types::ProtocolLabel, From, H, T, RoleT> ContainsRole<RoleT>
    for TRecv<IO, Lbl, From, H, T>
where
    From: RoleEq<RoleT>,
    <From as RoleEq<RoleT>>::Output: types::Bool,
    T: ContainsRole<RoleT> + crate::protocol::global::TSession<IO>,
    <T as ContainsRole<RoleT>>::Output: types::Bool,
    <From as RoleEq<RoleT>>::Output: types::BoolOr<<T as ContainsRole<RoleT>>::Output>,
{
    type Output = types::Or<<From as RoleEq<RoleT>>::Output, <T as ContainsRole<RoleT>>::Output>;
}

// TChoice contains the role if either branch contains it
impl<IO, Lbl: types::ProtocolLabel, L, R, RoleT> ContainsRole<RoleT> for TChoice<IO, Lbl, L, R>
where
    L: ContainsRole<RoleT> + crate::protocol::global::TSession<IO>,
    <L as ContainsRole<RoleT>>::Output: types::Bool,
    R: ContainsRole<RoleT> + crate::protocol::global::TSession<IO>,
    <R as ContainsRole<RoleT>>::Output: types::Bool,
    <L as ContainsRole<RoleT>>::Output: types::BoolOr<<R as ContainsRole<RoleT>>::Output>,
{
    type Output = types::Or<<L as ContainsRole<RoleT>>::Output, <R as ContainsRole<RoleT>>::Output>;
}

// TPar contains the role if either branch contains it
impl<IO, Lbl: types::ProtocolLabel, L, R, IsDisjoint, RoleT> ContainsRole<RoleT>
    for TPar<IO, Lbl, L, R, IsDisjoint>
where
    L: ContainsRole<RoleT> + crate::protocol::global::TSession<IO>,
    <L as ContainsRole<RoleT>>::Output: types::Bool,
    R: ContainsRole<RoleT> + crate::protocol::global::TSession<IO>,
    <R as ContainsRole<RoleT>>::Output: types::Bool,
    <L as ContainsRole<RoleT>>::Output: types::BoolOr<<R as ContainsRole<RoleT>>::Output>,
{
    type Output = types::Or<<L as ContainsRole<RoleT>>::Output, <R as ContainsRole<RoleT>>::Output>;
}
