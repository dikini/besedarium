//! # Global Protocol Type Definitions
//!
//! This module contains the struct definitions for all global protocol types
//! used in the enhanced MPST system. These represent the choreography of
//! multi-party protocols with explicit channel management.

use crate::protocol::foundation::{ActionIOTMarker, ChanId, GlobalProtocol, Message, MsgLbl, Role};
use std::fmt::Debug;
use std::marker::PhantomData;

// ============================================================================
// Core Global Protocol Types
// ============================================================================

/// A global protocol type representing a message send operation between two roles.
///
/// `TChanSend<S, R, C, L, Msg, P, AIO>` represents a protocol choreography where
/// sender role `S` sends a message of type `Msg` to receiver role `R` over channel `C`,
/// then continues with protocol `P`. This is a fundamental building block for
/// defining communication patterns in session types.
///
/// # Type Parameters
///
/// - `S`: The role that sends the message (must implement [`Role`])
/// - `R`: The role that receives the message (must implement [`Role`])
/// - `C`: The channel identifier for this communication (must implement [`ChanId`])
/// - `L`: The message label for protocol organization (must implement [`MsgLbl`])
/// - `Msg`: The message type being sent (must implement [`Message`])
/// - `P`: The continuation protocol after the send (must implement [`GlobalProtocol`])
/// - `AIO`: The action I/O capability required (must implement [`ActionIOTMarker`])
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::{foundation::*, global::*};
/// use std::marker::PhantomData;
///
/// // Define roles
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Client;
/// impl Role for Client {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Server;
/// impl Role for Server {}
///
/// // Define a login message
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct LoginMsg {
///     username: String,
///     password: String,
/// }
/// impl Message for LoginMsg {}
///
/// // Simple login protocol: Client sends credentials, Server responds
/// type LoginProtocol = TChanSend<
///     Client, Server, DefaultChan, RequestLbl, LoginMsg,
///     TChanRecv<
///         Server, Client, DefaultChan, ResponseLbl, bool,
///         TChanEnd<DefaultChan, ResponseLbl, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     BiDirectionalAction
/// >;
/// ```
///
/// # Protocol Projection
///
/// When projected to different roles:
/// - **Sender role (`S`)**: Becomes an [`EpSend`](crate::protocol::local::EpSend) endpoint
/// - **Receiver role (`R`)**: Becomes an [`EpRecv`](crate::protocol::local::EpRecv) endpoint  
/// - **Other roles**: See the continuation protocol `P`
///
/// # Duality
///
/// `TChanSend` is dual to [`TChanRecv`] with swapped sender/receiver roles.
/// The duality relationship ensures that communication protocols are well-formed
/// and that sends have corresponding receives.
///
/// # Integration Test Examples
///
/// For complete working examples, see:
/// - `tests/client_server_integration.rs::test_login_protocol_compilation`
/// - `tests/client_server_integration.rs::test_multi_party_protocol`
///
/// Common message types and roles are defined in `tests/integration_common.rs`.
///
/// # See Also
///
/// - [`TChanRecv`] for the dual receive operation
/// - [`EpSend`](crate::protocol::local::EpSend) for local projection
/// - [`Project`](crate::protocol::projection::Project) for projection details
/// - [`IsDual`](crate::protocol::duality::IsDual) for duality verification
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanSend<
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
> {
    pub(super) _sender: PhantomData<S>,
    pub(super) _receiver: PhantomData<R>,
    pub(super) _chan: PhantomData<C>,
    pub(super) _lbl: PhantomData<L>,
    pub(super) _msg: PhantomData<Msg>,
    pub(super) _protocol: PhantomData<P>,
    pub(super) _aio: PhantomData<AIO>,
}

/// A global protocol type representing a message receive operation between two roles.
///
/// `TChanRecv<R, S, C, L, Msg, P, AIO>` represents a protocol choreography where
/// receiver role `R` receives a message of type `Msg` from sender role `S` over channel `C`,
/// then continues with protocol `P`. This is the dual counterpart to [`TChanSend`] and
/// represents the receiving side of a communication.
///
/// # Type Parameters
///
/// - `R`: The role that receives the message (must implement [`Role`])
/// - `S`: The role that sends the message (must implement [`Role`])
/// - `C`: The channel identifier for this communication (must implement [`ChanId`])
/// - `L`: The message label for protocol organization (must implement [`MsgLbl`])
/// - `Msg`: The message type being received (must implement [`Message`])
/// - `P`: The continuation protocol after the receive (must implement [`GlobalProtocol`])
/// - `AIO`: The action I/O capability required (must implement [`ActionIOTMarker`])
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::{foundation::*, global::*};
/// use std::marker::PhantomData;
///
/// // Define roles
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Client;
/// impl Role for Client {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Server;
/// impl Role for Server {}
///
/// // Define an acknowledgment message
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct AckMsg {
///     success: bool,
///     message: String,
/// }
/// impl Message for AckMsg {}
///
/// // Server response protocol: Server receives request, sends acknowledgment
/// type ResponseProtocol = TChanRecv<
///     Server, Client, DefaultChan, RequestLbl, String,
///     TChanSend<
///         Server, Client, DefaultChan, ResponseLbl, AckMsg,
///         TChanEnd<DefaultChan, ResponseLbl, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     BiDirectionalAction
/// >;
/// ```
///
/// # Protocol Projection
///
/// When projected to different roles:
/// - **Receiver role (`R`)**: Becomes an [`EpRecv`](crate::protocol::local::EpRecv) endpoint
/// - **Sender role (`S`)**: Becomes an [`EpSend`](crate::protocol::local::EpSend) endpoint
/// - **Other roles**: See the continuation protocol `P`
///
/// # Duality
///
/// `TChanRecv` is dual to [`TChanSend`] with swapped sender/receiver roles.
/// This ensures that every receive operation has a corresponding send operation
/// in a well-formed protocol.
///
/// # Integration Test Examples
///
/// For complete working examples, see:
/// - `tests/client_server_integration.rs::test_echo_protocol_compilation`
/// - `tests/client_server_integration.rs::test_request_response_pattern`
///
/// Common message types and roles are defined in `tests/integration_common.rs`.
///
/// # See Also
///
/// - [`TChanSend`] for the dual send operation
/// - [`EpRecv`](crate::protocol::local::EpRecv) for local projection
/// - [`Project`](crate::protocol::projection::Project) for projection details
/// - [`IsDual`](crate::protocol::duality::IsDual) for duality verification
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanRecv<
    R: Role,
    S: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
> {
    pub(super) _receiver: PhantomData<R>,
    pub(super) _sender: PhantomData<S>,
    pub(super) _chan: PhantomData<C>,
    pub(super) _lbl: PhantomData<L>,
    pub(super) _msg: PhantomData<Msg>,
    pub(super) _protocol: PhantomData<P>,
    pub(super) _aio: PhantomData<AIO>,
}

/// A global protocol type representing a choice point where one role selects between protocol branches.
///
/// `TChanChoice<R, C, Lbl, Left, Right, AIO>` represents a protocol choreography where
/// role `R` makes a choice between two protocol branches (`Left` and `Right`). This choice
/// is communicated to other participants, allowing for conditional protocol execution
/// based on runtime decisions.
///
/// # Type Parameters
///
/// - `R`: The role making the choice (must implement [`Role`])
/// - `C`: The channel identifier for choice communication (must implement [`ChanId`])
/// - `Lbl`: The message label for the choice point (must implement [`MsgLbl`])
/// - `Left`: The left branch protocol (must implement [`GlobalProtocol`])
/// - `Right`: The right branch protocol (must implement [`GlobalProtocol`])
/// - `AIO`: The action I/O capability for choice communication (must implement [`ActionIOTMarker`])
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::{foundation::*, global::*};
/// use std::marker::PhantomData;
///
/// // Define roles
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Client;
/// impl Role for Client {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Server;
/// impl Role for Server {}
///
/// // Define message types
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct LoginMsg;
/// impl Message for LoginMsg {}
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct RegisterMsg;
/// impl Message for RegisterMsg {}
///
/// // Client chooses between login and registration
/// type AuthChoice = TChanChoice<
///     Client,
///     DefaultChan,
///     RequestLbl,
///     // Left branch: Login
///     TChanSend<
///         Client, Server, DefaultChan, RequestLbl, LoginMsg,
///         TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     // Right branch: Register
///     TChanSend<
///         Client, Server, DefaultChan, RequestLbl, RegisterMsg,
///         TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     BiDirectionalAction
/// >;
/// ```
///
/// # Protocol Projection
///
/// When projected to different roles:
/// - **Chooser role (`R`)**: Becomes an [`EpChoice`](crate::protocol::local::EpChoice) endpoint
/// - **Other roles**: Become [`EpOffer`](crate::protocol::local::EpOffer) endpoints that wait for the choice
///
/// # Duality
///
/// `TChanChoice` is dual to [`TChanOffer`] with the same role parameters.
/// The choice-maker in one protocol becomes the offer-receiver in the dual protocol.
///
/// # Choice Selection
///
/// At runtime, the choosing role determines which branch to execute. The choice
/// is communicated to other participants through the channel system, ensuring
/// all parties follow the same protocol branch.
///
/// # Integration Test Examples
///
/// For complete working examples, see:
/// - `tests/client_server_integration.rs::test_choice_protocol_compilation`
/// - `tests/client_server_integration.rs::test_branching_scenarios`
///
/// Common choice patterns are defined in `tests/integration_common.rs`.
///
/// # See Also
///
/// - [`TChanOffer`] for the dual offer operation
/// - [`EpChoice`](crate::protocol::local::EpChoice) for local projection (chooser)
/// - [`EpOffer`](crate::protocol::local::EpOffer) for local projection (others)
/// - [`Project`](crate::protocol::projection::Project) for projection details
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanChoice<
    R: Role,
    C: ChanId,
    Lbl: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    AIO: ActionIOTMarker,
> {
    pub(super) _chooser: PhantomData<R>,
    pub(super) _chan: PhantomData<C>,
    pub(super) _lbl: PhantomData<Lbl>,
    pub(super) _left: PhantomData<Left>,
    pub(super) _right: PhantomData<Right>,
    pub(super) _aio: PhantomData<AIO>,
}

/// A global protocol type representing an offer point where one role provides branches to a chooser.
///
/// `TChanOffer<R, C, Lbl, Left, Right, AIO>` represents a protocol choreography where
/// role `R` offers two protocol branches (`Left` and `Right`) to a choosing role.
/// This is the dual counterpart to [`TChanChoice`] and represents the server-side
/// or offering-side of a choice interaction.
///
/// # Type Parameters
///
/// - `R`: The role offering the branches (must implement [`Role`])
/// - `C`: The channel identifier for offer communication (must implement [`ChanId`])
/// - `Lbl`: The message label for the offer point (must implement [`MsgLbl`])
/// - `Left`: The left branch protocol (must implement [`GlobalProtocol`])
/// - `Right`: The right branch protocol (must implement [`GlobalProtocol`])
/// - `AIO`: The action I/O capability for offer communication (must implement [`ActionIOTMarker`])
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::{foundation::*, global::*};
/// use std::marker::PhantomData;
///
/// // Define roles
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Client;
/// impl Role for Client {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Server;
/// impl Role for Server {}
///
/// // Define service message types
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct QueryService;
/// impl Message for QueryService {}
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct UpdateService;
/// impl Message for UpdateService {}
///
/// // Server offers query or update services to client
/// type ServiceOffer = TChanOffer<
///     Server,
///     DefaultChan,
///     RequestLbl,
///     // Left branch: Query service
///     TChanRecv<
///         Server, Client, DefaultChan, RequestLbl, QueryService,
///         TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     // Right branch: Update service  
///     TChanRecv<
///         Server, Client, DefaultChan, RequestLbl, UpdateService,
///         TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     BiDirectionalAction
/// >;
/// ```
///
/// # Protocol Projection
///
/// When projected to different roles:
/// - **Offerer role (`R`)**: Becomes an [`EpOffer`](crate::protocol::local::EpOffer) endpoint
/// - **Other roles**: Become [`EpChoice`](crate::protocol::local::EpChoice) endpoints that make the choice
///
/// # Duality
///
/// `TChanOffer` is dual to [`TChanChoice`] with the same role parameters.
/// The offerer in one protocol becomes the choice-receiver in the dual protocol.
///
/// # Offer Response
///
/// At runtime, the offering role waits to receive the choice from the choosing role,
/// then executes the selected branch. This enables request-response patterns and
/// service-oriented architectures.
///
/// # Integration Test Examples
///
/// For complete working examples, see:
/// - `tests/client_server_integration.rs::test_offer_protocol_compilation`
/// - `tests/client_server_integration.rs::test_service_selection_pattern`
///
/// Common offer patterns are defined in `tests/integration_common.rs`.
///
/// # See Also
///
/// - [`TChanChoice`] for the dual choice operation
/// - [`EpOffer`](crate::protocol::local::EpOffer) for local projection (offerer)
/// - [`EpChoice`](crate::protocol::local::EpChoice) for local projection (choosers)
/// - [`Project`](crate::protocol::projection::Project) for projection details
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanOffer<
    R: Role,
    C: ChanId,
    Lbl: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    AIO: ActionIOTMarker,
> {
    pub(super) _offerer: PhantomData<R>,
    pub(super) _chan: PhantomData<C>,
    pub(super) _lbl: PhantomData<Lbl>,
    pub(super) _left: PhantomData<Left>,
    pub(super) _right: PhantomData<Right>,
    pub(super) _aio: PhantomData<AIO>,
}

/// A global protocol type representing parallel composition of two protocol branches.
///
/// `TChanPar<C, Lbl, Left, Right, IsDisjoint, AIO>` represents a protocol choreography
/// where two sub-protocols (`Left` and `Right`) execute concurrently. The `IsDisjoint`
/// parameter ensures that the parallel branches don't interfere with each other,
/// providing safety guarantees for concurrent execution.
///
/// # Type Parameters
///
/// - `C`: The channel identifier for coordination (must implement [`ChanId`])
/// - `Lbl`: The message label for parallel execution context (must implement [`MsgLbl`])
/// - `Left`: The left parallel branch (must implement [`GlobalProtocol`])
/// - `Right`: The right parallel branch (must implement [`GlobalProtocol`])
/// - `IsDisjoint`: Marker ensuring branches are disjoint (must be `Send + Sync + Debug`)
/// - `AIO`: The action I/O capability for coordination (must implement [`ActionIOTMarker`])
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::{foundation::*, global::*};
/// use std::marker::PhantomData;
///
/// // Define roles
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Client;
/// impl Role for Client {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Server;
/// impl Role for Server {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Monitor;
/// impl Role for Monitor {}
///
/// // Define message types
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct DataMsg;
/// impl Message for DataMsg {}
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct LogMsg;
/// impl Message for LogMsg {}
///
/// // Disjoint marker type
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct DisjointMarker;
///
/// // Parallel execution: Client-Server communication and Server-Monitor logging
/// type ParallelProtocol = TChanPar<
///     DefaultChan,
///     RequestLbl,
///     // Left branch: Client-Server data exchange
///     TChanSend<
///         Client, Server, DefaultChan, RequestLbl, DataMsg,
///         TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     // Right branch: Server-Monitor logging (disjoint from left)
///     TChanSend<
///         Server, Monitor, DefaultChan, RequestLbl, LogMsg,
///         TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     DisjointMarker,
///     BiDirectionalAction
/// >;
/// ```
///
/// # Protocol Projection
///
/// When projected to any role, parallel branches are projected independently:
/// - Each role participates in the branches where it's involved
/// - Uninvolved roles see identity/skip projections for irrelevant branches
/// - The parallel structure is preserved in local endpoint types
///
/// # Disjoint Safety
///
/// The `IsDisjoint` parameter provides compile-time guarantees that:
/// - Parallel branches don't share resources unsafely
/// - No role participates in conflicting communications simultaneously
/// - Concurrent execution won't cause race conditions or deadlocks
///
/// # Concurrency Model
///
/// Parallel protocols enable:
/// - **Independent Execution**: Branches run without waiting for each other
/// - **Resource Isolation**: Each branch uses distinct communication channels
/// - **Compositional Safety**: Parallel composition preserves protocol correctness
///
/// # Integration Test Examples
///
/// For complete working examples, see:
/// - `tests/client_server_integration.rs::test_parallel_protocol_compilation`
/// - `tests/client_server_integration.rs::test_concurrent_communication`
///
/// Common parallel patterns are defined in `tests/integration_common.rs`.
///
/// # See Also
///
/// - [`EpPar`](crate::protocol::local::EpPar) for local projection
/// - [`Project`](crate::protocol::projection::Project) for projection details
/// - [`IsDual`](crate::protocol::duality::IsDual) for duality preservation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanPar<
    C: ChanId,
    Lbl: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    IsDisjoint: Send + Sync + 'static + Debug,
    AIO: ActionIOTMarker,
> {
    pub(super) _chan: PhantomData<C>,
    pub(super) _lbl: PhantomData<Lbl>,
    pub(super) _left: PhantomData<Left>,
    pub(super) _right: PhantomData<Right>,
    pub(super) _disjoint: PhantomData<IsDisjoint>,
    pub(super) _aio: PhantomData<AIO>,
}

/// A global protocol type representing successful protocol termination.
///
/// `TChanEnd<C, L, AIO>` represents the completion of a protocol choreography.
/// This is the terminal state where all communications have finished successfully
/// and resources can be cleaned up. It serves as the base case for protocol
/// recursion and the natural conclusion of finite protocols.
///
/// # Type Parameters
///
/// - `C`: The channel identifier for termination context (must implement [`ChanId`])
/// - `L`: The message label for termination context (must implement [`MsgLbl`])
/// - `AIO`: The action I/O capability for cleanup operations (must implement [`ActionIOTMarker`])
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::{foundation::*, global::*};
/// use std::marker::PhantomData;
///
/// // Define roles
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Client;
/// impl Role for Client {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Server;
/// impl Role for Server {}
///
/// // Define a simple message
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct HelloMsg;
/// impl Message for HelloMsg {}
///
/// // Simple protocol: Client sends hello, then terminates
/// type GreetingProtocol = TChanSend<
///     Client, Server, DefaultChan, RequestLbl, HelloMsg,
///     TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,
///     BiDirectionalAction
/// >;
/// ```
///
/// # Protocol Projection
///
/// When projected to any role, `TChanEnd` becomes an [`EpEnd`](crate::protocol::local::EpEnd)
/// endpoint, signaling that the role's participation in the protocol has completed.
///
/// # Termination Semantics
///
/// - **Clean Completion**: All protocol obligations have been fulfilled
/// - **Resource Cleanup**: Channels and resources are properly released
/// - **No Further Communication**: No additional messages can be sent or received
/// - **Duality Preservation**: End is dual to itself in well-formed protocols
///
/// # Integration Test Examples
///
/// For complete working examples, see:
/// - `tests/client_server_integration.rs::test_simple_termination`
/// - `tests/client_server_integration.rs::test_protocol_lifecycle`
///
/// Common termination patterns are defined in `tests/integration_common.rs`.
///
/// # See Also
///
/// - [`TChanStart`] for protocol initialization
/// - [`EpEnd`](crate::protocol::local::EpEnd) for local projection
/// - [`IsDual`](crate::protocol::duality::IsDual) for duality verification
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanEnd<C: ChanId, L: MsgLbl, AIO: ActionIOTMarker> {
    pub(super) _chan: PhantomData<C>,
    pub(super) _lbl: PhantomData<L>,
    pub(super) _aio: PhantomData<AIO>,
}

/// A global protocol type representing protocol initialization and startup.
///
/// `TChanStart<C, L, Start, AIO>` represents the beginning of a protocol choreography.
/// It establishes the initial state and transitions to the main protocol `Start`.
/// This is used for protocol lifecycle management, resource initialization,
/// and setting up the communication infrastructure.
///
/// # Type Parameters
///
/// - `C`: The channel identifier for initialization context (must implement [`ChanId`])
/// - `L`: The message label for initialization context (must implement [`MsgLbl`])
/// - `Start`: The continuation protocol after initialization (must implement [`GlobalProtocol`])
/// - `AIO`: The action I/O capability for initialization (must implement [`ActionIOTMarker`])
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::{foundation::*, global::*};
/// use std::marker::PhantomData;
///
/// // Define roles
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Client;
/// impl Role for Client {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Server;
/// impl Role for Server {}
///
/// // Define initialization and main messages
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct ConnectMsg;
/// impl Message for ConnectMsg {}
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct DataMsg;
/// impl Message for DataMsg {}
///
/// // Protocol with explicit initialization phase
/// type ConnectionProtocol = TChanStart<
///     DefaultChan,
///     RequestLbl,
///     // Main protocol after initialization
///     TChanSend<
///         Client, Server, DefaultChan, RequestLbl, ConnectMsg,
///         TChanSend<
///             Client, Server, DefaultChan, RequestLbl, DataMsg,
///             TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,
///             BiDirectionalAction
///         >,
///         BiDirectionalAction
///     >,
///     BiDirectionalAction
/// >;
/// ```
///
/// # Protocol Projection
///
/// When projected to any role, `TChanStart` becomes an [`EpStart`](crate::protocol::local::EpStart)
/// endpoint that handles initialization before proceeding to the main protocol.
///
/// # Initialization Semantics
///
/// - **Setup Phase**: Establishes necessary resources and connections
/// - **Transition**: Automatically proceeds to the main protocol after setup
/// - **Resource Management**: Ensures proper initialization of communication channels
/// - **Error Handling**: Can handle initialization failures before main protocol begins
///
/// # Use Cases
///
/// - **Connection Establishment**: Setting up network connections
/// - **Authentication**: Performing initial authentication handshakes
/// - **Resource Allocation**: Initializing shared resources
/// - **Protocol Negotiation**: Agreeing on protocol parameters
///
/// # Integration Test Examples
///
/// For complete working examples, see:
/// - `tests/client_server_integration.rs::test_initialization_protocol`
/// - `tests/client_server_integration.rs::test_connection_lifecycle`
///
/// Common initialization patterns are defined in `tests/integration_common.rs`.
///
/// # See Also
///
/// - [`TChanEnd`] for protocol termination
/// - [`EpStart`](crate::protocol::local::EpStart) for local projection
/// - [`Project`](crate::protocol::projection::Project) for projection details
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TChanStart<C: ChanId, L: MsgLbl, Start: GlobalProtocol, AIO: ActionIOTMarker> {
    pub(super) _chan: PhantomData<C>,
    pub(super) _lbl: PhantomData<L>,
    pub(super) _start: PhantomData<Start>,
    pub(super) _aio: PhantomData<AIO>,
}
