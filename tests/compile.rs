//! Compile-time tests and examples for the session types library.
//! This module uses concrete roles and types to exercise all combinators and disjointness machinery.

use playground::*;
use playground::{assert_type_eq, assert_disjoint, tpar};

// --- Concrete Roles for Testing ---
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

// --- Example Protocols ---

// Short, disjoint: Ok
mod par_disjoint_test {
    use super::*;
    type ParDisjoint = TPar<
        TInteract<TClient, Message, TEnd>,
        TInteract<TServer, Response, TEnd>,
        FalseB
    >;
    assert_disjoint!(par ParDisjoint);
}

// Short, not disjoint: Compile error
// type ParOverlap = TPar<
//     TInteract<TClient, Message, TEnd>,
//     TInteract<TClient, Response, TEnd>,
//     FalseB
// >;
// assert_disjoint!(par ParOverlap);

// Long, disjoint: Ok
mod long_disjoint_test {
    use super::*;
    type LongDisjoint = TPar<
        TInteract<TClient, Message, TChoice<
            TInteract<TServer, Response, TEnd>,
            TRec<TInteract<TBroker, Publish, TEnd>>
        >>,
        TInteract<TWorker, Notify, TEnd>,
        FalseB
    >;
    assert_disjoint!(par LongDisjoint);
}

// Long, not disjoint: Compile error
// type LongOverlap = TPar<
//     TInteract<TClient, Message, TChoice<
//         TInteract<TServer, Response, TEnd>,
//         TRec<TInteract<TBroker, Publish, TEnd>>
//     >>,
//     TInteract<TClient, Notify, TEnd>,
//     FalseB
// >;
// assert_disjoint!(par LongOverlap);

// N-ary, all combinators, disjoint: Ok
mod nary_disjoint_test {
    use super::*;
    type NaryDisjoint = tpar!(
        TInteract<TClient, Message, TChoice<
            TInteract<TServer, Response, TEnd>,
            TRec<TInteract<TBroker, Publish, TEnd>>
        >>,
        TInteract<TWorker, Notify, TEnd>,
        TInteract<TBroker, Subscribe, TEnd>
    );
    assert_disjoint!(par NaryDisjoint);
}

// N-ary, not disjoint: Compile error
// type NaryOverlap = tpar!(
//     TInteract<TClient, Message, TChoice<
//         TInteract<TServer, Response, TEnd>,
//         TRec<TInteract<TBroker, Publish, TEnd>>
//     >>,
//     TInteract<TWorker, Notify, TEnd>,
//     TInteract<TClient, Subscribe, TEnd>
// );
// assert_disjoint!(par NaryOverlap);

// --- Choice/Equality Example ---
type PlainFourWayChoice = TChoice<
    TInteract<TClient, Message, TEnd>,
    TChoice<
        TInteract<TClient, Publish, TEnd>,
        TChoice<
            TInteract<TServer, Notify, TEnd>,
            TInteract<TWorker, Subscribe, TEnd>
        >
    >
>;

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

// Compile-time type equality assertion
assert_type_eq!(FourWayChoice, PlainFourWayChoice);

// --- Main function for manual test runs (does nothing at runtime) ---
pub fn main() {
    let _ = core::any::TypeId::of::<FourWayChoice>();
    let _ = core::any::TypeId::of::<PlainFourWayChoice>();
}
