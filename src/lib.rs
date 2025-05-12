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
/// struct Alice; impl Role for Alice {}; impl ProtocolLabel for Alice {};
/// struct Bob; impl Role for Bob {}; impl ProtocolLabel for Bob {};
/// impl RoleEq<Alice> for Alice { type Output = True; }
/// impl RoleEq<Bob> for Alice { type Output = False; }
/// impl RoleEq<Alice> for Bob { type Output = False; }
/// impl RoleEq<Bob> for Bob { type Output = True; }
/// struct L; impl ProtocolLabel for L {}
/// type Global = TInteract<Http, L, Alice, Message, TInteract<Http, L, Bob, Response, TEnd<Http, L>>>;
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
/// struct L1; impl ProtocolLabel for L1 {}
/// struct L2; impl ProtocolLabel for L2 {}
/// type Choice = tchoice!(Http;
///     TInteract<Http, L1, TClient, Message, TEnd<Http, L1>>,
///     TInteract<Http, L2, TServer, Response, TEnd<Http, L2>>,
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
///     TInteract<Http, L1, TClient, Message, TEnd<Http, L1>>,
///     TInteract<Http, L2, TServer, Response, TEnd<Http, L2>>,
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
/// struct L; impl ProtocolLabel for L {}
/// type Roles = extract_roles!(TInteract<Http, L, TClient, Message, TEnd<Http, L>>);
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
///     TInteract<Http, MyLabel1, TClient, Message, TEnd<Http, MyLabel1>>,
///     TInteract<Http, MyLabel2, TServer, Response, TEnd<Http, MyLabel2>>
/// >;
/// assert_unique_labels!(MyProtocol); // Compile-time error if labels are not unique
/// ```

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
/// use besedarium::*;
/// struct L; impl ProtocolLabel for L {}
/// type End = TEnd<Http, L>;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TEnd<IO, L = EmptyLabel>(PhantomData<(IO, L)>);

impl<IO, L> Sealed for TEnd<IO, L> {}
impl<IO, L> TSession<IO> for TEnd<IO, L> {
    type Compose<Rhs: TSession<IO>> = Rhs;
    const IS_EMPTY: bool = true;
}

/// Represents a single interaction in a protocol session.
///
/// `TInteract<IO, L, R, H, T>` means role `R` sends or receives message `H` over IO, then continues as `T`.
///
/// # Example
/// ```rust
/// use besedarium::*;
/// struct L; impl ProtocolLabel for L {}
/// type Step = TInteract<Http, L, TClient, Message, TEnd<Http, L>>;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TInteract<IO, L: ProtocolLabel, R, H, T: TSession<IO>>(PhantomData<(IO, L, R, H, T)>);

