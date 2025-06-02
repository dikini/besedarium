//! # Foundation Types for Enhanced MPST System
//!
//! This module provides the foundational trait definitions and core infrastructure
//! for the Besedarium MPST library as specified in `docs/duality.md`.
//!
//! ## Key Components
//!
//! - **Foundation Traits**: Basic traits for roles, messages, and protocol types
//! - **CommMetadata**: Communication metadata for precise channel and message identification
//! - **Channel and Message Labels**: Type-safe identifiers for communication channels and messages
//! - **Action I/O Types**: Markers for different I/O capabilities (Input, Output, BiDirectional)
//! - **SupportsActionIO**: Trait for verifying I/O capability compatibility
//!
//! ## Module Navigation
//!
//! This foundation module works closely with other protocol modules:
//!
//! - **[`crate::protocol::global`]**: Global protocol types that build on these foundations
//! - **[`crate::protocol::local`]**: Local endpoint types derived from foundation traits
//! - **[`crate::protocol::projection`]**: Projection system that maps between global and local
//! - **[`crate::protocol::duality`]**: Duality checking that validates protocol correctness
//! - **[`crate::macros`]**: Macro system for convenient protocol definition
//!
//! ## Quick Start Examples
//!
//! For complete working examples and integration patterns, see:
//! - `tests/client_server_integration.rs` - Real protocol implementations
//! - `tests/integration_common.rs` - Standard roles, messages, and patterns
//! - `docs/protocol-examples.md` - Comprehensive usage documentation

use std::fmt::Debug;
use std::hash::Hash;

// ============================================================================
// Task 1.1.1a: Foundation Trait Definitions
// ============================================================================

/// Fundamental trait for role identification in protocols.
///
/// A role represents a participant in a communication protocol. Each role must be
/// uniquely identifiable and have specific communication capabilities. Roles are used
/// throughout the protocol system to define who can send and receive messages.
///
/// # Type Requirements
///
/// Implementing types must be:
/// - [`Send`] + [`Sync`] + `'static` for safe cross-thread usage
/// - [`Debug`] for debugging and error reporting
/// - [`Clone`] for creating multiple instances
/// - [`PartialEq`] + [`Eq`] + [`Hash`] for use in collections and comparisons
///
/// # Examples
///
/// Basic role implementation:
///
/// ```rust
/// use besedarium::Role;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct Client;
///
/// impl Role for Client {}
/// ```
///
/// Multiple roles in a client-server protocol:
///
/// ```rust
/// use besedarium::Role;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct Client;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct Server;
///
/// impl Role for Client {}
/// impl Role for Server {}
/// ```
///
/// # See Also
///
/// - [`GlobalProtocol`] for defining communication protocols between roles
/// - [`LocalProtocol`] for endpoint-specific protocol views
/// - [`SupportsActionIO`] for defining I/O capabilities of roles
pub trait Role: Send + Sync + 'static + Debug + Clone + PartialEq + Eq + Hash {}

/// Trait for messages that can be exchanged in protocols.
///
/// A message represents data that flows between roles in a protocol. Messages must be
/// serializable and transferable across process boundaries to support distributed
/// communication patterns.
///
/// # Type Requirements
///
/// Implementing types must be:
/// - [`Send`] + [`Sync`] + `'static` for safe cross-thread usage
/// - [`Debug`] for debugging and logging
/// - [`Clone`] for message duplication when needed
///
/// # Examples
///
/// Simple message types:
///
/// ```rust
/// use besedarium::Message;
///
/// #[derive(Debug, Clone)]
/// pub struct LoginRequest {
///     pub username: String,
///     pub password: String,
/// }
///
/// impl Message for LoginRequest {}
///
/// #[derive(Debug, Clone)]
/// pub struct LoginResponse {
///     pub success: bool,
///     pub token: Option<String>,
/// }
///
/// impl Message for LoginResponse {}
/// ```
///
/// Enum-based message for multiple message types:
///
/// ```rust
/// use besedarium::Message;
///
/// #[derive(Debug, Clone)]
/// pub enum ChatMessage {
///     TextMessage { content: String },
///     ImageMessage { url: String, alt_text: String },
///     SystemMessage { info: String },
/// }
///
/// impl Message for ChatMessage {}
/// ```
///
/// # Implementation Notes
///
/// While not enforced by this trait, it's recommended that message types also
/// implement [`serde::Serialize`] and [`serde::Deserialize`] for network communication.
///
/// # See Also
///
/// - [`MsgLbl`] for message type identifiers
/// - [`CommMetadata`] for associating messages with channels
/// - [`GlobalProtocol`] for defining message flow patterns
pub trait Message: Send + Sync + 'static + Debug + Clone {}

