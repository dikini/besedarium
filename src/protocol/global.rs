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
// Corrected and refined imports
use crate::types::{
    ActionIOTMarker, CommMetadata, ProtocolLabel, RoleMarker, SessionType, SupportsActionIO, Tcp, True,
};

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
#[derive(Debug, Clone)]
pub struct TStart<IO: SessionType, Lbl: ProtocolLabel, S: GlobalProtocol> {
    _io: PhantomData<IO>,
    _lbl: PhantomData<Lbl>,
    _s: PhantomData<S>,
}

impl<IO: SessionType, Lbl: ProtocolLabel, S: GlobalProtocol> sealed::Sealed for TStart<IO, Lbl, S> {}
impl<IO: SessionType, Lbl: ProtocolLabel, S: GlobalProtocol> TSession<IO> for TStart<IO, Lbl, S> {}
impl<IO: SessionType, Lbl: ProtocolLabel, S: GlobalProtocol> GlobalProtocol for TStart<IO, Lbl, S> {}


/// Protocol termination.
///
/// - `IO`: Overall session I/O capability type.
/// - `Lbl`: Label for this termination point.
#[derive(Debug, Clone)]
pub struct TEnd<IO: SessionType, Lbl: ProtocolLabel> {
    _io: PhantomData<IO>,
    _lbl: PhantomData<Lbl>,
}

impl<IO: SessionType, Lbl: ProtocolLabel> sealed::Sealed for TEnd<IO, Lbl> {}
impl<IO: SessionType, Lbl: ProtocolLabel> TSession<IO> for TEnd<IO, Lbl> {}
impl<IO: SessionType, Lbl: ProtocolLabel> GlobalProtocol for TEnd<IO, Lbl> {}

/// Global Type: Represents sending a message.
///
/// - `Snd`: Sender Role.
/// - `Rcv`: Receiver Role.
/// - `M`: CommMetadata type (e.g., `crate::CommMetadata`).
/// - `Msg`: Type of the message payload being sent.
/// - `G`: Continuation Global Protocol after the send.
/// - `AIO`: ActionIOTMarker specifying the I/O type (e.g., Tcp, HttpIo).
/// - `IO`: Overall session I/O capability type. Must support `AIO`.
#[derive(Debug, Clone)]
pub struct TChanSend<
    Snd: RoleMarker,
    Rcv: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static, // CommMetadata type
    Msg: core::fmt::Debug + Send + Sync + 'static, // Payload type
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + SupportsActionIO<AIO>,
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
    Snd: RoleMarker,
    Rcv: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static,
    Msg: core::fmt::Debug + Send + Sync + 'static,
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + SupportsActionIO<AIO>,
> sealed::Sealed for TChanSend<Snd, Rcv, M, Msg, G, AIO, IO> {}

impl<
    Snd: RoleMarker,
    Rcv: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static,
    Msg: core::fmt::Debug + Send + Sync + 'static,
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + SupportsActionIO<AIO>,
> TSession<IO> for TChanSend<Snd, Rcv, M, Msg, G, AIO, IO> {}

impl<
    Snd: RoleMarker,
    Rcv: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static,
    Msg: core::fmt::Debug + Send + Sync + 'static,
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + SupportsActionIO<AIO>,
> GlobalProtocol for TChanSend<Snd, Rcv, M, Msg, G, AIO, IO> {}


/// Global Type: Represents receiving a message.
///
/// - `Rcv`: Receiver Role.
/// - `Snd`: Sender Role.
/// - `M`: CommMetadata type (e.g., `crate::CommMetadata`).
/// - `Msg`: Type of the message payload being received.
/// - `G`: Continuation Global Protocol after the receive.
/// - `AIO`: ActionIOTMarker specifying the I/O type (e.g., Tcp, HttpIo).
/// - `IO`: Overall session I/O capability type. Must support `AIO`.
#[derive(Debug, Clone)]
pub struct TChanRecv<
    Rcv: RoleMarker,
    Snd: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static, // CommMetadata type
    Msg: core::fmt::Debug + Send + Sync + 'static, // Payload type
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + SupportsActionIO<AIO>,
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
    Rcv: RoleMarker,
    Snd: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static,
    Msg: core::fmt::Debug + Send + Sync + 'static,
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + SupportsActionIO<AIO>,
> sealed::Sealed for TChanRecv<Rcv, Snd, M, Msg, G, AIO, IO> {}

