//! # Message Derive Macro Implementation
//!
//! This module implements the `#[derive(Message)]` procedural macro for
//! automatically implementing the `Message` trait from the Besedarium foundation.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Result};

use crate::utils::{basic_trait_impl, get_type_name, handle_result, is_enum, is_struct};

/// Implementation of the `#[derive(Message)]` macro
pub fn derive_message_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    handle_result(derive_message_inner(&input))
}

/// Internal implementation logic for the Message derive
fn derive_message_inner(input: &DeriveInput) -> Result<TokenStream2> {
    let type_name = get_type_name(input);

    // Message trait can be derived for structs and enums
    if !is_struct(input) && !is_enum(input) {
        return Err(syn::Error::new_spanned(
            input,
            "#[derive(Message)] can only be derived for structs and enums",
        ));
    }

    // Check for message-specific attributes
    let _message_attrs = crate::utils::parse_derive_attributes(input, "message");

    // Generate the Message trait implementation
    let trait_impl = generate_message_impl(type_name);

    Ok(trait_impl)
}

/// Generate the actual Message trait implementation
fn generate_message_impl(type_name: &syn::Ident) -> TokenStream2 {
    // Message is a marker trait with no required methods
    // It requires: Send + Sync + 'static + Debug + Clone
    let trait_path = quote! { ::besedarium::protocol::foundation::Message };

    basic_trait_impl(type_name, trait_path, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_message_derive_struct() {
        let input: DeriveInput = parse_quote! {
            struct LoginRequest {
                username: String,
                password: String,
            }
        };

        let result = derive_message_inner(&input).unwrap();
        let expected = quote! {
            impl ::besedarium::protocol::foundation::Message for LoginRequest {
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_message_derive_enum() {
        let input: DeriveInput = parse_quote! {
            enum Response {
                Success(String),
                Error(u32),
            }
        };

        let result = derive_message_inner(&input).unwrap();
        let expected = quote! {
            impl ::besedarium::protocol::foundation::Message for Response {
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_message_derive_unit_struct() {
        let input: DeriveInput = parse_quote! {
            struct Ping;
        };

        let result = derive_message_inner(&input).unwrap();
        let expected = quote! {
            impl ::besedarium::protocol::foundation::Message for Ping {
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }
}