/// Marker trait for Global Protocol types.
///
/// Global protocols define the complete communication pattern between all participants
/// in a multi-party protocol. They specify the overall message flow, choice points,
/// and termination conditions from a global perspective.
///
/// # Purpose
///
/// This marker trait serves to:
/// - Type-check that a type represents a valid global protocol
/// - Enable generic functions that work specifically with global protocols
/// - Distinguish global protocols from local endpoint protocols
///
/// # Type Requirements
///
/// Implementing types must be:
/// - [`Send`] + [`Sync`] + `'static` for safe cross-thread usage
/// - [`Debug`] for debugging and protocol visualization
///
/// # Examples
///
/// Basic global protocol structure:
///
/// ```rust
/// use besedarium::GlobalProtocol;
///
/// #[derive(Debug)]
/// pub struct LoginProtocol<Next> {
///     _phantom: std::marker::PhantomData<Next>,
/// }
///
/// impl<Next> GlobalProtocol for LoginProtocol<Next>
/// where
///     Next: GlobalProtocol
/// {}
/// ```
///
/// Terminal global protocol:
///
/// ```rust
/// use besedarium::GlobalProtocol;
///
/// #[derive(Debug)]
/// pub struct End;
///
/// impl GlobalProtocol for End {}
/// ```
///
/// # Usage Patterns
///
/// Global protocols are typically used in:
/// - Protocol definition and specification
/// - Global protocol composition and nesting
/// - Projection to local endpoint protocols
/// - Protocol verification and analysis
///
/// # See Also
///
/// - [`LocalProtocol`] for endpoint-specific protocol views
/// - [`Project`] trait for converting global to local protocols
/// - [`IsDual`] for protocol duality verification
pub trait GlobalProtocol: Send + Sync + 'static + Debug {}

/// Marker trait for Local Endpoint Protocol types.
///
/// Local protocols represent the communication behavior from the perspective of a
/// specific participant (endpoint) in a multi-party protocol. They are derived from
/// global protocols through projection and specify what actions a particular role
/// must perform.
///
/// # Purpose
///
/// This marker trait serves to:
/// - Type-check that a type represents a valid local endpoint protocol
/// - Enable generic functions that work specifically with local protocols
/// - Distinguish local protocols from global protocols
/// - Support runtime session management and execution
///
/// # Type Requirements
///
/// Implementing types must be:
/// - [`Send`] + [`Sync`] + `'static` for safe cross-thread usage
/// - [`Debug`] for debugging and session monitoring
///
/// # Examples
///
/// Basic local protocol actions:
///
/// ```rust
/// use besedarium::LocalProtocol;
/// use std::fmt::Debug;
///
/// #[derive(Debug)]
/// pub struct SendThenReceive<M1, M2, Next>
/// where
///     M1: Send + Sync + 'static + Debug,
///     M2: Send + Sync + 'static + Debug,
///     Next: LocalProtocol,
/// {
///     _phantom: std::marker::PhantomData<(M1, M2, Next)>,
/// }
///
/// impl<M1, M2, Next> LocalProtocol for SendThenReceive<M1, M2, Next>
/// where
///     M1: Send + Sync + 'static + Debug,
///     M2: Send + Sync + 'static + Debug,
///     Next: LocalProtocol
/// {}
/// ```
///
/// Terminal local protocol:
///
/// ```rust
/// use besedarium::LocalProtocol;
///
/// #[derive(Debug)]
/// pub struct Close;
///
/// impl LocalProtocol for Close {}
/// ```
///
/// # Usage Patterns
///
/// Local protocols are typically used in:
/// - Session type implementation and runtime execution
/// - Endpoint behavior specification and validation
/// - Session state management and progression
/// - Protocol compliance checking at runtime
///
/// # Relationship to Global Protocols
///
/// Local protocols are derived from global protocols through projection:
/// ```text
/// GlobalProtocol --[project]--> LocalProtocol
/// ```
///
/// Each role in a global protocol gets its own local protocol view that
/// specifies exactly what that role should do.
///
/// # See Also
///
/// - [`GlobalProtocol`] for complete multi-party protocol specifications
/// - [`Project`] trait for projecting global protocols to local views
/// - [`SessionType`] for runtime session implementation
pub trait LocalProtocol: Send + Sync + 'static + Debug {}

// ============================================================================
// Task 1.1.1c: Channel and Message Label Traits
// ============================================================================

/// Trait for channel identifiers.
///
/// Channel identifiers provide type-safe identification of communication channels
/// within protocols. They enable the distinction between different logical channels
/// and support multiplexing of communications.
///
/// # Purpose
///
/// Channel IDs serve to:
/// - Uniquely identify communication channels in multi-channel protocols
/// - Enable type-safe channel management and routing
/// - Support protocol composition and channel isolation
/// - Facilitate debugging and monitoring of channel-specific communication
///
/// # Type Requirements
///
/// Implementing types must be:
/// - [`Send`] + [`Sync`] + `'static` for safe cross-thread usage
/// - [`Debug`] for debugging and logging
/// - [`Clone`] for creating channel references
/// - [`PartialEq`] + [`Eq`] + [`Hash`] for use in collections and lookups
///
/// # Examples
///
/// Simple channel identifiers:
///
/// ```rust
/// use besedarium::ChanId;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct ControlChannel;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct DataChannel;
///
/// impl ChanId for ControlChannel {}
/// impl ChanId for DataChannel {}
/// ```
///
/// Parameterized channel identifiers:
///
/// ```rust
/// use besedarium::ChanId;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct Channel<const ID: u32>;
///
/// impl<const ID: u32> ChanId for Channel<ID> {}
///
/// // Usage
/// type Channel1 = Channel<1>;
/// type Channel2 = Channel<2>;
/// ```
///
/// # Implementation Notes
///
/// For serialization support, implement [`serde::Serialize`] and [`serde::Deserialize`]:
///
/// ```rust
/// use besedarium::ChanId;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// pub struct NetworkChannel {
///     pub name: String,
/// }
///
/// impl ChanId for NetworkChannel {}
/// ```
///
/// # See Also
///
/// - [`MsgLbl`] for message type identification within channels
/// - [`CommMetadata`] for associating channels with message labels
/// - [`Metadata`] trait for extensible metadata systems
pub trait ChanId: Send + Sync + 'static + Debug + Clone + PartialEq + Eq + Hash {}

