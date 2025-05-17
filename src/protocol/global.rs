//! # Global Protocol Types
//!
//! This module contains the global protocol session types that model
//! multi-party communication protocols. These types represent the
//! choreography-level view of interactions between participants.
//!
//! Key components:
//!
//! - `TSession`: Core trait for all global session type combinators
//! - `TEnd`: Protocol termination
//! - `TSend`: Individual send action between roles
//! - `TRecv`: Individual receive action between roles
//! - `TChoice`: Binary protocol choice
//! - `TPar`: Parallel protocol composition
//! - `TRec`: Recursive protocol definition
//!
//! Global protocols are designed to be projected onto specific roles to
//! produce local (endpoint) protocols that describe the behavior of
//! individual participants.

use super::base::{Cons, Nil};
use crate::sealed;
use crate::types;
use core::marker::PhantomData;

/// Core trait for all global session type combinators.
///
/// - `IO`: Protocol marker type (e.g., Http, Mqtt).
/// - Implemented by all protocol combinators (TEnd, TSend, TRecv, TChoice, TPar, TRec).
/// - Used for type-level composition and compile-time protocol checks.
pub trait TSession<IO>: sealed::Sealed {
    /// Compose this session with another session of the same IO type.
    type Compose<Rhs: TSession<IO>>: TSession<IO>;
    /// Is this session type empty (TEnd)?
    const IS_EMPTY: bool;
}

/// End of a protocol session.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this end (default: EmptyLabel).
///
/// Used to indicate protocol termination.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TEnd<IO, Lbl = types::EmptyLabel>(PhantomData<(IO, Lbl)>);

impl<IO, Lbl> sealed::Sealed for TEnd<IO, Lbl> {}
impl<IO, Lbl> TSession<IO> for TEnd<IO, Lbl> {
    type Compose<Rhs: TSession<IO>> = Rhs;
    const IS_EMPTY: bool = true;
}

/// Represents a single send action in a protocol session.
///
/// - `IO`: Protocol marker type (e.g., Http, Mqtt).
/// - `Lbl`: Label for this send (for projection and debugging).
/// - `R`: Role performing the send.
/// - `H`: Message type being sent.
/// - `T`: Continuation protocol after this send.
///
/// Used to model a single send step in a protocol.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TSend<IO, Lbl: types::ProtocolLabel, R, H, T: TSession<IO>>(
    PhantomData<(IO, Lbl, R, H, T)>,
);

impl<IO, Lbl: types::ProtocolLabel, R, H, T: TSession<IO>> sealed::Sealed
    for TSend<IO, Lbl, R, H, T>
{
}
impl<IO, Lbl: types::ProtocolLabel, R, H, T: TSession<IO>> TSession<IO>
    for TSend<IO, Lbl, R, H, T>
{
    type Compose<Rhs: TSession<IO>> = TSend<IO, Lbl, R, H, T::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

/// Represents a single receive action in a protocol session.
///
/// - `IO`: Protocol marker type (e.g., Http, Mqtt).
/// - `Lbl`: Label for this receive (for projection and debugging).
/// - `R`: Role performing the receive.
/// - `H`: Message type being received.
/// - `T`: Continuation protocol after this receive.
///
/// Used to model a single receive step in a protocol.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TRecv<IO, Lbl: types::ProtocolLabel, R, H, T: TSession<IO>>(
    PhantomData<(IO, Lbl, R, H, T)>,
);

impl<IO, Lbl: types::ProtocolLabel, R, H, T: TSession<IO>> sealed::Sealed
    for TRecv<IO, Lbl, R, H, T>
{
}
impl<IO, Lbl: types::ProtocolLabel, R, H, T: TSession<IO>> TSession<IO>
    for TRecv<IO, Lbl, R, H, T>
{
    type Compose<Rhs: TSession<IO>> = TRecv<IO, Lbl, R, H, T::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

/// Binary protocol choice between two branches.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this choice (for projection and debugging).
/// - `L`, `R`: The two protocol branches.
///
/// Used to model branching points in a protocol (e.g., offer/choose).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TChoice<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>>(
    PhantomData<(IO, Lbl, L, R)>,
);

