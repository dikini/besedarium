#![doc = include_str!("../README.md")]

//! # Session Types Playground
//!
//! Welcome to the Session Types Playground! This crate lets you build, compose, and verify communication protocols at the type level in Rust.
//!
//! - **Catch protocol mistakes early:** Get compile-time errors for protocol mismatches.
//! - **Readable and reusable:** Protocols are just Rust types—easy to read, share, and reuse.
//! - **Great for learning:** See real-world protocol examples in `tests/protocols/`.
//!
//! ## Main Concepts
//! - **Session combinators:** Compose protocols from simple building blocks.
//! - **Macros:** Ergonomic construction of n-ary choices and parallel branches.
//! - **Disjointness checks:** Ensure parallel branches do not overlap roles.
//!
//! ## Safety Guarantees
//! - Protocols are checked at compile time.
//! - Parallel branches must be disjoint (no overlapping roles).
//! - Macros and traits prevent invalid protocol construction.
//!
//! ## See Also
//! - Protocol examples: `tests/protocols/`
//! - Negative/compile-fail tests: `tests/trybuild/`
//! - More docs: `README.md`, `docs/`

//! # Projection: From Global to Local Session Types
//!
//! The projection machinery allows you to derive the local (endpoint) session type for a given role from a global protocol specification.
//!
//! ## How it works
//! - The [`ProjectRole`] trait recursively traverses a global protocol (a type implementing [`TSession`]) and produces the local protocol for a specific role.
//! - Each global combinator (`TSend`, `TRecv`, `TChoice`, `TPar`, etc.) has a corresponding endpoint type (`EpSend`, `EpRecv`, `EpChoice`, `EpPar`, etc.).
//! - Helper traits (e.g., `ProjectInteract`, `ProjectChoice`, `ProjectPar`) are used to avoid overlapping trait impls and to dispatch on type-level booleans.
//!
//! ## Example
//! ```ignore
//! use besedarium::*;
//! struct Alice; impl Role for Alice {}; impl ProtocolLabel for Alice {};
//! struct Bob; impl Role for Bob {}; impl ProtocolLabel for Bob {};
//! impl RoleEq<Alice> for Alice { type Output = True; }
//! impl RoleEq<Bob> for Alice { type Output = False; }
//! impl RoleEq<Alice> for Bob { type Output = False; }
//! impl RoleEq<Bob> for Bob { type Output = True; }
//! struct L; impl ProtocolLabel for L {}
//! type Global = TSend<Http, L, Alice, Message, TRecv<Http, L, Bob, Response, TEnd<Http, L>>>;
//! type AliceLocal = <() as ProjectRole<Alice, Http, Global>>::Out;
//! type BobLocal = <() as ProjectRole<Bob, Http, Global>>::Out;
//! ```
//!
//! See the README and protocol examples for more details.
#[macro_export]
macro_rules! tlist {
    () => { Nil };
    ($head:ty $(, $tail:ty )* $(,)?) => {
        Cons<$head, tlist!($($tail),*)>
    };
}

/// Macro for building n-ary protocol choices.
///
/// # Example
/// ```rust
/// use besedarium::*;
/// struct L1; impl ProtocolLabel for L1 {}
/// struct L2; impl ProtocolLabel for L2 {}
/// type Choice = tchoice!(Http;
///     TSend<Http, L1, TClient, Message, TEnd<Http, L1>>,
///     TRecv<Http, L2, TServer, Response, TEnd<Http, L2>>,
/// );
/// ```
#[macro_export]
macro_rules! tchoice {
    ($io:ty; $($branch:ty),+ $(,)?) => {
        <tlist!($($branch),*) as ToTChoice<$io>>::Output
    };
}

/// Macro for building n-ary protocol parallel compositions.
///
/// # Example
/// ```rust
/// use besedarium::*;
/// struct L1; impl ProtocolLabel for L1 {}
/// struct L2; impl ProtocolLabel for L2 {}
/// type Par = tpar!(Http;
///     TSend<Http, L1, TClient, Message, TEnd<Http, L1>>,
///     TRecv<Http, L2, TServer, Response, TEnd<Http, L2>>,
/// );
/// ```
#[macro_export]
macro_rules! tpar {
    ($io:ty; $($branch:ty),* $(,)?) => {
        <tlist!($($branch),*) as ToTPar<$io>>::Output
    };
}

#[macro_export]
macro_rules! assert_type_eq {
    ($A:ty, $B:ty) => {
        const _: fn() = || {
            fn _assert_type_eq()
            where
                $A: $crate::TypeEq<$B>,
            {
            }
        };
    };
}

#[macro_export]
macro_rules! assert_disjoint {
    ($A:ty, $B:ty) => {
        const _: fn() = || {
            fn _assert_disjoint()
            where
                (): $crate::Disjoint<
                    <$A as $crate::RolesOf>::Roles,
                    <$B as $crate::RolesOf>::Roles,
                >,
            {
            }
        };
    };
    (par $TPar:ty) => {
        type _Checked = <$TPar as $crate::AssertDisjoint>::Output;
    };
}

/// Macro to extract the set of roles from a protocol type as a type-level list.
///
/// # Example
/// ```rust
/// use besedarium::*;
/// struct L; impl ProtocolLabel for L {}
/// type Roles = extract_roles!(TSend<Http, L, TClient, Message, TEnd<Http, L>>);
/// ```
#[macro_export]
macro_rules! extract_roles {
    ($T:ty) => {
        <$T as $crate::RolesOf>::Roles
    };
}

#[macro_export]
macro_rules! assert_unique_labels {
    ($T:ty) => {
        const _: fn() = || {
            fn _assert_unique_labels()
            where
                <$T as $crate::LabelsOf>::Labels: $crate::UniqueList,
            {
            }
        };
    };
}

/// ## Compile-time Label Uniqueness Assertion
///
/// To ensure that all protocol labels are unique (no duplicates), use the [`assert_unique_labels!`] macro:
///
/// ```rust
/// use besedarium::*;
/// struct MyLabel1; impl ProtocolLabel for MyLabel1 {}
/// struct MyLabel2; impl ProtocolLabel for MyLabel2 {}
/// type MyProtocol = TChoice<
///     Http,
///     MyLabel1,
///     TSend<Http, MyLabel1, TClient, Message, TEnd<Http, MyLabel1>>,
///     TRecv<Http, MyLabel2, TServer, Response, TEnd<Http, MyLabel2>>
/// >;
/// assert_unique_labels!(MyProtocol); // Compile-time error if labels are not unique
/// ```
pub(crate) mod sealed {
    pub trait Sealed {}
}

// Update protocol module reference to use the directory module
mod protocol;
pub use protocol::*;
mod introspection;
mod types;
pub use types::*;

// Re-export key introspection traits
pub use introspection::{LabelsOf, RolesOf};

// Note: Most protocol types are now re-exported via protocol/mod.rs
// so we don't need to repeat those here.

// Re-export canonical type-level booleans from types
pub use types::{Bool, False, True};
