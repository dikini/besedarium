//! Projection logic for recursion combinators (TRec -> EpRec)
//!
//! This module implements the projection of global recursion points (TRec)
//! to local recursion points (EpRec) for a specific role.
//!
//! Invariants:
//! - Labels must be faithfully propagated from TRec to EpRec.
//! - Labels must be globally unique and non-empty.
//! - The structure of recursion must be preserved.
//!
//! Preconditions:
//! - The input global protocol is well-formed (labels unique, non-empty, valid TRec/TContinue structure).
//!
//! Postconditions:
//! - The projected local protocol uses EpRec with the same label as TRec.
//! - Recursion structure and label mapping are preserved.

use crate::protocol::global::TRec;
use crate::protocol::local::EpRec;
use crate::protocol::local::EpSession;
use crate::types::ProtocolLabel;

/// Trait for projecting a TRec recursion point to EpRec for a given role.
pub trait ProjectRec<Me, IO, Lbl, S>
where
    Lbl: ProtocolLabel,
{
    type Out: EpSession<IO, Me>;
}

impl<Me, IO, Lbl, S, EpS> ProjectRec<Me, IO, Lbl, S> for TRec<IO, Lbl, S>
where
    Lbl: ProtocolLabel,
    S: super::projection::ProjectRole<Me, IO, S, Out = EpS>,
    EpS: EpSession<IO, Me>,
{
    type Out = EpRec<IO, Lbl, Me, EpS>;
}