impl<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>> sealed::Sealed
    for TChoice<IO, Lbl, L, R>
{
}
impl<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>> TSession<IO>
    for TChoice<IO, Lbl, L, R>
{
    type Compose<Rhs: TSession<IO>> = TChoice<IO, Lbl, L::Compose<Rhs>, R::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

/// Recursive session type for repeating protocol fragments.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this recursion (for projection and debugging).
/// - `S`: The protocol fragment to repeat (may refer to itself).
///
/// Used to model loops or streaming protocols.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TRec<IO, Lbl: types::ProtocolLabel, S: TSession<IO>>(PhantomData<(IO, Lbl, S)>);

impl<IO, Lbl: types::ProtocolLabel, S: TSession<IO>> sealed::Sealed for TRec<IO, Lbl, S> {}
impl<IO, Lbl: types::ProtocolLabel, S: TSession<IO>> TSession<IO> for TRec<IO, Lbl, S> {
    type Compose<Rhs: TSession<IO>> = TRec<IO, Lbl, S::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

/// Branded parallel composition of two protocol branches.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this parallel composition.
/// - `L`, `R`: The two protocol branches to run in parallel.
/// - `IsDisjoint`: Type-level boolean indicating if branches are disjoint.
///
/// Used to model concurrency in protocols. Disjointness is enforced at compile time.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TPar<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>, IsDisjoint>(
    PhantomData<(IO, Lbl, L, R, IsDisjoint)>,
);

impl<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>, IsDisjoint> sealed::Sealed
    for TPar<IO, Lbl, L, R, IsDisjoint>
{
}
impl<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>, IsDisjoint> TSession<IO>
    for TPar<IO, Lbl, L, R, IsDisjoint>
{
    type Compose<Rhs: TSession<IO>> = TPar<IO, Lbl, L::Compose<Rhs>, R::Compose<Rhs>, IsDisjoint>;
    const IS_EMPTY: bool = false;
}

/// Trait for mapping a type-level list to a nested `TChoice`.
///
/// # Examples
/// ```ignore
/// use besedarium::*;
/// // Create a list of two end-of-session branches
/// type EndList = tlist!(
///     TEnd<Http>,
///     TEnd<Http>,
/// );
/// // Map to a protocol choice
/// type Choice = <EndList as ToTChoice<Http>>::Output;
/// // Equivalent to a binary choice between two ends
/// assert_type_eq!(
///     Choice,
///     TChoice<
///         Http,
///         EmptyLabel,
///         TEnd<Http>,
///         TEnd<Http>
///     >
/// );
/// ```
pub trait ToTChoice<IO> {
    type Output: TSession<IO>;
}

/// Trait for mapping a type-level list to a nested `TPar`.
///
/// # Examples
/// ```ignore
/// use besedarium::*;
/// // Create a list of two end-of-session branches
/// type EndList = tlist!(
///     TEnd<Http>,
///     TEnd<Http>,
/// );
/// // Map to a parallel composition
/// type Par = <EndList as ToTPar<Http>>::Output;
/// // Equivalent to running two sessions in parallel
/// assert_type_eq!(
///     Par,
///     TPar<
///         Http,
///         EmptyLabel,
///         TEnd<Http>,
///         TEnd<Http>,
///         FalseB
///     >
/// );
/// ```
pub trait ToTPar<IO> {
    type Output: TSession<IO>;
}

// --- ToTChoice trait, base case for Nil ---
impl<IO> ToTChoice<IO> for Nil {
    type Output = TEnd<IO>;
}

// --- ToTChoice trait, recursive case ---
impl<IO, H: TSession<IO>, T: ToTChoice<IO>> ToTChoice<IO> for Cons<H, T> {
    type Output = TChoice<IO, types::EmptyLabel, H, <T as ToTChoice<IO>>::Output>;
}

// --- ToTPar trait, base case for Nil ---
impl<IO> ToTPar<IO> for Nil {
    type Output = TEnd<IO>;
}

// --- ToTPar trait, recursive case ---
impl<IO, H: TSession<IO>, T: ToTPar<IO>> ToTPar<IO> for Cons<H, T> {
    type Output = TPar<IO, types::EmptyLabel, H, <T as ToTPar<IO>>::Output, types::False>;
}

/// Compile-time Disjointness Assertion Machinery
pub trait AssertDisjoint {
    type Output;
}
