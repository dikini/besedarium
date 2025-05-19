//! Start projection trait and implementation
//!
//! Contains the `ProjectStartCase` trait and its implementation, which handles
//! the projection of global `TStart` combinators to local `EpStart` types.
//!
//! This module implements the role projection mechanism specifically for
//! `TStart` combinators, mapping them to local `EpStart` types for each role.

use crate::{
    protocol::{
        global::TSession,
        local::{EpSession, EpStart, Role},
    },
    types::{ProtocolLabel, SessionType},
};

use super::projection::ProjectRole;

/// Helper trait for projecting `TStart` combinators to local session types.
///
/// This trait handles the projection of a `TStart` operation to an `EpStart`
/// operation for a specific role, maintaining the protocol's structure and label.
///
/// # Type Parameters
///
/// * `Me`: The role being projected
/// * `IO`: Protocol marker type (e.g., Http, Mqtt)
/// * `Lbl`: Label for the start operation
/// * `S`: Continuation protocol after this start point
///
/// # Associated Types
///
/// * `Out`: The resulting local endpoint protocol for role `Me`
pub trait ProjectStartCase<Me, IO, Lbl, S>
where
    Me: Role,
    IO: SessionType,
    Lbl: ProtocolLabel,
    S: TSession<IO>,
{
    /// The resulting local endpoint protocol for role `Me`
    type Out: EpSession<IO, Me>;
}

// Implementation for ProjectStartCase
impl<Me, IO, Lbl, S> ProjectStartCase<Me, IO, Lbl, S> for ()
where
    Me: Role,
    IO: SessionType,
    Lbl: ProtocolLabel,
    S: TSession<IO> + ProjectRole<Me, IO, S>,
    <S as ProjectRole<Me, IO, S>>::Out: EpSession<IO, Me>,
{
    type Out = EpStart<IO, Lbl, Me, <S as ProjectRole<Me, IO, S>>::Out>;
}
