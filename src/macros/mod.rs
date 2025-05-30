//! Enhanced Declarative Macros for Protocol Definition
//!
//! This module provides a comprehensive set of declarative macros organized
//! into logical sub-modules for different aspects of protocol definition.

pub mod label;
pub mod role;
pub mod message;
pub mod protocol;

// Re-export all macros for convenience
pub use label::*;
pub use role::*;
pub use message::*;
pub use protocol::*;

#[cfg(test)]
mod tests;
