//! # Local Endpoint Type Definitions
//!
//! This module contains the struct definitions for all local endpoint types
//! used in the Enhanced MPST System. These types represent the projected view
//! of protocols from individual role perspectives.

use crate::protocol::foundation::{
    ActionIOTMarker, CommMetadataTrait, LocalProtocol, Message, SupportsActionIO,
};
use std::fmt::Debug;
use std::marker::PhantomData;

/// A local endpoint type representing a message send operation from this role's perspective.
///
/// `EpChanSend<IO, M, Msg, P, AIO>` represents a local endpoint where this role
/// sends a message of type `Msg` to another role, then continues with protocol `P`.
/// This is the local projection of a global [`TChanSend`](crate::protocol::global::TChanSend)
/// when projected to the sender role.
///
/// # Type Parameters
///
/// - `IO`: IO capability type that must support the required action (must implement [`SupportsActionIO<AIO>`])
/// - `M`: Extensible metadata type providing channel and label information (must implement [`CommMetadataTrait`])
/// - `Msg`: The message type being sent (must implement [`Message`])
/// - `P`: The continuation local protocol after the send (must implement [`LocalProtocol`])
/// - `AIO`: The action I/O capability required (must implement [`ActionIOTMarker`])
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::{foundation::*, local::*};
/// use std::marker::PhantomData;
///
/// // Define roles
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct ClientIO;
/// impl SupportsActionIO<BiDirectionalAction> for ClientIO {}
///
/// // Define a request message
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct LoginRequest {
///     username: String,
///     password: String,
/// }
/// impl Message for LoginRequest {}
///
/// // Client endpoint that sends login request
/// type ClientLoginEndpoint = EpChanSend<
///     ClientIO,
///     CommMetadata<DefaultChan, RequestLbl>,
///     LoginRequest,
///     EpChanRecv<
///         ClientIO,
///         CommMetadata<DefaultChan, ResponseLbl>,
///         bool,
///         EpChanEnd<ClientIO, CommMetadata<DefaultChan, ResponseLbl>, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     BiDirectionalAction
/// >;
/// ```
///
/// # Local Endpoint Semantics
///
/// - **Send Operation**: This role performs an active send operation
/// - **Blocking Behavior**: May block until the message is successfully sent
/// - **Type Safety**: Compile-time verification that IO capabilities match requirements
/// - **Continuation**: Automatically transitions to the next protocol step after send
///
/// # Global Protocol Projection
///
/// This endpoint type results from projecting a global [`TChanSend`](crate::protocol::global::TChanSend)
/// to the sender role. Other roles in the same global protocol will see different projections:
/// - **Receiver role**: Gets [`EpChanRecv`] for the same message
/// - **Uninvolved roles**: See the continuation protocol or identity
///
/// # Integration Test Examples
///
/// For complete working examples, see:
/// - `tests/client_server_integration.rs::test_send_endpoint_projection`
/// - `tests/client_server_integration.rs::test_local_protocol_execution`
///
/// Common usage patterns are defined in `tests/integration_common.rs`.
///
/// # See Also
///
/// - [`EpChanRecv`] for the receiving counterpart
/// - [`TChanSend`](crate::protocol::global::TChanSend) for the global protocol type
/// - [`Project`](crate::protocol::projection::Project) for projection mechanics
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpChanSend<IO, M, Msg, P, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    AIO: ActionIOTMarker,
{
    pub(super) _io: PhantomData<IO>,
    pub(super) _metadata: PhantomData<M>,
    pub(super) _msg: PhantomData<Msg>,
    pub(super) _protocol: PhantomData<P>,
    pub(super) _aio: PhantomData<AIO>,
}

/// A local endpoint type representing a message receive operation from this role's perspective.
///
/// `EpChanRecv<IO, M, Msg, P, AIO>` represents a local endpoint where this role
/// receives a message of type `Msg` from another role, then continues with protocol `P`.
/// This is the local projection of a global [`TChanRecv`](crate::protocol::global::TChanRecv)
/// when projected to the receiver role.
///
/// # Type Parameters
///
/// - `IO`: IO capability type that must support the required action (must implement [`SupportsActionIO<AIO>`])
/// - `M`: Extensible metadata type providing channel and label information (must implement [`CommMetadataTrait`])
/// - `Msg`: The message type being received (must implement [`Message`])
/// - `P`: The continuation local protocol after the receive (must implement [`LocalProtocol`])
/// - `AIO`: The action I/O capability required (must implement [`ActionIOTMarker`])
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::{foundation::*, local::*};
/// use std::marker::PhantomData;
///
/// // Define roles
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct ServerIO;
/// impl SupportsActionIO<BiDirectionalAction> for ServerIO {}
///
/// // Define a response message
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct AuthResponse {
///     success: bool,
///     token: Option<String>,
/// }
/// impl Message for AuthResponse {}
///
/// // Server endpoint that receives auth request
/// type ServerAuthEndpoint = EpChanRecv<
///     ServerIO,
///     CommMetadata<DefaultChan, RequestLbl>,
///     String,
///     EpChanSend<
///         ServerIO,
///         CommMetadata<DefaultChan, ResponseLbl>,
///         AuthResponse,
///         EpChanEnd<ServerIO, CommMetadata<DefaultChan, ResponseLbl>, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     BiDirectionalAction
/// >;
/// ```
///
/// # Local Endpoint Semantics
///
/// - **Receive Operation**: This role performs a passive receive operation
/// - **Blocking Behavior**: Blocks until a message is successfully received
/// - **Type Safety**: Compile-time verification that received message type matches expectations
/// - **Continuation**: Automatically transitions to the next protocol step after receive
///
/// # Global Protocol Projection
///
/// This endpoint type results from projecting a global [`TChanRecv`](crate::protocol::global::TChanRecv)
/// to the receiver role. Other roles in the same global protocol will see different projections:
/// - **Sender role**: Gets [`EpChanSend`] for the same message
/// - **Uninvolved roles**: See the continuation protocol or identity
///
/// # Integration Test Examples
///
/// For complete working examples, see:
/// - `tests/client_server_integration.rs::test_recv_endpoint_projection`
/// - `tests/client_server_integration.rs::test_server_protocol_execution`
///
/// Common usage patterns are defined in `tests/integration_common.rs`.
///
/// # See Also
///
/// - [`EpChanSend`] for the sending counterpart
/// - [`TChanRecv`](crate::protocol::global::TChanRecv) for the global protocol type
/// - [`Project`](crate::protocol::projection::Project) for projection mechanics
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpChanRecv<IO, M, Msg, P, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    AIO: ActionIOTMarker,
{
    pub(super) _io: PhantomData<IO>,
    pub(super) _metadata: PhantomData<M>,
    pub(super) _msg: PhantomData<Msg>,
    pub(super) _protocol: PhantomData<P>,
    pub(super) _aio: PhantomData<AIO>,
}

/// A local endpoint type representing an offer point from this role's perspective.
///
/// `EpChanOffer<IO, M, Left, Right, AIO>` represents a local endpoint where this role
/// offers two protocol branches (`Left` and `Right`) to a choosing role and waits
/// for the choice. This is the local projection of a global [`TChanOffer`](crate::protocol::global::TChanOffer)
/// when projected to the offering role.
///
/// # Type Parameters
///
/// - `IO`: IO capability type that must support the required action (must implement [`SupportsActionIO<AIO>`])
/// - `M`: Extensible metadata type providing channel and label information (must implement [`CommMetadataTrait`])
/// - `Left`: The left branch local protocol (must implement [`LocalProtocol`])
/// - `Right`: The right branch local protocol (must implement [`LocalProtocol`])
/// - `AIO`: The action I/O capability for choice communication (must implement [`ActionIOTMarker`])
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::{foundation::*, local::*};
/// use std::marker::PhantomData;
///
/// // Define roles
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct ServerIO;
/// impl SupportsActionIO<BiDirectionalAction> for ServerIO {}
///
/// // Define service message types
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct QueryData;
/// impl Message for QueryData {}
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct UpdateData;
/// impl Message for UpdateData {}
///
/// // Server offers query or update services
/// type ServerServiceOffer = EpChanOffer<
///     ServerIO,
///     CommMetadata<DefaultChan, RequestLbl>,
///     // Left branch: Query service
///     EpChanRecv<
///         ServerIO,
///         CommMetadata<DefaultChan, RequestLbl>,
///         QueryData,
///         EpChanEnd<ServerIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     // Right branch: Update service
///     EpChanRecv<
///         ServerIO,
///         CommMetadata<DefaultChan, RequestLbl>,
///         UpdateData,
///         EpChanEnd<ServerIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     BiDirectionalAction
/// >;
/// ```
///
/// # Local Endpoint Semantics
///
/// - **Passive Choice**: This role waits to receive the choice from the chooser
/// - **Branch Execution**: Executes the selected branch based on the choice received
/// - **Type Safety**: Ensures both branches are valid continuations
/// - **Deterministic**: The choice determines exactly which branch executes
///
/// # Global Protocol Projection
///
/// This endpoint type results from projecting a global [`TChanOffer`](crate::protocol::global::TChanOffer)
/// to the offering role. Other roles in the same global protocol will see different projections:
/// - **Choosing role**: Gets [`EpChanChoice`] with the same branches
/// - **Uninvolved roles**: See projections of the individual branches
///
/// # Runtime Behavior
///
/// At runtime, the offering endpoint:
/// 1. Waits for a choice message from the chooser
/// 2. Determines which branch was selected
/// 3. Executes the corresponding branch protocol
/// 4. Continues with the selected branch's continuation
///
/// # Integration Test Examples
///
/// For complete working examples, see:
/// - `tests/client_server_integration.rs::test_offer_endpoint_projection`
/// - `tests/client_server_integration.rs::test_service_selection_server`
///
/// Common offer patterns are defined in `tests/integration_common.rs`.
///
/// # See Also
///
/// - [`EpChanChoice`] for the choosing counterpart
/// - [`TChanOffer`](crate::protocol::global::TChanOffer) for the global protocol type
/// - [`Project`](crate::protocol::projection::Project) for projection mechanics
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpChanOffer<IO, M, Left, Right, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    AIO: ActionIOTMarker,
{
    pub(super) _io: PhantomData<IO>,
    pub(super) _metadata: PhantomData<M>,
    pub(super) _left: PhantomData<Left>,
    pub(super) _right: PhantomData<Right>,
    pub(super) _aio: PhantomData<AIO>,
}

/// A local endpoint type representing a choice point from this role's perspective.
///
/// `EpChanChoice<IO, M, Left, Right, AIO>` represents a local endpoint where this role
/// makes a choice between two protocol branches (`Left` and `Right`) and communicates
/// the choice to other participants. This is the local projection of a global
/// [`TChanChoice`](crate::protocol::global::TChanChoice) when projected to the choosing role.
///
/// # Type Parameters
///
/// - `IO`: IO capability type that must support the required action (must implement [`SupportsActionIO<AIO>`])
/// - `M`: Extensible metadata type providing channel and label information (must implement [`CommMetadataTrait`])
/// - `Left`: The left branch local protocol (must implement [`LocalProtocol`])
/// - `Right`: The right branch local protocol (must implement [`LocalProtocol`])
/// - `AIO`: The action I/O capability for choice communication (must implement [`ActionIOTMarker`])
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::{foundation::*, local::*};
/// use std::marker::PhantomData;
///
/// // Define roles
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct ClientIO;
/// impl SupportsActionIO<BiDirectionalAction> for ClientIO {}
///
/// // Define authentication message types
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct LoginMsg;
/// impl Message for LoginMsg {}
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct RegisterMsg;
/// impl Message for RegisterMsg {}
///
/// // Client chooses between login and registration
/// type ClientAuthChoice = EpChanChoice<
///     ClientIO,
///     CommMetadata<DefaultChan, RequestLbl>,
///     // Left branch: Login flow
///     EpChanSend<
///         ClientIO,
///         CommMetadata<DefaultChan, RequestLbl>,
///         LoginMsg,
///         EpChanEnd<ClientIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     // Right branch: Registration flow
///     EpChanSend<
///         ClientIO,
///         CommMetadata<DefaultChan, RequestLbl>,
///         RegisterMsg,
///         EpChanEnd<ClientIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     BiDirectionalAction
/// >;
/// ```
///
/// # Local Endpoint Semantics
///
/// - **Active Choice**: This role actively decides which branch to execute
/// - **Choice Communication**: Communicates the choice to other participants
/// - **Branch Execution**: Executes the selected branch protocol
/// - **Deterministic Flow**: The choice determines the exact protocol continuation
///
/// # Global Protocol Projection
///
/// This endpoint type results from projecting a global [`TChanChoice`](crate::protocol::global::TChanChoice)
/// to the choosing role. Other roles in the same global protocol will see different projections:
/// - **Offering roles**: Get [`EpChanOffer`] with the same branches
/// - **Uninvolved roles**: See projections of the individual branches
///
/// # Runtime Behavior
///
/// At runtime, the choice endpoint:
/// 1. Makes a decision between the available branches
/// 2. Sends a choice message to inform other participants
/// 3. Executes the selected branch protocol
/// 4. Continues with the selected branch's continuation
///
/// # Choice Selection
///
/// The choice can be made based on:
/// - **Runtime conditions**: Business logic, user input, system state
/// - **Configuration**: Predefined preferences or settings
/// - **External events**: Network conditions, resource availability
/// - **Interactive input**: User selections, API requests
///
/// # Integration Test Examples
///
/// For complete working examples, see:
/// - `tests/client_server_integration.rs::test_choice_endpoint_projection`
/// - `tests/client_server_integration.rs::test_client_authentication_choice`
///
/// Common choice patterns are defined in `tests/integration_common.rs`.
///
/// # See Also
///
/// - [`EpChanOffer`] for the offering counterpart
/// - [`TChanChoice`](crate::protocol::global::TChanChoice) for the global protocol type
/// - [`Project`](crate::protocol::projection::Project) for projection mechanics
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpChanChoice<IO, M, Left, Right, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    AIO: ActionIOTMarker,
{
    pub(super) _io: PhantomData<IO>,
    pub(super) _metadata: PhantomData<M>,
    pub(super) _left: PhantomData<Left>,
    pub(super) _right: PhantomData<Right>,
    pub(super) _aio: PhantomData<AIO>,
}

/// A local endpoint type representing parallel composition from this role's perspective.
///
/// `EpChanPar<IO, M, Left, Right, IsDisjoint, AIO>` represents a local endpoint where this role
/// participates in two parallel protocol branches (`Left` and `Right`) that execute concurrently.
/// This is the local projection of a global [`TChanPar`](crate::protocol::global::TChanPar)
/// when projected to a role that participates in the parallel composition.
///
/// # Type Parameters
///
/// - `IO`: IO capability type that must support the required action (must implement [`SupportsActionIO<AIO>`])
/// - `M`: Extensible metadata type providing channel and label information (must implement [`CommMetadataTrait`])
/// - `Left`: The left parallel branch local protocol (must implement [`LocalProtocol`])
/// - `Right`: The right parallel branch local protocol (must implement [`LocalProtocol`])
/// - `IsDisjoint`: Disjointness marker ensuring parallel branches don't interfere (must implement [`Send + Sync + 'static + Debug`])
/// - `AIO`: The action I/O capability for parallel coordination (must implement [`ActionIOTMarker`])
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::{foundation::*, local::*};
/// use std::marker::PhantomData;
///
/// // Define roles
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct CoordinatorIO;
/// impl SupportsActionIO<BiDirectionalAction> for CoordinatorIO {}
///
/// // Define message types for different services
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct LogMessage;
/// impl Message for LogMessage {}
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct MetricsMessage;
/// impl Message for MetricsMessage {}
///
/// // Disjointness marker for independent services
/// #[derive(Debug)]
/// struct ServicesAreDisjoint;
///
/// // Coordinator endpoint handling logging and metrics in parallel
/// type CoordinatorParallelEndpoint = EpChanPar<
///     CoordinatorIO,
///     CommMetadata<DefaultChan, RequestLbl>,
///     // Left branch: Logging service interaction
///     EpChanSend<
///         CoordinatorIO,
///         CommMetadata<DefaultChan, RequestLbl>,
///         LogMessage,
///         EpChanEnd<CoordinatorIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     // Right branch: Metrics service interaction
///     EpChanSend<
///         CoordinatorIO,
///         CommMetadata<DefaultChan, RequestLbl>,
///         MetricsMessage,
///         EpChanEnd<CoordinatorIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     ServicesAreDisjoint,
///     BiDirectionalAction
/// >;
/// ```
///
/// # Local Endpoint Semantics
///
/// - **Concurrent Execution**: Both parallel branches execute simultaneously
/// - **Resource Coordination**: Manages shared resources across parallel branches
/// - **Disjoint Guarantee**: Ensures parallel branches don't interfere with each other
/// - **Synchronization Points**: Coordinates completion of both branches
///
/// # Global Protocol Projection
///
/// This endpoint type results from projecting a global [`TChanPar`](crate::protocol::global::TChanPar)
/// to a role that participates in the parallel composition. Different scenarios:
/// - **Participating role**: Gets `EpChanPar` with both branch projections
/// - **Left-only role**: Gets projection of only the left branch
/// - **Right-only role**: Gets projection of only the right branch
/// - **Non-participating role**: May see empty protocol or specific synchronization points
///
/// # Runtime Behavior
///
/// At runtime, the parallel endpoint:
/// 1. Initiates both parallel branches concurrently
/// 2. Manages resource allocation and coordination
/// 3. Handles communication for both branches independently
/// 4. Synchronizes completion of both branches before continuing
///
/// # Disjointness Requirements
///
/// The `IsDisjoint` parameter ensures:
/// - **Channel separation**: Parallel branches use different communication channels
/// - **Resource independence**: No shared mutable state between branches
/// - **Deadlock prevention**: Eliminates circular dependencies between branches
/// - **Type safety**: Compile-time verification of parallelization constraints
///
/// # Integration Test Examples
///
/// For complete working examples, see:
/// - `tests/client_server_integration.rs::test_parallel_endpoint_projection`
/// - `tests/client_server_integration.rs::test_concurrent_service_coordination`
///
/// Common parallel patterns are defined in `tests/integration_common.rs`.
///
/// # See Also
///
/// - [`TChanPar`](crate::protocol::global::TChanPar) for the global protocol type
/// - [`Project`](crate::protocol::projection::Project) for projection mechanics
/// - [`Disjoint`](crate::protocol::foundation::Disjoint) for disjointness analysis
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpChanPar<IO, M, Left, Right, IsDisjoint, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    IsDisjoint: Send + Sync + 'static + Debug,
    AIO: ActionIOTMarker,
{
    pub(super) _io: PhantomData<IO>,
    pub(super) _metadata: PhantomData<M>,
    pub(super) _left: PhantomData<Left>,
    pub(super) _right: PhantomData<Right>,
    pub(super) _disjoint: PhantomData<IsDisjoint>,
    pub(super) _aio: PhantomData<AIO>,
}

/// A local endpoint type representing protocol termination from this role's perspective.
///
/// `EpChanEnd<IO, M, AIO>` represents a local endpoint where this role concludes its
/// participation in the protocol. This marks the successful completion of all required
/// communication and the cleanup of associated resources. This is the local projection
/// of a global [`TChanEnd`](crate::protocol::global::TChanEnd) for any participating role.
///
/// # Type Parameters
///
/// - `IO`: IO capability type that must support the required action (must implement [`SupportsActionIO<AIO>`])
/// - `M`: Extensible metadata type providing channel and label information (must implement [`CommMetadataTrait`])
/// - `AIO`: The action I/O capability for cleanup operations (must implement [`ActionIOTMarker`])
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::{foundation::*, local::*};
/// use std::marker::PhantomData;
///
/// // Define roles
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct ClientIO;
/// impl SupportsActionIO<BiDirectionalAction> for ClientIO {}
///
/// // Define a farewell message
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct GoodbyeMessage;
/// impl Message for GoodbyeMessage {}
///
/// // Client endpoint that sends goodbye and terminates
/// type ClientFarewellEndpoint = EpChanSend<
///     ClientIO,
///     CommMetadata<DefaultChan, RequestLbl>,
///     GoodbyeMessage,
///     EpChanEnd<ClientIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>,
///     BiDirectionalAction
/// >;
///
/// // Simple termination endpoint
/// type ClientTerminationEndpoint = EpChanEnd<
///     ClientIO,
///     CommMetadata<DefaultChan, RequestLbl>,
///     BiDirectionalAction
/// >;
/// ```
///
/// # Local Endpoint Semantics
///
/// - **Protocol Completion**: Marks successful conclusion of all protocol obligations
/// - **Resource Cleanup**: Handles deallocation of communication resources
/// - **State Finalization**: Ensures proper final state for the endpoint
/// - **Graceful Shutdown**: Provides clean termination without resource leaks
///
/// # Global Protocol Projection
///
/// This endpoint type results from projecting a global [`TChanEnd`](crate::protocol::global::TChanEnd)
/// to any participating role. The projection is identical for all roles:
/// - **All participants**: Get `EpChanEnd` with the same metadata context
/// - **Consistent termination**: All roles terminate simultaneously
/// - **Resource coordination**: Synchronized cleanup across all endpoints
///
/// # Runtime Behavior
///
/// At runtime, the termination endpoint:
/// 1. Completes any pending operations
/// 2. Releases allocated communication resources
/// 3. Performs final state cleanup
/// 4. Signals protocol completion to the runtime system
///
/// # Resource Management
///
/// The termination endpoint ensures:
/// - **Channel cleanup**: Closes communication channels properly
/// - **Memory deallocation**: Releases protocol-specific memory
/// - **Connection teardown**: Properly terminates network connections
/// - **State consistency**: Maintains invariants during cleanup
///
/// # Integration Test Examples
///
/// For complete working examples, see:
/// - `tests/client_server_integration.rs::test_termination_endpoint_projection`
/// - `tests/client_server_integration.rs::test_graceful_protocol_completion`
///
/// Common termination patterns are defined in `tests/integration_common.rs`.
///
/// # See Also
///
/// - [`TChanEnd`](crate::protocol::global::TChanEnd) for the global protocol type
/// - [`Project`](crate::protocol::projection::Project) for projection mechanics
/// - [`LocalProtocol`] for the trait implemented by this type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpChanEnd<IO, M, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    AIO: ActionIOTMarker,
{
    pub(super) _io: PhantomData<IO>,
    pub(super) _metadata: PhantomData<M>,
    pub(super) _aio: PhantomData<AIO>,
}

/// A local endpoint type representing protocol initialization from this role's perspective.
///
/// `EpChanStart<IO, M, Start, AIO>` represents a local endpoint where this role initiates
/// its participation in a protocol session. This handles the setup and initialization
/// required before engaging in the main protocol communication. This is the local projection
/// of a global [`TChanStart`](crate::protocol::global::TChanStart) for any participating role.
///
/// # Type Parameters
///
/// - `IO`: IO capability type that must support the required action (must implement [`SupportsActionIO<AIO>`])
/// - `M`: Extensible metadata type providing channel and label information (must implement [`CommMetadataTrait`])
/// - `Start`: The continuation local protocol after initialization (must implement [`LocalProtocol`])
/// - `AIO`: The action I/O capability for initialization operations (must implement [`ActionIOTMarker`])
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::{foundation::*, local::*};
/// use std::marker::PhantomData;
///
/// // Define roles
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct ClientIO;
/// impl SupportsActionIO<BiDirectionalAction> for ClientIO {}
///
/// // Define connection establishment message
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct ConnectMessage;
/// impl Message for ConnectMessage {}
///
/// // Client endpoint that starts with connection establishment
/// type ClientStartEndpoint = EpChanStart<
///     ClientIO,
///     CommMetadata<DefaultChan, RequestLbl>,
///     // After start, send connection message
///     EpChanSend<
///         ClientIO,
///         CommMetadata<DefaultChan, RequestLbl>,
///         ConnectMessage,
///         EpChanEnd<ClientIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>,
///         BiDirectionalAction
///     >,
///     BiDirectionalAction
/// >;
///
/// // Simple initialization endpoint
/// type ServiceInitEndpoint = EpChanStart<
///     ClientIO,
///     CommMetadata<DefaultChan, RequestLbl>,
///     EpChanEnd<ClientIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>,
///     BiDirectionalAction
/// >;
/// ```
///
/// # Local Endpoint Semantics
///
/// - **Protocol Initiation**: Begins a new protocol session for this role
/// - **Resource Allocation**: Sets up communication channels and resources
/// - **State Initialization**: Establishes initial protocol state
/// - **Connection Setup**: Handles network or communication setup
///
/// # Global Protocol Projection
///
/// This endpoint type results from projecting a global [`TChanStart`](crate::protocol::global::TChanStart)
/// to any participating role. Different roles may have different continuation protocols:
/// - **Initiating role**: May have different initial behavior than responders
/// - **Responding roles**: May wait for initiation signals before proceeding
/// - **Service roles**: May perform setup specific to their service type
/// - **Client roles**: May perform connection-specific initialization
///
/// # Runtime Behavior
///
/// At runtime, the start endpoint:
/// 1. Allocates necessary communication resources
/// 2. Establishes connections with other protocol participants
/// 3. Initializes protocol-specific state
/// 4. Transitions to the continuation protocol (`Start`)
///
/// # Initialization Responsibilities
///
/// The start endpoint handles:
/// - **Channel allocation**: Setting up communication channels
/// - **Resource management**: Allocating memory and system resources
/// - **Connection establishment**: Creating network connections
/// - **State setup**: Initializing protocol-specific data structures
/// - **Synchronization**: Coordinating startup with other participants
///
/// # Integration Test Examples
///
/// For complete working examples, see:
/// - `tests/client_server_integration.rs::test_start_endpoint_projection`
/// - `tests/client_server_integration.rs::test_protocol_initialization_lifecycle`
///
/// Common initialization patterns are defined in `tests/integration_common.rs`.
///
/// # See Also
///
/// - [`TChanStart`](crate::protocol::global::TChanStart) for the global protocol type
/// - [`Project`](crate::protocol::projection::Project) for projection mechanics
/// - [`LocalProtocol`] for the trait implemented by the continuation protocol
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpChanStart<IO, M, Start, AIO>
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Start: LocalProtocol,
    AIO: ActionIOTMarker,
{
    pub(super) _io: PhantomData<IO>,
    pub(super) _metadata: PhantomData<M>,
    pub(super) _start: PhantomData<Start>,
    pub(super) _aio: PhantomData<AIO>,
}
