//! Enhanced Declarative Macros for Protocol Definition
//!
//! This module provides a comprehensive set of declarative macros for defining
//! multiparty session type protocols, roles, messages, and labels. The macro
//! system enables clean, readable protocol specifications while ensuring
//! compile-time type safety and correctness.
//!
//! # Module Organization
//!
//! The macro infrastructure is organized into specialized modules:
//!
//! - **[`role`]**: Macros for defining protocol participants
//! - **[`message`]**: Macros for defining message types and content
//! - **[`label`]**: Macros for defining message labels and metadata
//! - **[`protocol`]**: Macros for defining global and local protocol structures
//!
//! # Integration with Core System
//!
//! The macro system works closely with the core protocol framework:
//!
//! - **[`crate::protocol::foundation`]**: Generated types implement foundation traits
//! - **[`crate::protocol::global`]**: Macros generate global protocol structures
//! - **[`crate::protocol::local`]**: Generated protocols project to local endpoints
//! - **[`crate::protocol::projection`]**: Macro-generated types support projection
//! - **[`crate::protocol::duality`]**: Generated protocols participate in duality checking
//!
//! # Advanced Features
//!
//! For derive macro functionality and attributes, see:
//! - `besedarium-derive` crate - Procedural macros for automatic trait derivation
//! - `#[derive(Role, Message, GlobalProtocol)]` - Automatic trait implementations
//! - `#[protocol]` attribute macro - Advanced protocol definition syntax
//!
//! # Integration Test Examples
//!
//! For complete working macro examples, see:
//! - `tests/derive_macros.rs` - Comprehensive derive macro testing
//! - `tests/client_server_integration.rs` - Protocols using macro-generated types
//! - `tests/integration_common.rs` - Standard macro usage patterns
//!
//! # Design Philosophy
//!
//! The macro system follows these key principles:
//!
//! ## Declarative Syntax
//!
//! Protocols are defined using intuitive, declarative syntax that closely
//! mirrors the mathematical notation of session types:
//!
//! ```rust
//! use besedarium::define_protocol;
//! use besedarium::define_role;
//! use besedarium::define_message;
//!
//! // Define protocol participants
//! define_role!(Alice);
//! define_role!(Bob);
//!
//! // Define message types
//! define_message!(Request);
//! define_message!(Response);
//!
//! // Define protocol structure (conceptual syntax)
//! // define_protocol! {
//! //     ClientServer:
//! //     Alice -> Bob: Request,
//! //     Bob -> Alice: Response,
//! //     End
//! // }
//! ```
//!
//! ## Type Safety
//!
//! All macros generate strongly-typed protocol specifications that leverage
//! Rust's type system for compile-time verification:
//!
//! - **Role Safety**: Roles are distinct types preventing confusion
//! - **Message Safety**: Messages have typed payloads
//! - **Protocol Safety**: Global protocols project to safe local types
//! - **Duality Safety**: Complementary protocols are verifiably dual
//!
//! ## Zero-Cost Abstractions
//!
//! The macro-generated code compiles to efficient runtime representations
//! with no additional overhead compared to hand-written implementations.
//!
//! # Usage Patterns
//!
//! ## Basic Protocol Definition
//!
//! ```rust
//! use besedarium::{define_role, define_message, define_protocol};
//!
//! // 1. Define participants
//! define_role!(Client);
//! define_role!(Server);
//!
//! // 2. Define message types
//! define_message!(LoginRequest { username: String, password: String });
//! define_message!(LoginResponse { success: bool, token: Option<String> });
//!
//! // 3. Define protocol structure
//! // (Syntax may vary based on specific macro implementation)
//! ```
//!
//! ## Multi-Role Protocols
//!
//! ```rust
//! use besedarium::{define_role, define_message};
//!
//! // Define multiple participants
//! define_role!(Alice);
//! define_role!(Bob);
//! define_role!(Charlie);
//!
//! // Messages can involve any combination of roles
//! define_message!(Broadcast { content: String });
//! define_message!(DirectMessage { recipient: String, content: String });
//! ```
//!
//! ## Protocol Composition
//!
//! The macro system supports building complex protocols from simpler components:
//!
//! ```rust
//! // Define reusable protocol fragments
//! // define_protocol!(Handshake: /* ... */);
//! // define_protocol!(DataExchange: /* ... */);
//! // define_protocol!(Complete: Handshake, DataExchange, End);
//! ```
//!
//! # Compile-Time Guarantees
//!
//! The macro system provides several compile-time guarantees:
//!
//! ## Well-Formedness
//!
//! - **Syntactic Correctness**: Invalid protocol syntax is rejected at compile time
//! - **Type Consistency**: All references to roles, messages, and labels are verified
//! - **Structural Validity**: Protocol structures follow session type rules
//!
//! ## Safety Properties
//!
//! - **Progress**: Well-formed protocols cannot deadlock
//! - **Communication Safety**: Send/receive operations are type-safe
//! - **Resource Safety**: Channels are used linearly (no double-use)
//!
//! # Integration with Core System
//!
//! The macro-generated types integrate seamlessly with the core protocol system:
//!
//! - **Projection**: Global protocols project to local endpoint types
//! - **Duality**: Complementary protocols are automatically dual-compatible
//! - **Runtime**: Generated types work with the runtime session management
//!
//! # Error Handling
//!
//! Macro errors are designed to be helpful and informative:
//!
//! - **Clear Messages**: Error messages indicate exactly what went wrong
//! - **Span Information**: Errors point to the specific problematic code
//! - **Suggestions**: Where possible, errors include fix suggestions
//!
//! # See Also
//!
//! - [`crate::protocol`] - Core protocol types that macros generate
//! - [`crate::runtime`] - Runtime system for executing macro-defined protocols
//! - [`crate::examples`] - Complete examples using the macro system

pub mod label;
pub mod message;
pub mod protocol;
pub mod role;

#[cfg(test)]
mod tests;
