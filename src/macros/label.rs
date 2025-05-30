//! Label-related declarative macros
//!
//! This module provides macros for implementing traits on message label types,
//! which are core to the session type system's message identification.

/// Enhanced macro for implementing standard traits on message label types.
///
/// This macro provides automatic implementations of the core traits required
/// for message labels in the session type system.
///
/// # Basic Usage
/// ```rust
/// use besedarium::impl_traits_for_label;
///
/// struct MyLabel;
/// impl_traits_for_label!(MyLabel);
/// ```
///
/// # Custom Dual Usage
/// ```rust
/// use besedarium::impl_traits_for_label;
///
/// struct RequestLabel;
/// struct ResponseLabel;
///
/// impl_traits_for_label!(RequestLabel, ResponseLabel);
/// impl_traits_for_label!(ResponseLabel, RequestLabel);
/// ```
#[macro_export]
macro_rules! impl_traits_for_label {
    // Standard implementation - label is its own dual
    ($label:ident) => {
        impl $crate::protocol::foundation::MsgLbl for $label {}

        impl ::std::fmt::Display for $label {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", stringify!($label))
            }
        }
    };

    // Custom dual implementation
    ($label:ident, $dual:ident) => {
        impl $crate::protocol::foundation::MsgLbl for $label {}

        impl ::std::fmt::Display for $label {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", stringify!($label))
            }
        }
    };
}
