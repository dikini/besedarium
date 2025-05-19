//! Parallel composition traits and implementations
//!
//! Contains all parallel composition traits and impls (e.g., `ProjectPar`, `ComposeProjectedParBranches`).

use crate::protocol::global::TSession;
use crate::protocol::local::IsEpEndVariant;
use crate::protocol::local::{EpSession, Role};
use crate::types;

// ProjectPar: Helper trait for projecting a protocol parallel composition.
//
// - `Me`: The role being projected.
// - `IO`: Protocol marker type.
// - `Lbl`: Label from the TPar construct.
// - `L`, `R`: The two protocol branches.
pub trait ProjectPar<Me: Role, IO, Lbl: types::ProtocolLabel, L: EpSession<IO, Me>, R: EpSession<IO, Me>> {
    type Out: EpSession<IO, Me>;
}

// ProjectRoleOrSkip: Helper trait to project role or create skip with the parent label
pub trait ProjectRoleOrSkip<Me: Role, IO, G: TSession<IO>, Flag, ParentLbl: types::ProtocolLabel> {
    type Out: EpSession<IO, Me>;
}

// ComposeProjectedParBranches: Main flag-based composition trait for projected parallel branches
pub trait ComposeProjectedParBranches<IO, Me: Role, L, R>
where
    R: IsEpEndVariant<IO, Me>,
    <R as IsEpEndVariant<IO, Me>>::Output: types::Bool,
{
    type Out: EpSession<IO, Me>;
}

// ComposeProjectedParBranchesCase: Helper trait for case selection in composition of parallel branches
pub trait ComposeProjectedParBranchesCase<LSkip, RSkip, LEnd, REnd, IO, Me: Role, L, R>
where
    R: EpSession<IO, Me>,
{
    type Out: EpSession<IO, Me>;
}

// TParContainsRoleImpl: Helper trait for TPar role containment logic
pub trait TParContainsRoleImpl<LContains, RContains> {
    type Output: types::Bool;
}