//! # GlobalProtocol Derive Macro Implementation
//!
//! This module implements the `#[derive(GlobalProtocol)]` procedural macro for
//! automatically implementing the `GlobalProtocol` trait from the Besedarium foundation.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Result};

use crate::utils::{basic_trait_impl, get_type_name, handle_result, is_enum, is_struct};

/// Implementation of the `#[derive(GlobalProtocol)]` macro
pub fn derive_global_protocol_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    handle_result(derive_global_protocol_inner(&input))
}

/// Internal implementation logic for the GlobalProtocol derive
fn derive_global_protocol_inner(input: &DeriveInput) -> Result<TokenStream2> {
    let type_name = get_type_name(input);

    // GlobalProtocol trait can be derived for structs and enums
    if !is_struct(input) && !is_enum(input) {
        return Err(syn::Error::new_spanned(
            input,
            "#[derive(GlobalProtocol)] can only be derived for structs and enums",
        ));
    }

    // Check for protocol-specific attributes
    let _protocol_attrs = crate::utils::parse_derive_attributes(input, "protocol");

    // Generate the GlobalProtocol trait implementation
    let trait_impl = generate_global_protocol_impl(type_name);

    Ok(trait_impl)
}

/// Generate the actual GlobalProtocol trait implementation
fn generate_global_protocol_impl(type_name: &syn::Ident) -> TokenStream2 {
    // GlobalProtocol is a marker trait with no required methods
    // It requires: Send + Sync + 'static + Debug
    let trait_path = quote! { ::besedarium::protocol::foundation::GlobalProtocol };

    basic_trait_impl(type_name, trait_path, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_global_protocol_derive_struct() {
        let input: DeriveInput = parse_quote! {
            struct SimpleProtocol;
        };

        let result = derive_global_protocol_inner(&input).unwrap();
        let expected = quote! {
            impl ::besedarium::protocol::foundation::GlobalProtocol for SimpleProtocol {
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_global_protocol_derive_enum() {
        let input: DeriveInput = parse_quote! {
            enum ProtocolState {
                Initial,
                Active,
                Terminated,
            }
        };

        let result = derive_global_protocol_inner(&input).unwrap();
        let expected = quote! {
            impl ::besedarium::protocol::foundation::GlobalProtocol for ProtocolState {
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_global_protocol_derive_complex_struct() {
        let input: DeriveInput = parse_quote! {
            struct ComplexProtocol {
                participants: Vec<String>,
                state: u32,
            }
        };

        let result = derive_global_protocol_inner(&input).unwrap();
        let expected = quote! {
            impl ::besedarium::protocol::foundation::GlobalProtocol for ComplexProtocol {
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }
}
