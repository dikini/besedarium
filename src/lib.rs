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
    ($($branch:ty),+ $(,)?) => {
        <tlist!($($branch),+ ) as ToTChoice>::Output
    };
}

#[macro_export]
macro_rules! tpar {
    ($($branch:ty),+ $(,)?) => {
        <tlist!($($branch),+ ) as ToTPar>::Output
    };
}

#[macro_export]
macro_rules! assert_type_eq {
    ($A:ty, $B:ty) => {
        const _: fn() = || {
            fn assert_eq_types(_: $A, _: $B) {}
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

pub trait TSession: Sealed {
    type Compose<Rhs: TSession>: TSession;
    const IS_EMPTY: bool;
}

// The empty TSession.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TEnd;

impl Sealed for TEnd {}
impl TSession for TEnd {
    type Compose<Rhs: TSession> = Rhs;
    const IS_EMPTY: bool = true;
}

// An interaction TSession for role R.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TInteract<R, H, T: TSession>(PhantomData<(R, H, T)>);

impl<R, H, T: TSession> Sealed for TInteract<R, H, T> {}
impl<R, H, T: TSession> TSession for TInteract<R, H, T> {
    type Compose<Rhs: TSession> = TInteract<R, H, T::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

// Recursive session type
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TRec<S: TSession>(PhantomData<S>);

impl<S: TSession> Sealed for TRec<S> {}
impl<S: TSession> TSession for TRec<S> {
    type Compose<Rhs: TSession> = TRec<S::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

// Binary choice
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TChoice<L: TSession, R: TSession>(PhantomData<(L, R)>);

impl<L: TSession, R: TSession> Sealed for TChoice<L, R> {}
impl<L: TSession, R: TSession> TSession for TChoice<L, R> {
    type Compose<Rhs: TSession> = TChoice<L::Compose<Rhs>, R::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

// Map type-level list to nested TChoice
pub trait ToTChoice {
    type Output: TSession;
}
impl<H: TSession> ToTChoice for Cons<H, Nil> {
    type Output = H;
}
impl<H: TSession, T: ToTChoice> ToTChoice for Cons<H, T> {
    type Output = TChoice<H, <T as ToTChoice>::Output>;
}

// Branded parallel composition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TPar<L: TSession, R: TSession, IsDisjoint>(PhantomData<(L, R, IsDisjoint)>);

impl<L: TSession, R: TSession, IsDisjoint> Sealed for TPar<L, R, IsDisjoint> {}
impl<L: TSession, R: TSession, IsDisjoint> TSession for TPar<L, R, IsDisjoint> {
    type Compose<Rhs: TSession> = TPar<L::Compose<Rhs>, R::Compose<Rhs>, IsDisjoint>;
    const IS_EMPTY: bool = false;
}

pub trait ToTPar {
    type Output: TSession;
}
impl<H: TSession> ToTPar for Cons<H, Nil> {
    type Output = H;
}
impl<H: TSession, T: ToTPar> ToTPar for Cons<H, T> {
    type Output = TPar<H, <T as ToTPar>::Output, FalseB>;
}

// Type-level booleans for branding
pub struct TrueB;
pub struct FalseB;

// --- Role Extraction Machinery ---
pub trait RolesOf {
    type Roles;
}
impl RolesOf for TEnd {
    type Roles = Nil;
}
impl<R, H, T: TSession + RolesOf> RolesOf for TInteract<R, H, T> {
    type Roles = Cons<R, <T as RolesOf>::Roles>;
}
impl<L: TSession + RolesOf, R: TSession + RolesOf> RolesOf for TChoice<L, R> {
    type Roles = <L as RolesOf>::Roles;
}
impl<L: TSession + RolesOf, R: TSession + RolesOf, IsDisjoint> RolesOf for TPar<L, R, IsDisjoint> {
    type Roles = <L as RolesOf>::Roles;
}
impl<S: TSession + RolesOf> RolesOf for TRec<S> {
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
impl<L: TSession + RolesOf, R: TSession + RolesOf> AssertDisjoint for TPar<L, R, FalseB>
where
    (): Disjoint<<L as RolesOf>::Roles, <R as RolesOf>::Roles>,
    (): Disjoint<<R as RolesOf>::Roles, <L as RolesOf>::Roles>,
{
    type Output = TPar<L, R, TrueB>;
}
impl<L: TSession, R: TSession> AssertDisjoint for TPar<L, R, TrueB> {
    type Output = TPar<L, R, TrueB>;
}
