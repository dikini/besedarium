//! Message definition declarative macros
//!
//! This module provides macros for defining message types with automatic
//! trait implementations and serialization support.

/// Define a message type with automatic trait implementations.
///
/// This macro creates a message type and implements the necessary traits for
/// use in the session type system, including serialization support.
///
/// # Simple Message (No Fields)
/// ```rust
/// use besedarium::define_message;
/// 
/// define_message!(Ping);
/// define_message!(Pong);
/// ```
///
/// # Message with Fields
/// ```rust
/// use besedarium::define_message;
/// 
/// define_message!(Login {
///     username: String,
///     password: String,
/// });
/// ```
///
/// # Message with Optional Fields
/// ```rust
/// use besedarium::define_message;
/// 
/// define_message!(UserInfo {
///     id: u64,
///     name: String,
///     email: Option<String>,
/// });
/// ```
#[macro_export]
macro_rules! define_message {
    // Simple message without fields
    ($message:ident) => {
        #[derive(Clone, PartialEq, Eq, Hash, Debug)]
        pub struct $message;
        
        impl $crate::protocol::foundation::Message for $message {}
        
        impl ::std::fmt::Display for $message {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", stringify!($message))
            }
        }
        
        impl ::std::default::Default for $message {
            fn default() -> Self {
                $message
            }
        }
    };
    
    // Message with fields
    ($message:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        #[derive(Clone, PartialEq, Eq, Hash, Debug)]
        pub struct $message {
            $(pub $field: $field_type,)*
        }
        
        impl $crate::protocol::foundation::Message for $message {}
        
        impl ::std::fmt::Display for $message {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", stringify!($message))
            }
        }
    };
}

/// Define multiple message types at once.
///
/// This macro allows for batch definition of message types, supporting both
/// simple messages and messages with fields.
///
/// # Simple Messages
/// ```rust
/// use besedarium::messages;
/// 
/// messages!(
///     Start,
///     Stop,
///     Restart,
/// );
/// ```
///
/// # Mixed Messages
/// ```rust
/// use besedarium::messages;
/// 
/// messages!(
///     Ping,
///     Pong,
///     Login {
///         username: String,
///         password: String,
///     },
///     ErrorMessage {
///         code: i32,
///         description: String,
///     },
/// );
/// ```
#[macro_export]
macro_rules! messages {
    (
        $(
            $message:ident $({ $($field:ident: $field_type:ty),* $(,)? })?
        ),*
        $(,)?
    ) => {
        $(
            $crate::define_message!($message $({ $($field: $field_type),* })?);
        )*
    };
}
