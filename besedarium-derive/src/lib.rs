//! # Besedarium Derive Macros
//!
//! This crate provides procedural derive macros for the Besedarium MPST library,
//! offering ergonomic `#[derive(...)]` syntax for protocol types.
//!
//! ## Available Derive Macros
//!
//! - `#[derive(Message)]` - Automatic implementation of the `Message` trait
//! - `#[derive(Role)]` - Automatic implementation of the `Role` trait
//! - `#[derive(MsgLbl)]` - Automatic implementation of the `MsgLbl` trait
//! - `#[derive(GlobalProtocol)]` - Basic protocol trait derivation
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! besedarium = "0.1"
//! besedarium-derive = "0.1"
//! ```
//!
//! Then use the derive macros:
//!
//! ```rust,ignore
//! use besedarium::Message;
//! use besedarium_derive::Message;
//!
//! #[derive(Message)]
//! struct LoginRequest {
//!     username: String,
//!     password: String,
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;

mod diagram_generation;
mod dual_generation;
mod label;
mod message;
mod protocol;
mod role;
mod utils;

#[cfg(test)]
mod dual_integration_tests;
#[cfg(test)]
mod protocol_tests;

/// Derive macro for the `Message` trait
///
/// This automatically implements the `Message` trait for structs and enums.
/// The `Message` trait is a marker trait that indicates the type can be used
/// as a message in protocol communications.
///
/// # Example
///
/// ```rust,ignore
/// use besedarium_derive::Message;
///
/// #[derive(Message)]
/// struct LoginRequest {
///     username: String,
///     password: String,
/// }
/// ```
#[proc_macro_derive(Message, attributes(message))]
pub fn derive_message(input: TokenStream) -> TokenStream {
    message::derive_message_impl(input)
}

/// Derive macro for the `Role` trait
///
/// This automatically implements the `Role` trait for structs and enums.
/// The `Role` trait is used for identifying participants in protocols.
///
/// # Example
///
/// ```rust,ignore
/// use besedarium_derive::Role;
///
/// #[derive(Role)]
/// struct Client;
///
/// #[derive(Role)]
/// #[role(display_name = "Authentication Server")]
/// struct AuthServer;
/// ```
#[proc_macro_derive(Role, attributes(role))]
pub fn derive_role(input: TokenStream) -> TokenStream {
    role::derive_role_impl(input)
}

/// Derive macro for the `MsgLbl` trait
///
/// This automatically implements the `MsgLbl` trait for types used as
/// message labels in protocol communications.
///
/// # Example
///
/// ```rust,ignore
/// use besedarium_derive::MsgLbl;
///
/// #[derive(MsgLbl)]
/// struct RequestLabel;
///
/// #[derive(MsgLbl)]
/// struct ResponseLabel;
/// ```
#[proc_macro_derive(MsgLbl, attributes(label))]
pub fn derive_msg_lbl(input: TokenStream) -> TokenStream {
    label::derive_msg_lbl_impl(input)
}

/// Derive macro for the `GlobalProtocol` trait
///
/// This automatically implements the `GlobalProtocol` trait for types
/// representing global protocol choreography.
///
/// # Example
///
/// ```rust,ignore
/// use besedarium_derive::GlobalProtocol;
///
/// #[derive(GlobalProtocol)]
/// struct SimpleProtocol;
/// ```
#[proc_macro_derive(GlobalProtocol, attributes(protocol))]
pub fn derive_global_protocol(input: TokenStream) -> TokenStream {
    protocol::derive_global_protocol_impl(input)
}

/// Attribute macro for defining protocol specifications
///
/// This macro transforms user-friendly protocol specifications into
/// underlying session type constructs. It supports role declarations,
/// message flows, and property specifications.
///
/// # Example
///
/// ```rust,ignore
/// use besedarium_derive::protocol;
///
/// #[protocol(io = "sync", metadata = "standard")]
/// /// protocol SimpleAuth {
/// ///     roles: Client, Server;
/// ///     Client -> Server: Login(username: String, password: String);
/// ///     Server -> Client: LoginResponse(success: bool);
/// /// }
/// struct SimpleAuth;
/// ```
#[proc_macro_attribute]
pub fn protocol(args: TokenStream, input: TokenStream) -> TokenStream {
    protocol::protocol_attribute_impl(args, input)
}

/// Attribute macro for enhanced role specification
///
/// This macro provides enhanced role specification with metadata
/// and additional capabilities.
///
/// # Example
///
/// ```rust,ignore
/// use besedarium_derive::role;
///
/// #[role(display_name = "Client Endpoint")]
/// struct Client;
/// ```
#[proc_macro_attribute]
pub fn role(args: TokenStream, input: TokenStream) -> TokenStream {
    role::role_attribute_impl(args, input)
}

/// Attribute macro for endpoint behavior specification
///
/// This macro specifies endpoint behavior and adds metadata
/// for runtime configuration.
///
/// # Example
///
/// ```rust,ignore
/// use besedarium_derive::endpoint;
///
/// #[endpoint(timeout = 5000, retry_count = 3)]
/// fn handle_connection() -> Result<(), Error> {
///     // endpoint implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn endpoint(args: TokenStream, input: TokenStream) -> TokenStream {
    protocol::endpoint_attribute_impl(args, input)
}

/// Attribute macro for session type annotations
///
/// This macro adds session type metadata and validation
/// for type aliases and session type definitions.
///
/// # Example
///
/// ```rust,ignore
/// use besedarium_derive::session_type;
///
/// #[session_type(validate = true, duality_check = true, role = "Client")]
/// type ClientSession = TSend<Message, TRecv<Response, TEnd>>;
/// ```
#[proc_macro_attribute]
pub fn session_type(args: TokenStream, input: TokenStream) -> TokenStream {
    protocol::session_type_attribute_impl(args, input)
}

/// Derive macro for automatic diagram generation from protocol definitions
///
/// This automatically implements the `ProtocolFlow` trait and generates
/// Mermaid sequence diagrams from protocol structure.
///
/// # Example
///
/// ```rust,ignore
/// use besedarium_derive::{GlobalProtocol, GenerateDiagram};
///
/// #[derive(GlobalProtocol, GenerateDiagram)]
/// #[protocol(roles = "Client, Server")]
/// struct SimpleProtocol;
/// ```
#[proc_macro_derive(GenerateDiagram, attributes(protocol))]
pub fn derive_generate_diagram(input: TokenStream) -> TokenStream {
    protocol::derive_generate_diagram_impl(input)
}
