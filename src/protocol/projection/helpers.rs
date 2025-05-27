//! Helper traits for role-based dispatch in protocol projection
//!
//! This module provides supporting traits for the projection system,
//! including role equality checking and case-specific projection handlers.

use super::Project;
use crate::protocol::foundation::{
    ActionIOTMarker, ChanId, CommMetadata, GlobalProtocol, LocalProtocol, Message, MsgLbl, Role,
    SupportsActionIO,
};
use crate::protocol::local::{EpChanRecv, EpChanSend};

/// Type-level boolean trait for case selection
pub trait Bool: Send + Sync + 'static {}

/// Type-level True
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct True;
impl Bool for True {}

/// Type-level False  
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct False;
impl Bool for False {}

/// Check if two roles are equal at the type level
pub trait RoleEq<Other: Role>: Role {
    type Output: Bool;
}

/// Reflexive case: a role equals itself
impl<R> RoleEq<R> for R
where
    R: Role,
{
    type Output = True;
}

/// Helper trait for projecting TSend operations based on role equality
pub trait ProjectSendCase<Me, S, R, C, L, Msg, P, AIO, IsEqual>
where
    Me: Role,
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    IsEqual: Bool,
{
    type Output: LocalProtocol;
}

/// Case when Me == Sender: Project as EpChanSend
impl<Me, S, R, C, L, Msg, P, AIO> ProjectSendCase<Me, S, R, C, L, Msg, P, AIO, True> for ()
where
    Me: Role + SupportsActionIO<AIO>,
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): Project<P, Me>,
    <() as Project<P, Me>>::Output: LocalProtocol,
{
    type Output = EpChanSend<Me, CommMetadata<C, L>, Msg, <() as Project<P, Me>>::Output, AIO>;
}

/// Case when Me != Sender: Just project the continuation
impl<Me, S, R, C, L, Msg, P, AIO> ProjectSendCase<Me, S, R, C, L, Msg, P, AIO, False> for ()
where
    Me: Role,
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): Project<P, Me>,
{
    type Output = <() as Project<P, Me>>::Output;
}

/// Helper trait for projecting TRecv operations based on role equality
pub trait ProjectRecvCase<Me, S, R, C, L, Msg, P, AIO, IsEqual>
where
    Me: Role,
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    IsEqual: Bool,
{
    type Output: LocalProtocol;
}

/// Case when Me == Receiver: Project as EpChanRecv
impl<Me, S, R, C, L, Msg, P, AIO> ProjectRecvCase<Me, S, R, C, L, Msg, P, AIO, True> for ()
where
    Me: Role + SupportsActionIO<AIO>,
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): Project<P, Me>,
    <() as Project<P, Me>>::Output: LocalProtocol,
{
    type Output = EpChanRecv<Me, CommMetadata<C, L>, Msg, <() as Project<P, Me>>::Output, AIO>;
}

/// Case when Me != Receiver: Just project the continuation
impl<Me, S, R, C, L, Msg, P, AIO> ProjectRecvCase<Me, S, R, C, L, Msg, P, AIO, False> for ()
where
    Me: Role,
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): Project<P, Me>,
{
    type Output = <() as Project<P, Me>>::Output;
}

// ============================================================================
// Non-Reflexive RoleEq Implementations (Required for Boolean Logic)
// ============================================================================

// Note: Non-reflexive RoleEq implementations need to be added for specific
// role pairs as needed. For now, the reflexive case (R == R -> True) is handled
// by the blanket implementation above.

// Example implementations would look like:
// impl RoleEq<Bob> for Alice { type Output = False; }
// impl RoleEq<Alice> for Bob { type Output = False; }
