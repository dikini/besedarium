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
//! ## Example: Client-Server Handshake
//! ```rust
//! use playground::*;
//! type Handshake = TInteract<Http, TClient, Message, TInteract<Http, TServer, Response, TEnd<Http>>>;
//! ```
//!
//! ## Example: N-ary Choice
//! ```rust
//! use playground::*;
//! type Choice = tchoice!(Http;
//!     TInteract<Http, TClient, Message, TEnd<Http>>,
//!     TInteract<Http, TServer, Response, TEnd<Http>>,
//! );
//! ```
//!
//! ## Example: Parallel Composition
//! ```rust
//! use playground::*;
//! type Par = tpar!(Http;
//!     TInteract<Http, TClient, Message, TEnd<Http>>,
//!     TInteract<Http, TServer, Response, TEnd<Http>>,
//! );
//! ```
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

/// # Projection: From Global to Local Session Types
///
/// The projection machinery allows you to derive the local (endpoint) session type for a given role from a global protocol specification.
///
/// ## How it works
/// - The [`ProjectRole`] trait recursively traverses a global protocol (a type implementing [`TSession`]) and produces the local protocol for a specific role.
/// - Each global combinator (`TInteract`, `TChoice`, `TPar`, etc.) has a corresponding endpoint type (`EpSend`, `EpRecv`, `EpChoice`, `EpPar`, etc.).
/// - Helper traits (e.g., `ProjectInteract`, `ProjectChoice`, `ProjectPar`) are used to avoid overlapping trait impls and to dispatch on type-level booleans.
///
/// ## Example
/// ```rust
/// use besedarium::*;
/// struct Alice;
/// struct Bob;
/// impl Role for Alice {}
/// impl Role for Bob {}
/// impl RoleEq<Alice> for Alice { type Output = True; }
/// impl RoleEq<Bob> for Alice { type Output = False; }
/// impl RoleEq<Alice> for Bob { type Output = False; }
/// impl RoleEq<Bob> for Bob { type Output = True; }
///
/// type Global = TInteract<Http, Alice, Message, TInteract<Http, Bob, Response, TEnd<Http>>>;
/// type AliceLocal = <() as ProjectRole<Alice, Http, Global>>::Out;
/// type BobLocal = <() as ProjectRole<Bob, Http, Global>>::Out;
/// ```
///
/// See the README and protocol examples for more details.
use core::marker::PhantomData;

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
/// type Choice = tchoice!(Http;
///     TInteract<Http, TClient, Message, TEnd<Http>>,
///     TInteract<Http, TServer, Response, TEnd<Http>>,
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
/// type Par = tpar!(Http;
///     TInteract<Http, TClient, Message, TEnd<Http>>,
///     TInteract<Http, TServer, Response, TEnd<Http>>,
/// );
/// ```
#[macro_export]
macro_rules! tpar {
    ($io:ty; $($branch:ty),+ $(,)?) => {
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
/// type Roles = extract_roles!(TInteract<Http, TClient, Message, TEnd<Http>>);
/// ```
#[macro_export]
macro_rules! extract_roles {
    ($T:ty) => {
        <$T as $crate::RolesOf>::Roles
    };
}

pub(crate) mod sealed {
    pub trait Sealed {}
}

use sealed::Sealed;

// Type-level list for n-ary combinators
pub struct Nil;
pub struct Cons<H, T>(PhantomData<(H, T)>);

pub trait TSession<IO>: Sealed {
    type Compose<Rhs: TSession<IO>>: TSession<IO>;
    const IS_EMPTY: bool;
}

/// Marker for the end of a protocol session.
///
/// This type is used to indicate that a protocol branch or sequence has finished.
///
/// # Example
/// ```rust
/// use playground::*;
/// type End = TEnd<Http>;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TEnd<IO>(PhantomData<IO>);

impl<IO> Sealed for TEnd<IO> {}
impl<IO> TSession<IO> for TEnd<IO> {
    type Compose<Rhs: TSession<IO>> = Rhs;
    const IS_EMPTY: bool = true;
}

/// Represents a single interaction in a protocol session.
///
/// `TInteract<IO, R, H, T>` means role `R` sends or receives message `H` over IO, then continues as `T`.
///
/// # Example
/// ```rust
/// use playground::*;
/// type Step = TInteract<Http, TClient, Message, TEnd<Http>>;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TInteract<IO, R, H, T: TSession<IO>>(PhantomData<(IO, R, H, T)>);

impl<IO, R, H, T: TSession<IO>> Sealed for TInteract<IO, R, H, T> {}
impl<IO, R, H, T: TSession<IO>> TSession<IO> for TInteract<IO, R, H, T> {
    type Compose<Rhs: TSession<IO>> = TInteract<IO, R, H, T::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

/// Recursive session type for repeating protocol fragments.
///
/// `TRec<IO, S>` means repeat the protocol `S` (which may refer to itself).
///
/// # Example
/// ```rust
/// use playground::*;
/// type Streaming = TRec<Http, TInteract<Http, TClient, Message, TEnd<Http>>>;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TRec<IO, S: TSession<IO>>(PhantomData<(IO, S)>);

impl<IO, S: TSession<IO>> Sealed for TRec<IO, S> {}
impl<IO, S: TSession<IO>> TSession<IO> for TRec<IO, S> {
    type Compose<Rhs: TSession<IO>> = TRec<IO, S::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

/// Binary protocol choice between two branches.
///
/// `TChoice<IO, L, R>` means the protocol can proceed as either `L` or `R`.
///
/// # Example
/// ```rust
/// use playground::*;
/// type Choice = TChoice<Http, TInteract<Http, TClient, Message, TEnd<Http>>, TInteract<Http, TServer, Response, TEnd<Http>>>;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TChoice<IO, L: TSession<IO>, R: TSession<IO>>(PhantomData<(IO, L, R)>);

impl<IO, L: TSession<IO>, R: TSession<IO>> Sealed for TChoice<IO, L, R> {}
impl<IO, L: TSession<IO>, R: TSession<IO>> TSession<IO> for TChoice<IO, L, R> {
    type Compose<Rhs: TSession<IO>> = TChoice<IO, L::Compose<Rhs>, R::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

// Map type-level list to nested TChoice
/// Trait for mapping a type-level list to a nested `TChoice`.
///
/// Used by the `tchoice!` macro for n-ary protocol branching.
///
/// # Example
/// ```rust
/// use besedarium::*;
/// type Choice = <tlist!(TInteract<Http, TClient, Message, TEnd<Http>>, TInteract<Http, TServer, Response, TEnd<Http>>) as ToTChoice<Http>>::Output;
/// ```
pub trait ToTChoice<IO> {
    type Output: TSession<IO>;
}

/// Branded parallel composition of two protocol branches.
///
/// `TPar<IO, L, R, IsDisjoint>` means run `L` and `R` in parallel, with a marker for disjointness.
///
/// # Example
/// ```rust
/// use playground::*;
/// type Par = TPar<Http, TInteract<Http, TClient, Message, TEnd<Http>>, TInteract<Http, TServer, Response, TEnd<Http>>, FalseB>;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TPar<IO, L: TSession<IO>, R: TSession<IO>, IsDisjoint>(
    PhantomData<(IO, L, R, IsDisjoint)>,
);

impl<IO, L: TSession<IO>, R: TSession<IO>, IsDisjoint> Sealed for TPar<IO, L, R, IsDisjoint> {}
impl<IO, L: TSession<IO>, R: TSession<IO>, IsDisjoint> TSession<IO> for TPar<IO, L, R, IsDisjoint> {
    type Compose<Rhs: TSession<IO>> = TPar<IO, L::Compose<Rhs>, R::Compose<Rhs>, IsDisjoint>;
    const IS_EMPTY: bool = false;
}

/// Trait for mapping a type-level list to a nested `TPar`.
///
/// Used by the `tpar!` macro for n-ary protocol parallel composition.
///
/// # Example
/// ```rust
/// use besedarium::*;
/// type Par = <tlist!(TInteract<Http, TClient, Message, TEnd<Http>>, TInteract<Http, TServer, Response, TEnd<Http>>) as ToTPar<Http>>::Output;
/// ```
pub trait ToTPar<IO> {
    type Output: TSession<IO>;
}

// Type-level booleans for branding
pub struct TrueB;
pub struct FalseB;

// --- Role Extraction Machinery ---
/// Extracts the set of roles used in a protocol type as a type-level list.
///
/// Used for disjointness checks and compile-time assertions.
///
/// # Example
/// ```rust
/// use besedarium::*;
/// type Roles = <TInteract<Http, TClient, Message, TEnd<Http>> as RolesOf>::Roles;
/// ```
pub trait RolesOf {
    type Roles;
}
impl<IO> RolesOf for TEnd<IO> {
    type Roles = Nil;
}
impl<IO, R, H, T: TSession<IO> + RolesOf> RolesOf for TInteract<IO, R, H, T> {
    type Roles = Cons<R, <T as RolesOf>::Roles>;
}
impl<IO, L: TSession<IO> + RolesOf, R: TSession<IO> + RolesOf> RolesOf for TChoice<IO, L, R> {
    type Roles = <L as RolesOf>::Roles;
}
impl<IO, L: TSession<IO> + RolesOf, R: TSession<IO> + RolesOf, IsDisjoint> RolesOf
    for TPar<IO, L, R, IsDisjoint>
{
    type Roles = <L as RolesOf>::Roles;
}
impl<IO, S: TSession<IO> + RolesOf> RolesOf for TRec<IO, S> {
    type Roles = <S as RolesOf>::Roles;
}

// --- Type-level list concatenation for roles ---
pub trait ConcatRoles<Rhs> {
    type Output;
}
impl<Rhs> ConcatRoles<Rhs> for Nil {
    type Output = Rhs;
}
impl<H, T, Rhs> ConcatRoles<Rhs> for Cons<H, T>
where
    T: ConcatRoles<Rhs>,
{
    type Output = Cons<H, <T as ConcatRoles<Rhs>>::Output>;
}

// --- Disjointness Traits ---
pub trait Contains<X> {}
impl<X> Contains<X> for Nil {}
impl<X, H, T> Contains<X> for Cons<H, T> where T: Contains<X> {}

pub trait NotContains<X> {}
impl<X> NotContains<X> for Nil {}
impl<X, H, T> NotContains<X> for Cons<H, T> where T: NotContains<X> {}

pub trait Disjoint<A, B> {}
impl<B> Disjoint<Nil, B> for () {}
impl<H, T, B> Disjoint<Cons<H, T>, B> for ()
where
    B: NotContains<H>,
    (): Disjoint<T, B>,
{
}

// --- Compile-time Disjointness Assertion Machinery ---
/// Compile-time assertion that a parallel protocol is disjoint.
///
/// Used by the `assert_disjoint!` macro to rebrand a `TPar` as disjoint.
///
/// # Example
/// ```rust
/// use besedarium::*;
/// type Checked = <TPar<Http, TInteract<Http, TClient, Message, TEnd<Http>>, TInteract<Http, TServer, Response, TEnd<Http>>, FalseB> as AssertDisjoint>::Output;
/// ```
pub trait AssertDisjoint {
    type Output;
}
impl<IO, L: TSession<IO> + RolesOf, R: TSession<IO> + RolesOf> AssertDisjoint
    for TPar<IO, L, R, FalseB>
where
    (): Disjoint<<L as RolesOf>::Roles, <R as RolesOf>::Roles>,
    (): Disjoint<<R as RolesOf>::Roles, <L as RolesOf>::Roles>,
{
    type Output = TPar<IO, L, R, TrueB>;
}
impl<IO, L: TSession<IO>, R: TSession<IO>> AssertDisjoint for TPar<IO, L, R, TrueB> {
    type Output = TPar<IO, L, R, TrueB>;
}

/// Type-level equality trait for compile-time assertions.
///
/// This trait is implemented only when `A` and `B` are the same type.
/// If you see an error involving `TypeEq`, it means the types you are comparing are not equal.
/// Double-check your type parameters and protocol structure.
pub trait TypeEq<B> {}
impl<A> TypeEq<A> for A {}

// --- Type-level list kind markers for trait overlap resolution ---
pub trait ListKind {}
pub struct NilType;
pub struct ConsType;
impl ListKind for NilType {}
impl ListKind for ConsType {}

pub trait ListKindOf {
    type Kind: ListKind;
}
impl ListKindOf for Nil {
    type Kind = NilType;
}
impl<H, T> ListKindOf for Cons<H, T> {
    type Kind = ConsType;
}

// --- ToTChoice trait, base case for Nil ---
impl<IO> ToTChoice<IO> for Nil {
    type Output = TEnd<IO>;
}

// --- ToTChoice trait, recursive case ---
impl<IO, H: TSession<IO>, T: ToTChoice<IO>> ToTChoice<IO> for Cons<H, T> {
    type Output = TChoice<IO, H, <T as ToTChoice<IO>>::Output>;
}

// --- ToTPar trait, base case for Nil ---
impl<IO> ToTPar<IO> for Nil {
    type Output = TEnd<IO>;
}

// --- ToTPar trait, recursive case ---
impl<IO, H: TSession<IO>, T: ToTPar<IO>> ToTPar<IO> for Cons<H, T> {
    type Output = TPar<IO, H, <T as ToTPar<IO>>::Output, FalseB>;
}

// --- Type level Booleans ---
pub struct True;
pub struct False;

pub trait Bool {}
impl Bool for True {}
impl Bool for False {}

/// Trait for type-level equality between roles.
///
/// # Usage
/// - This trait is required for session type projection.
/// - **You must implement this trait for every pair of roles in your protocol.**
///   - For the same role: `type Output = True;`
///   - For different roles: `type Output = False;`
///
/// # Example
/// ```rust
/// use besedarium::{Role, RoleEq, True, False};
/// struct Alice;
/// struct Bob;
/// impl Role for Alice {}
/// impl Role for Bob {}
/// impl RoleEq<Alice> for Alice { type Output = True; }
/// impl RoleEq<Bob> for Alice { type Output = False; }
/// impl RoleEq<Alice> for Bob { type Output = False; }
/// impl RoleEq<Bob> for Bob { type Output = True; }
/// ```
///
/// This trait must be implemented by protocol authors for all roles they use.
pub trait RoleEq<Other: Role> {
    type Output;
}

// --- Local end point (Ep) session type trait---
pub trait EpSession<IO, R>: Sealed {}

// Define a trait that projects a global `TSession` onto a single role `Me`:
pub trait ProjectRole<Me, IO, G: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}

// Base case: end of Ep (end point) protocol EpEnd
pub struct EpEnd<IO, R>(PhantomData<(IO, R)>);

impl<IO, R> EpSession<IO, R> for EpEnd<IO, R> {}
impl<IO, R> Sealed for EpEnd<IO, R> {}

// Base case: end of protocol
impl<R, IO> ProjectRole<R, IO, TEnd<IO>> for () {
    type Out = EpEnd<IO, R>;
}

pub struct EpSend<IO, R, H, T>(PhantomData<(IO, R, H, T)>);
impl<IO, R, H, T> EpSession<IO, R> for EpSend<IO, R, H, T> {}
impl<IO, R, H, T> Sealed for EpSend<IO, R, H, T> {}

pub struct EpRecv<IO, R, H, T>(PhantomData<(IO, R, H, T)>);
impl<IO, R, H, T> EpSession<IO, R> for EpRecv<IO, R, H, T> {}
impl<IO, R, H, T> Sealed for EpRecv<IO, R, H, T> {}

// --- Helper Trait to Dispatch on the Boolean Flag ---

pub trait ProjectInteract<Flag: Bool, Me: Role, IO, R: Role, H, T: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}

// Send-case when Flag = True
impl<Me, IO, R: Role, H, T: TSession<IO>> ProjectInteract<True, Me, IO, R, H, T> for ()
where
    Me: Role,
    (): ProjectRole<Me, IO, T>,
{
    type Out = EpSend<IO, Me, H, <() as ProjectRole<Me, IO, T>>::Out>;
}

// Recv-case when Flag = False
impl<Me, IO, R: Role, H, T: TSession<IO>> ProjectInteract<False, Me, IO, R, H, T> for ()
where
    Me: Role,
    (): ProjectRole<Me, IO, T>,
{
    type Out = EpRecv<IO, Me, H, <() as ProjectRole<Me, IO, T>>::Out>;
}

// --S-ingle `ProjectRole` Impl for `TInteract` --

impl<Flag, Me, IO, R: Role, H, T: TSession<IO>> ProjectRole<Me, IO, TInteract<IO, R, H, T>> for ()
where
    Flag: Bool,
    Me: RoleEq<R, Output = Flag> + Role,
    (): ProjectInteract<Flag, Me, IO, R, H, T>,
{
    type Out = <() as ProjectInteract<Flag, Me, IO, R, H, T>>::Out;
}

// This avoids overlapping impls by dispatching inside the helper trait based on the computed `Flag`.
// Projection for Other Global Combinators
// For each global combinator, add a `ProjectRole` impl:

pub struct EpChoice<IO, Me, L, R>(PhantomData<(IO, Me, L, R)>);
impl<IO, Me, L, R> EpSession<IO, Me> for EpChoice<IO, Me, L, R> {}
impl<IO, Me, L, R> Sealed for EpChoice<IO, Me, L, R> {}
// Helper trait for projecting TChoice
pub trait ProjectChoice<Me, IO, L: TSession<IO>, R: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}

// Blanket impl for ProjectChoice
impl<Me, IO, L: TSession<IO>, R: TSession<IO>, OutL, OutR> ProjectChoice<Me, IO, L, R> for ()
where
    (): ProjectRole<Me, IO, L, Out = OutL>,
    (): ProjectRole<Me, IO, R, Out = OutR>,
{
    type Out = EpChoice<IO, Me, OutL, OutR>;
}

// ProjectRole for TChoice delegates to ProjectChoice
impl<Me, IO, L: TSession<IO>, R: TSession<IO>> ProjectRole<Me, IO, TChoice<IO, L, R>> for ()
where
    (): ProjectChoice<Me, IO, L, R>,
{
    type Out = <() as ProjectChoice<Me, IO, L, R>>::Out;
}

pub struct EpPar<IO, Me, L, R>(PhantomData<(IO, Me, L, R)>);
impl<IO, Me, L, R> EpSession<IO, Me> for EpPar<IO, Me, L, R> {}
impl<IO, Me, L, R> Sealed for EpPar<IO, Me, L, R> {}

pub trait ProjectPar<Me, IO, L: TSession<IO>, R: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}