/// Trait for message labels within channels.
///
/// Message labels provide type-safe identification of message types within specific
/// channels. They enable fine-grained control over message routing and protocol
/// state management.
///
/// # Purpose
///
/// Message labels serve to:
/// - Distinguish between different message types within the same channel
/// - Enable type-safe message handling and routing
/// - Support protocol state transitions based on message types
/// - Facilitate debugging and monitoring of message flows
///
/// # Type Requirements
///
/// Implementing types must be:
/// - [`Send`] + [`Sync`] + `'static` for safe cross-thread usage
/// - [`Debug`] for debugging and logging
/// - [`Clone`] for creating message label references
/// - [`PartialEq`] + [`Eq`] + [`Hash`] for use in collections and matching
///
/// # Examples
///
/// Request-response message labels:
///
/// ```rust
/// use besedarium::MsgLbl;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct Request;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct Response;
///
/// impl MsgLbl for Request {}
/// impl MsgLbl for Response {}
/// ```
///
/// Enumerated message labels:
///
/// ```rust
/// use besedarium::MsgLbl;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub enum ChatLabel {
///     Message,
///     JoinRoom,
///     LeaveRoom,
///     UserList,
/// }
///
/// impl MsgLbl for ChatLabel {}
/// ```
///
/// Parameterized message labels:
///
/// ```rust
/// use besedarium::MsgLbl;
/// use std::fmt::Debug;
/// use std::hash::Hash;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct TypedMessage<T>
/// where
///     T: Send + Sync + 'static + Debug + Clone + PartialEq + Eq + Hash,
/// {
///     _phantom: std::marker::PhantomData<T>,
/// }
///
/// impl<T> MsgLbl for TypedMessage<T>
/// where
///     T: Send + Sync + 'static + Debug + Clone + PartialEq + Eq + Hash,
/// {}
/// ```
///
/// # Usage in Protocols
///
/// Message labels are commonly used with [`CommMetadata`] to create complete
/// message identification:
///
/// ```rust
/// use besedarium::{CommMetadata, ChanId, MsgLbl};
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct MyChannel;
/// impl ChanId for MyChannel {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct MyMessage;
/// impl MsgLbl for MyMessage {}
///
/// let metadata = CommMetadata::new(MyChannel, MyMessage);
/// ```
///
/// # See Also
///
/// - [`ChanId`] for channel identification
/// - [`CommMetadata`] for combining channels and message labels
/// - [`Message`] for the actual message content
pub trait MsgLbl: Send + Sync + 'static + Debug + Clone + PartialEq + Eq + Hash {}

// Example concrete channel types
/// Default channel identifier for simple protocols.
///
/// `DefaultChan` provides a basic channel identifier suitable for protocols
/// that only need a single communication channel or where channel distinction
/// is not required.
///
/// # Examples
///
/// ```rust
/// use besedarium::{DefaultChan, ChanId, CommMetadata, MsgLbl};
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct SimpleMessage;
/// impl MsgLbl for SimpleMessage {}
///
/// let metadata = CommMetadata::new(DefaultChan, SimpleMessage);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct DefaultChan;

/// Channel identifier for handshake and negotiation protocols.
///
/// `HandshakeChan` represents a dedicated channel for protocol initiation,
/// capability negotiation, and session establishment procedures.
///
/// # Examples
///
/// ```rust
/// use besedarium::{HandshakeChan, ChanId, CommMetadata, MsgLbl};
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct NegotiationMessage;
/// impl MsgLbl for NegotiationMessage {}
///
/// let metadata = CommMetadata::new(HandshakeChan, NegotiationMessage);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct HandshakeChan;

impl ChanId for DefaultChan {}
impl ChanId for HandshakeChan {}

// Example concrete message label types
/// Message label for request operations.
///
/// `RequestLbl` identifies messages that initiate some operation or request
/// information from another participant. Commonly used in request-response
/// communication patterns.
///
/// # Examples
///
/// ```rust
/// use besedarium::{RequestLbl, MsgLbl, CommMetadata, DefaultChan};
///
/// let metadata = CommMetadata::new(DefaultChan, RequestLbl);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct RequestLbl;

/// Message label for response operations.
///
/// `ResponseLbl` identifies messages that respond to previous requests,
/// providing requested information or confirming completion of operations.
///
/// # Examples
///
/// ```rust
/// use besedarium::{ResponseLbl, MsgLbl, CommMetadata, DefaultChan};
///
/// let metadata = CommMetadata::new(DefaultChan, ResponseLbl);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct ResponseLbl;

