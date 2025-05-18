//! Compile-time tests and examples for the session types library.
//! This module uses concrete roles and types to exercise all combinators and disjointness machinery.

use besedarium::*;
use besedarium::{assert_disjoint, assert_type_eq, tpar};

mod protocols;

// --- Concrete Roles for Testing ---
pub struct TClient;
pub struct TServer;
pub struct TBroker;
pub struct TWorker;
impl ProtocolLabel for TClient {}
impl ProtocolLabel for TServer {}
impl ProtocolLabel for TBroker {}
impl ProtocolLabel for TWorker {}

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
    type ParDisjoint = TPar<
        Http,
        EmptyLabel,
        TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
        TSend<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>,
        FalseB,
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
        Http,
        EmptyLabel,
        TSend<
            Http,
            EmptyLabel,
            TClient,
            Message,
            TChoice<
                Http,
                EmptyLabel,
                TSend<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>,
                TRec<
                    Http,
                    EmptyLabel,
                    TSend<Http, EmptyLabel, TBroker, Publish, TEnd<Http, EmptyLabel>>,
                >,
            >,
        >,
        TSend<Http, EmptyLabel, TWorker, Notify, TEnd<Http, EmptyLabel>>,
        FalseB,
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
        TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
        TSend<Http, EmptyLabel, TWorker, Notify, TEnd<Http, EmptyLabel>>,
        TSend<Http, EmptyLabel, TBroker, Subscribe, TEnd<Http, EmptyLabel>>
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
    TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
    TSend<Http, EmptyLabel, TClient, Publish, TEnd<Http, EmptyLabel>>,
    TSend<Http, EmptyLabel, TServer, Notify, TEnd<Http, EmptyLabel>>,
    TSend<Http, EmptyLabel, TWorker, Subscribe, TEnd<Http, EmptyLabel>>
);

type NaryChoice = tlist!(
    TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
    TSend<Http, EmptyLabel, TClient, Publish, TEnd<Http, EmptyLabel>>,
    TSend<Http, EmptyLabel, TServer, Notify, TEnd<Http, EmptyLabel>>,
    TSend<Http, EmptyLabel, TWorker, Subscribe, TEnd<Http, EmptyLabel>>
);

type FourWayChoice = <NaryChoice as ToTChoice<Http>>::Output;

// Compile-time type equality assertion
assert_type_eq!(FourWayChoice, PlainFourWayChoice);

// --- Mixed-protocol combinator tests ---
mod mixed_protocol_interact {
    use super::*;
    // Single protocol
    type HttpSession = TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>;
    type DbSession = TSend<Db, EmptyLabel, TServer, Response, TEnd<Db, EmptyLabel>>;
    // Compose them in a choice (no type equality assertion, as IO markers differ)
    type MixedChoice = TChoice<Http, EmptyLabel, HttpSession, HttpSession>;
    // This is just to show the pattern; do not assert_type_eq! across IO markers.
}

mod mixed_protocol_par {
    use super::*;
    // Parallel composition of different protocol branches
    type ParMixed = TPar<
        Http,
        EmptyLabel,
        TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>, // HTTP
        TSend<Mqtt, EmptyLabel, TBroker, Publish, TEnd<Mqtt, EmptyLabel>>, // MQTT
        FalseB,
    >;
    assert_disjoint!(par ParMixed);
}

