//! Receive projection traits and implementations
//!
//! Contains the `ProjectRecvCase` trait and its implementations, which handles
//! the projection of global `TRecv` combinators to local protocol types based on
//! whether the role being projected is the receiver or not.

use crate::{
    protocol::{
        global::TSession,
        local::{EpRecv, EpSession, Role},
    },
    types::{ActionIOTMarker, Bool, ProtocolLabel, SessionType, SupportsActionIO}, // Added ActionIOTMarker and SupportsActionIO
};

use super::projection::ProjectRole;

/// Helper trait for projecting `TRecv` combinators based on role equality.
///
/// This trait handles the two possible cases when projecting a `TRecv` operation:
/// 1. If `Me == RReceiver` (Flag = True): Project as an `EpRecv` for that role
/// 2. If `Me != RReceiver` (Flag = False): Skip this action or project the continuation
///
/// # Type Parameters
///
/// * `Me`: The role being projected
/// * `IO`: Protocol marker type
/// * `Lbl`: Label for the receive operation
/// * `RReceiver`: Role performing the receive
/// * `RPeer`: Role sending the message (new)
/// * `P`: Message type being received
/// * `G`: Continuation protocol after this receive
/// * `Flag`: Type-level boolean indicating if Me == RReceiver
/// * `AIO`: Action I/O Type marker (e.g. HttpIo, MqttIo)
///
/// # Associated Types
///
/// * `Output`: The resulting local endpoint protocol for role `Me`
pub trait ProjectRecvCase<Me, IO, Lbl, RReceiver, RPeer, P, G, Flag, AIO> // Added RPeer
where
    Me: Role,
    IO: SessionType + SupportsActionIO<AIO>,
    Lbl: ProtocolLabel,
    RReceiver: Role,
    RPeer: Role, // Added RPeer bound
    G: TSession<IO>,
    Flag: Bool,
    AIO: ActionIOTMarker, // Added AIO bound
{
    /// The resulting local endpoint protocol for role `Me`
    type Output: EpSession<IO, Me>;
}

// --- Implementation for when Me is the receiver (Flag = True) ---

impl<Me, IO, Lbl, RReceiver, RPeer, P, G, AIO> ProjectRecvCase<Me, IO, Lbl, RReceiver, RPeer, P, G, crate::types::True, AIO> // Added RPeer
    for ()
where
    Me: Role,
    IO: SessionType + SupportsActionIO<AIO>,
    Lbl: ProtocolLabel,
    RReceiver: Role,
    RPeer: Role, // Added RPeer bound
    P: Send + 'static,
    G: TSession<IO>,
    (): ProjectRole<Me, IO, G>,
    <() as ProjectRole<Me, IO, G>>::Out: EpSession<IO, Me>,
    AIO: ActionIOTMarker,
{
    // If Me is the receiver, produce EpRecv with Me as the role parameter
    type Output = EpRecv<IO, Lbl, Me, RPeer, P, <() as ProjectRole<Me, IO, G>>::Out>; // Added RPeer
}

// --- Implementation for when Me is not the receiver (Flag = False) ---

impl<Me, IO, Lbl, RReceiver, RPeer, P, G, AIO>
    ProjectRecvCase<Me, IO, Lbl, RReceiver, RPeer, P, G, crate::types::False, AIO> for () // Added RPeer
where
    Me: Role,
    IO: SessionType + SupportsActionIO<AIO>,
    Lbl: ProtocolLabel,
    RReceiver: Role,
    RPeer: Role, // Added RPeer bound
    P: Send + 'static,
    G: TSession<IO>,
    (): ProjectRole<Me, IO, G>,
    <() as ProjectRole<Me, IO, G>>::Out: EpSession<IO, Me>,
    AIO: ActionIOTMarker, // Added AIO bound
{
    // If Me is not the receiver, just project the continuation
    // This behavior matches the "sender's view" of a TRecv in a global protocol
    type Output = <() as ProjectRole<Me, IO, G>>::Out;
}
