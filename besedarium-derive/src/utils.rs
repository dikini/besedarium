//! # Utility Functions for Derive Macros
//!
//! This module provides common utilities and helper functions used across
//! all derive macro implementations.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Result};

/// Extract the type name from a DeriveInput
pub fn get_type_name(input: &DeriveInput) -> &syn::Ident {
    &input.ident
}

/// Check if the input is a struct
pub fn is_struct(input: &DeriveInput) -> bool {
    matches!(input.data, Data::Struct(_))
}

/// Check if the input is an enum
pub fn is_enum(input: &DeriveInput) -> bool {
    matches!(input.data, Data::Enum(_))
}

/// Check if the input is a unit struct (no fields)
#[allow(dead_code)]
pub fn is_unit_struct(input: &DeriveInput) -> bool {
    if let Data::Struct(ref data) = input.data {
        matches!(data.fields, Fields::Unit)
    } else {
        false
    }
}

/// Check if the input is a struct with named fields
#[allow(dead_code)]
pub fn is_named_struct(input: &DeriveInput) -> bool {
    if let Data::Struct(ref data) = input.data {
        matches!(data.fields, Fields::Named(_))
    } else {
        false
    }
}

/// Generate an error for unsupported input types
#[allow(dead_code)]
pub fn unsupported_type_error(input: &DeriveInput, trait_name: &str) -> Error {
    Error::new_spanned(
        input,
        format!("#{trait_name} can only be derived for structs and enums"),
    )
}

/// Generate an error for unsupported struct types
#[allow(dead_code)]
pub fn unsupported_struct_error(input: &DeriveInput, trait_name: &str) -> Error {
    Error::new_spanned(
        input,
        format!("#{trait_name} does not support this struct type"),
    )
}

/// Create a basic trait implementation with no additional methods
pub fn basic_trait_impl(
    type_name: &syn::Ident,
    trait_path: TokenStream2,
    additional_items: Option<TokenStream2>,
) -> TokenStream2 {
    let items = additional_items.unwrap_or_else(|| quote! {});

    quote! {
        impl #trait_path for #type_name {
            #items
        }
    }
}

/// Convert a Result<TokenStream2> to TokenStream, handling errors
pub fn handle_result(result: Result<TokenStream2>) -> TokenStream {
    match result {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Parse derive attributes for custom configurations
pub fn parse_derive_attributes(input: &DeriveInput, attr_name: &str) -> Vec<syn::Attribute> {
    input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident(attr_name))
        .cloned()
        .collect()
}
