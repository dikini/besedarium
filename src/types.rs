//! # Protocol Marker Types and Message Primitives
//!
//! This module defines marker types for protocol IO (e.g., Http, Mqtt) and
//! message primitives (e.g., Message, Response, Publish). These are used as
//! type parameters in protocol combinators and endpoint types.
//!
//! - See `protocol.rs` for how these types are used in session combinators.
//! - See crate-level docs for protocol examples and macro usage.

use crate::sealed;
use crate::EpSession;
use core::marker::PhantomData;

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

/// Type-level boolean: True
pub struct True;
/// Type-level boolean: False
pub struct False;
/// Marker trait for type-level booleans.
pub trait Bool {}
impl Bool for True {}
impl Bool for False {}

/// Alias for type-level boolean True (for legacy naming in tests).
/// Alias for the type-level boolean `True`, used by legacy tests and macros.
pub type TrueB = True;
/// Alias for type-level boolean False (for legacy naming in tests).
/// Alias for the type-level boolean `False`, used by legacy tests and macros.
pub type FalseB = False;

/// Trait for compile-time type equality assertions.
/// Implemented only for identical types.
/// Implemented only when two types are identical.
pub trait TypeEq<A> {}

impl<T> TypeEq<T> for T {}

/// Boolean OR type-level function
/// Returns `True` if either A or B is `True`, otherwise `False`
pub type Or<A, B> = <A as BoolOr<B>>::Output;

/// Helper trait for implementing boolean OR at the type level
pub trait BoolOr<B> {
    type Output: Bool;
}

impl BoolOr<True> for True {
    type Output = True;
}

impl BoolOr<False> for True {
    type Output = True;
}

impl BoolOr<True> for False {
    type Output = True;
}

impl BoolOr<False> for False {
    type Output = False;
}

/// Boolean NOT type-level function
/// Returns `True` if input is `False`, otherwise `False`
pub trait Not {
    type Output: Bool;
}

impl Not for True {
    type Output = False;
}

impl Not for False {
    type Output = True;
}

/// Marker trait for user-definable protocol labels.
///
/// Implement this trait for any type you want to use as a protocol label.
/// Labels are used for recursion, branching, and protocol analysis.
pub trait ProtocolLabel {}

/// Empty label type for protocol ends or unlabeled combinators.
pub struct EmptyLabel;
impl ProtocolLabel for EmptyLabel {}

/// Silent/no-op endpoint type for roles not present in any protocol branch.
///
/// Used in endpoint projection to represent a role that is uninvolved in a parallel composition.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EpSilent<IO, R>(PhantomData<(IO, R)>);
impl<IO, R> EpSession<IO, R> for EpSilent<IO, R> {}
impl<IO, R> sealed::Sealed for EpSilent<IO, R> {}
