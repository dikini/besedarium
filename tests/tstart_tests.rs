//! Tests for TStart combinator functionality
//!
//! This module verifies that the TStart combinator behaves correctly at the type level,
//! focusing on basic structure and composition properties.

// Import all types and traits from besedarium
use besedarium::*;

// --- Define test types ---
// Roles for testing
struct TClient;
impl Role for TClient {}

struct TServer;
impl Role for TServer {}

// Labels for testing
struct TStartLabel;
impl ProtocolLabel for TStartLabel {}

struct TEndLabel;
impl ProtocolLabel for TEndLabel {}

struct TSendLabel;
impl ProtocolLabel for TSendLabel {}

// Message types
struct TestMessage;
struct TestResponse;

#[test]
fn test_tstart_basic() {
    // Basic type check - can we define a protocol with TStart?
    type Protocol = TStart<Http, TStartLabel, TEnd<Http, TEndLabel>>;

    // Verify that TStart is a TSession (compiles if true)
    fn _assert_is_tsession<T: TSession<Http>>() {}
    fn _verify() {
        _assert_is_tsession::<Protocol>();
    }
}

#[test]
fn test_tstart_composition() {
    // Test that TStart composes correctly with other protocols
    type Part1 = TStart<Http, TStartLabel, TEnd<Http, TEndLabel>>;
    type Part2 = TSend<Http, TSendLabel, TClient, TestMessage, TEnd<Http, TEndLabel>>;

    type Composed = <Part1 as TSession<Http>>::Compose<Part2>;

    // Expected: TStart<Http, TStartLabel, TSend<...>>
    type Expected = TStart<
        Http,
        TStartLabel,
        TSend<Http, TSendLabel, TClient, TestMessage, TEnd<Http, TEndLabel>>,
    >;

    // Type equality check (compiles if types are equal)
    fn _assert_eq<T, U>()
    where
        T: TypeEq<U>,
    {
    }
    fn _verify_eq() {
        _assert_eq::<Composed, Expected>();
    }
}

#[test]
fn test_tstart_is_not_empty() {
    // TStart should never be an empty session
    type Protocol = TStart<Http, TStartLabel, TEnd<Http, TEndLabel>>;

    const IS_EMPTY: bool = Protocol::IS_EMPTY;
    const _EXPECTED: bool = false;

    fn _verify() {
        const _: () = assert!(IS_EMPTY == _EXPECTED);
    }
}

#[test]
fn test_tstart_label_preservation() {
    // Test that TStart preserves its label for GetProtocolLabel
    type Protocol = TStart<Http, TStartLabel, TEnd<Http, TEndLabel>>;

    type ExtractedLabel = <Protocol as GetProtocolLabel>::Label;

    // Verify label type equality (compiles if types are equal)
    fn _assert_eq<T, U>()
    where
        T: TypeEq<U>,
    {
    }
    fn _verify_eq() {
        _assert_eq::<ExtractedLabel, TStartLabel>();
    }
}
