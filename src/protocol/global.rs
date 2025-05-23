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
use crate::types::{self, ActionIOTMarker, CommMetadata, GlobalProtocol, Message, ProtocolLabel, Role, SessionType};

/// Core trait for all global session type combinators.
///
/// - `IO`: Overall session I/O capability type (e.g., `HttpOnlySessionIO`).
///         This type must implement `SupportsActionIO<AIO>` for any `AIO`
///         used by actions within the protocol.
/// - Implemented by all protocol combinators (TEnd, TSend, TRecv, TChoice, TPar, TRec).
/// - Used for type-level composition and compile-time protocol checks.
pub trait TSession<IO: SessionType>: sealed::Sealed + GlobalProtocol {}

/// Protocol entry point.
///
/// - `IO`: Overall session I/O capability type.
/// - `S`: Continuation protocol.
pub struct TStart<IO: SessionType, S: GlobalProtocol> {
    _io: PhantomData<IO>,
    _s: PhantomData<S>,
}

impl<IO: SessionType, S: GlobalProtocol> sealed::Sealed for TStart<IO, S> {}
impl<IO: SessionType, S: GlobalProtocol> TSession<IO> for TStart<IO, S> {}
impl<IO: SessionType, S: GlobalProtocol> GlobalProtocol for TStart<IO, S> {}


/// Protocol termination.
///
/// - `IO`: Overall session I/O capability type.
pub struct TEnd<IO: SessionType> {
    _io: PhantomData<IO>,
}

impl<IO: SessionType> sealed::Sealed for TEnd<IO> {}
impl<IO: SessionType> TSession<IO> for TEnd<IO> {}
impl<IO: SessionType> GlobalProtocol for TEnd<IO> {}

/// Global Type: Represents sending a message.
///
/// - `Snd`: Sender Role.
/// - `Rcv`: Receiver Role.
/// - `M`: CommMetadata (e.g., ChanId, MsgLbl).
/// - `Msg`: Type of the message being sent.
/// - `G`: Continuation Global Protocol after the send.
/// - `AIO`: ActionIOTMarker specifying the I/O type (e.g., Tcp, HttpIo).
/// - `IO`: Overall session I/O capability type. Must support `AIO`.
pub struct TChanSend<
    Snd: Role,
    Rcv: Role,
    M: CommMetadata,
    Msg: Message,
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + types::SupportsActionIO<AIO>,
> {
    _sender: PhantomData<Snd>,
    _receiver: PhantomData<Rcv>,
    _m: PhantomData<M>,
    _msg: PhantomData<Msg>,
    _g: PhantomData<G>,
    _aio: PhantomData<AIO>,
    _io: PhantomData<IO>,
}

impl<
    Snd: Role,
    Rcv: Role,
    M: CommMetadata,
    Msg: Message,
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + types::SupportsActionIO<AIO>,
> sealed::Sealed for TChanSend<Snd, Rcv, M, Msg, G, AIO, IO> {}

impl<
    Snd: Role,
    Rcv: Role,
    M: CommMetadata,
    Msg: Message,
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + types::SupportsActionIO<AIO>,
> TSession<IO> for TChanSend<Snd, Rcv, M, Msg, G, AIO, IO> {}

impl<
    Snd: Role,
    Rcv: Role,
    M: CommMetadata,
    Msg: Message,
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + types::SupportsActionIO<AIO>,
> GlobalProtocol for TChanSend<Snd, Rcv, M, Msg, G, AIO, IO> {}


/// Global Type: Represents receiving a message.
///
/// - `Rcv`: Receiver Role.
/// - `Snd`: Sender Role.
/// - `M`: CommMetadata (e.g., ChanId, MsgLbl).
/// - `Msg`: Type of the message being received.
/// - `G`: Continuation Global Protocol after the receive.
/// - `AIO`: ActionIOTMarker specifying the I/O type (e.g., Tcp, HttpIo).
/// - `IO`: Overall session I/O capability type. Must support `AIO`.
pub struct TChanRecv<
    Rcv: Role,
    Snd: Role,
    M: CommMetadata,
    Msg: Message,
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + types::SupportsActionIO<AIO>,
> {
    _receiver: PhantomData<Rcv>,
    _sender: PhantomData<Snd>,
    _m: PhantomData<M>,
    _msg: PhantomData<Msg>,
    _g: PhantomData<G>,
    _aio: PhantomData<AIO>,
    _io: PhantomData<IO>,
}