impl MsgLbl for RequestLbl {}
impl MsgLbl for ResponseLbl {}

// ============================================================================
// Task 1.1.1b: CommMetadata Implementation
// ============================================================================

/// Trait for communication metadata types that can be used in protocols.
///
/// This trait enables downstream implementations to extend metadata capabilities
/// while maintaining compatibility with the core protocol system. Metadata provides
/// the essential information needed to route and identify messages correctly.
///
/// # Purpose
///
/// The Metadata trait serves to:
/// - Associate channels with message labels for complete message identification
/// - Enable extensible metadata systems for different protocol requirements
/// - Provide a common interface for all metadata implementations
/// - Support type-safe message routing and handling
///
/// # Type Parameters
///
/// - `ChanId`: The type used for channel identification, must implement [`ChanId`]
/// - `MsgLbl`: The type used for message labeling, must implement [`MsgLbl`]
///
/// # Required Methods
///
/// - [`chan_id()`](Self::chan_id): Returns a reference to the channel identifier
/// - [`msg_lbl()`](Self::msg_lbl): Returns a reference to the message label
///
/// # Examples
///
/// Using the standard [`CommMetadata`]:
///
/// ```rust
/// use besedarium::{CommMetadata, Metadata, ChanId, MsgLbl};
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct HttpChannel;
/// impl ChanId for HttpChannel {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct GetRequest;
/// impl MsgLbl for GetRequest {}
///
/// let metadata = CommMetadata::new(HttpChannel, GetRequest);
/// assert_eq!(metadata.chan_id(), &HttpChannel);
/// assert_eq!(metadata.msg_lbl(), &GetRequest);
/// ```
///
/// Creating a custom metadata type:
///
/// ```rust
/// use besedarium::{Metadata, ChanId, MsgLbl};
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct MyChannel;
/// impl ChanId for MyChannel {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct MyLabel;
/// impl MsgLbl for MyLabel {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct TimestampedMetadata {
///     chan_id: MyChannel,
///     msg_lbl: MyLabel,
///     timestamp: u64,
/// }
///
/// impl Metadata for TimestampedMetadata {
///     type ChanId = MyChannel;
///     type MsgLbl = MyLabel;
///
///     fn chan_id(&self) -> &Self::ChanId {
///         &self.chan_id
///     }
///
///     fn msg_lbl(&self) -> &Self::MsgLbl {
///         &self.msg_lbl
///     }
/// }
/// ```
///
/// # Extension Patterns
///
/// Common metadata extensions include:
/// - **Timestamped metadata**: Adding creation/processing timestamps
/// - **Priority metadata**: Supporting QoS and priority handling
/// - **Routing metadata**: Including source/destination information
/// - **Security metadata**: Adding authentication and authorization data
///
/// # See Also
///
/// - [`CommMetadata`] for the standard metadata implementation
/// - [`ChanId`] and [`MsgLbl`] for the constituent identifier types
/// - [`CommMetadataTrait`] for an alternative extensible metadata interface
pub trait Metadata: Send + Sync + 'static + Debug + Clone + PartialEq + Eq + Hash {
    /// Channel identifier type for this metadata
    type ChanId: ChanId;
    /// Message label type for this metadata  
    type MsgLbl: MsgLbl;

    /// Get the channel ID from this metadata
    fn chan_id(&self) -> &Self::ChanId;
    /// Get the message label from this metadata
    fn msg_lbl(&self) -> &Self::MsgLbl;
}

/// Communication metadata for precise channel and message identification.
///
/// `CommMetadata` is the standard implementation of communication metadata that
/// combines a channel identifier with a message label to provide complete
/// identification of messages within protocols.
///
/// # Type Parameters
///
/// - `C`: Channel identifier type, must implement [`ChanId`]
/// - `L`: Message label type, must implement [`MsgLbl`]
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use besedarium::{CommMetadata, ChanId, MsgLbl};
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct ApiChannel;
/// impl ChanId for ApiChannel {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct LoginRequest;
/// impl MsgLbl for LoginRequest {}
///
/// let metadata = CommMetadata::new(ApiChannel, LoginRequest);
/// ```
///
/// Using with protocol messages:
///
/// ```rust
/// use besedarium::{CommMetadata, ChanId, MsgLbl, Message};
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct ChatChannel;
/// impl ChanId for ChatChannel {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct TextMessage;
/// impl MsgLbl for TextMessage {}
///
/// #[derive(Debug, Clone)]
/// pub struct ChatContent {
///     pub text: String,
///     pub sender: String,
/// }
/// impl Message for ChatContent {}
///
/// let metadata = CommMetadata::new(ChatChannel, TextMessage);
/// let content = ChatContent {
///     text: "Hello, world!".to_string(),
///     sender: "Alice".to_string(),
/// };
/// ```
///
/// # Serialization Support
///
/// `CommMetadata` supports serialization via [`serde`] when both the channel ID
/// and message label types implement [`serde::Serialize`] and [`serde::Deserialize`]:
///
/// ```rust
/// use besedarium::{CommMetadata, ChanId, MsgLbl};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// pub struct NetworkChannel {
///     pub id: u32,
/// }
/// impl ChanId for NetworkChannel {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// pub struct RequestType {
///     pub name: String,
/// }
/// impl MsgLbl for RequestType {}
///
/// let metadata = CommMetadata::new(
///     NetworkChannel { id: 1 },
///     RequestType { name: "GET".to_string() }
/// );
///
/// // Can now serialize/deserialize metadata
/// let json = serde_json::to_string(&metadata).unwrap();
/// let deserialized: CommMetadata<NetworkChannel, RequestType> =
///     serde_json::from_str(&json).unwrap();
/// ```
///
/// # See Also
///
/// - [`Metadata`] trait for the interface this type implements
/// - [`ChanId`] and [`MsgLbl`] for the constituent identifier types
/// - [`CommMetadataTrait`] for an alternative extensible interface
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommMetadata<C: ChanId, L: MsgLbl> {
    pub chan_id: C,
    pub msg_lbl: L,
}

