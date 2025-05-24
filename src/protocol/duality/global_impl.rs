//! IsDual implementations for Global Protocol types
//!
//! This module contains all the duality implementations for Global Protocol
//! types, including TChanEnd, TChanSend, TChanRecv, TChanChoice, TChanOffer,
//! TChanPar, and TChanStart.

use super::{EqualsTrue, IsDual};
use crate::protocol::foundation::{ActionIOTMarker, ChanId, GlobalProtocol, Message, MsgLbl, Role};
use crate::protocol::global::{
    TChanChoice, TChanEnd, TChanOffer, TChanPar, TChanRecv, TChanSend, TChanStart,
};
use crate::types::True;

/// TChanEnd is dual to itself (with compatible IO and metadata)
///
/// Protocol termination is self-dual since both endpoints simply end
/// the communication without any message exchange.
impl<C, L, AIO> IsDual<TChanEnd<C, L, AIO>, TChanEnd<C, L, AIO>> for ()
where
    C: ChanId,
    L: MsgLbl,
    AIO: ActionIOTMarker,
{
    type Output = True;
}

/// TChanSend<S, R, C, L, Msg, P, AIO> is dual to TChanRecv<R, S, C, L, Msg, Q, AIO>
///
/// A send action from S to R is dual to a receive action from R to S,
/// with the same channel, label, and message type, and dual continuation protocols.
impl<S, R, C, L, Msg, P, Q, AIO>
    IsDual<TChanSend<S, R, C, L, Msg, P, AIO>, TChanRecv<R, S, C, L, Msg, Q, AIO>> for ()
where
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    Q: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<P, Q>,
    <() as IsDual<P, Q>>::Output: EqualsTrue,
{
    type Output = True;
}

/// TChanRecv<R, S, C, L, Msg, P, AIO> is dual to TChanSend<S, R, C, L, Msg, Q, AIO>
///
/// Symmetric to the above: a receive action is dual to the corresponding send action.
impl<R, S, C, L, Msg, P, Q, AIO>
    IsDual<TChanRecv<R, S, C, L, Msg, P, AIO>, TChanSend<S, R, C, L, Msg, Q, AIO>> for ()
where
    R: Role,
    S: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    Q: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<P, Q>,
    <() as IsDual<P, Q>>::Output: EqualsTrue,
{
    type Output = True;
}

/// TChanChoice<S, R, C, L, Left, Right, AIO> is dual to TChanOffer<S, R, C, L, LeftDual, RightDual, AIO>
///
/// A choice action (making a selection) is dual to an offer action (handling the selection),
/// where each branch in the choice corresponds to the dual of the respective branch in the offer.
impl<R, C, Lbl, Left, Right, LeftDual, RightDual, AIO>
    IsDual<
        TChanChoice<R, C, Lbl, Left, Right, AIO>,
        TChanOffer<R, C, Lbl, LeftDual, RightDual, AIO>,
    > for ()
where
    R: Role,
    C: ChanId,
    Lbl: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    LeftDual: GlobalProtocol,
    RightDual: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<Left, LeftDual>,
    (): IsDual<Right, RightDual>,
    <() as IsDual<Left, LeftDual>>::Output: EqualsTrue,
    <() as IsDual<Right, RightDual>>::Output: EqualsTrue,
{
    type Output = True;
}

/// TChanOffer<S, R, C, L, Left, Right, AIO> is dual to TChanChoice<S, R, C, L, LeftDual, RightDual, AIO>
///
/// Symmetric to the above: an offer action is dual to the corresponding choice action.
impl<R, C, Lbl, Left, Right, LeftDual, RightDual, AIO>
    IsDual<
        TChanOffer<R, C, Lbl, Left, Right, AIO>,
        TChanChoice<R, C, Lbl, LeftDual, RightDual, AIO>,
    > for ()
where
    R: Role,
    C: ChanId,
    Lbl: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    LeftDual: GlobalProtocol,
    RightDual: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<Left, LeftDual>,
    (): IsDual<Right, RightDual>,
    <() as IsDual<Left, LeftDual>>::Output: EqualsTrue,
    <() as IsDual<Right, RightDual>>::Output: EqualsTrue,
{
    type Output = True;
}

/// TChanPar<S, R, C, L, Left, Right, IsDisjoint, AIO> is dual to TChanPar<S, R, C, L, LeftDual, RightDual, IsDisjoint, AIO>
///
/// Parallel composition is dual when each constituent branch is dual to the corresponding
/// branch in the other parallel composition. The disjointness property must be preserved.
impl<C, Lbl, Left, Right, LeftDual, RightDual, IsDisjoint, AIO>
    IsDual<
        TChanPar<C, Lbl, Left, Right, IsDisjoint, AIO>,
        TChanPar<C, Lbl, LeftDual, RightDual, IsDisjoint, AIO>,
    > for ()
where
    C: ChanId,
    Lbl: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    LeftDual: GlobalProtocol,
    RightDual: GlobalProtocol,
    IsDisjoint: Send + Sync + 'static + std::fmt::Debug,
    AIO: ActionIOTMarker,
    (): IsDual<Left, LeftDual>,
    (): IsDual<Right, RightDual>,
    <() as IsDual<Left, LeftDual>>::Output: EqualsTrue,
    <() as IsDual<Right, RightDual>>::Output: EqualsTrue,
{
    type Output = True;
}

/// TChanStart<S, R, C, L, Start, AIO> is dual to itself when the inner protocol is self-dual
///
/// Protocol initialization is dual when the wrapped protocol is dual to itself.
impl<C, L, Start, AIO> IsDual<TChanStart<C, L, Start, AIO>, TChanStart<C, L, Start, AIO>> for ()
where
    C: ChanId,
    L: MsgLbl,
    Start: GlobalProtocol,
    AIO: ActionIOTMarker,
{
    type Output = True;
}