impl<IO, L: ProtocolLabel, R, H, T: TSession<IO>> Sealed for TInteract<IO, L, R, H, T> {}
impl<IO, L: ProtocolLabel, R, H, T: TSession<IO>> TSession<IO> for TInteract<IO, L, R, H, T> {
    type Compose<Rhs: TSession<IO>> = TInteract<IO, L, R, H, T::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

/// Recursive session type for repeating protocol fragments.
///
/// `TRec<IO, L, S>` means repeat the protocol `S` (which may refer to itself), with label `L`.
///
/// # Example
/// ```rust
/// use besedarium::*;
/// struct LoopLabel; impl ProtocolLabel for LoopLabel {}
/// type Streaming = TRec<Http, LoopLabel, TInteract<Http, LoopLabel, TClient, Message, TEnd<Http, LoopLabel>>>;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TRec<IO, L: ProtocolLabel, S: TSession<IO>>(PhantomData<(IO, L, S)>);

impl<IO, L: ProtocolLabel, S: TSession<IO>> Sealed for TRec<IO, L, S> {}
impl<IO, L: ProtocolLabel, S: TSession<IO>> TSession<IO> for TRec<IO, L, S> {
    type Compose<Rhs: TSession<IO>> = TRec<IO, L, S::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

/// Binary protocol choice between two branches.
///
/// `TChoice<IO, Lbl, L, R>` means the protocol can proceed as either `L` or `R`, with label `Lbl`.
///
/// # Example
/// ```rust
/// use besedarium::*;
/// struct AcceptLabel; impl ProtocolLabel for AcceptLabel {}
/// struct RejectLabel; impl ProtocolLabel for RejectLabel {}
/// type Choice = TChoice<Http, AcceptLabel, TInteract<Http, AcceptLabel, TClient, Message, TEnd<Http, AcceptLabel>>, TInteract<Http, RejectLabel, TServer, Response, TEnd<Http, RejectLabel>>>;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TChoice<IO, Lbl: ProtocolLabel, L: TSession<IO>, R: TSession<IO>>(
    PhantomData<(IO, Lbl, L, R)>,
);

impl<IO, Lbl: ProtocolLabel, L: TSession<IO>, R: TSession<IO>> Sealed for TChoice<IO, Lbl, L, R> {}
impl<IO, Lbl: ProtocolLabel, L: TSession<IO>, R: TSession<IO>> TSession<IO>
    for TChoice<IO, Lbl, L, R>
{
    type Compose<Rhs: TSession<IO>> = TChoice<IO, Lbl, L::Compose<Rhs>, R::Compose<Rhs>>;
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
/// struct L1; impl ProtocolLabel for L1 {}
/// struct L2; impl ProtocolLabel for L2 {}
/// type Choice = <tlist!(TInteract<Http, L1, TClient, Message, TEnd<Http, L1>>, TInteract<Http, L2, TServer, Response, TEnd<Http, L2>>) as ToTChoice<Http>>::Output;
/// ```
pub trait ToTChoice<IO> {
    type Output: TSession<IO>;
}

/// Branded parallel composition of two protocol branches.
///
/// `TPar<IO, Lbl, L, R, IsDisjoint>` means run `L` and `R` in parallel, with a marker for disjointness and label `Lbl`.
///
/// # Example
/// ```rust
/// use besedarium::*;
/// struct ParLabel; impl ProtocolLabel for ParLabel {}
/// type Par = TPar<Http, ParLabel, TInteract<Http, ParLabel, TClient, Message, TEnd<Http, ParLabel>>, TInteract<Http, ParLabel, TServer, Response, TEnd<Http, ParLabel>>, FalseB>;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TPar<IO, Lbl: ProtocolLabel, L: TSession<IO>, R: TSession<IO>, IsDisjoint>(
    PhantomData<(IO, Lbl, L, R, IsDisjoint)>,
);

impl<IO, Lbl: ProtocolLabel, L: TSession<IO>, R: TSession<IO>, IsDisjoint> Sealed
    for TPar<IO, Lbl, L, R, IsDisjoint>
{
}
impl<IO, Lbl: ProtocolLabel, L: TSession<IO>, R: TSession<IO>, IsDisjoint> TSession<IO>
    for TPar<IO, Lbl, L, R, IsDisjoint>
{
    type Compose<Rhs: TSession<IO>> = TPar<IO, Lbl, L::Compose<Rhs>, R::Compose<Rhs>, IsDisjoint>;
    const IS_EMPTY: bool = false;
}

/// Trait for mapping a type-level list to a nested `TPar`.
///
/// Used by the `tpar!` macro for n-ary protocol parallel composition.
///
/// # Example
/// ```rust
/// use besedarium::*;
/// struct L1; impl ProtocolLabel for L1 {}
/// struct L2; impl ProtocolLabel for L2 {}
/// type Par = <tlist!(TInteract<Http, L1, TClient, Message, TEnd<Http, L1>>, TInteract<Http, L2, TServer, Response, TEnd<Http, L2>>) as ToTPar<Http>>::Output;
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
/// struct L; impl ProtocolLabel for L {}
/// type Roles = <TInteract<Http, L, TClient, Message, TEnd<Http, L>> as RolesOf>::Roles;
/// ```
pub trait RolesOf {
    type Roles;
}
impl<IO> RolesOf for TEnd<IO> {
    type Roles = Nil;
}
impl<IO, L: ProtocolLabel, R, H, T: TSession<IO> + RolesOf> RolesOf for TInteract<IO, L, R, H, T> {
    type Roles = Cons<R, <T as RolesOf>::Roles>;
}
impl<IO, Lbl: ProtocolLabel, L: TSession<IO> + RolesOf, R: TSession<IO>> RolesOf
    for TChoice<IO, Lbl, L, R>
{
    type Roles = <L as RolesOf>::Roles;
}
impl<IO, Lbl: ProtocolLabel, L: TSession<IO> + RolesOf, R: TSession<IO>, IsDisjoint> RolesOf
    for TPar<IO, Lbl, L, R, IsDisjoint>
{
    type Roles = <L as RolesOf>::Roles;
}
impl<IO, L: ProtocolLabel, S: TSession<IO> + RolesOf> RolesOf for TRec<IO, L, S> {
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
/// struct MyLabel1; impl ProtocolLabel for MyLabel1 {}
/// struct MyLabel2; impl ProtocolLabel for MyLabel2 {}
/// type Checked = <TPar<Http, EmptyLabel, TInteract<Http, MyLabel1, TClient, Message, TEnd<Http, MyLabel1>>, TInteract<Http, MyLabel2, TServer, Response, TEnd<Http, MyLabel2>>, FalseB> as AssertDisjoint>::Output;
/// ```
pub trait AssertDisjoint {
    type Output;
}
impl<IO, L: TSession<IO> + RolesOf, R: TSession<IO> + RolesOf> AssertDisjoint
    for TPar<IO, EmptyLabel, L, R, FalseB>
where
    (): Disjoint<<L as RolesOf>::Roles, <R as RolesOf>::Roles>,
    (): Disjoint<<R as RolesOf>::Roles, <L as RolesOf>::Roles>,
{
    type Output = TPar<IO, EmptyLabel, L, R, TrueB>;
}
impl<IO, L: TSession<IO>, R: TSession<IO>> AssertDisjoint for TPar<IO, EmptyLabel, L, R, TrueB> {
    type Output = TPar<IO, EmptyLabel, L, R, TrueB>;
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
    type Output = TChoice<IO, EmptyLabel, H, <T as ToTChoice<IO>>::Output>;
}

// --- ToTPar trait, base case for Nil ---
impl<IO> ToTPar<IO> for Nil {
    type Output = TEnd<IO>;
}

// --- ToTPar trait, recursive case ---
impl<IO, H: TSession<IO>, T: ToTPar<IO>> ToTPar<IO> for Cons<H, T> {
    type Output = TPar<IO, EmptyLabel, H, <T as ToTPar<IO>>::Output, FalseB>;
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

impl<Flag, Me, IO, L: ProtocolLabel, R: Role, H, T: TSession<IO>>
    ProjectRole<Me, IO, TInteract<IO, L, R, H, T>> for ()
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
impl<Me, IO, Lbl: ProtocolLabel, L: TSession<IO>, R: TSession<IO>>
    ProjectRole<Me, IO, TChoice<IO, Lbl, L, R>> for ()
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

impl<Me, IO, Lbl: ProtocolLabel, L: TSession<IO>, R: TSession<IO>, IsDisjoint>
    ProjectRole<Me, IO, TPar<IO, Lbl, L, R, IsDisjoint>> for ()
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

// --- Protocol Label Type ---
/// Marker trait for user-definable protocol labels.
pub trait ProtocolLabel {}

/// Placeholder for an empty label (use when a label is not meaningful).
pub struct EmptyLabel;
impl ProtocolLabel for EmptyLabel {}

// Example user-defined label (for docs/tests)
// pub struct MyLabel;
// impl ProtocolLabel for MyLabel {}

// --- Label Extraction Machinery ---
/// Extracts the set of labels used in a protocol type as a type-level list.
///
/// Used for uniqueness checks and compile-time assertions.
///
/// # Example
/// ```rust
/// use besedarium::*;
/// struct L; impl ProtocolLabel for L {}
/// type Labels = <TInteract<Http, L, TClient, Message, TEnd<Http, L>> as LabelsOf>::Labels;
/// ```
pub trait LabelsOf {
    type Labels;
}
impl<IO, L> LabelsOf for TEnd<IO, L> {
    type Labels = Cons<L, Nil>;
}
impl<IO, L: ProtocolLabel, R, H, T: TSession<IO> + LabelsOf> LabelsOf
    for TInteract<IO, L, R, H, T>
{
    type Labels = Cons<L, <T as LabelsOf>::Labels>;
}
impl<IO, Lbl: ProtocolLabel, L: TSession<IO> + LabelsOf, R: TSession<IO> + LabelsOf> LabelsOf
    for TChoice<IO, Lbl, L, R>
{
    type Labels = Cons<Lbl, <L as LabelsOf>::Labels>;
}
impl<
        IO,
        Lbl: ProtocolLabel,
        L: TSession<IO> + LabelsOf,
        R: TSession<IO> + LabelsOf,
        IsDisjoint,
    > LabelsOf for TPar<IO, Lbl, L, R, IsDisjoint>
{
    type Labels = Cons<Lbl, <L as LabelsOf>::Labels>;
}
impl<IO, L: ProtocolLabel, S: TSession<IO> + LabelsOf> LabelsOf for TRec<IO, L, S> {
    type Labels = Cons<L, <S as LabelsOf>::Labels>;
}
impl LabelsOf for Nil {
    type Labels = Nil;
}
impl<H, T> LabelsOf for Cons<H, T>
where
    H: LabelsOf,
    T: LabelsOf,
{
    type Labels = <H as LabelsOf>::Labels;
    // Note: For a type-level list of protocol types, only the head's labels are included.
    // If you want to aggregate all, you may want to concatenate <H as LabelsOf>::Labels and <T as LabelsOf>::Labels.
}

// --- Uniqueness Traits for Type-level Lists ---
/// Trait to check that a type-level list contains only unique elements (no duplicates).
///
/// Used for compile-time uniqueness assertions (e.g., for protocol labels).
pub trait NotInList<X> {}
impl<X> NotInList<X> for Nil {}
impl<X, H, T> NotInList<X> for Cons<H, T>
where
    X: NotSame<H>,
    T: NotInList<X>,
{
}

/// Trait for type-level inequality (negation of TypeEq).
pub trait NotSame<T> {}
impl<A, B> NotSame<B> for A where A: NotTypeEq<B> {}

// Helper trait: implemented for all pairs except when A == B
pub trait NotTypeEq<B> {}
impl<A, B> NotTypeEq<B> for A {}
// Overlap: no impl for A == A

/// Trait to check that all elements in a type-level list are unique.
pub trait UniqueList {}
impl UniqueList for Nil {}
impl<H, T> UniqueList for Cons<H, T> where T: NotInList<H> + UniqueList {}
