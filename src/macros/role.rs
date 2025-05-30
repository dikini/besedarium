//! Role definition declarative macros
//!
//! This module provides macros for defining role types with automatic
//! trait implementations for use in session type protocols.

/// Define a role type with automatic trait implementations.
///
/// This macro creates a role type and implements the necessary traits for
/// use in the session type system.
///
/// # Basic Usage
/// ```rust
/// use besedarium::define_role;
///
/// define_role!(Client);
/// define_role!(Server);
/// ```
///
/// # With Custom Display Name
/// ```rust
/// use besedarium::define_role;
///
/// define_role!(DatabaseServer, "Database Server");
/// ```
#[macro_export]
macro_rules! define_role {
    ($role:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $role;

        impl $crate::protocol::foundation::Role for $role {}

        impl ::std::fmt::Display for $role {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", stringify!($role))
            }
        }

        impl ::std::fmt::Debug for $role {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", stringify!($role))
            }
        }

        impl ::std::default::Default for $role {
            fn default() -> Self {
                $role
            }
        }
    };

    ($role:ident, $display_name:expr) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $role;

        impl $crate::protocol::foundation::Role for $role {}

        impl ::std::fmt::Display for $role {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", $display_name)
            }
        }

        impl ::std::fmt::Debug for $role {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", $display_name)
            }
        }

        impl ::std::default::Default for $role {
            fn default() -> Self {
                $role
            }
        }
    };
}
