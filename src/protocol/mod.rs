//! # Protocol System
//!
//! This module provides a type-level approach to designing, verifying,
//! and implementing communication protocols using session types.
//!
//! ## Module Structure
//!
//! The implementation follows Task 1.1 as outlined in `work/TASKS.md`:
//!
//! - **foundation**: Core types and traits (Task 1.1.1)
//! - **global**: Global protocol types representing multi-party choreography (Task 1.1.2)
//! - **local**: Local protocol types representing endpoint behavior (Task 1.1.3)
//! - **duality**: Duality checking and validation (Task 1.1.4)
//! - **projection**: Protocol projection from global to local (Task 1.1.5)
//!
//! ## Key Concepts
//!
//! - **Global Protocols**: Describe the overall choreography between participants
//! - **Local Protocols**: Describe the behavior of a single participant  
//! - **Projection**: The process of deriving local protocols from global ones
//! - **Duality**: Checking compatibility between protocol types
//! - **Type-Level Operations**: Compile-time reasoning about protocol properties

// Module structure for Task 1.1 implementation
// Ready for implementation following the prompts in work/prompts/

// Foundation types (Task 1.1.1) - ✅ IMPLEMENTED
pub mod foundation;

// Global protocol types (Task 1.1.2) - ✅ IMPLEMENTED
pub mod global;

// Local endpoint types (Task 1.1.3) - ✅ IMPLEMENTED
pub mod local;

// Duality checking (Task 1.1.4) - ✅ IMPLEMENTED
pub mod duality;

// Projection implementation (Task 1.1.5) - ✅ IMPLEMENTED
pub mod projection;

// Protocol introspection for diagram generation (Task 3.5.2a) - ✅ IMPLEMENTED
pub mod introspection;

// Re-exports will be added as modules are implemented
// pub use foundation::*;
// pub use global::*;
// pub use local::*;
// pub use duality::*;
// pub use projection::*;
