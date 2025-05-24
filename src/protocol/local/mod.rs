//! # Local Endpoint Types for Enhanced MPST System
//!
//! This module provides Local Endpoint Types that represent the projected view
//! of protocols from individual role perspectives. These types use extensible
//! metadata patterns to enable downstream implementations to enhance metadata
//! while maintaining type safety and specification compliance.
//!
//! ## Key Design Features
//!
//! - **Extensible Metadata**: Uses `CommMetadataTrait` to allow downstream extensions
//! - **IO Capability Integration**: All endpoints verify required I/O capabilities  
//! - **Type Safety**: Compile-time verification of channel and message compatibility
//! - **Specification Compliance**: Follows the patterns established in `docs/duality.md`
//!
//! ## Core Local Endpoint Types
//!
//! - **EpChanSend/EpChanRecv**: Local endpoint message sending and receiving
//! - **EpChanChoice/EpChanOffer**: Local endpoint protocol branching and choice handling
//! - **EpChanPar**: Local endpoint parallel composition
//! - **EpChanEnd/EpChanStart**: Local endpoint protocol lifecycle management
//! - **Type Aliases**: Convenience types for common patterns

use crate::protocol::foundation::{
    ActionIOTMarker, BiDirectionalAction, CommMetadata, CommMetadataTrait, DefaultChan,
    LocalProtocol, Message, RequestLbl, ResponseLbl, SupportsActionIO,
};
use std::fmt::Debug;
use std::marker::PhantomData;

// ============================================================================
// Core Local Endpoint Types
// ============================================================================

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
    _io: PhantomData<IO>,
    _metadata: PhantomData<M>,
    _msg: PhantomData<Msg>,
    _protocol: PhantomData<P>,
    _aio: PhantomData<AIO>,
}

impl<IO, M, Msg, P, AIO> LocalProtocol for EpChanSend<IO, M, Msg, P, AIO>
where
    IO: SupportsActionIO<AIO> + Debug + Send + Sync + 'static,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    AIO: ActionIOTMarker,
{
}

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
    _io: PhantomData<IO>,
    _metadata: PhantomData<M>,
    _msg: PhantomData<Msg>,
    _protocol: PhantomData<P>,
    _aio: PhantomData<AIO>,
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
    _io: PhantomData<IO>,
    _metadata: PhantomData<M>,
    _left: PhantomData<Left>,
    _right: PhantomData<Right>,
    _aio: PhantomData<AIO>,
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
    _io: PhantomData<IO>,
    _metadata: PhantomData<M>,
    _left: PhantomData<Left>,
    _right: PhantomData<Right>,
    _aio: PhantomData<AIO>,
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
    _io: PhantomData<IO>,
    _metadata: PhantomData<M>,
    _left: PhantomData<Left>,
    _right: PhantomData<Right>,
    _disjoint: PhantomData<IsDisjoint>,
    _aio: PhantomData<AIO>,
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
    _io: PhantomData<IO>,
    _metadata: PhantomData<M>,
    _aio: PhantomData<AIO>,
}

impl<IO, M, AIO> LocalProtocol for EpChanEnd<IO, M, AIO>
where
    IO: SupportsActionIO<AIO> + Debug + Send + Sync + 'static,
    M: CommMetadataTrait,
    AIO: ActionIOTMarker,
{
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
    _io: PhantomData<IO>,
    _metadata: PhantomData<M>,
    _start: PhantomData<Start>,
    _aio: PhantomData<AIO>,
}

impl<IO, M, Start, AIO> LocalProtocol for EpChanStart<IO, M, Start, AIO>
where
    IO: SupportsActionIO<AIO> + Debug + Send + Sync + 'static,
    M: CommMetadataTrait,
    Start: LocalProtocol,
    AIO: ActionIOTMarker,
{
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
// Type Aliases for Common Patterns
// ============================================================================

/// Convenience type alias for simple send with default CommMetadata
pub type SimpleEpSend<IO, Msg, P> =
    EpChanSend<IO, CommMetadata<DefaultChan, RequestLbl>, Msg, P, BiDirectionalAction>;

/// Convenience type alias for simple receive with default CommMetadata
pub type SimpleEpRecv<IO, Msg, P> =
    EpChanRecv<IO, CommMetadata<DefaultChan, ResponseLbl>, Msg, P, BiDirectionalAction>;

/// Convenience type alias for simple offer with default CommMetadata
pub type SimpleEpOffer<IO, Left, Right> =
    EpChanOffer<IO, CommMetadata<DefaultChan, RequestLbl>, Left, Right, BiDirectionalAction>;

/// Convenience type alias for simple choice with default CommMetadata
pub type SimpleEpChoice<IO, Left, Right> =
    EpChanChoice<IO, CommMetadata<DefaultChan, RequestLbl>, Left, Right, BiDirectionalAction>;

/// Convenience type alias for simple parallel with default CommMetadata
pub type SimpleEpPar<IO, Left, Right, IsDisjoint> = EpChanPar<
    IO,
    CommMetadata<DefaultChan, RequestLbl>,
    Left,
    Right,
    IsDisjoint,
    BiDirectionalAction,
>;

/// Convenience type alias for simple termination with default CommMetadata
pub type SimpleEpEnd<IO> =
    EpChanEnd<IO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;

/// Convenience type alias for simple start with default CommMetadata
pub type SimpleEpStart<IO, Start> =
    EpChanStart<IO, CommMetadata<DefaultChan, RequestLbl>, Start, BiDirectionalAction>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests;