impl<Me, IO, L: TSession<IO>, R: TSession<IO>, OutL, OutR> ProjectPar<Me, IO, L, R> for ()
where
    (): ProjectRole<Me, IO, L, Out = OutL>,
    (): ProjectRole<Me, IO, R, Out = OutR>,
{
    type Out = EpPar<IO, Me, OutL, OutR>;
}

impl<Me, IO, L: TSession<IO>, R: TSession<IO>, IsDisjoint>
    ProjectRole<Me, IO, TPar<IO, L, R, IsDisjoint>> for ()
where
    (): ProjectPar<Me, IO, L, R>,
{
    type Out = <() as ProjectPar<Me, IO, L, R>>::Out;
}

// --- Example Messages ---
pub struct Message;
pub struct Response;
pub struct Publish;
pub struct Notify;
pub struct Subscribe;

// --- IO protocol marker types for mixed-protocol tests ---
pub struct Http;
pub struct Db;
pub struct Mqtt;
pub struct Cache;
pub struct Mixed;

pub mod test_types;

// Role trait for protocol participants
pub trait Role {}

// --- Concrete Roles for Testing and Protocol Examples ---
pub struct TClient;
pub struct TServer;
pub struct TBroker;
pub struct TWorker;
pub struct Void;

// Role implementations for concrete roles
impl Role for TClient {}
impl Role for TServer {}
impl Role for TBroker {}
impl Role for TWorker {}
impl Role for Void {}
