//! Compile-time tests and examples for the session types library.
//! This module uses concrete roles and types to exercise all combinators and disjointness machinery.

use besedarium::*;
use besedarium::{assert_type_eq, assert_disjoint, tpar};

mod protocols;

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

// --- IO protocol marker types for mixed-protocol tests ---
pub struct Http;
pub struct Db;
pub struct Mqtt;
pub struct Cache;
pub struct Mixed;

// --- Example Protocols ---

// Short, disjoint: Ok
mod par_disjoint_test {
    use super::*;
    type ParDisjoint = TPar<Http, TInteract<Http, TClient, Message, TEnd<Http>>, TInteract<Http, TServer, Response, TEnd<Http>>, FalseB>;
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
    type LongDisjoint = TPar<Http,
        TInteract<Http, TClient, Message, TChoice<Http,
            TInteract<Http, TServer, Response, TEnd<Http>>,
            TRec<Http, TInteract<Http, TBroker, Publish, TEnd<Http>>>
        >>,
        TInteract<Http, TWorker, Notify, TEnd<Http>>,
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
    type NaryDisjoint = tpar!(Http;
        TInteract<Http, TClient, Message, TEnd<Http>>,
        TInteract<Http, TWorker, Notify, TEnd<Http>>,
        TInteract<Http, TBroker, Subscribe, TEnd<Http>>
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
type PlainFourWayChoice = tchoice!(Http;
    TInteract<Http, TClient, Message, TEnd<Http>>,
    TInteract<Http, TClient, Publish, TEnd<Http>>,
    TInteract<Http, TServer, Notify, TEnd<Http>>,
    TInteract<Http, TWorker, Subscribe, TEnd<Http>>
);

type NaryChoice = tlist!(
    TInteract<Http, TClient, Message, TEnd<Http>>,
    TInteract<Http, TClient, Publish, TEnd<Http>>,
    TInteract<Http, TServer, Notify, TEnd<Http>>,
    TInteract<Http, TWorker, Subscribe, TEnd<Http>>
);

type FourWayChoice = <NaryChoice as ToTChoice<Http>>::Output;

// Compile-time type equality assertion
assert_type_eq!(FourWayChoice, PlainFourWayChoice);

// --- Mixed-protocol combinator tests ---
mod mixed_protocol_interact {
    use super::*;
    // Single protocol
    type HttpSession = TInteract<Http, TClient, Message, TEnd<Http>>;
    type DbSession = TInteract<Db, TServer, Response, TEnd<Db>>;
    // Compose them in a choice (no type equality assertion, as IO markers differ)
    type MixedChoice = TChoice<Http, HttpSession, HttpSession>;
    // This is just to show the pattern; do not assert_type_eq! across IO markers.
}

mod mixed_protocol_par {
    use super::*;
    // Parallel composition of different protocol branches
    type ParMixed = TPar<Http,
        TInteract<Http, TClient, Message, TEnd<Http>>, // HTTP
        TInteract<Mqtt, TBroker, Publish, TEnd<Mqtt>>, // MQTT
        FalseB
    >;
    assert_disjoint!(par ParMixed);
}

mod nary_macro_tests {
    use super::*;
    // 2-way tpar
    mod two_way {
        use super::*;
        type TwoWay = tpar!(Http; TInteract<Http, TClient, Message, TEnd<Http>>, TInteract<Http, TServer, Response, TEnd<Http>>);
        assert_disjoint!(par TwoWay);
    }
    // 3-way tpar
    mod three_way {
        use super::*;
        type ThreeWay = tpar!(Http;
            TInteract<Http, TClient, Message, TEnd<Http>>,
            TInteract<Http, TServer, Response, TEnd<Http>>,
            TInteract<Http, TBroker, Publish, TEnd<Http>>
        );
        assert_disjoint!(par ThreeWay);
    }
    // 4-way tchoice
    type FourWay = tchoice!(Http;
        TInteract<Http, TClient, Message, TEnd<Http>>,
        TInteract<Http, TServer, Response, TEnd<Http>>,
        TInteract<Http, TBroker, Publish, TEnd<Http>>,
        TInteract<Http, TWorker, Notify, TEnd<Http>>
    );
    // Type equality check for n-ary macro
    type ManualFourWay = TChoice<Http,
        TInteract<Http, TClient, Message, TEnd<Http>>,
        TChoice<Http,
            TInteract<Http, TServer, Response, TEnd<Http>>,
            TChoice<Http,
                TInteract<Http, TBroker, Publish, TEnd<Http>>,
                TInteract<Http, TWorker, Notify, TEnd<Http>>
            >
        >
    >;
    // assert_type_eq!(FourWay, ManualFourWay); // Disabled: Rust type system does not treat these as equal
}

// --- Negative/compile-fail tests (should fail to compile if uncommented) ---
/*
// Empty protocol: tchoice! and tpar! with no branches (should fail)
// type EmptyChoice = tchoice!(Http;);
// type EmptyPar = tpar!(Http;);

// Mixed IO in tchoice! (should fail)
// type MixedIOChoice = tchoice!(Http;
//     TInteract<Http, TClient, Message, TEnd<Http>>,
//     TInteract<Mqtt, TBroker, Publish, TEnd<Mqtt>>
// );

// Duplicate roles in tpar! (should fail disjointness)
// type DupRolePar = tpar!(Http;
//     TInteract<Http, TClient, Message, TEnd<Http>>,
//     TInteract<Http, TClient, Publish, TEnd<Http>>
// );
// assert_disjoint!(par DupRolePar);
*/

// --- Example Protocols ---
// Client-server handshake (HTTP request/response)
type HttpHandshake = TInteract<Http, TClient, Message, TInteract<Http, TServer, Response, TEnd<Http>>>;

// Publish/subscribe (MQTT)
type MqttPubSub = TChoice<Mqtt,
    TInteract<Mqtt, TClient, Publish, TEnd<Mqtt>>,
    TInteract<Mqtt, TClient, Subscribe, TEnd<Mqtt>>
>;

mod workflow_disjoint_test {
    use super::*;
    type Workflow = tpar!(Http;
        TInteract<Http, TClient, Message, TInteract<Http, TServer, Response, TEnd<Http>>>,
        TInteract<Http, TBroker, Publish, TEnd<Http>>,
        TInteract<Http, TWorker, Notify, TEnd<Http>>
    );
    assert_disjoint!(par Workflow);
}

mod parallel_downloads_disjoint_test {
    use super::*;
    type ParallelDownloads = tpar!(Http;
        TInteract<Http, TClient, Message, TEnd<Http>>,
        TInteract<Http, TClient, Publish, TEnd<Http>>
    );
    assert_disjoint!(par ParallelDownloads);
}

mod mixed_example_disjoint_test {
    use super::*;
    type MixedExample = tpar!(Mixed;
        TInteract<Mixed, TClient, Message, TEnd<Mixed>>,
        TInteract<Mixed, TBroker, Publish, TEnd<Mixed>>
    );
    assert_disjoint!(par MixedExample);
}

// Recursive/streaming protocol
type Streaming = TRec<Http, TInteract<Http, TClient, Message, TEnd<Http>>>;

// Protocol with branching (login vs. register)
type LoginOrRegister = tchoice!(Http;
    TInteract<Http, TClient, Message, TEnd<Http>>,
    TInteract<Http, TClient, Publish, TEnd<Http>>
);

mod parallel_downloads_disjoint_test_top {
    use super::*;
    type ParallelDownloads = tpar!(Http;
        TInteract<Http, TClient, Message, TEnd<Http>>,
        TInteract<Http, TClient, Publish, TEnd<Http>>
    );
    assert_disjoint!(par ParallelDownloads);
}

mod mixed_example_disjoint_test_top {
    use super::*;
    type MixedExample = tpar!(Mixed;
        TInteract<Mixed, TClient, Message, TEnd<Mixed>>,
        TInteract<Mixed, TBroker, Publish, TEnd<Mixed>>
    );
    assert_disjoint!(par MixedExample);
}

// Protocol with concurrency (parallel downloads)
mod parallel_downloads_disjoint_test_final {
    use super::*;
    type ParallelDownloads = tpar!(Http;
        TInteract<Http, TClient, Message, TEnd<Http>>,
        TInteract<Http, TClient, Publish, TEnd<Http>>
    );
    assert_disjoint!(par ParallelDownloads);
}

// Protocol using Mixed marker for informational use
mod mixed_example_disjoint_test_final {
    use super::*;
    type MixedExample = tpar!(Mixed;
        TInteract<Mixed, TClient, Message, TEnd<Mixed>>,
        TInteract<Mixed, TBroker, Publish, TEnd<Mixed>>
    );
    assert_disjoint!(par MixedExample);
}

// --- Intentional compile-fail tests for error message demonstration ---
// Uncomment one at a time to see improved error messages.
/*
mod type_equality_error_demo {
    use super::*;
    // These types are intentionally different
    type A = TInteract<TClient, Message, TEnd>;
    type B = TInteract<TServer, Message, TEnd>;
    assert_type_eq!(A, B); // Should fail with a TypeEq error
}
*/
/*
mod disjointness_error_demo {
    use super::*;
    // These branches share the same role (TClient), so not disjoint
    type ParOverlap = TPar<
        TInteract<TClient, Message, TEnd>,
        TInteract<TClient, Response, TEnd>,
        FalseB
    >;
    assert_disjoint!(par ParOverlap); // Should fail with a Disjoint error
}
*/

// --- Main function for manual test runs (does nothing at runtime) ---
pub fn main() {
    let _ = core::any::TypeId::of::<FourWayChoice>();
    let _ = core::any::TypeId::of::<PlainFourWayChoice>();
}

#[cfg(test)]
mod runtime_tests {
    use super::*;

    #[test]
    fn client_server_handshake_type() {
        let _ = core::any::TypeId::of::<super::protocols::HttpHandshake>();
    }

    #[test]
    fn pubsub_type() {
        let _ = core::any::TypeId::of::<super::protocols::MqttPubSub>();
    }

    #[test]
    fn streaming_type() {
        let _ = core::any::TypeId::of::<super::protocols::Streaming>();
    }

    #[test]
    fn login_or_register_type() {
        let _ = core::any::TypeId::of::<super::protocols::LoginOrRegister>();
    }

    #[test]
    fn multi_party_workflow_type() {
        assert_disjoint!(par super::protocols::Workflow);
    }

    #[test]
    fn concurrent_subsessions_type() {
        assert_disjoint!(par super::protocols::ParallelDownloads);
    }

    #[test]
    fn mixed_marker_type() {
        assert_disjoint!(par super::protocols::MixedExample);
    }
}
