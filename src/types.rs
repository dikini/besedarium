//! # Type-Level Programming Types and Utilities
//!
//! This module provides the foundational types and traits for type-level programming
//! used throughout the protocol system, including:
//!
//! - Type-level booleans (`True`, `False`, `Bool`)
//! - Boolean operations and logic
//! - Protocol labels and markers
//! - IO and session type markers
//! - Type equality and comparison traits
//!
//! These types enable compile-time reasoning about protocol properties and ensure
//! type safety in the session types system.

use core::marker::PhantomData;

// Type-level boolean types and operations

/// Type-level boolean: True
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct True;

/// Type-level boolean: False  
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct False;

/// Marker trait for type-level booleans.
pub trait Bool {}

impl Bool for True {}
impl Bool for False {}

/// Alias for type-level boolean True (for legacy naming in tests).
pub type TrueB = True;

/// Alias for type-level boolean False (for legacy naming in tests).
pub type FalseB = False;

/// Trait for compile-time type equality assertions.
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

/// Type-level equality comparison for boolean types
/// Used to check if a type is equal to another specific type
pub trait IsEq<T> {}

impl IsEq<True> for True {}
impl IsEq<False> for False {}

// Protocol and session type markers

/// Marker trait for user-definable protocol labels.
///
/// Implement this trait for any type you want to use as a protocol label.
/// Labels are used for recursion, branching, and protocol analysis.
pub trait ProtocolLabel {}

/// Empty label type for protocol ends or unlabeled combinators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EmptyLabel;

impl ProtocolLabel for EmptyLabel {}

// IO and session type markers

/// Marker type for HTTP session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Http;

/// Marker type for database session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Db;

/// Marker type for MQTT session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Mqtt;

/// Marker type for cache session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Cache;

/// Marker type for mixed/multi-protocol session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Mixed;

/// A marker trait for all session types (e.g., Http, Db, Mqtt, etc.).
/// This helps in constraining generic parameters to valid session type markers.
pub trait SessionType {}

/// A trait indicating that a session type has a dual.
/// For bidirectional session types, the dual is typically itself.
pub trait HasDual {
    /// The dual session type.
    type Dual: SessionType;
}

// Implementations for IO marker types
impl SessionType for Http {}
impl HasDual for Http {
    type Dual = Http;
}

impl SessionType for Db {}
impl HasDual for Db {
    type Dual = Db;
}

impl SessionType for Mqtt {}
impl HasDual for Mqtt {
    type Dual = Mqtt;
}

impl SessionType for Cache {}
impl HasDual for Cache {
    type Dual = Cache;
}

impl SessionType for Mixed {}
impl HasDual for Mixed {
    type Dual = Mixed;
}

// Additional endpoint types

/// Silent/no-op endpoint type for roles not present in any protocol branch.
///
/// Used in endpoint projection to represent a role that is uninvolved in a parallel composition.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EpSilent<IO, R>(PhantomData<(IO, R)>);

// Sealed trait for controlled trait implementations
pub(crate) mod sealed {
    pub trait Sealed {}
}

/// Marker trait for endpoint session types.
pub trait EpSession<IO, R>: sealed::Sealed {}

impl<IO, R> EpSession<IO, R> for EpSilent<IO, R> {}
impl<IO, R> sealed::Sealed for EpSilent<IO, R> {}
