//! # Type-Level Introspection Utilities
//!
//! This module provides traits for extracting metadata from protocol types at the type level.
//! These are used for compile-time checks, protocol analysis, and macro support.
//!
//! - See `protocol.rs` for the type-level list pattern and map/fold idioms.
//! - See crate-level docs for usage examples and macro integration.

use crate::protocol;
use crate::types;

/// Extracts the set of roles used in a protocol as a type-level list.
///
/// - Implemented for all protocol combinators.
/// - Used for disjointness checks, macro expansion, and compile-time assertions.
/// - See also: [`Disjoint`], [`extract_roles!`] macro.
pub trait RolesOf {
    type Roles;
}
impl<IO> RolesOf for protocol::TEnd<IO> {
    type Roles = protocol::Nil;
}
impl<IO, L: types::ProtocolLabel, R, H, T: protocol::TSession<IO> + RolesOf> RolesOf
    for protocol::TInteract<IO, L, R, H, T>
{
    type Roles = protocol::Cons<R, <T as RolesOf>::Roles>;
}
impl<
        IO,
        Lbl: types::ProtocolLabel,
        L: protocol::TSession<IO> + RolesOf,
        R: protocol::TSession<IO>,
    > RolesOf for protocol::TChoice<IO, Lbl, L, R>
{
    type Roles = <L as RolesOf>::Roles;
}
impl<
        IO,
        Lbl: types::ProtocolLabel,
        L: protocol::TSession<IO> + RolesOf,
        R: protocol::TSession<IO>,
        IsDisjoint,
    > RolesOf for protocol::TPar<IO, Lbl, L, R, IsDisjoint>
{
    type Roles = <L as RolesOf>::Roles;
}
impl<IO, L: types::ProtocolLabel, S: protocol::TSession<IO> + RolesOf> RolesOf
    for protocol::TRec<IO, L, S>
{
    type Roles = <S as RolesOf>::Roles;
}

/// Extracts the set of protocol labels as a type-level list.
///
/// - Implemented for all protocol combinators.
/// - Used for uniqueness checks and macro expansion.
/// - See also: [`UniqueList`], [`assert_unique_labels!`] macro.
pub trait LabelsOf {
    type Labels;
}
impl<IO, L> LabelsOf for protocol::TEnd<IO, L> {
    type Labels = protocol::Cons<L, protocol::Nil>;
}
impl<IO, L: types::ProtocolLabel, R, H, T: protocol::TSession<IO> + LabelsOf> LabelsOf
    for protocol::TInteract<IO, L, R, H, T>
{
    type Labels = protocol::Cons<L, <T as LabelsOf>::Labels>;
}
impl<
        IO,
        Lbl: types::ProtocolLabel,
        L: protocol::TSession<IO> + LabelsOf,
        R: protocol::TSession<IO> + LabelsOf,
    > LabelsOf for protocol::TChoice<IO, Lbl, L, R>
{
    type Labels = protocol::Cons<Lbl, <L as LabelsOf>::Labels>;
}
impl<
        IO,
        Lbl: types::ProtocolLabel,
        L: protocol::TSession<IO> + LabelsOf,
        R: protocol::TSession<IO> + LabelsOf,
        IsDisjoint,
    > LabelsOf for protocol::TPar<IO, Lbl, L, R, IsDisjoint>
{
    type Labels = protocol::Cons<Lbl, <L as LabelsOf>::Labels>;
}
impl<IO, L: types::ProtocolLabel, S: protocol::TSession<IO> + LabelsOf> LabelsOf
    for protocol::TRec<IO, L, S>
{
    type Labels = protocol::Cons<L, <S as LabelsOf>::Labels>;
}
impl LabelsOf for protocol::Nil {
    type Labels = protocol::Nil;
}
impl<H, T> LabelsOf for protocol::Cons<H, T>
where
    H: LabelsOf,
    T: LabelsOf,
{
    type Labels = <H as LabelsOf>::Labels;
}
