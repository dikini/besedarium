//! # Local Endpoint Type Definitions
//!
//! This module contains the struct definitions for all local endpoint types
//! used in the Enhanced MPST System. These types represent the projected view
//! of protocols from individual role perspectives.

use crate::protocol::foundation::{
    ActionIOTMarker, CommMetadataTrait, LocalProtocol, Message, SupportsActionIO,
};
use std::fmt::Debug;
use std::marker::PhantomData;

/// Local Endpoint Type: Represents sending a message from this endpoint's perspective
///
/// - `IO`: IO capability type that must support the required action
/// - `M`: Extensible metadata type (implements CommMetadataTrait)
/// - `Msg`: Message type being sent
/// - `P`: Continuation Local Protocol after the send
/// - `AIO`: ActionIOTMarker specifying required I/O type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpChanSend<IO, M, Msg, P, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    AIO: ActionIOTMarker,
{
    pub(super) _io: PhantomData<IO>,
    pub(super) _metadata: PhantomData<M>,
    pub(super) _msg: PhantomData<Msg>,
    pub(super) _protocol: PhantomData<P>,
    pub(super) _aio: PhantomData<AIO>,
}

/// Local Endpoint Type: Represents receiving a message from this endpoint's perspective
///
/// - `IO`: IO capability type that must support the required action
/// - `M`: Extensible metadata type (implements CommMetadataTrait)
/// - `Msg`: Message type being received
/// - `P`: Continuation Local Protocol after the receive
/// - `AIO`: ActionIOTMarker specifying required I/O type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpChanRecv<IO, M, Msg, P, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    AIO: ActionIOTMarker,
{
    pub(super) _io: PhantomData<IO>,
    pub(super) _metadata: PhantomData<M>,
    pub(super) _msg: PhantomData<Msg>,
    pub(super) _protocol: PhantomData<P>,
    pub(super) _aio: PhantomData<AIO>,
}

/// Local Endpoint Type: Represents offering a choice from this endpoint's perspective
///
/// - `IO`: IO capability type that must support the required action
/// - `M`: Extensible metadata type (implements CommMetadataTrait)
/// - `Left`: Left branch local protocol
/// - `Right`: Right branch local protocol
/// - `AIO`: ActionIOTMarker for choice communication
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpChanOffer<IO, M, Left, Right, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    AIO: ActionIOTMarker,
{
    pub(super) _io: PhantomData<IO>,
    pub(super) _metadata: PhantomData<M>,
    pub(super) _left: PhantomData<Left>,
    pub(super) _right: PhantomData<Right>,
    pub(super) _aio: PhantomData<AIO>,
}

/// Local Endpoint Type: Represents making a choice from this endpoint's perspective
///
/// - `IO`: IO capability type that must support the required action
/// - `M`: Extensible metadata type (implements CommMetadataTrait)
/// - `Left`: Left branch local protocol
/// - `Right`: Right branch local protocol
/// - `AIO`: ActionIOTMarker for choice communication
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpChanChoice<IO, M, Left, Right, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    AIO: ActionIOTMarker,
{
    pub(super) _io: PhantomData<IO>,
    pub(super) _metadata: PhantomData<M>,
    pub(super) _left: PhantomData<Left>,
    pub(super) _right: PhantomData<Right>,
    pub(super) _aio: PhantomData<AIO>,
}

/// Local Endpoint Type: Represents parallel composition from this endpoint's perspective
///
/// - `IO`: IO capability type that must support the required action
/// - `M`: Extensible metadata type (implements CommMetadataTrait)
/// - `Left`: Left parallel branch local protocol
/// - `Right`: Right parallel branch local protocol
/// - `IsDisjoint`: Marker ensuring branches are disjoint
/// - `AIO`: ActionIOTMarker for parallel coordination
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpChanPar<IO, M, Left, Right, IsDisjoint, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    IsDisjoint: Send + Sync + 'static + Debug,
    AIO: ActionIOTMarker,
{
    pub(super) _io: PhantomData<IO>,
    pub(super) _metadata: PhantomData<M>,
    pub(super) _left: PhantomData<Left>,
    pub(super) _right: PhantomData<Right>,
    pub(super) _disjoint: PhantomData<IsDisjoint>,
    pub(super) _aio: PhantomData<AIO>,
}

/// Local Endpoint Type: Represents protocol termination from this endpoint's perspective
///
/// - `IO`: IO capability type that must support the required action
/// - `M`: Extensible metadata type (implements CommMetadataTrait)
/// - `AIO`: ActionIOTMarker for cleanup operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpChanEnd<IO, M, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    AIO: ActionIOTMarker,
{
    pub(super) _io: PhantomData<IO>,
    pub(super) _metadata: PhantomData<M>,
    pub(super) _aio: PhantomData<AIO>,
}

/// Local Endpoint Type: Represents protocol initialization from this endpoint's perspective
///
/// - `IO`: IO capability type that must support the required action
/// - `M`: Extensible metadata type (implements CommMetadataTrait)
/// - `Start`: Continuation local protocol after start
/// - `AIO`: ActionIOTMarker for initialization operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpChanStart<IO, M, Start, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Start: LocalProtocol,
    AIO: ActionIOTMarker,
{
    pub(super) _io: PhantomData<IO>,
    pub(super) _metadata: PhantomData<M>,
    pub(super) _start: PhantomData<Start>,
    pub(super) _aio: PhantomData<AIO>,
}
