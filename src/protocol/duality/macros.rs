//! Compile-time assertion macros for duality checking
//!
//! This module provides macros for compile-time verification of duality
//! relationships between protocol types.

/// Compile-time assertion that two types are dual
///
/// This macro generates a compile-time check that will fail if the two types
/// are not provably dual according to the `IsDual` trait implementations.
///
/// # Examples
///
/// ```ignore
/// assert_dual!(
///     TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>,
///     TChanRecv<Bob, Alice, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>
/// );
/// ```
#[macro_export]
macro_rules! assert_dual {
    ($P:ty, $Q:ty) => {
        const _: () = {
            fn _assert_dual()
            where
                (): $crate::protocol::duality::IsDual<$P, $Q>,
                <() as $crate::protocol::duality::IsDual<$P, $Q>>::Output:
                    $crate::protocol::duality::EqualsTrue,
            {
            }
        };
    };
}

/// Compile-time assertion that two types are NOT dual
///
/// This macro generates a compile-time check that will fail if the two types
/// are provably dual according to the `IsDual` trait implementations.
///
/// # Examples
///
/// ```ignore
/// assert_not_dual!(
///     TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>,
///     TChanSend<Bob, Alice, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>
/// );
/// ```
#[macro_export]
macro_rules! assert_not_dual {
    ($P:ty, $Q:ty) => {
        const _: () = {
            fn _assert_not_dual()
            where
                (): $crate::protocol::duality::IsDual<$P, $Q>,
                <() as $crate::protocol::duality::IsDual<$P, $Q>>::Output:
                    $crate::protocol::duality::EqualsFalse,
            {
            }
        };
    };
}