impl<
    Rcv: RoleMarker,
    Snd: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static,
    Msg: core::fmt::Debug + Send + Sync + 'static,
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + SupportsActionIO<AIO>,
> TSession<IO> for TChanRecv<Rcv, Snd, M, Msg, G, AIO, IO> {}

impl<
    Rcv: RoleMarker,
    Snd: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static,
    Msg: core::fmt::Debug + Send + Sync + 'static,
    G: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + SupportsActionIO<AIO>,
> GlobalProtocol for TChanRecv<Rcv, Snd, M, Msg, G, AIO, IO> {}


/// Global Type: Represents a choice offered by one role to another.
/// The choice itself might be communicated, hence `AIO`.
///
/// - `ROfferer`: Role offering the choice.
/// - `RChooser`: Role making the choice.
/// - `M`: CommMetadata type for the choice interaction.
/// - `L`: Protocol for the left branch of the choice.
/// - `R`: Protocol for the right branch of the choice.
/// - `AIO`: ActionIOTMarker for communicating the choice (if applicable).
/// - `IO`: Overall session I/O capability type. Must support `AIO` if choice is communicated.
#[derive(Debug, Clone)]
pub struct TChanOffer<
    ROfferer: RoleMarker,
    RChooser: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static, // CommMetadata type
    L: GlobalProtocol,
    R: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + SupportsActionIO<AIO>,
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
    ROfferer: RoleMarker,
    RChooser: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static,
    L: GlobalProtocol,
    R: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + SupportsActionIO<AIO>,
> sealed::Sealed for TChanOffer<ROfferer, RChooser, M, L, R, AIO, IO> {}

impl<
    ROfferer: RoleMarker,
    RChooser: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static,
    L: GlobalProtocol,
    R: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + SupportsActionIO<AIO>,
> TSession<IO> for TChanOffer<ROfferer, RChooser, M, L, R, AIO, IO> {}

impl<
    ROfferer: RoleMarker,
    RChooser: RoleMarker,
    M: core::fmt::Debug + Send + Sync + 'static,
    L: GlobalProtocol,
    R: GlobalProtocol,
    AIO: ActionIOTMarker,
    IO: SessionType + SupportsActionIO<AIO>,
> GlobalProtocol for TChanOffer<ROfferer, RChooser, M, L, R, AIO, IO> {}


/// Parallel protocol composition.
///
/// - `M`: CommMetadata type for this parallel composition block (logical grouping).
/// - `L`: Left protocol branch.
/// - `R`: Right protocol branch.
/// - `IsDisjoint`: Type-level boolean (e.g. `crate::True` or `crate::False`) indicating if roles in L and R are disjoint.
/// - `IO`: Overall session I/O capability type.
#[derive(Debug, Clone)]
pub struct TChanPar<
    M: core::fmt::Debug + Send + Sync + 'static, // CommMetadata type
    L: GlobalProtocol,
    R: GlobalProtocol,
    IsDisjoint: core::fmt::Debug + Send + Sync + 'static, // Type for a type-level boolean marker
    IO: SessionType,
> {
    _m: PhantomData<M>,
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _is_disjoint: PhantomData<IsDisjoint>,
    _io: PhantomData<IO>,
}

impl<
    M: core::fmt::Debug + Send + Sync + 'static,
    L: GlobalProtocol,
    R: GlobalProtocol,
    IsDisjoint: core::fmt::Debug + Send + Sync + 'static,
    IO: SessionType,
> sealed::Sealed for TChanPar<M, L, R, IsDisjoint, IO> {}

impl<
    M: core::fmt::Debug + Send + Sync + 'static,
    L: GlobalProtocol,
    R: GlobalProtocol,
    IsDisjoint: core::fmt::Debug + Send + Sync + 'static,
    IO: SessionType,
> TSession<IO> for TChanPar<M, L, R, IsDisjoint, IO> {}

impl<
    M: core::fmt::Debug + Send + Sync + 'static,
    L: GlobalProtocol,
    R: GlobalProtocol,
    IsDisjoint: core::fmt::Debug + Send + Sync + 'static,
    IO: SessionType,
> GlobalProtocol for TChanPar<M, L, R, IsDisjoint, IO> {}


