//! # Global Protocol Types for Enhanced MPST System
//!
//! This module provides Global Protocol Types that incorporate `CommMetadata` and 
//! `ActionIOType` concepts as specified in `docs/duality.md`. These types represent
//! the choreography of multi-party protocols with explicit channel management.
//!
//! ## Key Components
//!
//! - **TChanSend/TChanRecv**: Channel-based message sending and receiving
//! - **TChanChoice**: Protocol branching and choice constructions
//! - **TChanPar**: Parallel composition of protocol branches
//! - **TChanEnd/TChanStart**: Protocol lifecycle management
//! - **Type Aliases**: Convenience types for common patterns
//! - **Builder Functions**: Easy construction of complex protocols

use crate::protocol::foundation::{
    Role, Message, GlobalProtocol, CommMetadata, ActionIOTMarker, ChanId, MsgLbl,
    DefaultChan, RequestLbl, ResponseLbl, BiDirectionalAction,
};
use std::marker::PhantomData;
use std::fmt::Debug;

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
pub struct TChanSend<S: Role, R: Role, C: ChanId, L: MsgLbl, Msg: Message, P: GlobalProtocol, AIO: ActionIOTMarker> {
    _sender: PhantomData<S>,
    _receiver: PhantomData<R>,
    _chan: PhantomData<C>,
    _lbl: PhantomData<L>,
    _msg: PhantomData<Msg>,
    _protocol: PhantomData<P>,
    _aio: PhantomData<AIO>,
}

impl<S: Role, R: Role, C: ChanId, L: MsgLbl, Msg: Message, P: GlobalProtocol, AIO: ActionIOTMarker> 
    GlobalProtocol for TChanSend<S, R, C, L, Msg, P, AIO> {}

