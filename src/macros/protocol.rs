//! Protocol definition declarative macros
//!
//! This module provides macros for basic protocol construction and
//! integration with the foundation type system.

/// Basic protocol construction helper.
///
/// This macro provides a foundation for protocol definition by creating
/// the basic structure and integrating with the foundation type system.
///
/// # Basic Usage
/// ```rust
/// use besedarium::define_protocol;
///
/// define_protocol!(MyProtocol);
/// ```
///
/// # With Description
/// ```rust
/// use besedarium::define_protocol;
///
/// define_protocol!(AuthenticationProtocol, "User authentication protocol");
/// ```
#[macro_export]
macro_rules! define_protocol {
    ($protocol:ident) => {
        #[derive(Clone, PartialEq, Eq, Hash, Debug)]
        pub struct $protocol;

        impl $crate::protocol::foundation::GlobalProtocol for $protocol {}

        impl ::std::fmt::Display for $protocol {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", stringify!($protocol))
            }
        }
    };

    ($protocol:ident, $description:expr) => {
        #[derive(Clone, PartialEq, Eq, Hash, Debug)]
        pub struct $protocol;

        impl $crate::protocol::foundation::GlobalProtocol for $protocol {}

        impl ::std::fmt::Display for $protocol {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}: {}", stringify!($protocol), $description)
            }
        }
    };
}
