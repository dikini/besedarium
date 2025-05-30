//! # Besedarium: Multi-Party Session Types for Rust
//!
//! A type-level session types library implementing enhanced Multi-Party Session Types (MPST)
//! with channel-aware types, IO capability validation, and comprehensive duality checking.
//!
//! ## Current Status
//!
//! This crate is undergoing a major restructuring to implement an enhanced MPST system
//! based on the theoretical foundation in `docs/duality.md`. The current implementation
//! is being replaced with a new system featuring:
//!
//! - Foundation types with CommMetadata and ActionIO capabilities
//! - Enhanced global and local protocol types  
//! - Comprehensive duality checking
//! - Advanced projection with validation
//!
//! ## Implementation Progress
//!
//! The implementation follows Task 1.1 as outlined in `work/TASKS.md`:
//! - [x] Task 1.1.1: Foundation types and CommMetadata
//! - [x] Task 1.1.2: Global protocol types (TChan*)
//! - [ ] Task 1.1.3: Local endpoint types (Ep*)
//! - [ ] Task 1.1.4: Duality checking (IsDual trait)
//! - [ ] Task 1.1.5: Projection (Project<P, Role> trait)
//!
//! See `work/prompts/` for detailed implementation guidance.

#![cfg_attr(docsrs, doc = include_str!("../README.md"))]

pub mod macros; // Added module declaration

// Core module containing the new MPST implementation
pub mod protocol;

// Runtime components for session type execution
pub mod runtime;

// Type-level programming types and utilities
pub mod types;

// Re-export key components for easier use
pub use protocol::foundation::*;
pub use protocol::global::*;
pub use types::*;
