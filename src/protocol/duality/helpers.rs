//! Helper traits for type-level boolean constraints in duality checking
//!
//! This module provides supporting traits for the duality checking system,
//! particularly for type-level boolean operations and constraints.

use crate::protocol::foundation::GlobalProtocol;
use crate::types::{False, True};

/// Helper trait to ensure a type-level boolean is True
///
/// This trait is implemented only for the `True` type, allowing us to
/// constrain generic parameters to be provably true at compile time.
pub trait EqualsTrue {}
impl EqualsTrue for True {}

/// Helper trait to ensure a type-level boolean is False
///
/// This trait is implemented only for the `False` type, allowing us to
/// constrain generic parameters to be provably false at compile time.
pub trait EqualsFalse {}
impl EqualsFalse for False {}

/// Marker trait for types that can participate in duality checking
///
/// This trait serves as a safety mechanism to ensure only appropriate
/// protocol types are used in duality relationships.
pub trait DualityCheck: Send + Sync + 'static {}

// Implement for all protocol types
// Note: Only implementing for GlobalProtocol to avoid conflicts
impl<T: GlobalProtocol> DualityCheck for T {}
