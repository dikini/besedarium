//! # Protocol Marker Types and Message Primitives
//!
//! This module defines marker types for protocol IO (e.g., Http, Mqtt) and
//! message primitives (e.g., Message, Response, Publish). These are used as
//! type parameters in protocol combinators and endpoint types.
//!
//! - See `protocol.rs` for how these types are used in session combinators.
//! - See crate-level docs for protocol examples and macro usage.

/// Marker type for a generic protocol message.
pub struct Message;
/// Marker type for a generic protocol response.
pub struct Response;
/// Marker type for a publish event (e.g., in pub/sub protocols).
pub struct Publish;
/// Marker type for a notification event.
pub struct Notify;
/// Marker type for a subscribe event.
pub struct Subscribe;

/// Marker type for HTTP protocol.
pub struct Http;
/// Marker type for a database protocol.
pub struct Db;
/// Marker type for MQTT protocol.
pub struct Mqtt;
/// Marker type for a cache protocol.
pub struct Cache;
/// Marker type for a mixed/multi-protocol session.
pub struct Mixed;

/// Marker trait for user-definable protocol labels.
///
/// Implement this trait for any type you want to use as a protocol label.
/// Labels are used for recursion, branching, and protocol analysis.
pub trait ProtocolLabel {}

/// Empty label type for protocol ends or unlabeled combinators.
pub struct EmptyLabel;
impl ProtocolLabel for EmptyLabel {}