impl<C: ChanId, L: MsgLbl> CommMetadata<C, L> {
    /// Creates new communication metadata from a channel ID and message label.
    ///
    /// # Arguments
    ///
    /// * `chan_id` - The channel identifier
    /// * `msg_lbl` - The message label
    ///
    /// # Examples
    ///
    /// ```rust
    /// use besedarium::{CommMetadata, ChanId, MsgLbl};
    ///
    /// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    /// pub struct Channel1;
    /// impl ChanId for Channel1 {}
    ///
    /// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    /// pub struct Hello;
    /// impl MsgLbl for Hello {}
    ///
    /// let metadata = CommMetadata::new(Channel1, Hello);
    /// ```
    pub fn new(chan_id: C, msg_lbl: L) -> Self {
        Self { chan_id, msg_lbl }
    }
}

impl<C: ChanId, L: MsgLbl> Metadata for CommMetadata<C, L> {
    type ChanId = C;
    type MsgLbl = L;

    fn chan_id(&self) -> &Self::ChanId {
        &self.chan_id
    }

    fn msg_lbl(&self) -> &Self::MsgLbl {
        &self.msg_lbl
    }
}

// Manual serde implementations for CommMetadata
impl<C, L> serde::Serialize for CommMetadata<C, L>
where
    C: ChanId + serde::Serialize,
    L: MsgLbl + serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("CommMetadata", 2)?;
        state.serialize_field("chan_id", &self.chan_id)?;
        state.serialize_field("msg_lbl", &self.msg_lbl)?;
        state.end()
    }
}

impl<'de, C, L> serde::Deserialize<'de> for CommMetadata<C, L>
where
    C: ChanId + serde::Deserialize<'de>,
    L: MsgLbl + serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            ChanId,
            MsgLbl,
        }

        struct CommMetadataVisitor<C, L>(std::marker::PhantomData<(C, L)>);

        impl<'de, C, L> Visitor<'de> for CommMetadataVisitor<C, L>
        where
            C: ChanId + serde::Deserialize<'de>,
            L: MsgLbl + serde::Deserialize<'de>,
        {
            type Value = CommMetadata<C, L>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct CommMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CommMetadata<C, L>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut chan_id = None;
                let mut msg_lbl = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ChanId => {
                            if chan_id.is_some() {
                                return Err(de::Error::duplicate_field("chan_id"));
                            }
                            chan_id = Some(map.next_value()?);
                        }
                        Field::MsgLbl => {
                            if msg_lbl.is_some() {
                                return Err(de::Error::duplicate_field("msg_lbl"));
                            }
                            msg_lbl = Some(map.next_value()?);
                        }
                    }
                }

                let chan_id = chan_id.ok_or_else(|| de::Error::missing_field("chan_id"))?;
                let msg_lbl = msg_lbl.ok_or_else(|| de::Error::missing_field("msg_lbl"))?;

                Ok(CommMetadata { chan_id, msg_lbl })
            }
        }

        const FIELDS: &[&str] = &["chan_id", "msg_lbl"];
        deserializer.deserialize_struct(
            "CommMetadata",
            FIELDS,
            CommMetadataVisitor(std::marker::PhantomData),
        )
    }
}

// ============================================================================
// Task 1.1.1d: ActionIOTMarker System
// ============================================================================

