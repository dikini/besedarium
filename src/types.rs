//! # Protocol Marker Types and Message Primitives
//!
//! This module defines marker types for protocol IO (e.g., Http, Mqtt) and
//! message primitives (e.g., Message, Response, Publish). These are used as
//! type parameters in protocol combinators and endpoint types.
//!
//! - See `protocol.rs` for how these types are used in session combinators.
//! - See crate-level docs for protocol examples and macro usage.

use crate::sealed;

/// Marker type for a generic protocol message.
#[derive(Debug)]
pub struct Message;
/// Marker type for a generic protocol response.
#[derive(Debug)]
pub struct Response;
/// Marker type for a publish event (e.g., in pub/sub protocols).
#[derive(Debug)]
pub struct Publish;
/// Marker type for a notification event.
#[derive(Debug)]
pub struct Notify;
/// Marker type for a subscribe event.
#[derive(Debug)]
pub struct Subscribe;

/// Marker type for HTTP protocol.
#[derive(Debug)]
pub struct Http;
/// Marker type for a database protocol.
#[derive(Debug)]
pub struct Db;
/// Marker type for MQTT protocol.
#[derive(Debug)]
pub struct Mqtt;
/// Marker type for a cache protocol.
#[derive(Debug)]
pub struct Cache;
/// Marker type for a mixed/multi-protocol session.
#[derive(Debug)]
pub struct Mixed;

/// Marker trait for types representing specific I/O mechanisms (e.g., TCP, HTTP).
///
/// This trait is implemented by types like `Tcp`, `Http`, `Mqtt` to indicate
/// that they represent a specific kind of I/O operation.
pub trait ActionIOTMarker: sealed::Sealed + Send + Sync + 'static + core::fmt::Debug {}

// Example Action I/O Type markers
/// Marker type for TCP I/O actions.
#[derive(Debug)]
pub struct Tcp;
impl sealed::Sealed for Tcp {}
impl ActionIOTMarker for Tcp {}

/// Marker type for HTTP I/O actions.
#[derive(Debug)]
pub struct HttpIo; // Renamed from Http to avoid conflict with the existing Http marker type
impl sealed::Sealed for HttpIo {}
impl ActionIOTMarker for HttpIo {}

/// Marker type for MQTT I/O actions.
#[derive(Debug)]
pub struct MqttIo; // Renamed from Mqtt to avoid conflict with the existing Mqtt marker type
impl sealed::Sealed for MqttIo {}
impl ActionIOTMarker for MqttIo {}

/// Indicates that a session's overall I/O capability (`Self`, the `IO` parameter)
/// can support a specific `ActionIOType` (`AIO`).
///
/// This trait is crucial for ensuring that a role's provided I/O infrastructure
/// is compatible with the requirements of the protocol actions it needs to perform.
pub trait SupportsActionIO<AIO: ActionIOTMarker>: Send + Sync + 'static + core::fmt::Debug {}

/// Type-level boolean: True
pub struct True;
/// Marker type for a type-level boolean: False
pub struct False;

/// Type-level boolean trait.
///
/// Implemented by `True` and `False` to enable type-level conditional logic.
/// This is crucial for many protocol transformation and verification patterns.
pub trait Bool {}
impl Bool for True {}
impl Bool for False {}

/// Type-level OR operation for booleans.
///
/// Computes the logical OR of two type-level booleans.
/// - `True || B = True`
/// - `False || B = B`
pub trait BoolOr<B: Bool> {
    type Output: Bool;
}

impl<B: Bool> BoolOr<B> for True {
    type Output = True;
}

impl<B: Bool> BoolOr<B> for False {
    type Output = B;
}

/// Represents a unique identifier for a communication channel.
///
/// Used within `CommMetadata` to distinguish between different logical
/// communication pathways in a protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChanId(pub u64); // Made ChanId public for construction

/// Represents a label for a message within a communication channel.
///
/// Used within `CommMetadata` to provide context or routing information
/// for messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MsgLbl(pub u64); // Made MsgLbl public for construction

/// Communication Metadata.
///
/// Contains a channel identifier (`ChanId`) and a message label (`MsgLbl`)
/// to provide context for communication actions. The `ActionIOType` is
/// typically handled as a separate generic parameter on action types like
/// `TChanSend` or `EpSend`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommMetadata {
    pub chan_id: ChanId,
    pub msg_lbl: MsgLbl,
}

/// Trait to get the Channel ID from a type that contains it.
pub trait GetChanId {
    fn get_chan_id(&self) -> ChanId;
}

impl GetChanId for CommMetadata {
    fn get_chan_id(&self) -> ChanId {
        self.chan_id
    }
}

/// Trait to get the Message Label from a type that contains it.
pub trait GetMsgLbl {
    fn get_msg_lbl(&self) -> MsgLbl;
}

impl GetMsgLbl for CommMetadata {
    fn get_msg_lbl(&self) -> MsgLbl {
        self.msg_lbl
    }
}

/// Trait for types that represent a protocol label.
///
/// Protocol labels are used to identify specific points or segments
/// within a protocol, such_as choice branches, recursion points, or
/// specific send/receive operations.
///
/// They must be unique where required by the protocol semantics (e.g.,
/// recursion labels within a scope, choice branch labels).
pub trait ProtocolLabel: sealed::Sealed + core::fmt::Debug + Send + Sync + 'static {}

/// Trait for types that can act as session types (e.g. Http, Mqtt).
///
/// This marker trait is used to constrain generic parameters in protocol
/// definitions, ensuring they represent valid session types.
pub trait SessionType: sealed::Sealed + core::fmt::Debug + Send + Sync + 'static {}

/// Marker trait for types that represent a global protocol.
pub trait GlobalProtocol: sealed::Sealed + Send + Sync + 'static + core::fmt::Debug {}

/// Marker trait for types that represent a local protocol endpoint.
pub trait LocalProtocol: sealed::Sealed + Send + Sync + 'static + core::fmt::Debug {}

// Base implementations for ProtocolLabel and SessionType for marker types
// This allows marker types like Http, Mqtt, etc., to be used directly
// where a ProtocolLabel or SessionType is expected, assuming they also
// implement sealed::Sealed (which is typically done in the respective
// protocol modules or via a macro).

// Example (assuming sealed::Sealed is handled elsewhere or derived):
// impl sealed::Sealed for Http {}
// impl ProtocolLabel for Http {}
// impl SessionType for Http {}

// impl sealed::Sealed for Mqtt {}
// impl ProtocolLabel for Mqtt {}
// impl SessionType for Mqtt {}

// ... and so on for other marker types ...

// Note: The actual `impl sealed::Sealed for ...` will likely be in the
// `crate::sealed` module or handled by macros to ensure proper sealing.
