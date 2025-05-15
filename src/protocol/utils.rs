//! # Protocol Utilities
//!
//! This module provides helper traits and type-level operations that support
//! the protocol system. These utilities enable compile-time verification
//! and transformation of protocol types.
//!
//! Key components:
//!
//! - Type-level boolean operations and checks
//! - Disjointness assertions for parallel composition
//! - Uniqueness checks for type-level lists
//! - Other helper traits for type-level programming
//!
//! These utilities ensure protocol safety and correctness at compile time.

use crate::sealed;
use crate::types;
use core::marker::PhantomData;
use super::base::*;

// Contents will be migrated from protocol.rs
// during the code migration phase