/// Marker trait for Action I/O Types - defines what I/O capability an action requires.
///
/// This trait categorizes different types of I/O operations that can be performed
/// in protocols, enabling compile-time verification that roles and communication
/// channels support the required I/O patterns.
///
/// # Purpose
///
/// Action I/O markers serve to:
/// - Classify I/O operations as input, output, or bidirectional
/// - Enable compile-time checking of I/O capability compatibility
/// - Support fine-grained control over role permissions and capabilities
/// - Facilitate protocol analysis and optimization based on I/O patterns
///
/// # Type Requirements
///
/// Implementing types must be:
/// - [`Send`] + [`Sync`] + `'static` for safe cross-thread usage
/// - [`Debug`] for debugging and introspection
/// - [`Clone`] for creating I/O type references
/// - [`PartialEq`] + [`Eq`] for I/O type comparison and matching
///
/// # Standard Action Types
///
/// The library provides three standard action I/O types:
/// - [`InputAction`]: For receive-only operations
/// - [`OutputAction`]: For send-only operations  
/// - [`BiDirectionalAction`]: For operations that both send and receive
///
/// # Examples
///
/// Creating custom action I/O types:
///
/// ```rust
/// use besedarium::ActionIOTMarker;
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// pub struct StreamingAction;
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// pub struct BroadcastAction;
///
/// impl ActionIOTMarker for StreamingAction {}
/// impl ActionIOTMarker for BroadcastAction {}
/// ```
///
/// Using with [`SupportsActionIO`]:
///
/// ```rust
/// use besedarium::{ActionIOTMarker, SupportsActionIO, InputAction, OutputAction};
///
/// #[derive(Debug)]
/// pub struct ReadOnlyChannel;
///
/// impl SupportsActionIO<InputAction> for ReadOnlyChannel {
///     fn supports_action_io() -> bool { true }
/// }
///
/// // ReadOnlyChannel doesn't implement SupportsActionIO<OutputAction>,
/// // so it cannot be used for output operations
/// ```
///
/// # Design Patterns
///
/// Common usage patterns include:
/// - **Channel Capability Verification**: Ensuring channels support required I/O
/// - **Role Permission Checking**: Verifying roles can perform required actions
/// - **Protocol Validation**: Checking protocol compatibility with I/O constraints
/// - **Runtime Optimization**: Selecting optimal implementations based on I/O patterns
///
/// # See Also
///
/// - [`SupportsActionIO`] for checking I/O capability compatibility
/// - [`InputAction`], [`OutputAction`], [`BiDirectionalAction`] for standard action types
/// - [`Role`] for role-based I/O capability association
pub trait ActionIOTMarker: Send + Sync + 'static + Debug + Clone + PartialEq + Eq {}

/// Standard Action I/O Types
///
/// These types represent the three fundamental I/O patterns in communication protocols.
/// Marker type for input (receive-only) actions.
///
/// `InputAction` represents operations that only receive data without sending any
/// response. This is typically used for passive data collection, monitoring, or
/// one-way notifications.
///
/// # Examples
///
/// ```rust
/// use besedarium::{InputAction, SupportsActionIO};
///
/// #[derive(Debug)]
/// pub struct LogReceiver;
///
/// impl SupportsActionIO<InputAction> for LogReceiver {}
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputAction;

/// Marker type for output (send-only) actions.
///
/// `OutputAction` represents operations that only send data without expecting
/// any response. This is typically used for fire-and-forget messaging, logging,
/// or broadcast scenarios.
///
/// # Examples
///
/// ```rust
/// use besedarium::{OutputAction, SupportsActionIO};
///
/// #[derive(Debug)]
/// pub struct EventPublisher;
///
/// impl SupportsActionIO<OutputAction> for EventPublisher {}
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputAction;

/// Marker type for bidirectional actions.
///
/// `BiDirectionalAction` represents operations that both send and receive data,
/// typically in request-response patterns or interactive communication scenarios.
///
/// # Examples
///
/// ```rust
/// use besedarium::{BiDirectionalAction, SupportsActionIO};
///
/// #[derive(Debug)]
/// pub struct InteractiveSession;
///
/// impl SupportsActionIO<BiDirectionalAction> for InteractiveSession {}
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BiDirectionalAction;

impl ActionIOTMarker for InputAction {}
impl ActionIOTMarker for OutputAction {}
impl ActionIOTMarker for BiDirectionalAction {}

// ============================================================================
// Task 1.1.1e: SupportsActionIO Trait
// ============================================================================

