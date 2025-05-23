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
    types::{ActionIOTMarker, Bool, ProtocolLabel, SessionType, SupportsActionIO}, // Added ActionIOTMarker and SupportsActionIO
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
/// * `RPeer`: Role receiving the message (new)
/// * `P`: Message type being sent
/// * `G`: Continuation protocol after this send
/// * `Flag`: Type-level boolean indicating if Me == RSender
/// * `AIO`: Action I/O Type marker (e.g. HttpIo, MqttIo)
///
/// # Associated Types
///
/// * `Output`: The resulting local endpoint protocol for role `Me`
pub trait ProjectSendCase<Me, IO, Lbl, RSender, RPeer, P, G, Flag, AIO> // Added RPeer
where
    Me: Role,
    IO: SessionType + SupportsActionIO<AIO>,
    Lbl: ProtocolLabel,
    RSender: Role,
    RPeer: Role, // Added RPeer bound
    G: TSession<IO>,
    Flag: Bool,
    AIO: ActionIOTMarker, // Added AIO bound
{
    /// The resulting local endpoint protocol for role `Me`
    type Output: EpSession<IO, Me>;
}

// --- Implementation for when Me is the sender (Flag = True) ---

impl<Me, IO, Lbl, RSender, RPeer, P, G, AIO> ProjectSendCase<Me, IO, Lbl, RSender, RPeer, P, G, crate::types::True, AIO> // Added RPeer
    for ()
where
    Me: Role,
    IO: SessionType + SupportsActionIO<AIO>,
    Lbl: ProtocolLabel,
    RSender: Role,
    RPeer: Role, // Added RPeer bound
    P: Send + 'static,
    G: TSession<IO>,
    (): ProjectRole<Me, IO, G>,
    <() as ProjectRole<Me, IO, G>>::Out: EpSession<IO, Me>,
    AIO: ActionIOTMarker,
{
    // If Me is the sender, produce EpSend with Me as the role parameter
    type Output = EpSend<IO, Lbl, Me, RPeer, P, <() as ProjectRole<Me, IO, G>>::Out>; // Added RPeer
}

// --- Implementation for when Me is not the sender (Flag = False) ---

impl<Me, IO, Lbl, RSender, RPeer, P, G, AIO> ProjectSendCase<Me, IO, Lbl, RSender, RPeer, P, G, crate::types::False, AIO> // Added RPeer
    for ()
where
    Me: Role,
    IO: SessionType + SupportsActionIO<AIO>,
    Lbl: ProtocolLabel,
    RSender: Role,
    RPeer: Role, // Added RPeer bound
    P: Send + 'static,
    G: TSession<IO>,
    (): ProjectRole<Me, IO, G>,
    <() as ProjectRole<Me, IO, G>>::Out: EpSession<IO, Me>,
    AIO: ActionIOTMarker, // Added AIO bound
{
    // If Me is not the sender, just project the continuation
    // This behavior matches the "receiver's view" of a TSend in a global protocol
    type Output = <() as ProjectRole<Me, IO, G>>::Out;
}
