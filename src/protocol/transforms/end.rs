//! End projection trait and implementation
//!
//! Contains the `ProjectEndCase` trait and its implementation, which handles
//! the projection of global `TEnd` combinators to local `EpEnd` types.
//!
//! This module implements the role projection mechanism specifically for
//! `TEnd` combinators, mapping them to local `EpEnd` types for each role.

use crate::{
    protocol::{
        local::{EpEnd, EpSession, Role},
    },
    types::{ProtocolLabel, SessionType},
};

/// Helper trait for projecting `TEnd` combinators to local session types.
///
/// This trait handles the projection of a `TEnd` operation to an `EpEnd`
/// operation for a specific role, maintaining the protocol's structure and label.
///
/// # Type Parameters
///
/// * `Me`: The role being projected
/// * `IO`: Protocol marker type (e.g., Http, Mqtt)
/// * `Lbl`: Label for the end operation
///
/// # Associated Types
///
/// * `Out`: The resulting local endpoint protocol for role `Me`
pub trait ProjectEndCase<Me, IO, Lbl>
where
    Me: Role,
    IO: SessionType,
    Lbl: ProtocolLabel,
{
    /// The resulting local endpoint protocol for role `Me`
    type Out: EpSession<IO, Me>;
}

// Implementation for ProjectEndCase
impl<Me, IO, Lbl> ProjectEndCase<Me, IO, Lbl> for ()
where
    Me: Role,
    IO: SessionType,
    Lbl: ProtocolLabel,
{
    // When projecting TEnd to a role, we get EpEnd with that role
    type Out = EpEnd<IO, Lbl, Me>;
}
