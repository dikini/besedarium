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

mod label;
mod message;
mod protocol;
mod role;
mod utils;

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
