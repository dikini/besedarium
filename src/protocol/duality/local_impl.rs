//! IsDual implementations for Local Endpoint types
//!
//! This module contains all the duality implementations for Local Protocol
//! endpoint types, including EpChanEnd, EpChanSend, EpChanRecv, EpChanChoice,
//! EpChanOffer, EpChanPar, and EpChanStart.

use super::{EqualsTrue, IsDual};
use crate::protocol::foundation::{
    ActionIOTMarker, CommMetadataTrait, LocalProtocol, Message, SupportsActionIO,
};
use crate::protocol::local::{
    EpChanChoice, EpChanEnd, EpChanOffer, EpChanPar, EpChanRecv, EpChanSend, EpChanStart,
};
use crate::types::True;

/// EpChanEnd is dual to itself (with compatible IO and metadata)
///
/// Local endpoint termination is self-dual regardless of IO capabilities,
/// as long as both endpoints use compatible metadata.
impl<IO1, IO2, M, AIO> IsDual<EpChanEnd<IO1, M, AIO>, EpChanEnd<IO2, M, AIO>> for ()
where
    IO1: SupportsActionIO<AIO>,
    IO2: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    AIO: ActionIOTMarker,
{
    type Output = True;
}

/// EpChanSend<IO1, M, Msg, P, AIO> is dual to EpChanRecv<IO2, M, Msg, Q, AIO>
///
/// A local send endpoint is dual to a local receive endpoint when they
/// handle the same message type and have dual continuation protocols.
impl<IO1, IO2, M, Msg, P, Q, AIO>
    IsDual<EpChanSend<IO1, M, Msg, P, AIO>, EpChanRecv<IO2, M, Msg, Q, AIO>> for ()
where
    IO1: SupportsActionIO<AIO>,
    IO2: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    Q: LocalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<P, Q>,
    <() as IsDual<P, Q>>::Output: EqualsTrue,
{
    type Output = True;
}

/// EpChanRecv<IO1, M, Msg, P, AIO> is dual to EpChanSend<IO2, M, Msg, Q, AIO>
///
/// Symmetric to the above: a local receive endpoint is dual to a local send endpoint.
impl<IO1, IO2, M, Msg, P, Q, AIO>
    IsDual<EpChanRecv<IO1, M, Msg, P, AIO>, EpChanSend<IO2, M, Msg, Q, AIO>> for ()
where
    IO1: SupportsActionIO<AIO>,
    IO2: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    Q: LocalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<P, Q>,
    <() as IsDual<P, Q>>::Output: EqualsTrue,
{
    type Output = True;
}

/// EpChanChoice<IO1, M, Left, Right, AIO> is dual to EpChanOffer<IO2, M, LeftDual, RightDual, AIO>
///
/// A local choice endpoint is dual to a local offer endpoint when each branch
/// is dual to the corresponding branch in the other endpoint.
impl<IO1, IO2, M, Left, Right, LeftDual, RightDual, AIO>
    IsDual<EpChanChoice<IO1, M, Left, Right, AIO>, EpChanOffer<IO2, M, LeftDual, RightDual, AIO>>
    for ()
where
    IO1: SupportsActionIO<AIO>,
    IO2: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    LeftDual: LocalProtocol,
    RightDual: LocalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<Left, LeftDual>,
    (): IsDual<Right, RightDual>,
    <() as IsDual<Left, LeftDual>>::Output: EqualsTrue,
    <() as IsDual<Right, RightDual>>::Output: EqualsTrue,
{
    type Output = True;
}

/// EpChanOffer<IO1, M, Left, Right, AIO> is dual to EpChanChoice<IO2, M, LeftDual, RightDual, AIO>
///
/// Symmetric to the above: a local offer endpoint is dual to a local choice endpoint.
impl<IO1, IO2, M, Left, Right, LeftDual, RightDual, AIO>
    IsDual<EpChanOffer<IO1, M, Left, Right, AIO>, EpChanChoice<IO2, M, LeftDual, RightDual, AIO>>
    for ()
where
    IO1: SupportsActionIO<AIO>,
    IO2: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    LeftDual: LocalProtocol,
    RightDual: LocalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<Left, LeftDual>,
    (): IsDual<Right, RightDual>,
    <() as IsDual<Left, LeftDual>>::Output: EqualsTrue,
    <() as IsDual<Right, RightDual>>::Output: EqualsTrue,
{
    type Output = True;
}

/// EpChanPar<IO1, M, Left, Right, IsDisjoint, AIO> is dual to EpChanPar<IO2, M, LeftDual, RightDual, IsDisjoint, AIO>
///
/// Local parallel endpoints are dual when each constituent branch is dual and
/// the disjointness property is preserved.
impl<IO1, IO2, M, Left, Right, LeftDual, RightDual, IsDisjoint, AIO>
    IsDual<
        EpChanPar<IO1, M, Left, Right, IsDisjoint, AIO>,
        EpChanPar<IO2, M, LeftDual, RightDual, IsDisjoint, AIO>,
    > for ()
where
    IO1: SupportsActionIO<AIO>,
    IO2: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    LeftDual: LocalProtocol,
    RightDual: LocalProtocol,
    IsDisjoint: Send + Sync + 'static + std::fmt::Debug,
    AIO: ActionIOTMarker,
    (): IsDual<Left, LeftDual>,
    (): IsDual<Right, RightDual>,
    <() as IsDual<Left, LeftDual>>::Output: EqualsTrue,
    <() as IsDual<Right, RightDual>>::Output: EqualsTrue,
{
    type Output = True;
}

/// EpChanStart<IO1, M, Start, AIO> is dual to EpChanStart<IO2, M, Start, AIO>
///
/// Local start endpoints are self-dual when they wrap the same protocol.
impl<IO1, IO2, M, Start, AIO>
    IsDual<EpChanStart<IO1, M, Start, AIO>, EpChanStart<IO2, M, Start, AIO>> for ()
where
    IO1: SupportsActionIO<AIO>,
    IO2: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Start: LocalProtocol,
    AIO: ActionIOTMarker,
{
    type Output = True;
}
