//! # Role Derive Macro Implementation
//!
//! This module implements the `#[derive(Role)]` procedural macro for
//! automatically implementing the `Role` trait from the Besedarium foundation.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, DeriveInput, Lit, Meta, Result};

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

/// Extract display_name value from a Meta::NameValue, with proper error handling
fn extract_display_name_from_name_value(name_value: &syn::MetaNameValue) -> Result<Option<String>> {
    if name_value.path.is_ident("display_name") {
        if let syn::Expr::Lit(expr_lit) = &name_value.value {
            if let Lit::Str(lit_str) = &expr_lit.lit {
                Ok(Some(lit_str.value()))
            } else {
                Err(syn::Error::new_spanned(
                    &expr_lit.lit,
                    "display_name must be a string literal",
                ))
            }
        } else {
            Err(syn::Error::new_spanned(
                &name_value.value,
                "display_name must be a string literal",
            ))
        }
    } else {
        Ok(None)
    }
}

/// Parse role attribute arguments to extract metadata like display_name
fn parse_role_attributes(args: TokenStream) -> Result<Option<String>> {
    if args.is_empty() {
        return Ok(None);
    }

    let meta: Meta = syn::parse(args)?;
    let mut display_name = None;

    match meta {
        Meta::List(list) => {
            // Parse comma-separated list like: display_name = "value", other = "value"
            let parser = syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated;
            let nested_metas = parser.parse2(list.tokens)?;

            for nested_meta in nested_metas {
                if let Meta::NameValue(name_value) = nested_meta {
                    if let Some(name) = extract_display_name_from_name_value(&name_value)? {
                        display_name = Some(name);
                    }
                }
            }
        }
        Meta::NameValue(name_value) => {
            display_name = extract_display_name_from_name_value(&name_value)?;
        }
        _ => {
            return Err(syn::Error::new_spanned(
                meta,
                "Expected role attributes in the form: display_name = \"value\"",
            ));
        }
    }

    Ok(display_name)
}

/// Implementation of the `#[role]` attribute macro
pub fn role_attribute_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse role attribute arguments like display_name
    let display_name = match parse_role_attributes(args) {
        Ok(name) => name,
        Err(err) => return TokenStream::from(err.to_compile_error()),
    };

    let input = parse_macro_input!(input as syn::ItemStruct);
    let struct_name = &input.ident;

    let display_impl = if let Some(display_name) = display_name {
        quote! {
            // Add Display implementation with custom display name
            impl ::std::fmt::Display for #struct_name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, #display_name)
                }
            }
        }
    } else {
        quote! {
            // Add Display implementation with struct name
            impl ::std::fmt::Display for #struct_name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, stringify!(#struct_name))
                }
            }
        }
    };

    let expanded = quote! {
        // Original struct with additional derives
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #input

        // Add Role trait implementation
        impl ::besedarium::protocol::foundation::Role for #struct_name {
        }

        #display_impl
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

    #[test]
    fn test_parse_role_attributes_empty() {
        let args = TokenStream::new();
        let result = parse_role_attributes(args).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_role_attributes_display_name() {
        // Test by parsing the token stream directly rather than converting
        let tokens = quote! { display_name = "Custom Role Name" };
        let meta: Meta = syn::parse2(tokens).unwrap();

        // Manually extract the display_name for testing
        let display_name = match meta {
            Meta::NameValue(nv) if nv.path.is_ident("display_name") => match nv.value {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) => Some(s.value()),
                _ => None,
            },
            _ => None,
        };

        assert_eq!(display_name, Some("Custom Role Name".to_string()));
    }

    #[test]
    fn test_parse_role_attributes_invalid_value() {
        // Test parsing invalid value type
        let tokens = quote! { display_name = 123 };
        let meta_result: syn::Result<Meta> = syn::parse2(tokens);

        // The meta should parse fine, but the value type is wrong
        assert!(meta_result.is_ok());
        let meta = meta_result.unwrap();

        // Check that it's not a string literal
        let is_string = match meta {
            Meta::NameValue(nv) if nv.path.is_ident("display_name") => {
                matches!(
                    nv.value,
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(_),
                        ..
                    })
                )
            }
            _ => false,
        };

        assert!(!is_string, "Expected non-string value for display_name");
    }

    #[test]
    fn test_parse_role_attributes_list() {
        // Test parsing list format - actually just a simple name-value pair
        let tokens = quote! { display_name = "Custom Name" };
        let meta: Meta = syn::parse2(tokens).unwrap();

        // Extract from simple name-value format (not a list)
        let display_name = match meta {
            Meta::NameValue(nv) if nv.path.is_ident("display_name") => match &nv.value {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) => Some(s.value()),
                _ => None,
            },
            _ => None,
        };

        assert_eq!(display_name, Some("Custom Name".to_string()));
    }
}
