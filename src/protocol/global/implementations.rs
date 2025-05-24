//! # Global Protocol Trait Implementations
//!
//! This module contains the trait implementations and constructor methods
//! for all global protocol types. These provide the behavioral interface
//! for the global protocol choreography system.

use super::protocols::*;
use crate::protocol::foundation::{
    ActionIOTMarker, ChanId, CommMetadata, GlobalProtocol, Message, MsgLbl, Role,
};
use std::fmt::Debug;
use std::marker::PhantomData;

// ============================================================================
// GlobalProtocol Trait Implementations
// ============================================================================

impl<
        S: Role,
        R: Role,
        C: ChanId,
        L: MsgLbl,
        Msg: Message,
        P: GlobalProtocol,
        AIO: ActionIOTMarker,
    > GlobalProtocol for TChanSend<S, R, C, L, Msg, P, AIO>
{
}

impl<
        R: Role,
        S: Role,
        C: ChanId,
        L: MsgLbl,
        Msg: Message,
        P: GlobalProtocol,
        AIO: ActionIOTMarker,
    > GlobalProtocol for TChanRecv<R, S, C, L, Msg, P, AIO>
{
}

impl<
        R: Role,
        C: ChanId,
        Lbl: MsgLbl,
        Left: GlobalProtocol,
        Right: GlobalProtocol,
        AIO: ActionIOTMarker,
    > GlobalProtocol for TChanChoice<R, C, Lbl, Left, Right, AIO>
{
}

impl<
        R: Role,
        C: ChanId,
        Lbl: MsgLbl,
        Left: GlobalProtocol,
        Right: GlobalProtocol,
        AIO: ActionIOTMarker,
    > GlobalProtocol for TChanOffer<R, C, Lbl, Left, Right, AIO>
{
}

impl<
        C: ChanId,
        Lbl: MsgLbl,
        Left: GlobalProtocol,
        Right: GlobalProtocol,
        IsDisjoint: Send + Sync + 'static + Debug,
        AIO: ActionIOTMarker,
    > GlobalProtocol for TChanPar<C, Lbl, Left, Right, IsDisjoint, AIO>
{
}

impl<C: ChanId, L: MsgLbl, AIO: ActionIOTMarker> GlobalProtocol for TChanEnd<C, L, AIO> {}

impl<C: ChanId, L: MsgLbl, Start: GlobalProtocol, AIO: ActionIOTMarker> GlobalProtocol
    for TChanStart<C, L, Start, AIO>
{
}

// ============================================================================
// Constructor Method Implementations
// ============================================================================

impl<
        S: Role,
        R: Role,
        C: ChanId,
        L: MsgLbl,
        Msg: Message,
        P: GlobalProtocol,
        AIO: ActionIOTMarker,
    > TChanSend<S, R, C, L, Msg, P, AIO>
{
    pub fn new() -> Self {
        Self {
            _sender: PhantomData,
            _receiver: PhantomData,
            _chan: PhantomData,
            _lbl: PhantomData,
            _msg: PhantomData,
            _protocol: PhantomData,
            _aio: PhantomData,
        }
    }

    /// Get CommMetadata for this send operation
    pub fn metadata() -> CommMetadata<C, L>
    where
        C: Default,
        L: Default,
    {
        CommMetadata::new(C::default(), L::default())
    }
}

impl<
        R: Role,
        S: Role,
        C: ChanId,
        L: MsgLbl,
        Msg: Message,
        P: GlobalProtocol,
        AIO: ActionIOTMarker,
    > TChanRecv<R, S, C, L, Msg, P, AIO>
{
    pub fn new() -> Self {
        Self {
            _receiver: PhantomData,
            _sender: PhantomData,
            _chan: PhantomData,
            _lbl: PhantomData,
            _msg: PhantomData,
            _protocol: PhantomData,
            _aio: PhantomData,
        }
    }

    /// Get CommMetadata for this receive operation
    pub fn metadata() -> CommMetadata<C, L>
    where
        C: Default,
        L: Default,
    {
        CommMetadata::new(C::default(), L::default())
    }
}

impl<
        R: Role,
        C: ChanId,
        Lbl: MsgLbl,
        Left: GlobalProtocol,
        Right: GlobalProtocol,
        AIO: ActionIOTMarker,
    > TChanChoice<R, C, Lbl, Left, Right, AIO>
{
    pub fn new() -> Self {
        Self {
            _chooser: PhantomData,
            _chan: PhantomData,
            _lbl: PhantomData,
            _left: PhantomData,
            _right: PhantomData,
            _aio: PhantomData,
        }
    }
}

impl<
        R: Role,
        C: ChanId,
        Lbl: MsgLbl,
        Left: GlobalProtocol,
        Right: GlobalProtocol,
        AIO: ActionIOTMarker,
    > TChanOffer<R, C, Lbl, Left, Right, AIO>
{
    pub fn new() -> Self {
        Self {
            _offerer: PhantomData,
            _chan: PhantomData,
            _lbl: PhantomData,
            _left: PhantomData,
            _right: PhantomData,
            _aio: PhantomData,
        }
    }
}

impl<
        C: ChanId,
        Lbl: MsgLbl,
        Left: GlobalProtocol,
        Right: GlobalProtocol,
        IsDisjoint: Send + Sync + 'static + Debug,
        AIO: ActionIOTMarker,
    > TChanPar<C, Lbl, Left, Right, IsDisjoint, AIO>
{
    pub fn new() -> Self {
        Self {
            _chan: PhantomData,
            _lbl: PhantomData,
            _left: PhantomData,
            _right: PhantomData,
            _disjoint: PhantomData,
            _aio: PhantomData,
        }
    }
}

impl<C: ChanId, L: MsgLbl, AIO: ActionIOTMarker> TChanEnd<C, L, AIO> {
    pub fn new() -> Self {
        Self {
            _chan: PhantomData,
            _lbl: PhantomData,
            _aio: PhantomData,
        }
    }
}

impl<C: ChanId, L: MsgLbl, Start: GlobalProtocol, AIO: ActionIOTMarker>
    TChanStart<C, L, Start, AIO>
{
    pub fn new() -> Self {
        Self {
            _chan: PhantomData,
            _lbl: PhantomData,
            _start: PhantomData,
            _aio: PhantomData,
        }
    }
}
