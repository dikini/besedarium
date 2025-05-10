use tlist::*;
use core::marker::PhantomData;

#[macro_use]
extern crate static_assertions as sa;

mod sealed {
    pub trait Sealed {}
}

use sealed::Sealed;


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

// An non-empty interaction TSession whose first element is `H` and whose tail is the TSession `T` for Role R.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TInteract<R: TRole, H, T: TSession>(PhantomData<(R, H, T)>);


impl Sealed for TEnd {}

impl TSession for TEnd {
    type Compose<Rhs: TSession> = Rhs;
    const IS_EMPTY: bool = true;
}


impl<R: TRole, H, T: TSession> Sealed for TInteract<R, H, T> {}

impl<R: TRole, H, T: TSession> TSession for TInteract<R, H, T> {
    type Compose<Rhs: TSession> = TInteract<R, H, T::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}


struct Message {}
struct Response {}
struct Publish {}
struct Notify {}
struct Subscribe {}


type ClientServer = TInteract<TClient, Message, TInteract<TServer, Response, TEnd>>;
type ServerBroker = TInteract<TServer, Publish, TInteract<TBroker, Message, TEnd>>;
type BrokerWorker = TInteract<TBroker, Notify, TInteract<TWorker, Response, TEnd>>;

type Chain = Compose<Compose<ClientServer, ServerBroker>, BrokerWorker>;

fn main() {
    println!("{:?}", "roles");
}