/// Trait to verify I/O capability compatibility between roles and action types.
///
/// This trait enables compile-time and runtime verification that a role or
/// communication channel supports the required I/O patterns for specific actions.
/// It provides a type-safe way to ensure protocol compatibility.
///
/// # Type Parameters
///
/// - `AIO`: The action I/O type that must implement [`ActionIOTMarker`]
///
/// # Purpose
///
/// `SupportsActionIO` serves to:
/// - Verify that roles can perform required I/O operations
/// - Enable compile-time checking of protocol compatibility
/// - Support runtime capability negotiation and validation
/// - Facilitate protocol optimization based on supported I/O patterns
///
/// # Default Implementation
///
/// The default implementation returns `true`, assuming support for all action types.
/// Override this method to provide specific capability restrictions:
///
/// ```rust
/// use besedarium::{SupportsActionIO, InputAction, OutputAction};
///
/// #[derive(Debug)]
/// pub struct ReadOnlyRole;
///
/// impl SupportsActionIO<InputAction> for ReadOnlyRole {
///     fn supports_action_io() -> bool { true }
/// }
///
/// impl SupportsActionIO<OutputAction> for ReadOnlyRole {
///     fn supports_action_io() -> bool { false } // Read-only role cannot send
/// }
/// ```
///
/// # Examples
///
/// Basic role with full I/O capabilities:
///
/// ```rust
/// use besedarium::{SupportsActionIO, InputAction, OutputAction, BiDirectionalAction, Role};
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct FullRole;
///
/// impl Role for FullRole {}
/// impl SupportsActionIO<InputAction> for FullRole {}
/// impl SupportsActionIO<OutputAction> for FullRole {}
/// impl SupportsActionIO<BiDirectionalAction> for FullRole {}
/// ```
///
/// Specialized role with limited capabilities:
///
/// ```rust
/// use besedarium::{SupportsActionIO, InputAction, OutputAction, Role};
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct MonitorRole;
///
/// impl Role for MonitorRole {}
///
/// // Only supports input (monitoring)
/// impl SupportsActionIO<InputAction> for MonitorRole {}
///
/// // Explicitly doesn't support output
/// impl SupportsActionIO<OutputAction> for MonitorRole {
///     fn supports_action_io() -> bool { false }
/// }
/// ```
///
/// Runtime capability checking:
///
/// ```rust
/// use besedarium::{SupportsActionIO, InputAction, OutputAction};
///
/// fn can_perform_input<T: SupportsActionIO<InputAction>>() -> bool {
///     T::supports_action_io()
/// }
///
/// fn can_perform_output<T: SupportsActionIO<OutputAction>>() -> bool {
///     T::supports_action_io()
/// }
/// ```
///
/// # Design Patterns
///
/// Common usage patterns include:
///
/// - **Capability-based Role Design**: Different roles with different I/O permissions
/// - **Protocol Validation**: Ensuring all required capabilities are available
/// - **Runtime Negotiation**: Dynamically checking available capabilities
/// - **Security Constraints**: Restricting roles to specific I/O operations
///
/// # See Also
///
/// - [`ActionIOTMarker`] for defining action I/O types
/// - [`Role`] for role-based capability association
/// - [`InputAction`], [`OutputAction`], [`BiDirectionalAction`] for standard I/O types
pub trait SupportsActionIO<AIO: ActionIOTMarker> {
    /// Returns true if this implementation can handle the specified action type.
    ///
    /// # Default Behavior
    ///
    /// The default implementation returns `true`, assuming support for all action types.
    /// Override this method to provide specific capability restrictions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use besedarium::{SupportsActionIO, OutputAction};
    ///
    /// #[derive(Debug)]
    /// pub struct ReadOnlyChannel;
    ///
    /// impl SupportsActionIO<OutputAction> for ReadOnlyChannel {
    ///     fn supports_action_io() -> bool {
    ///         false // This channel cannot perform output operations
    ///     }
    /// }
    /// ```
    fn supports_action_io() -> bool {
        true // Default implementation assumes support
    }
}

// Example implementation: TCP-based session I/O that supports all actions
/// Example I/O implementation that supports all action types.
///
/// `TcpOnlySessionIO` represents a TCP-based communication channel that can
/// handle input, output, and bidirectional operations. This serves as an
/// example of a fully-capable I/O implementation.
///
/// # Examples
///
/// ```rust
/// use besedarium::{SupportsActionIO, InputAction, OutputAction, BiDirectionalAction};
///
/// # #[derive(Debug)]
/// # pub struct TcpOnlySessionIO;
/// # impl SupportsActionIO<InputAction> for TcpOnlySessionIO {}
/// # impl SupportsActionIO<OutputAction> for TcpOnlySessionIO {}  
/// # impl SupportsActionIO<BiDirectionalAction> for TcpOnlySessionIO {}
/// // TcpOnlySessionIO can be used for any action type
/// assert!(<TcpOnlySessionIO as SupportsActionIO<InputAction>>::supports_action_io());
/// assert!(<TcpOnlySessionIO as SupportsActionIO<OutputAction>>::supports_action_io());
/// assert!(<TcpOnlySessionIO as SupportsActionIO<BiDirectionalAction>>::supports_action_io());
/// ```
#[derive(Debug)]
pub struct TcpOnlySessionIO;

impl SupportsActionIO<InputAction> for TcpOnlySessionIO {}
impl SupportsActionIO<OutputAction> for TcpOnlySessionIO {}
impl SupportsActionIO<BiDirectionalAction> for TcpOnlySessionIO {}

// Example implementation: HTTP-based session I/O that only supports output and bidirectional
/// Example I/O implementation with limited capabilities.
///
/// `HttpOnlySessionIO` represents an HTTP-based communication channel that only
/// supports output and bidirectional operations. It cannot handle pure input
/// operations, demonstrating how to create capability-restricted I/O implementations.
///
/// # Capability Restrictions
///
/// - ✅ Supports [`OutputAction`]: Can send HTTP requests/responses
/// - ✅ Supports [`BiDirectionalAction`]: Can handle request-response patterns
/// - ❌ Does not support [`InputAction`]: HTTP is inherently request-driven
///
/// # Examples
///
/// ```rust
/// use besedarium::{SupportsActionIO, InputAction, OutputAction, BiDirectionalAction};
///
/// # #[derive(Debug)]
/// # pub struct HttpOnlySessionIO;
/// # impl SupportsActionIO<OutputAction> for HttpOnlySessionIO {}
/// # impl SupportsActionIO<BiDirectionalAction> for HttpOnlySessionIO {}
/// // HttpOnlySessionIO has limited capabilities
/// // Note: Cannot test InputAction support because it's not implemented
/// assert!(<HttpOnlySessionIO as SupportsActionIO<OutputAction>>::supports_action_io());
/// assert!(<HttpOnlySessionIO as SupportsActionIO<BiDirectionalAction>>::supports_action_io());
/// ```
#[derive(Debug)]
pub struct HttpOnlySessionIO;