impl<S: Role, R: Role, C: ChanId, L: MsgLbl, Msg: Message, P: GlobalProtocol, AIO: ActionIOTMarker> 
    TChanSend<S, R, C, L, Msg, P, AIO> {
    
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
pub struct TChanRecv<R: Role, S: Role, C: ChanId, L: MsgLbl, Msg: Message, P: GlobalProtocol, AIO: ActionIOTMarker> {
    _receiver: PhantomData<R>,
    _sender: PhantomData<S>,
    _chan: PhantomData<C>,
    _lbl: PhantomData<L>,
    _msg: PhantomData<Msg>,
    _protocol: PhantomData<P>,
    _aio: PhantomData<AIO>,
}

impl<R: Role, S: Role, C: ChanId, L: MsgLbl, Msg: Message, P: GlobalProtocol, AIO: ActionIOTMarker> 
    GlobalProtocol for TChanRecv<R, S, C, L, Msg, P, AIO> {}

impl<R: Role, S: Role, C: ChanId, L: MsgLbl, Msg: Message, P: GlobalProtocol, AIO: ActionIOTMarker> 
    TChanRecv<R, S, C, L, Msg, P, AIO> {
    
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

/// Global Type: Represents a choice between protocol branches
///
/// - `R`: Role making the choice
/// - `C`: Channel ID type
/// - `Lbl`: Message label type for the choice point
/// - `Left`: Left branch protocol
/// - `Right`: Right branch protocol  
/// - `AIO`: ActionIOTMarker for choice communication
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanChoice<R: Role, C: ChanId, Lbl: MsgLbl, Left: GlobalProtocol, Right: GlobalProtocol, AIO: ActionIOTMarker> {
    _chooser: PhantomData<R>,
    _chan: PhantomData<C>,
    _lbl: PhantomData<Lbl>,
    _left: PhantomData<Left>,
    _right: PhantomData<Right>,
    _aio: PhantomData<AIO>,
}

impl<R: Role, C: ChanId, Lbl: MsgLbl, Left: GlobalProtocol, Right: GlobalProtocol, AIO: ActionIOTMarker> 
    GlobalProtocol for TChanChoice<R, C, Lbl, Left, Right, AIO> {}

impl<R: Role, C: ChanId, Lbl: MsgLbl, Left: GlobalProtocol, Right: GlobalProtocol, AIO: ActionIOTMarker> 
    TChanChoice<R, C, Lbl, Left, Right, AIO> {
    
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

/// Global Type: Represents parallel composition of protocols
///
/// - `C`: Channel ID type
/// - `Lbl`: Message label type for parallel execution context
/// - `Left`: Left parallel branch
/// - `Right`: Right parallel branch
/// - `IsDisjoint`: Marker ensuring branches are disjoint (must be Send + Sync + Debug)
/// - `AIO`: ActionIOTMarker for parallel coordination
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanPar<C: ChanId, Lbl: MsgLbl, Left: GlobalProtocol, Right: GlobalProtocol, IsDisjoint: Send + Sync + 'static + Debug, AIO: ActionIOTMarker> {
    _chan: PhantomData<C>,
    _lbl: PhantomData<Lbl>,
    _left: PhantomData<Left>,
    _right: PhantomData<Right>,
    _disjoint: PhantomData<IsDisjoint>,
    _aio: PhantomData<AIO>,
}

impl<C: ChanId, Lbl: MsgLbl, Left: GlobalProtocol, Right: GlobalProtocol, IsDisjoint: Send + Sync + 'static + Debug, AIO: ActionIOTMarker> 
    GlobalProtocol for TChanPar<C, Lbl, Left, Right, IsDisjoint, AIO> {}

impl<C: ChanId, Lbl: MsgLbl, Left: GlobalProtocol, Right: GlobalProtocol, IsDisjoint: Send + Sync + 'static + Debug, AIO: ActionIOTMarker> 
    TChanPar<C, Lbl, Left, Right, IsDisjoint, AIO> {
    
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

/// Global Type: Represents protocol termination
///
/// - `C`: Channel ID type
/// - `L`: Message label type for termination context
/// - `AIO`: ActionIOTMarker for cleanup operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanEnd<C: ChanId, L: MsgLbl, AIO: ActionIOTMarker> {
    _chan: PhantomData<C>,
    _lbl: PhantomData<L>,
    _aio: PhantomData<AIO>,
}

impl<C: ChanId, L: MsgLbl, AIO: ActionIOTMarker> 
    GlobalProtocol for TChanEnd<C, L, AIO> {}

impl<C: ChanId, L: MsgLbl, AIO: ActionIOTMarker> 
    TChanEnd<C, L, AIO> {
    
    pub fn new() -> Self {
        Self {
            _chan: PhantomData,
            _lbl: PhantomData,
            _aio: PhantomData,
        }
    }
}

/// Global Type: Represents protocol initialization
///
/// - `C`: Channel ID type
/// - `L`: Message label type for initialization context
/// - `Start`: Continuation protocol after start
/// - `AIO`: ActionIOTMarker for initialization operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanStart<C: ChanId, L: MsgLbl, Start: GlobalProtocol, AIO: ActionIOTMarker> {
    _chan: PhantomData<C>,
    _lbl: PhantomData<L>,
    _start: PhantomData<Start>,
    _aio: PhantomData<AIO>,
}

impl<C: ChanId, L: MsgLbl, Start: GlobalProtocol, AIO: ActionIOTMarker> 
    GlobalProtocol for TChanStart<C, L, Start, AIO> {}

impl<C: ChanId, L: MsgLbl, Start: GlobalProtocol, AIO: ActionIOTMarker> 
    TChanStart<C, L, Start, AIO> {
    
    pub fn new() -> Self {
        Self {
            _chan: PhantomData,
            _lbl: PhantomData,
            _start: PhantomData,
            _aio: PhantomData,
        }
    }
}

// ============================================================================
// Type Aliases for Common Patterns
// ============================================================================

/// Convenience type alias for simple send with default channel
pub type SimpleChannelSend<S, R, Msg, P> = TChanSend<
    S, 
    R, 
    DefaultChan,
    RequestLbl,
    Msg, 
    P, 
    BiDirectionalAction
>;

/// Convenience type alias for simple receive with default channel  
pub type SimpleChannelRecv<R, S, Msg, P> = TChanRecv<
    R, 
    S, 
    DefaultChan,
    ResponseLbl,
    Msg, 
    P, 
    BiDirectionalAction
>;

/// Convenience type alias for simple choice with default channel
pub type SimpleChannelChoice<R, Left, Right> = TChanChoice<
    R,
    DefaultChan,
    RequestLbl,
    Left,
    Right,
    BiDirectionalAction
>;

/// Convenience type alias for simple parallel composition with default channel  
pub type SimpleChannelPar<Left, Right, IsDisjoint> = TChanPar<
    DefaultChan,
    RequestLbl,
    Left,
    Right,
    IsDisjoint,
    BiDirectionalAction
>;

/// Convenience type alias for simple termination with default channel
pub type SimpleChannelEnd = TChanEnd<
    DefaultChan,
    RequestLbl,
    BiDirectionalAction
>;

/// Convenience type alias for simple start with default channel
pub type SimpleChannelStart<Start> = TChanStart<
    DefaultChan,
    RequestLbl,
    Start,
    BiDirectionalAction
>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests;
