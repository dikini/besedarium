//! # Global Protocol Type Definitions
//!
//! This module contains the struct definitions for all global protocol types
//! used in the enhanced MPST system. These represent the choreography of
//! multi-party protocols with explicit channel management.

use crate::protocol::foundation::{ActionIOTMarker, ChanId, GlobalProtocol, Message, MsgLbl, Role};
use std::fmt::Debug;
use std::marker::PhantomData;

// ============================================================================
// Core Global Protocol Types
// ============================================================================

/// Global Type: Represents sending a message over a specific channel
///
/// - `S`: Sender Role
/// - `R`: Receiver Role  
/// - `C`: Channel ID type
/// - `L`: Message label type
/// - `Msg`: Message type being sent
/// - `P`: Continuation Global Protocol after the send
/// - `AIO`: ActionIOTMarker specifying required I/O type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanSend<
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
> {
    pub(super) _sender: PhantomData<S>,
    pub(super) _receiver: PhantomData<R>,
    pub(super) _chan: PhantomData<C>,
    pub(super) _lbl: PhantomData<L>,
    pub(super) _msg: PhantomData<Msg>,
    pub(super) _protocol: PhantomData<P>,
    pub(super) _aio: PhantomData<AIO>,
}

/// Global Type: Represents receiving a message over a specific channel
///
/// - `R`: Receiver Role
/// - `S`: Sender Role
/// - `C`: Channel ID type
/// - `L`: Message label type  
/// - `Msg`: Message type being received
/// - `P`: Continuation Global Protocol after the receive
/// - `AIO`: ActionIOTMarker specifying required I/O type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanRecv<
    R: Role,
    S: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
> {
    pub(super) _receiver: PhantomData<R>,
    pub(super) _sender: PhantomData<S>,
    pub(super) _chan: PhantomData<C>,
    pub(super) _lbl: PhantomData<L>,
    pub(super) _msg: PhantomData<Msg>,
    pub(super) _protocol: PhantomData<P>,
    pub(super) _aio: PhantomData<AIO>,
}

/// Global Type: Represents a choice between protocol branches
///
/// - `R`: Role making the choice
/// - `C`: Channel ID type
/// - `Lbl`: Message label type for the choice point
/// - `Left`: Left branch protocol
/// - `Right`: Right branch protocol  
/// - `AIO`: ActionIOTMarker for choice communication
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanChoice<
    R: Role,
    C: ChanId,
    Lbl: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    AIO: ActionIOTMarker,
> {
    pub(super) _chooser: PhantomData<R>,
    pub(super) _chan: PhantomData<C>,
    pub(super) _lbl: PhantomData<Lbl>,
    pub(super) _left: PhantomData<Left>,
    pub(super) _right: PhantomData<Right>,
    pub(super) _aio: PhantomData<AIO>,
}

/// Global Type: Represents offering branches to another role
///
/// - `R`: Role offering the branches (dual to chooser)
/// - `C`: Channel ID type
/// - `Lbl`: Message label type for the offer point
/// - `Left`: Left branch protocol
/// - `Right`: Right branch protocol  
/// - `AIO`: ActionIOTMarker for offer communication
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanOffer<
    R: Role,
    C: ChanId,
    Lbl: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    AIO: ActionIOTMarker,
> {
    pub(super) _offerer: PhantomData<R>,
    pub(super) _chan: PhantomData<C>,
    pub(super) _lbl: PhantomData<Lbl>,
    pub(super) _left: PhantomData<Left>,
    pub(super) _right: PhantomData<Right>,
    pub(super) _aio: PhantomData<AIO>,
}

/// Global Type: Represents parallel composition of protocols
///
/// - `C`: Channel ID type
/// - `Lbl`: Message label type for parallel execution context
/// - `Left`: Left parallel branch
/// - `Right`: Right parallel branch
/// - `IsDisjoint`: Marker ensuring branches are disjoint (must be Send + Sync + Debug)
/// - `AIO`: ActionIOTMarker for parallel coordination
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanPar<
    C: ChanId,
    Lbl: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    IsDisjoint: Send + Sync + 'static + Debug,
    AIO: ActionIOTMarker,
> {
    pub(super) _chan: PhantomData<C>,
    pub(super) _lbl: PhantomData<Lbl>,
    pub(super) _left: PhantomData<Left>,
    pub(super) _right: PhantomData<Right>,
    pub(super) _disjoint: PhantomData<IsDisjoint>,
    pub(super) _aio: PhantomData<AIO>,
}

/// Global Type: Represents protocol termination
///
/// - `C`: Channel ID type
/// - `L`: Message label type for termination context
/// - `AIO`: ActionIOTMarker for cleanup operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanEnd<C: ChanId, L: MsgLbl, AIO: ActionIOTMarker> {
    pub(super) _chan: PhantomData<C>,
    pub(super) _lbl: PhantomData<L>,
    pub(super) _aio: PhantomData<AIO>,
}

/// Global Type: Represents protocol initialization
///
/// - `C`: Channel ID type
/// - `L`: Message label type for initialization context
/// - `Start`: Continuation protocol after start
/// - `AIO`: ActionIOTMarker for initialization operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanStart<C: ChanId, L: MsgLbl, Start: GlobalProtocol, AIO: ActionIOTMarker> {
    pub(super) _chan: PhantomData<C>,
    pub(super) _lbl: PhantomData<L>,
    pub(super) _start: PhantomData<Start>,
    pub(super) _aio: PhantomData<AIO>,
}