impl<
    Rcv: Role,
    Snd: Role,
    M: CommMetadata,
    Msg: Message,
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + types::SupportsActionIO<AIO>,
> sealed::Sealed for TChanRecv<Rcv, Snd, M, Msg, G, AIO, IO> {}

impl<
    Rcv: Role,
    Snd: Role,
    M: CommMetadata,
    Msg: Message,
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + types::SupportsActionIO<AIO>,
> TSession<IO> for TChanRecv<Rcv, Snd, M, Msg, G, AIO, IO> {}

impl<
    Rcv: Role,
    Snd: Role,
    M: CommMetadata,
    Msg: Message,
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + types::SupportsActionIO<AIO>,
> GlobalProtocol for TChanRecv<Rcv, Snd, M, Msg, G, AIO, IO> {}


/// Global Type: Represents a choice offered by one role to another.
/// The choice itself might be communicated, hence `AIO`.
///
/// - `ROfferer`: Role offering the choice.
/// - `RChooser`: Role making the choice.
/// - `M`: CommMetadata for the choice interaction.
/// - `L`: Protocol for the left branch of the choice.
/// - `R`: Protocol for the right branch of the choice.
/// - `AIO`: ActionIOTMarker for communicating the choice (if applicable).
/// - `IO`: Overall session I/O capability type. Must support `AIO` if choice is communicated.
pub struct TChanOffer<
    ROfferer: Role,
    RChooser: Role,
    M: CommMetadata,
    L: GlobalProtocol,
    R: GlobalProtocol,
    AIO: ActionIOTMarker, // AIO for the act of offering/choosing
    IO: SessionType + types::SupportsActionIO<AIO>,
> {
    _offerer: PhantomData<ROfferer>,
    _chooser: PhantomData<RChooser>,
    _m: PhantomData<M>,
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _aio: PhantomData<AIO>,
    _io: PhantomData<IO>,
}

impl<
    ROfferer: Role,
    RChooser: Role,
    M: CommMetadata,
    L: GlobalProtocol,
    R: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + types::SupportsActionIO<AIO>,
> sealed::Sealed for TChanOffer<ROfferer, RChooser, M, L, R, AIO, IO> {}

impl<
    ROfferer: Role,
    RChooser: Role,
    M: CommMetadata,
    L: GlobalProtocol,
    R: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + types::SupportsActionIO<AIO>,
> TSession<IO> for TChanOffer<ROfferer, RChooser, M, L, R, AIO, IO> {}

impl<
    ROfferer: Role,
    RChooser: Role,
    M: CommMetadata,
    L: GlobalProtocol,
    R: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + types::SupportsActionIO<AIO>,
> GlobalProtocol for TChanOffer<ROfferer, RChooser, M, L, R, AIO, IO> {}


// TChanChoice would be the dual to TChanOffer, representing the chooser's side.
// For now, focusing on TChanOffer as the primary construct for defining choices.

/// Parallel protocol composition.
///
/// - `M`: CommMetadata for this parallel composition block (logical grouping).
/// - `L`: Left protocol branch.
/// - `R`: Right protocol branch.
/// - `IsDisjoint`: Type-level boolean indicating if roles in L and R are disjoint.
/// - `IO`: Overall session I/O capability type.
pub struct TChanPar<
    M: CommMetadata, // Metadata for the parallel block itself
    L: GlobalProtocol,
    R: GlobalProtocol,
    IsDisjoint: types::Bool, // Assuming Bool is defined in types
    IO: SessionType,
> {
    _m: PhantomData<M>,
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _is_disjoint: PhantomData<IsDisjoint>,
    _io: PhantomData<IO>,
}

impl<
    M: CommMetadata,
    L: GlobalProtocol,
    R: GlobalProtocol,
    IsDisjoint: types::Bool,
    IO: SessionType,
> sealed::Sealed for TChanPar<M, L, R, IsDisjoint, IO> {}

impl<
    M: CommMetadata,
    L: GlobalProtocol,
    R: GlobalProtocol,
    IsDisjoint: types::Bool,
    IO: SessionType,
> TSession<IO> for TChanPar<M, L, R, IsDisjoint, IO> {}

