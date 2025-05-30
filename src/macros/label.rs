//! Label-related declarative macros
//!
//! This module provides macros for implementing traits on message label types,
//! which are core to the session type system's message identification.

/// Helper macro for implementing Display trait on label types.
#[macro_export]
macro_rules! impl_label_display {
    ($label:ident) => {
        impl ::std::fmt::Display for $label {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", stringify!($label))
            }
        }
    };
}

/// Enhanced macro for implementing standard traits on message label types.
///
/// This macro provides automatic implementations of the core traits required
/// for message labels in the session type system.
///
/// # Usage
/// ```rust
/// use besedarium::impl_traits_for_label;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct MyLabel;
/// impl_traits_for_label!(MyLabel);
/// ```
#[macro_export]
macro_rules! impl_traits_for_label {
    ($label:ident) => {
        impl $crate::protocol::foundation::MsgLbl for $label {}
        $crate::impl_label_display!($label);
    };
}
