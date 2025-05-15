//! # Protocol Base Types and Traits
//!
//! This module provides the foundational types and traits for building
//! protocol types in the Besedarium library. It includes:
//!
//! - Type-level list primitives (`Nil`, `Cons`)
//! - Core marker traits
//! - Base implementations for type-level operations
//!
//! These types form the foundation for the type-level programming patterns
//! used throughout the protocol system.

use crate::sealed;
use crate::types;
use core::marker::PhantomData;

// Contents will be migrated from protocol.rs
// during the code migration phase