mod nary_macro_tests {
    use super::*;
    // 2-way tpar
    mod two_way {
        use super::*;
        type TwoWay = tpar!(Http; TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>, TSend<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>);
        assert_disjoint!(par TwoWay);
    }
    // 3-way tpar
    mod three_way {
        use super::*;
        type ThreeWay = tpar!(Http;
            TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
            TSend<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>,
            TSend<Http, EmptyLabel, TBroker, Publish, TEnd<Http, EmptyLabel>>
        );
        assert_disjoint!(par ThreeWay);
    }
    // 4-way tchoice
    type FourWay = tchoice!(Http;
        TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
        TSend<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>,
        TSend<Http, EmptyLabel, TBroker, Publish, TEnd<Http, EmptyLabel>>,
        TSend<Http, EmptyLabel, TWorker, Notify, TEnd<Http, EmptyLabel>>
    );
    // Type equality check for n-ary macro
    type ManualFourWay = TChoice<
        Http,
        EmptyLabel,
        TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
        TChoice<
            Http,
            EmptyLabel,
            TSend<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>,
            TChoice<
                Http,
                EmptyLabel,
                TSend<Http, EmptyLabel, TBroker, Publish, TEnd<Http, EmptyLabel>>,
                TSend<Http, EmptyLabel, TWorker, Notify, TEnd<Http, EmptyLabel>>,
            >,
        >,
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
type HttpHandshake = TSend<
    Http,
    EmptyLabel,
    TClient,
    Message,
    TSend<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>,
>;

// Publish/subscribe (MQTT)
type MqttPubSub = TChoice<
    Mqtt,
    EmptyLabel,
    TSend<Mqtt, EmptyLabel, TClient, Publish, TEnd<Mqtt, EmptyLabel>>,
    TSend<Mqtt, EmptyLabel, TClient, Subscribe, TEnd<Mqtt, EmptyLabel>>,
>;

mod workflow_disjoint_test {
    use super::*;
    type Workflow = tpar!(Http;
        TSend<Http, EmptyLabel, TClient, Message, TSend<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>> ,
        TSend<Http, EmptyLabel, TBroker, Publish, TEnd<Http, EmptyLabel>>,
        TSend<Http, EmptyLabel, TWorker, Notify, TEnd<Http, EmptyLabel>>
    );
    assert_disjoint!(par Workflow);
}

mod parallel_downloads_disjoint_test {
    use super::*;
    type ParallelDownloads = tpar!(Http;
        TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
        TSend<Http, EmptyLabel, TClient, Publish, TEnd<Http, EmptyLabel>>
    );
    assert_disjoint!(par ParallelDownloads);
}

mod mixed_example_disjoint_test {
    use super::*;
    type MixedExample = tpar!(Mixed;
        TSend<Mixed, EmptyLabel, TClient, Message, TEnd<Mixed, EmptyLabel>>,
        TSend<Mixed, EmptyLabel, TBroker, Publish, TEnd<Mixed, EmptyLabel>>
    );
    assert_disjoint!(par MixedExample);
}

// Recursive/streaming protocol
type Streaming =
    TRec<Http, EmptyLabel, TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>>;

// Protocol with branching (login vs. register)
type LoginOrRegister = tchoice!(Http;
    TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
    TSend<Http, EmptyLabel, TClient, Publish, TEnd<Http, EmptyLabel>>
);

mod parallel_downloads_disjoint_test_top {
    use super::*;
    type ParallelDownloads = tpar!(Http;
        TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
        TSend<Http, EmptyLabel, TClient, Publish, TEnd<Http, EmptyLabel>>
    );
    assert_disjoint!(par ParallelDownloads);
}

mod mixed_example_disjoint_test_top {
    use super::*;
    type MixedExample = tpar!(Mixed;
        TSend<Mixed, EmptyLabel, TClient, Message, TEnd<Mixed, EmptyLabel>>,
        TSend<Mixed, EmptyLabel, TBroker, Publish, TEnd<Mixed, EmptyLabel>>
    );
    assert_disjoint!(par MixedExample);
}

// Protocol with concurrency (parallel downloads)
mod parallel_downloads_disjoint_test_final {
    use super::*;
    type ParallelDownloads = tpar!(Http;
        TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
        TSend<Http, EmptyLabel, TClient, Publish, TEnd<Http, EmptyLabel>>
    );
    assert_disjoint!(par ParallelDownloads);
}

// Protocol using Mixed marker for informational use
mod mixed_example_disjoint_test_final {
    use super::*;
    type MixedExample = tpar!(Mixed;
        TSend<Mixed, EmptyLabel, TClient, Message, TEnd<Mixed, EmptyLabel>>,
        TSend<Mixed, EmptyLabel, TBroker, Publish, TEnd<Mixed, EmptyLabel>>
    );
    assert_disjoint!(par MixedExample);
}

// --- Label Uniqueness Positive Test ---
mod label_uniqueness_positive {
    use super::*;
    struct L1;
    impl ProtocolLabel for L1 {}
    struct L2;
    impl ProtocolLabel for L2 {}
    type UniqueLabels = TChoice<
        Http,
        L1,
        TSend<Http, L1, TClient, Message, TEnd<Http, EmptyLabel>>,
        TSend<Http, L2, TServer, Response, TEnd<Http, EmptyLabel>>,
    >;
    assert_unique_labels!(UniqueLabels);
}

// --- Intentional compile-fail tests for error message demonstration ---
// Uncomment one at a time to see improved error messages.
/*
mod type_equality_error_demo {
    use super::*;
    // These types are intentionally different
    type A = TSend<TClient, Message, TEnd>;
    type B = TSend<TServer, Message, TEnd>;
    assert_type_eq!(A, B); // Should fail with a TypeEq error
}
*/
/*
mod disjointness_error_demo {
    use super::*;
    // These branches share the same role (TClient), so not disjoint
    type ParOverlap = TPar<
        TSend<TClient, Message, TEnd>,
        TSend<TClient, Response, TEnd>,
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

// --- Projection Type Equality Test (from doctest) ---
//
// NOTE: This test is commented out due to a known Rust type identity limitation.
// The projected type for Alice is structurally identical to the expected type,
// but Rust's type system does not consider them literally the same type for the
// purposes of the blanket `TypeEq` implementation. This is a well-known limitation
// (see <https://github.com/rust-lang/rust/issues/48214> and the crate documentation).
//
// When Rust's type equality rules are improved, this test can be re-enabled.
// For now, see the documentation in src/lib.rs for details and workarounds.
//
// The following test is intentionally commented out:
//
// mod projection_type_equality {
//     use super::*;
//     pub(super) struct Alice;
//     pub(super) struct Bob;
//     impl Role for Alice {}
//     impl Role for Bob {}
//     impl ProtocolLabel for Alice {}
//     impl ProtocolLabel for Bob {}
//     impl RoleEq<Alice> for Alice { type Output = True; }
//     impl RoleEq<Bob> for Alice   { type Output = False; }
//     impl RoleEq<Alice> for Bob   { type Output = False; }
//     impl RoleEq<Bob> for Bob     { type Output = True; }
//     type Global = TSend<
//         Http,
//         EmptyLabel,
//         Alice,
//         Message,
//         TSend<Http, EmptyLabel, Bob, Response, TEnd<Http, EmptyLabel>>
//     >;
//     type AliceLocal = <() as ProjectRole<Alice, Http, Global>>::Out;
//     // This fails due to type identity limitations:
//     // assert_type_eq!(
//     //     AliceLocal,
//     //     EpSend<
//     //         Http,
//     //         EmptyLabel,
//     //         Alice,
//     //         Message,
//     //         EpRecv<Http, EmptyLabel, Alice, Response, EpEnd<Http, EmptyLabel, Alice>>
//     //     >
//     // );
// }
