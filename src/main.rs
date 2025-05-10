#![forbid(unsafe_code)]
#![no_std]

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
            fn assert_disjoint_impl()
            where (): Disjoint<$A, $B> {}
        };
    };
}


mod sealed {
    pub trait Sealed {}
}

use sealed::Sealed;


// Type-level list for n-ary combinators
pub struct Nil;
pub struct Cons<H, T>(PhantomData<(H, T)>);


pub trait TRole {}

pub struct TClient;
pub struct TServer;
pub struct TBroker;
pub struct TWorker;

impl TRole for TClient {}
impl TRole for TServer {}
impl TRole for TBroker {}
impl TRole for TWorker {}


pub trait TSession:  Sealed {
    /// Implementation of [type@Concat].
    type Compose<Rhs: TSession>: TSession;

    /// True iff the list is empty, false otherwise.
    /// Returns a bool as associated const value.
    /// If you'd rather use a type, see [IsEmpty]
    /// (which needs `typenum` feature to be enabled)
    /// Also see the [Empty] and [NonEmpty] traits.
    /// Lifted list from tlist
    const IS_EMPTY: bool;
}

pub type Compose<Lhs, Rhs> = <Lhs as TSession>::Compose<Rhs>;

/// The empty TSession.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TEnd;


impl Sealed for TEnd {}

impl TSession for TEnd {
    type Compose<Rhs: TSession> = Rhs;
    const IS_EMPTY: bool = true;
}

// An non-empty interaction TSession whose first element is `H` and whose tail is the TSession `T` for Role R.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TInteract<R: TRole, H, T: TSession>(PhantomData<(R, H, T)>);

impl<R: TRole, H, T: TSession> Sealed for TInteract<R, H, T> {}

impl<R: TRole, H, T: TSession> TSession for TInteract<R, H, T> {
    type Compose<Rhs: TSession> = TInteract<R, H, T::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}


/// A TSession that is a recursive type.
/// It is used to represent a TSession that can be composed with itself.
/// 
/// This is useful for representing infinite TSession types.
///
/// For example, a TSession that represents a stream of messages can be represented as:
/// ```
/// type Stream = TRec<TInteract<TClient, Message, TEnd>>;
/// ```
/// This represents a stream of messages where each message is sent by the client.
/// The `TRec` type is used to represent the recursive nature of the stream.
///
/// The `TRec` type is a wrapper around the `TSession` type that allows for recursive types.
/// It is used to represent a TSession that can be composed with itself.
/// It is a marker type that indicates that the TSession is recursive. 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TRec<S: TSession>(PhantomData<S>);

impl<S: TSession> Sealed for TRec<S> {}

impl<S: TSession> TSession for TRec<S> {
    type Compose<Rhs: TSession> = TRec<S::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

/// A TSession that represents a binary choice between two branches.
/// The protocol can proceed as either the left or right branch.
/// Both branches must have the same continuation type for safety.
/// 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TChoice<L: TSession, R: TSession>(PhantomData<(L, R)>);

impl<L: TSession, R: TSession> Sealed for TChoice<L, R> {}

impl<L: TSession, R: TSession> TSession for TChoice<L, R> {
    // Compose applies the continuation to both branches (scatter-gather pattern).
    type Compose<Rhs: TSession> = TChoice<L::Compose<Rhs>, R::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

// Trait to map a type-level list to nested TChoice
pub trait ToTChoice {
    type Output: TSession;
}

// Base case: single element, just return it
impl<H: TSession> ToTChoice for Cons<H, Nil> {
    type Output = H;
}

// Recursive case: Cons<H, T> becomes TChoice<H, Map<T>>
impl<H: TSession, T: ToTChoice> ToTChoice for Cons<H, T> {
    type Output = TChoice<H, <T as ToTChoice>::Output>;
}


/// A TSession that represents a binary parallell composition of two threads.
/// The protocol can proceed simultaneously on both.
/// Both threads must have the same continuation type for safety.
/// 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TPar<L: TSession, R: TSession>(PhantomData<(L, R)>);

impl<L: TSession, R: TSession> Sealed for TPar<L, R> {}

impl<L: TSession, R: TSession> TSession for TPar<L, R> {
    // Compose applies the continuation to both threads (scatter-gather pattern).
    type Compose<Rhs: TSession> = TPar<L::Compose<Rhs>, R::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

// Trait to map a type-level list to nested TChoice
pub trait ToTPar {
    type Output: TSession;
}

// Base case: single element, just return it
impl<H: TSession> ToTPar for Cons<H, Nil> {
    type Output = H;
}

// Recursive case: Cons<H, T> becomes TPar<H, Map<T>>
impl<H: TSession, T: ToTPar> ToTPar for Cons<H, T> {
    type Output = TPar<H, <T as ToTPar>::Output>;
}

//
struct Message {}
struct Response {}
struct Publish {}
struct Notify {}
struct Subscribe {}

type ClientServer = TInteract<TClient, Message, TInteract<TServer, Response, TEnd>>;
type ServerBroker = TInteract<TServer, Publish, TInteract<TBroker, Message, TEnd>>;
type BrokerWorker = TInteract<TBroker, Notify, TInteract<TWorker, Response, TEnd>>;

type Chain = Compose<Compose<ClientServer, ServerBroker>, TRec<BrokerWorker>>;

type ChoiceSession = TChoice<
    TInteract<TClient, Message, TEnd>,
    TInteract<TClient, Publish, TEnd>
>;

// Example usage for 4-way choice
type PlainFourWayChoice = TChoice<
    TInteract<TClient, Message, TEnd>, // Choice 1
    TChoice<
        TInteract<TClient, Publish, TEnd>, // Choice 2
        TChoice<
            TInteract<TServer, Notify, TEnd>, // Choice 3
            TInteract<TWorker, Subscribe, TEnd> // Choice 4
        >
    >
>;

// Example usage for n-ary choice
type NaryChoice = Cons<
    TInteract<TClient, Message, TEnd>,
    Cons<
        TInteract<TClient, Publish, TEnd>,
        Cons<
            TInteract<TServer, Notify, TEnd>,
            Cons<
                TInteract<TWorker, Subscribe, TEnd>,
                Nil
            >
        >
    >
>;

type FourWayChoice = <NaryChoice as ToTChoice>::Output;


fn _assert_choices_equal(_: FourWayChoice, _: PlainFourWayChoice) {}

fn main() {
    _assert_choices_equal(
        core::panic!(), // never called, just for type-checking
        core::panic!(),
    );
}