impl SupportsActionIO<OutputAction> for HttpOnlySessionIO {}
impl SupportsActionIO<BiDirectionalAction> for HttpOnlySessionIO {}
// Note: HttpOnlySessionIO doesn't support InputAction

// ============================================================================
// Extensible Metadata Infrastructure
// ============================================================================

/// Trait for extensible communication metadata.
///
/// This trait provides an alternative interface for communication metadata that
/// includes a factory method for creating new instances. It enables downstream
/// implementations to extend metadata while maintaining compatibility with the
/// core protocol system.
///
/// # Relationship to [`Metadata`]
///
/// While [`Metadata`] provides the basic interface for accessing channel and message
/// information, `CommMetadataTrait` extends this with creation capabilities,
/// making it more suitable for dynamic metadata construction.
///
/// # Type Parameters
///
/// - `ChanId`: The channel identifier type, must implement [`ChanId`]
/// - `MsgLbl`: The message label type, must implement [`MsgLbl`]
///
/// # Required Methods
///
/// - [`chan_id()`](Self::chan_id): Returns the channel identifier
/// - [`msg_lbl()`](Self::msg_lbl): Returns the message label  
/// - [`new()`](Self::new): Creates new metadata from constituents
///
/// # Examples
///
/// Using with the standard [`CommMetadata`]:
///
/// ```rust
/// use besedarium::{CommMetadataTrait, CommMetadata, ChanId, MsgLbl};
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct TestChannel;
/// impl ChanId for TestChannel {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct TestLabel;
/// impl MsgLbl for TestLabel {}
///
/// let metadata = CommMetadata::new(TestChannel, TestLabel);
/// assert_eq!(metadata.chan_id(), &TestChannel);
/// assert_eq!(metadata.msg_lbl(), &TestLabel);
/// ```
///
/// Creating a custom metadata implementation:
///
/// ```rust
/// use besedarium::{CommMetadataTrait, ChanId, MsgLbl};
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct MyChannel;
/// impl ChanId for MyChannel {}
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// pub struct MyLabel;
/// impl MsgLbl for MyLabel {}
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// pub struct PriorityMetadata {
///     chan_id: MyChannel,
///     msg_lbl: MyLabel,
///     priority: u8,
/// }
///
/// impl CommMetadataTrait for PriorityMetadata {
///     type ChanId = MyChannel;
///     type MsgLbl = MyLabel;
///
///     fn chan_id(&self) -> &Self::ChanId {
///         &self.chan_id
///     }
///
///     fn msg_lbl(&self) -> &Self::MsgLbl {
///         &self.msg_lbl
///     }
///
///     fn new(chan_id: Self::ChanId, msg_lbl: Self::MsgLbl) -> Self {
///         Self {
///             chan_id,
///             msg_lbl,
///             priority: 0, // Default priority
///         }
///     }
/// }
/// ```
///
/// # Extension Patterns
///
/// Common metadata extensions include:
/// - **Priority-aware metadata**: Adding QoS priority information
/// - **Timestamped metadata**: Including creation/expiry timestamps
/// - **Routing metadata**: Adding source/destination routing information
/// - **Security metadata**: Including authentication tokens or signatures
///
/// # See Also
///
/// - [`Metadata`] for the basic metadata interface
/// - [`CommMetadata`] for the standard metadata implementation
/// - [`ChanId`] and [`MsgLbl`] for the constituent identifier types
pub trait CommMetadataTrait: Send + Sync + 'static + Debug + Clone + PartialEq + Eq {
    /// The channel identifier type
    type ChanId: ChanId;

    /// The message label type
    type MsgLbl: MsgLbl;

    /// Get the channel identifier
    fn chan_id(&self) -> &Self::ChanId;

    /// Get the message label
    fn msg_lbl(&self) -> &Self::MsgLbl;

    /// Create new metadata from channel and label
    fn new(chan_id: Self::ChanId, msg_lbl: Self::MsgLbl) -> Self;
}

/// Implementation of CommMetadataTrait for the standard CommMetadata type
impl<C: ChanId, L: MsgLbl> CommMetadataTrait for CommMetadata<C, L> {
    type ChanId = C;
    type MsgLbl = L;

    fn chan_id(&self) -> &Self::ChanId {
        &self.chan_id
    }

    fn msg_lbl(&self) -> &Self::MsgLbl {
        &self.msg_lbl
    }

    fn new(chan_id: Self::ChanId, msg_lbl: Self::MsgLbl) -> Self {
        CommMetadata::new(chan_id, msg_lbl)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests;

#[cfg(test)]
mod action_io_tests;

/// Label transformation and preservation logic for session types
pub mod labels;

// Re-export key label transformation traits for convenience
pub use labels::{
    ExtractLabels, LList, Label, LabelComposition, LabelCons, LabelList, LabelNil, LabelPredicate,
    LabelPreservation, LabelTransform, LabelValidation, LabelValidationError, TCollect, TFilter,
    TMap, UniqueLabels, ValidateChoiceLabels,
};
