//! # Protocol Module
//!
//! This module defines the core protocol types and type-level programming patterns
//! for session types in Besedarium. It provides the foundation for specifying,
//! validating, and projecting communication protocols.
//!
//! The module is split into several components based on abstraction layers:
//!
//! * `base`: Foundational types like `Nil`, `Cons` and base traits
//! * `global`: Global protocol types (`TSession`, `TEnd`, `TInteract`, etc.)
//! * `local`: Local (endpoint) protocol types (`EpSend`, `EpRecv`, etc.)
//! * `transforms`: Transformation logic including projection (`ProjectRole`, etc.)
//! * `utils`: Helper traits and type-level operations
//!
//! ## Type-Level Programming
//!
//! Many traits in this module use a type-level map/fold pattern to recursively
//! process type-level lists or protocol structures. This enables compile-time
//! verification of protocol properties and transformations.

// Declare submodules
mod base;
mod global;
mod local;
mod transforms;
mod utils;

// Re-export everything from submodules
pub use base::*;
pub use global::*;
pub use local::*;
pub use transforms::*;
pub use utils::*;