#![doc = include_str!("../README.md")]

//! Session types core library: abstract types, traits, combinators, and macros.
//! No concrete roles or example/test code.

use core::marker::PhantomData;

#[macro_export]
macro_rules! tlist {
    () => { Nil };
    ($head:ty $(, $tail:ty )* $(,)?) => {
        Cons<$head, tlist!($($tail),*)>
    };
}

#[macro_export]
macro_rules! tchoice {
    ($io:ty; $($branch:ty),+ $(,)?) => {
        <tlist!($($branch),*) as ToTChoice<$io>>::Output
    };
}

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
                $A: $crate::TypeEq<$B>
            {}
        };
    };
}

#[macro_export]
macro_rules! assert_disjoint {
    ($A:ty, $B:ty) => {
        const _: fn() = || {
            fn _assert_disjoint()
            where
                (): $crate::Disjoint<<$A as $crate::RolesOf>::Roles, <$B as $crate::RolesOf>::Roles>
            {}
        };
    };
    (par $TPar:ty) => {
        type _Checked = <$TPar as $crate::AssertDisjoint>::Output;
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

// The empty TSession.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TEnd<IO>(PhantomData<IO>);

impl<IO> Sealed for TEnd<IO> {}
impl<IO> TSession<IO> for TEnd<IO> {
    type Compose<Rhs: TSession<IO>> = Rhs;
    const IS_EMPTY: bool = true;
}

// An interaction TSession for role R.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TInteract<IO, R, H, T: TSession<IO>>(PhantomData<(IO, R, H, T)>);

impl<IO, R, H, T: TSession<IO>> Sealed for TInteract<IO, R, H, T> {}
impl<IO, R, H, T: TSession<IO>> TSession<IO> for TInteract<IO, R, H, T> {
    type Compose<Rhs: TSession<IO>> = TInteract<IO, R, H, T::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

// Recursive session type
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TRec<IO, S: TSession<IO>>(PhantomData<(IO, S)>);

impl<IO, S: TSession<IO>> Sealed for TRec<IO, S> {}
impl<IO, S: TSession<IO>> TSession<IO> for TRec<IO, S> {
    type Compose<Rhs: TSession<IO>> = TRec<IO, S::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

// Binary choice
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TChoice<IO, L: TSession<IO>, R: TSession<IO>>(PhantomData<(IO, L, R)>);

impl<IO, L: TSession<IO>, R: TSession<IO>> Sealed for TChoice<IO, L, R> {}
impl<IO, L: TSession<IO>, R: TSession<IO>> TSession<IO> for TChoice<IO, L, R> {
    type Compose<Rhs: TSession<IO>> = TChoice<IO, L::Compose<Rhs>, R::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

// Map type-level list to nested TChoice
pub trait ToTChoice<IO> {
    type Output: TSession<IO>;
}

// Branded parallel composition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TPar<IO, L: TSession<IO>, R: TSession<IO>, IsDisjoint>(PhantomData<(IO, L, R, IsDisjoint)>);

impl<IO, L: TSession<IO>, R: TSession<IO>, IsDisjoint> Sealed for TPar<IO, L, R, IsDisjoint> {}
impl<IO, L: TSession<IO>, R: TSession<IO>, IsDisjoint> TSession<IO> for TPar<IO, L, R, IsDisjoint> {
    type Compose<Rhs: TSession<IO>> = TPar<IO, L::Compose<Rhs>, R::Compose<Rhs>, IsDisjoint>;
    const IS_EMPTY: bool = false;
}

pub trait ToTPar<IO> {
    type Output: TSession<IO>;
}

// Type-level booleans for branding
pub struct TrueB;
pub struct FalseB;

// --- Role Extraction Machinery ---
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
impl<IO, L: TSession<IO> + RolesOf, R: TSession<IO> + RolesOf, IsDisjoint> RolesOf for TPar<IO, L, R, IsDisjoint> {
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
impl<X, H, T> Contains<X> for Cons<H, T>
where
    T: Contains<X>,
{}

pub trait NotContains<X> {}
impl<X> NotContains<X> for Nil {}
impl<X, H, T> NotContains<X> for Cons<H, T>
where
    T: NotContains<X>,
{}

pub trait Disjoint<A, B> {}
impl<B> Disjoint<Nil, B> for () {}
impl<H, T, B> Disjoint<Cons<H, T>, B> for ()
where
    B: NotContains<H>,
    (): Disjoint<T, B>,
{}

// --- Compile-time Disjointness Assertion Machinery ---
pub trait AssertDisjoint {
    type Output;
}
impl<IO, L: TSession<IO> + RolesOf, R: TSession<IO> + RolesOf> AssertDisjoint for TPar<IO, L, R, FalseB>
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

// --- Concrete Roles for Testing and Protocol Examples ---
pub struct TClient;
pub struct TServer;
pub struct TBroker;
pub struct TWorker;

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
