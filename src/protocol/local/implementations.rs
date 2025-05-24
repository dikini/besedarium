//! # Local Endpoint Implementation
//!
//! This module contains the trait implementations and constructor methods
//! for all local endpoint types in the Enhanced MPST System.

use super::endpoints::*;
use crate::protocol::foundation::{
    ActionIOTMarker, CommMetadataTrait, LocalProtocol, Message, SupportsActionIO,
};
use std::fmt::Debug;
use std::marker::PhantomData;

// ============================================================================
// LocalProtocol Trait Implementations
// ============================================================================

impl<IO, M, Msg, P, AIO> LocalProtocol for EpChanSend<IO, M, Msg, P, AIO>
where
    IO: SupportsActionIO<AIO> + Debug + Send + Sync + 'static,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    AIO: ActionIOTMarker,
{
}

impl<IO, M, Msg, P, AIO> LocalProtocol for EpChanRecv<IO, M, Msg, P, AIO>
where
    IO: SupportsActionIO<AIO> + Debug + Send + Sync + 'static,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    AIO: ActionIOTMarker,
{
}

impl<IO, M, Left, Right, AIO> LocalProtocol for EpChanOffer<IO, M, Left, Right, AIO>
where
    IO: SupportsActionIO<AIO> + Debug + Send + Sync + 'static,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    AIO: ActionIOTMarker,
{
}

impl<IO, M, Left, Right, AIO> LocalProtocol for EpChanChoice<IO, M, Left, Right, AIO>
where
    IO: SupportsActionIO<AIO> + Debug + Send + Sync + 'static,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    AIO: ActionIOTMarker,
{
}

impl<IO, M, Left, Right, IsDisjoint, AIO> LocalProtocol
    for EpChanPar<IO, M, Left, Right, IsDisjoint, AIO>
where
    IO: SupportsActionIO<AIO> + Debug + Send + Sync + 'static,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    IsDisjoint: Send + Sync + 'static + Debug,
    AIO: ActionIOTMarker,
{
}

impl<IO, M, AIO> LocalProtocol for EpChanEnd<IO, M, AIO>
where
    IO: SupportsActionIO<AIO> + Debug + Send + Sync + 'static,
    M: CommMetadataTrait,
    AIO: ActionIOTMarker,
{
}

impl<IO, M, Start, AIO> LocalProtocol for EpChanStart<IO, M, Start, AIO>
where
    IO: SupportsActionIO<AIO> + Debug + Send + Sync + 'static,
    M: CommMetadataTrait,
    Start: LocalProtocol,
    AIO: ActionIOTMarker,
{
}

// ============================================================================
// Constructor and Method Implementations
// ============================================================================

impl<IO, M, Msg, P, AIO> EpChanSend<IO, M, Msg, P, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    AIO: ActionIOTMarker,
{
    pub fn new() -> Self {
        Self {
            _io: PhantomData,
            _metadata: PhantomData,
            _msg: PhantomData,
            _protocol: PhantomData,
            _aio: PhantomData,
        }
    }

    /// Get metadata for this send operation
    pub fn metadata() -> M
    where
        M::ChanId: Default,
        M::MsgLbl: Default,
    {
        M::new(M::ChanId::default(), M::MsgLbl::default())
    }
}

impl<IO, M, Msg, P, AIO> EpChanRecv<IO, M, Msg, P, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    AIO: ActionIOTMarker,
{
    pub fn new() -> Self {
        Self {
            _io: PhantomData,
            _metadata: PhantomData,
            _msg: PhantomData,
            _protocol: PhantomData,
            _aio: PhantomData,
        }
    }

    /// Get metadata for this receive operation
    pub fn metadata() -> M
    where
        M::ChanId: Default,
        M::MsgLbl: Default,
    {
        M::new(M::ChanId::default(), M::MsgLbl::default())
    }
}

impl<IO, M, Left, Right, AIO> EpChanOffer<IO, M, Left, Right, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    AIO: ActionIOTMarker,
{
    pub fn new() -> Self {
        Self {
            _io: PhantomData,
            _metadata: PhantomData,
            _left: PhantomData,
            _right: PhantomData,
            _aio: PhantomData,
        }
    }
}

impl<IO, M, Left, Right, AIO> EpChanChoice<IO, M, Left, Right, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    AIO: ActionIOTMarker,
{
    pub fn new() -> Self {
        Self {
            _io: PhantomData,
            _metadata: PhantomData,
            _left: PhantomData,
            _right: PhantomData,
            _aio: PhantomData,
        }
    }
}

impl<IO, M, Left, Right, IsDisjoint, AIO> EpChanPar<IO, M, Left, Right, IsDisjoint, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    IsDisjoint: Send + Sync + 'static + Debug,
    AIO: ActionIOTMarker,
{
    pub fn new() -> Self {
        Self {
            _io: PhantomData,
            _metadata: PhantomData,
            _left: PhantomData,
            _right: PhantomData,
            _disjoint: PhantomData,
            _aio: PhantomData,
        }
    }
}

impl<IO, M, AIO> EpChanEnd<IO, M, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    AIO: ActionIOTMarker,
{
    pub fn new() -> Self {
        Self {
            _io: PhantomData,
            _metadata: PhantomData,
            _aio: PhantomData,
        }
    }
}

impl<IO, M, Start, AIO> EpChanStart<IO, M, Start, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Start: LocalProtocol,
    AIO: ActionIOTMarker,
{
    pub fn new() -> Self {
        Self {
            _io: PhantomData,
            _metadata: PhantomData,
            _start: PhantomData,
            _aio: PhantomData,
        }
    }
}

// ============================================================================
// Default Implementations
// ============================================================================

impl<IO, M, Msg, P, AIO> Default for EpChanSend<IO, M, Msg, P, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    AIO: ActionIOTMarker,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<IO, M, Msg, P, AIO> Default for EpChanRecv<IO, M, Msg, P, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    AIO: ActionIOTMarker,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<IO, M, Left, Right, AIO> Default for EpChanOffer<IO, M, Left, Right, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    AIO: ActionIOTMarker,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<IO, M, Left, Right, AIO> Default for EpChanChoice<IO, M, Left, Right, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    AIO: ActionIOTMarker,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<IO, M, Left, Right, IsDisjoint, AIO> Default for EpChanPar<IO, M, Left, Right, IsDisjoint, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    IsDisjoint: Send + Sync + 'static + Debug,
    AIO: ActionIOTMarker,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<IO, M, AIO> Default for EpChanEnd<IO, M, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    AIO: ActionIOTMarker,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<IO, M, Start, AIO> Default for EpChanStart<IO, M, Start, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Start: LocalProtocol,
    AIO: ActionIOTMarker,
{
    fn default() -> Self {
        Self::new()
    }
}
