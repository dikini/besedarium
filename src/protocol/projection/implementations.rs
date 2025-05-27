//! Project trait implementations for all global protocol types
//!
//! This module contains the core implementations of the Project trait
//! for projecting global protocols to local endpoint types based on roles.

use super::Project;
use crate::protocol::foundation::{
    ActionIOTMarker, ChanId, CommMetadata, GlobalProtocol, LocalProtocol, Message, MsgLbl, Role,
    SupportsActionIO,
};
use crate::protocol::global::{TChanChoice, TChanEnd, TChanPar, TChanRecv, TChanSend, TChanStart};
use crate::protocol::local::{EpChanChoice, EpChanEnd, EpChanPar, EpChanStart};
use crate::protocol::projection::helpers::{Bool, ProjectRecvCase, ProjectSendCase, RoleEq};
use std::fmt::Debug;

// ============================================================================
// Project Trait Implementations
// ============================================================================

/// Project TChanSend: Role-based dispatch to determine send vs recv vs continuation
impl<Me, S, R, C, L, Msg, P, AIO> Project<TChanSend<S, R, C, L, Msg, P, AIO>, Me> for ()
where
    Me: Role,
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    Me: RoleEq<S>,
    <Me as RoleEq<S>>::Output: Bool,
    (): ProjectSendCase<Me, S, R, C, L, Msg, P, AIO, <Me as RoleEq<S>>::Output>,
{
    type Output =
        <() as ProjectSendCase<Me, S, R, C, L, Msg, P, AIO, <Me as RoleEq<S>>::Output>>::Output;
}

/// Project TChanRecv: Similar to TChanSend but for receive operations
impl<Me, R, S, C, L, Msg, P, AIO> Project<TChanRecv<R, S, C, L, Msg, P, AIO>, Me> for ()
where
    Me: Role,
    R: Role,
    S: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    Me: RoleEq<R>,
    <Me as RoleEq<R>>::Output: Bool,
    (): ProjectRecvCase<Me, S, R, C, L, Msg, P, AIO, <Me as RoleEq<R>>::Output>,
{
    type Output =
        <() as ProjectRecvCase<Me, S, R, C, L, Msg, P, AIO, <Me as RoleEq<R>>::Output>>::Output;
}

/// Project TChanEnd: Always project to EpChanEnd
impl<Me, C, L, AIO> Project<TChanEnd<C, L, AIO>, Me> for ()
where
    Me: Role + SupportsActionIO<AIO>,
    C: ChanId,
    L: MsgLbl,
    AIO: ActionIOTMarker,
{
    type Output = EpChanEnd<Me, CommMetadata<C, L>, AIO>;
}

/// Project TChanStart: Project to EpChanStart with projected continuation
impl<Me, C, L, S, AIO> Project<TChanStart<C, L, S, AIO>, Me> for ()
where
    Me: Role + SupportsActionIO<AIO>,
    C: ChanId,
    L: MsgLbl,
    S: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): Project<S, Me>,
    <() as Project<S, Me>>::Output: LocalProtocol,
{
    type Output = EpChanStart<Me, CommMetadata<C, L>, <() as Project<S, Me>>::Output, AIO>;
}

/// Project TChanChoice: Project to EpChanChoice with projected branches
impl<Me, R, C, L, Left, Right, AIO> Project<TChanChoice<R, C, L, Left, Right, AIO>, Me> for ()
where
    Me: Role + SupportsActionIO<AIO>,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): Project<Left, Me>,
    (): Project<Right, Me>,
    <() as Project<Left, Me>>::Output: LocalProtocol,
    <() as Project<Right, Me>>::Output: LocalProtocol,
{
    type Output = EpChanChoice<
        Me,
        CommMetadata<C, L>,
        <() as Project<Left, Me>>::Output,
        <() as Project<Right, Me>>::Output,
        AIO,
    >;
}

/// Project TChanPar: Project to EpChanPar with projected branches
impl<Me, C, L, Left, Right, IsDisjoint, AIO>
    Project<TChanPar<C, L, Left, Right, IsDisjoint, AIO>, Me> for ()
where
    Me: Role + SupportsActionIO<AIO>,
    C: ChanId,
    L: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    IsDisjoint: Send + Sync + 'static + Debug,
    AIO: ActionIOTMarker,
    (): Project<Left, Me>,
    (): Project<Right, Me>,
    <() as Project<Left, Me>>::Output: LocalProtocol,
    <() as Project<Right, Me>>::Output: LocalProtocol,
{
    type Output = EpChanPar<
        Me,
        CommMetadata<C, L>,
        <() as Project<Left, Me>>::Output,
        <() as Project<Right, Me>>::Output,
        IsDisjoint,
        AIO,
    >;
}
