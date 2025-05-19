//! Send projection traits and implementations
//!
//! Contains the `ProjectSendCase` trait and its implementations, which handles
//! the projection of global `TSend` combinators to local protocol types based on
//! whether the role being projected is the sender or not.

use crate::{
    protocol::{
        global::TSession,
        local::{EpSend, EpSession, Role},
    },
    types::{Bool, ProtocolLabel, SessionType},
};

use super::projection::ProjectRole;

/// Helper trait for projecting `TSend` combinators based on role equality.
///
/// This trait handles the two possible cases when projecting a `TSend` operation:
/// 1. If `Me == RSender` (Flag = True): Project as an `EpSend` for that role
/// 2. If `Me != RSender` (Flag = False): Skip this action or project the continuation
///
/// # Type Parameters
///
/// * `Me`: The role being projected
/// * `IO`: Protocol marker type
/// * `Lbl`: Label for the send operation
/// * `RSender`: Role performing the send
/// * `P`: Message type being sent
/// * `G`: Continuation protocol after this send
/// * `Flag`: Type-level boolean indicating if Me == RSender
///
/// # Associated Types
///
/// * `Output`: The resulting local endpoint protocol for role `Me`
pub trait ProjectSendCase<Me, IO, Lbl, RSender, P, G, Flag>
where
    Me: Role,
    IO: SessionType,
    Lbl: ProtocolLabel,
    RSender: Role,
    G: TSession<IO>,
    Flag: Bool,
{
    /// The resulting local endpoint protocol for role `Me`
    type Output: EpSession<IO, Me>;
}

// --- Implementation for when Me is the sender (Flag = True) ---

impl<Me, IO, Lbl, RSender, P, G>
    ProjectSendCase<Me, IO, Lbl, RSender, P, G, crate::types::True> for ()
where
    Me: Role,
    IO: SessionType,
    Lbl: ProtocolLabel,
    RSender: Role,
    P: Send + 'static,
    G: TSession<IO>,
    (): ProjectRole<Me, IO, G>,
    <() as ProjectRole<Me, IO, G>>::Out: EpSession<IO, Me>,
{
    // If Me is the sender, produce EpSend with Me as the role parameter
    type Output = EpSend<
        IO,
        Lbl,
        Me,
        P,
        <() as ProjectRole<Me, IO, G>>::Out,
    >;
}

// --- Implementation for when Me is not the sender (Flag = False) ---

impl<Me, IO, Lbl, RSender, P, G>
    ProjectSendCase<Me, IO, Lbl, RSender, P, G, crate::types::False> for ()
where
    Me: Role,
    IO: SessionType,
    Lbl: ProtocolLabel,
    RSender: Role,
    P: Send + 'static,
    G: TSession<IO>,
    (): ProjectRole<Me, IO, G>,
    <() as ProjectRole<Me, IO, G>>::Out: EpSession<IO, Me>,
{
    // If Me is not the sender, just project the continuation
    // This behavior matches the "receiver's view" of a TSend in a global protocol
    type Output = <() as ProjectRole<Me, IO, G>>::Out;
}