/// Recursive protocol definition.
///
/// - `RecLbl`: Label for this recursion point (used by `TChanContinue`).
/// - `S`: Protocol body, which may contain `TChanContinue<RecLbl>`.
/// - `IO`: Overall session I/O capability type.
#[derive(Debug, Clone)]
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

/// Continuation of a recursive protocol.
///
/// - `RecLbl`: Label of the `TChanRec` to continue.
/// - `IO`: Overall session I/O capability type.
#[derive(Debug, Clone)]
pub struct TChanContinue<RecLbl: ProtocolLabel, IO: SessionType> {
    _lbl: PhantomData<RecLbl>,
    _io: PhantomData<IO>,
}

impl<RecLbl: ProtocolLabel, IO: SessionType> sealed::Sealed for TChanContinue<RecLbl, IO> {}
impl<RecLbl: ProtocolLabel, IO: SessionType> TSession<IO> for TChanContinue<RecLbl, IO> {}
impl<RecLbl: ProtocolLabel, IO: SessionType> GlobalProtocol for TChanContinue<RecLbl, IO> {}


/// Marker trait for types that represent a global protocol.
pub trait GlobalProtocol: sealed::Sealed + Send + Sync + 'static + core::fmt::Debug {}


// Deprecated Aliases
// These aliases attempt to bridge old type signatures to new ones.
// They might need further refinement based on how strictly the old types need to be supported.

#[deprecated(note = "Use TStart<IO, Lbl, S> instead.")]
pub type TStartOld<IO, Lbl: ProtocolLabel, S> = TStart<IO, Lbl, S>;

#[deprecated(note = "Use TEnd<IO, Lbl> instead.")]
pub type TEndOld<IO, Lbl: ProtocolLabel> = TEnd<IO, Lbl>;

#[deprecated(note = "Use TChanSend instead. This alias fixes CommMetadata to crate::types::CommMetadata and AIO to crate::types::Tcp.")]
pub type TSend<SndR, RcvR, MsgPayload, GProto, IOSess> = 
    TChanSend<SndR, RcvR, CommMetadata, MsgPayload, GProto, Tcp, IOSess>;

#[deprecated(note = "Use TChanRecv instead. This alias fixes CommMetadata to crate::types::CommMetadata and AIO to crate::types::Tcp.")]
pub type TRecv<RcvR, SndR, MsgPayload, GProto, IOSess> =
    TChanRecv<RcvR, SndR, CommMetadata, MsgPayload, GProto, Tcp, IOSess>;

#[deprecated(note = "Use TChanOffer instead. This alias fixes CommMetadata to crate::types::CommMetadata and AIO to crate::types::Tcp.")]
pub type TChoice<ROffererR, RChooserR, LProto, RProto, IOSess> =
    TChanOffer<ROffererR, RChooserR, CommMetadata, LProto, RProto, Tcp, IOSess>;

#[deprecated(note = "Use TChanPar instead. This alias fixes CommMetadata to crate::types::CommMetadata and IsDisjoint to crate::types::True (example).")]
pub type TPar<LProto, RProto, IOSess> = 
    TChanPar<CommMetadata, LProto, RProto, True, IOSess>;

#[deprecated(note = "Use TChanRec instead.")]
pub type TRec<RecLblProto, SProto, IOSess> = TChanRec<RecLblProto, SProto, IOSess>;

#[deprecated(note = "Use TChanContinue instead.")]
pub type TCont<RecLblProto, IOSess> = TChanContinue<RecLblProto, IOSess>;

// TODO: Review the `IsDisjoint` type parameter in TChanPar.
// It's currently `IsDisjoint` without a specific trait bound like `TypeLevelBool`.
// It should likely be `IsDisjoint: crate::TypeLevelBool` where `TypeLevelBool` is a trait
// implemented by `crate::True` and `crate::False`.
// For now, the alias TPar hardcodes `crate::True` as an example. This needs `crate::True` to be defined.
// If `crate::True` is not available, this alias will fail.
// The import `use crate::Bool` was for the enum, which is not suitable for type-level marker.
// The `IsDisjoint` in TChanPar itself is `IsDisjoint,` which means it's a generic type name.
// It should be `_is_disjoint: PhantomData<IsDisjoint>` where IsDisjoint is e.g. `True` or `False` type.
