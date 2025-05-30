//! # MsgLbl Derive Macro Implementation
//!
//! This module implements the `#[derive(MsgLbl)]` procedural macro for
//! automatically implementing the `MsgLbl` trait from the Besedarium foundation.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Result};

use crate::utils::{basic_trait_impl, get_type_name, handle_result, is_enum, is_struct};

/// Implementation of the `#[derive(MsgLbl)]` macro
pub fn derive_msg_lbl_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    handle_result(derive_msg_lbl_inner(&input))
}

/// Internal implementation logic for the MsgLbl derive
fn derive_msg_lbl_inner(input: &DeriveInput) -> Result<TokenStream2> {
    let type_name = get_type_name(input);

    // MsgLbl trait can be derived for structs and enums
    if !is_struct(input) && !is_enum(input) {
        return Err(syn::Error::new_spanned(
            input,
            "#[derive(MsgLbl)] can only be derived for structs and enums",
        ));
    }

    // Check for label-specific attributes
    let _label_attrs = crate::utils::parse_derive_attributes(input, "label");

    // Generate the MsgLbl trait implementation
    let trait_impl = generate_msg_lbl_impl(type_name);

    Ok(trait_impl)
}

/// Generate the actual MsgLbl trait implementation
fn generate_msg_lbl_impl(type_name: &syn::Ident) -> TokenStream2 {
    // MsgLbl is a marker trait with no required methods
    // It requires: Send + Sync + 'static + Debug + Clone + PartialEq + Eq + Hash
    let trait_path = quote! { ::besedarium::protocol::foundation::MsgLbl };

    basic_trait_impl(type_name, trait_path, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_msg_lbl_derive_struct() {
        let input: DeriveInput = parse_quote! {
            struct RequestLabel;
        };

        let result = derive_msg_lbl_inner(&input).unwrap();
        let expected = quote! {
            impl ::besedarium::protocol::foundation::MsgLbl for RequestLabel {
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_msg_lbl_derive_enum() {
        let input: DeriveInput = parse_quote! {
            enum ProtocolLabel {
                Request,
                Response,
                Error,
            }
        };

        let result = derive_msg_lbl_inner(&input).unwrap();
        let expected = quote! {
            impl ::besedarium::protocol::foundation::MsgLbl for ProtocolLabel {
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_msg_lbl_derive_named_struct() {
        let input: DeriveInput = parse_quote! {
            struct CustomLabel {
                id: u32,
                name: String,
            }
        };

        let result = derive_msg_lbl_inner(&input).unwrap();
        let expected = quote! {
            impl ::besedarium::protocol::foundation::MsgLbl for CustomLabel {
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }
}