impl<
    M: CommMetadata,
    L: GlobalProtocol,
    R: GlobalProtocol,
    IsDisjoint: types::Bool,
    IO: SessionType,
> GlobalProtocol for TChanPar<M, L, R, IsDisjoint, IO> {}


/// Recursive protocol definition.
///
/// - `RecLbl`: Label for this recursion point (used by `TChanContinue`).
/// - `S`: Protocol body, which may contain `TChanContinue<RecLbl>`.
/// - `IO`: Overall session I/O capability type.
pub struct TChanRec<RecLbl: ProtocolLabel, S: GlobalProtocol, IO: SessionType> {
    _lbl: PhantomData<RecLbl>,
    _s: PhantomData<S>,
    _io: PhantomData<IO>,
}

impl<RecLbl: ProtocolLabel, S: GlobalProtocol, IO: SessionType> sealed::Sealed
    for TChanRec<RecLbl, S, IO>
{
}
impl<RecLbl: ProtocolLabel, S: GlobalProtocol, IO: SessionType> TSession<IO>
    for TChanRec<RecLbl, S, IO>
{
}
impl<RecLbl: ProtocolLabel, S: GlobalProtocol, IO: SessionType> GlobalProtocol
    for TChanRec<RecLbl, S, IO>
{
}

/// Continue to a recursion point (variant of a recursion variable).
/// This is `TChanVar` or `TChanContinue` from the task list.
///
/// - `RecLbl`: Label of the `TChanRec` to continue to.
/// - `IO`: Overall session I/O capability type.
pub struct TChanContinue<RecLbl: ProtocolLabel, IO: SessionType> {
    _lbl: PhantomData<RecLbl>,
    _io: PhantomData<IO>,
}

impl<RecLbl: ProtocolLabel, IO: SessionType> sealed::Sealed for TChanContinue<RecLbl, IO> {}
impl<RecLbl: ProtocolLabel, IO: SessionType> TSession<IO> for TChanContinue<RecLbl, IO> {}
impl<RecLbl: ProtocolLabel, IO: SessionType> GlobalProtocol for TChanContinue<RecLbl, IO> {}


// Aliases for old names to reduce immediate breakage, will be removed later.
#[deprecated(note = "Use TStart instead")]
pub type TStartOld<IO, Lbl, S> = TStart<IO, S>; // Lbl removed from TStart
#[deprecated(note = "Use TEnd instead")]
pub type TEndOld<IO, Lbl> = TEnd<IO>; // Lbl removed from TEnd

#[deprecated(note = "Use TChanSend instead")]
pub type TSend<IO, M, RSender, RReceiver, Msg, G, AIO> = TChanSend<RSender, RReceiver, M, Msg, G, AIO, IO>;
#[deprecated(note = "Use TChanRecv instead")]
pub type TRecv<IO, M, RReceiver, RSender, Msg, G, AIO> = TChanRecv<RReceiver, RSender, M, Msg, G, AIO, IO>;

#[deprecated(note = "Use TChanOffer instead")]
pub type TChoice<IO, Lbl, ROfferer, RChooser, L, R, AIO> = TChanOffer<ROfferer, RChooser, Lbl, L, R, AIO, IO>; // Lbl was M

#[deprecated(note = "Use TChanPar instead")]
pub type TPar<IO, Lbl, L, R, IsDisjoint> = TChanPar<Lbl, L, R, IsDisjoint, IO>; // Lbl was M

#[deprecated(note = "Use TChanRec instead")]
pub type TRec<IO, Lbl, S> = TChanRec<Lbl, S, IO>;
#[deprecated(note = "Use TChanContinue instead")]
pub type TContinue<IO, Lbl> = TChanContinue<Lbl, IO>;

// TODO: Define TChanVar if it's distinct from TChanContinue.
// For now, TChanContinue serves as the recursion variable construct.

// TODO: Define TChanChoice (as dual to TChanOffer) if needed at Global level.
// It might be that TChanOffer is sufficient and choice selection is a local action.

// Helper for asserting disjointness, might be moved or become part of TPar's bound.
/// Trait to assert that two role lists (from protocol branches) are disjoint.
pub trait AssertDisjoint<RolesL, RolesR> {}
// Implementation would involve type-level list operations.
// Example: impl<L, R> AssertDisjoint<L, R> for () where L: DisjointFrom<R> {}
