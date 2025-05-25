use super::*;
use crate::protocol::foundation::{CommMetadata, GlobalProtocol, Message, Role};

// Define test roles
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Alice;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bob;

impl Role for Alice {}
impl Role for Bob {}

// Define test message
#[derive(Debug, Clone)]
struct HelloMsg;
impl Message for HelloMsg {}

// Define test disjoint marker
#[derive(Debug, Clone, PartialEq, Eq)]
struct DisjointMarker;

#[test]
fn test_tchan_send_creation() {
    type TestEnd = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type TestSend =
        TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, TestEnd, BiDirectionalAction>;

    let _send: TestSend = TChanSend::new();
    // Verify it compiles and type checks
}

#[test]
fn test_tchan_recv_creation() {
    type TestEnd = TChanEnd<DefaultChan, ResponseLbl, BiDirectionalAction>;
    type TestRecv =
        TChanRecv<Bob, Alice, DefaultChan, ResponseLbl, HelloMsg, TestEnd, BiDirectionalAction>;

    let _recv: TestRecv = TChanRecv::new();
    // Verify it compiles and type checks
}

#[test]
fn test_protocol_composition() {
    // Test that protocols can be composed correctly
    type End = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type Send = TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>;
    type Recv = TChanRecv<Bob, Alice, DefaultChan, RequestLbl, HelloMsg, Send, BiDirectionalAction>;

    // Should be able to create these types
    let _protocol: Recv = TChanRecv::new();
}

#[test]
fn test_tchan_choice_creation() {
    type TestEnd = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type TestSend =
        TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, TestEnd, BiDirectionalAction>;
    type TestChoice =
        TChanChoice<Alice, DefaultChan, RequestLbl, TestSend, TestEnd, BiDirectionalAction>;

    let _choice: TestChoice = TChanChoice::new();
    // Verify it compiles and type checks
}

#[test]
fn test_tchan_par_creation() {
    type TestEnd = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type TestSend =
        TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, TestEnd, BiDirectionalAction>;
    type TestPar =
        TChanPar<DefaultChan, RequestLbl, TestSend, TestEnd, DisjointMarker, BiDirectionalAction>;

    let _par: TestPar = TChanPar::new();
    // Verify it compiles and type checks
}

#[test]
fn test_tchan_start_creation() {
    type TestEnd = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type TestStart = TChanStart<DefaultChan, RequestLbl, TestEnd, BiDirectionalAction>;

    let _start: TestStart = TChanStart::new();
    // Verify it compiles and type checks
}

#[test]
fn test_simple_type_aliases() {
    // Test simple type aliases work correctly
    type TestEnd = SimpleChannelEnd;
    type TestSend = SimpleChannelSend<Alice, Bob, HelloMsg, TestEnd>;
    type TestRecv = SimpleChannelRecv<Bob, Alice, HelloMsg, TestEnd>;

    let _end: TestEnd = TChanEnd::new();
    let _send: TestSend = TChanSend::new();
    let _recv: TestRecv = TChanRecv::new();
}

#[test]
fn test_type_safety() {
    // Different channels should be type-distinct at the type level
    type Chan1End = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type Chan1Send =
        TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, Chan1End, BiDirectionalAction>;

    // Create instances to verify types work
    let _chan1: Chan1Send = TChanSend::new();

    // Verify different action I/O types are distinct at the type level
    use crate::protocol::foundation::{InputAction, OutputAction};
    type InputEnd = TChanEnd<DefaultChan, RequestLbl, InputAction>;
    type OutputEnd = TChanEnd<DefaultChan, RequestLbl, OutputAction>;

    let _input_end: InputEnd = TChanEnd::new();
    let _output_end: OutputEnd = TChanEnd::new();

    // These should be different types (verified by the fact they compile)
    assert_ne!(
        std::any::TypeId::of::<InputEnd>(),
        std::any::TypeId::of::<OutputEnd>()
    );
}

#[test]
fn test_global_protocol_trait() {
    type TestEnd = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;

    // Verify GlobalProtocol trait is properly implemented
    fn requires_global_protocol<P: GlobalProtocol>(_p: P) {}

    let end: TestEnd = TChanEnd::new();
    requires_global_protocol(end);
}

#[test]
fn test_metadata_access() {
    type TestSend = TChanSend<
        Alice,
        Bob,
        DefaultChan,
        RequestLbl,
        HelloMsg,
        SimpleChannelEnd,
        BiDirectionalAction,
    >;

    // Test metadata access where possible
    let _metadata = CommMetadata::new(DefaultChan, RequestLbl);

    // Create send instance
    let _send: TestSend = TChanSend::new();
}
