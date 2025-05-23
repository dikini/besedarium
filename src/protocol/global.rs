//! # Global Protocol Types
//!
//! This module contains the global protocol session types that model
//! multi-party communication protocols. These types represent the
//! choreography-level view of interactions between participants.
//!
//! Key components:
//!
//! - `TSession`: Core trait for all global session type combinators
//! - `TStart`: Protocol entry point
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

use crate::sealed;
use core::marker::PhantomData;

/// Core trait for all global session type combinators.
///
/// - `IO`: Protocol marker type (e.g., Http, Mqtt).
/// - Implemented by all protocol combinators (TEnd, TSend, TRecv, TChoice, TPar, TRec).
/// - Used for type-level composition and compile-time protocol checks.
pub trait TSession<IO>: sealed::Sealed {}

/// Protocol entry point.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this start point.
/// - `S`: Continuation protocol.
pub struct TStart<IO, Lbl, S> {
    _io: PhantomData<IO>,
    _lbl: PhantomData<Lbl>,
    _s: PhantomData<S>,
}

impl<IO, Lbl, S> sealed::Sealed for TStart<IO, Lbl, S> {}
impl<IO, Lbl, S> TSession<IO> for TStart<IO, Lbl, S> {}

/// Protocol termination.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this end point.
pub struct TEnd<IO, Lbl> {
    _io: PhantomData<IO>,
    _lbl: PhantomData<Lbl>,
}

impl<IO, Lbl> sealed::Sealed for TEnd<IO, Lbl> {}
impl<IO, Lbl> TSession<IO> for TEnd<IO, Lbl> {}

/// Individual send action between roles.
///
/// - `IO`: Protocol marker type.
/// - `M`: Communication metadata (`CommMetadata`).
/// - `RSender`: Sending role.
/// - `RReceiver`: Receiving role.
/// - `Msg`: Message type being sent.
/// - `G`: Continuation protocol.
pub struct TSend<IO, M, RSender, RReceiver, Msg, G> {
    _io: PhantomData<IO>,
    _m: PhantomData<M>,
    _sender: PhantomData<RSender>,
    _receiver: PhantomData<RReceiver>,
    _msg: PhantomData<Msg>,
    _g: PhantomData<G>,
}

impl<IO, M, RSender, RReceiver, Msg, G> sealed::Sealed
    for TSend<IO, M, RSender, RReceiver, Msg, G>
{
}
impl<IO, M, RSender, RReceiver, Msg, G> TSession<IO>
    for TSend<IO, M, RSender, RReceiver, Msg, G>
{
}

/// Individual receive action between roles.
///
/// - `IO`: Protocol marker type.
/// - `M`: Communication metadata (`CommMetadata`).
/// - `RReceiver`: Receiving role.
/// - `RSender`: Sending role.
/// - `Msg`: Message type being received.
/// - `G`: Continuation protocol.
pub struct TRecv<IO, M, RReceiver, RSender, Msg, G> {
    _io: PhantomData<IO>,
    _m: PhantomData<M>,
    _receiver: PhantomData<RReceiver>,
    _sender: PhantomData<RSender>,
    _msg: PhantomData<Msg>,
    _g: PhantomData<G>,
}

impl<IO, M, RReceiver, RSender, Msg, G> sealed::Sealed
    for TRecv<IO, M, RReceiver, RSender, Msg, G>
{
}
impl<IO, M, RReceiver, RSender, Msg, G> TSession<IO>
    for TRecv<IO, M, RReceiver, RSender, Msg, G>
{
}

/// Binary protocol choice offered by one role to another.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this choice.
/// - `ROfferer`: Role offering the choice.
/// - `RChooser`: Role making the choice.
/// - `L`: Protocol for the left branch of the choice.
/// - `R`: Protocol for the right branch of the choice.
pub struct TChoice<IO, Lbl, ROfferer, RChooser, L, R> {
    _io: PhantomData<IO>,
    _lbl: PhantomData<Lbl>,
    _offerer: PhantomData<ROfferer>,
    _chooser: PhantomData<RChooser>,
    _l: PhantomData<L>,
    _r: PhantomData<R>,
}

impl<IO, Lbl, ROfferer, RChooser, L, R> sealed::Sealed
    for TChoice<IO, Lbl, ROfferer, RChooser, L, R>
{
}
impl<IO, Lbl, ROfferer, RChooser, L, R> TSession<IO>
    for TChoice<IO, Lbl, ROfferer, RChooser, L, R>
{
}

/// Parallel protocol composition.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this parallel composition.
/// - `L`: Left protocol branch.
/// - `R`: Right protocol branch.
/// - `IsDisjoint`: Type-level boolean indicating if roles in L and R are disjoint.
pub struct TPar<IO, Lbl, L, R, IsDisjoint> {
    _io: PhantomData<IO>,
    _lbl: PhantomData<Lbl>,
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _is_disjoint: PhantomData<IsDisjoint>,
}

impl<IO, Lbl, L, R, IsDisjoint> sealed::Sealed for TPar<IO, Lbl, L, R, IsDisjoint> {}
impl<IO, Lbl, L, R, IsDisjoint> TSession<IO> for TPar<IO, Lbl, L, R, IsDisjoint> {}

/// Recursive protocol definition.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this recursion point (used by `TContinue`).
/// - `S`: Protocol body, which may contain `TContinue<Lbl>`.
pub struct TRec<IO, Lbl, S> {
    _io: PhantomData<IO>,
    _lbl: PhantomData<Lbl>,
    _s: PhantomData<S>,
}

impl<IO, Lbl, S> sealed::Sealed for TRec<IO, Lbl, S> {}
impl<IO, Lbl, S> TSession<IO> for TRec<IO, Lbl, S> {}

/// Continue to a recursion point.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label of the `TRec` to continue to.
pub struct TContinue<IO, Lbl> {
    _io: PhantomData<IO>,
    _lbl: PhantomData<Lbl>,
}

impl<IO, Lbl> sealed::Sealed for TContinue<IO, Lbl> {}
impl<IO, Lbl> TSession<IO> for TContinue<IO, Lbl> {}
