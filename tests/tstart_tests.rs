//! Tests for TStart combinator functionality
//!
//! This module verifies that the TStart combinator behaves correctly, including:
//! - Basic type-level structure
//! - Proper projection to EpStart
//! - Label preservation
//! - Composition with other combinators

use besedarium::protocol::transforms::*;
use besedarium::*;

// --- Custom Types for Testing ---

// Roles
struct Client;
struct Server;

// Messages
struct Message;
struct Response;

// Labels
struct StartLabel;
struct SendLabel;
struct RecvLabel;
struct EndLabel;

// Implementations
impl Role for Client {}
impl Role for Server {}

impl ProtocolLabel for StartLabel {}
impl ProtocolLabel for SendLabel {}
impl ProtocolLabel for RecvLabel {}
impl ProtocolLabel for EndLabel {}

// --- Tests ---

#[test]
fn test_tstart_basic() {
    // Basic type check - can we define a protocol with TStart?
    type Protocol = TStart<
        Http,
        StartLabel,
        TSend<
            Http,
            SendLabel,
            Client,
            Message,
            TRecv<Http, RecvLabel, Server, Response, TEnd<Http, EndLabel>>,
        >,
    >;

    // This is just a type-level test, no need for assertions
}

#[test]
fn test_tstart_projection_client() {
    // Project a protocol with TStart onto Client
    // We'll simplify the test and just use TStart<Http, StartLabel, TEnd<Http, EndLabel>>
    type Global = TStart<Http, StartLabel, TEnd<Http, EndLabel>>;

    type ClientLocal = <() as ProjectRole<Client, Http, Global>>::Out;

    // Expected: EpStart<Http, StartLabel, Client, EpEnd<Http, EndLabel, Client>>
    type Expected = EpStart<Http, StartLabel, Client, EpEnd<Http, EndLabel, Client>>;

    // Type-level assertion (will fail to compile if types don't match)
    assert_type_eq!(ClientLocal, Expected);
}

#[test]
fn test_tstart_projection_server() {
    // Project a protocol with TStart onto Server (simplified)
    type Global = TStart<Http, StartLabel, TEnd<Http, EndLabel>>;

    type ServerLocal = <() as ProjectRole<Server, Http, Global>>::Out;

    // Expected: EpStart<Http, StartLabel, Server, EpEnd<...>>
    type Expected = EpStart<Http, StartLabel, Server, EpEnd<Http, EndLabel, Server>>;

    // Type-level assertion (will fail to compile if types don't match)
    assert_type_eq!(ServerLocal, Expected);
}

#[test]
fn test_tstart_composition() {
    // Test that TStart composes correctly with other protocols
    type Part1 = TStart<Http, StartLabel, TEnd<Http, EndLabel>>;
    type Part2 = TSend<Http, SendLabel, Client, Message, TEnd<Http, EndLabel>>;

    type Composed = <Part1 as TSession<Http>>::Compose<Part2>;

    // Expected: TStart<Http, StartLabel, TSend<...>>
    type Expected =
        TStart<Http, StartLabel, TSend<Http, SendLabel, Client, Message, TEnd<Http, EndLabel>>>;

    // Type-level assertion (will fail to compile if types don't match)
    assert_type_eq!(Composed, Expected);
}

#[test]
fn test_tstart_label_preservation() {
    // Define a protocol with TStart and verify label preservation
    type Global = TStart<Http, StartLabel, TEnd<Http, EndLabel>>;

    // The label should be extracted as StartLabel
    type ExtractedLabel = <Global as GetProtocolLabel>::Label;

    // Type-level assertion for label
    assert_type_eq!(ExtractedLabel, StartLabel);
}
