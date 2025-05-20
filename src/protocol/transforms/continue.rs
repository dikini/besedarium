//! Projection logic for continue combinators (TContinue -> EpContinue)
//!
//! This module implements the projection of global continue points (TContinue)
//! to local continue points (EpContinue) for a specific role.
//!
//! Invariants:
//! - Labels must be faithfully propagated from TContinue to EpContinue.
//! - Labels must be globally unique and non-empty.
//! - The structure of recursion must be preserved.
//!
//! Preconditions:
//! - The input global protocol is well-formed (labels unique, non-empty, valid TRec/TContinue structure).
//!
//! Postconditions:
//! - The projected local protocol uses EpContinue with the same label as TContinue.
//! - Recursion structure and label mapping are preserved.

use crate::protocol::global::TContinue;
use crate::protocol::local::EpContinue;
use crate::protocol::local::EpSession;
use crate::types::ProtocolLabel;

/// Trait for projecting a TContinue recursion continue to EpContinue for a given role.
pub trait ProjectContinue<Me, IO, Lbl>
where
    Lbl: ProtocolLabel,
{
    type Out: EpSession<IO, Me>;
}

impl<Me, IO, Lbl> ProjectContinue<Me, IO, Lbl> for TContinue<IO, Lbl>
where
    Lbl: ProtocolLabel,
{
    type Out = EpContinue<IO, Lbl, Me>;
}
