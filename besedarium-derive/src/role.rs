//! # Role Derive Macro Implementation
//!
//! This module implements the `#[derive(Role)]` procedural macro for
//! automatically implementing the `Role` trait from the Besedarium foundation.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Result};

use crate::utils::{basic_trait_impl, get_type_name, handle_result, is_enum, is_struct};

/// Implementation of the `#[derive(Role)]` macro
pub fn derive_role_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    handle_result(derive_role_inner(&input))
}

/// Internal implementation logic for the Role derive
fn derive_role_inner(input: &DeriveInput) -> Result<TokenStream2> {
    let type_name = get_type_name(input);

    // Role trait can be derived for structs and enums
    if !is_struct(input) && !is_enum(input) {
        return Err(syn::Error::new_spanned(
            input,
            "#[derive(Role)] can only be derived for structs and enums",
        ));
    }

    // Check for role-specific attributes
    let _role_attrs = crate::utils::parse_derive_attributes(input, "role");

    // Generate the Role trait implementation
    let trait_impl = generate_role_impl(type_name);

    Ok(trait_impl)
}

/// Generate the actual Role trait implementation
fn generate_role_impl(type_name: &syn::Ident) -> TokenStream2 {
    // Role is a marker trait with no required methods
    // It requires: Send + Sync + 'static + Debug + Clone + PartialEq + Eq + Hash
    let trait_path = quote! { ::besedarium::protocol::foundation::Role };

    basic_trait_impl(type_name, trait_path, None)
}

/// Implementation of the `#[role]` attribute macro
pub fn role_attribute_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    // For now, implement a placeholder that passes through the original struct
    // and adds enhanced role metadata
    let _args = args; // TODO: Parse role attributes like display_name
    let input = parse_macro_input!(input as syn::ItemStruct);
    
    let struct_name = &input.ident;
    let expanded = quote! {
        // Original struct with additional derives
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #input
        
        // Add Role trait implementation
        impl ::besedarium::protocol::foundation::Role for #struct_name {
        }
        
        // Add Display implementation
        impl ::std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, stringify!(#struct_name))
            }
        }
        
        // TODO: Parse display_name from attributes and other metadata
    };
    
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_role_derive_struct() {
        let input: DeriveInput = parse_quote! {
            struct Client;
        };

        let result = derive_role_inner(&input).unwrap();
        let expected = quote! {
            impl ::besedarium::protocol::foundation::Role for Client {
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_role_derive_enum() {
        let input: DeriveInput = parse_quote! {
            enum Participant {
                Alice,
                Bob,
                Charlie,
            }
        };

        let result = derive_role_inner(&input).unwrap();
        let expected = quote! {
            impl ::besedarium::protocol::foundation::Role for Participant {
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_role_derive_named_struct() {
        let input: DeriveInput = parse_quote! {
            struct Server {
                name: String,
            }
        };

        let result = derive_role_inner(&input).unwrap();
        let expected = quote! {
            impl ::besedarium::protocol::foundation::Role for Server {
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }
}
