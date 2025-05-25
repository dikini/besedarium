use super::*;
use crate::protocol::foundation::{
    BiDirectionalAction, CommMetadata, DefaultChan, InputAction, LocalProtocol, Message,
    OutputAction, RequestLbl, ResponseLbl, Role, SupportsActionIO,
};

// Define test roles
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Alice;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bob;

impl Role for Alice {}
impl Role for Bob {}

// Define test disjoint marker
#[derive(Debug, Clone, PartialEq, Eq)]
struct DisjointMarker;

// Define test message
#[derive(Debug, Clone)]
struct HelloMsg;
impl Message for HelloMsg {}

// Define test IO capability
#[derive(Debug, Clone)]
struct TestIO;
impl SupportsActionIO<BiDirectionalAction> for TestIO {}
impl SupportsActionIO<InputAction> for TestIO {}
impl SupportsActionIO<OutputAction> for TestIO {}

#[test]
fn test_ep_chan_send_creation() {
    type TestEnd = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;
    type TestSend = EpChanSend<
        TestIO,
        CommMetadata<DefaultChan, RequestLbl>,
        HelloMsg,
        TestEnd,
        BiDirectionalAction,
    >;

    let _send: TestSend = EpChanSend::new();
    // Verify it compiles and type checks
}

#[test]
fn test_ep_chan_recv_creation() {
    type TestEnd = EpChanEnd<TestIO, CommMetadata<DefaultChan, ResponseLbl>, BiDirectionalAction>;
    type TestRecv = EpChanRecv<
        TestIO,
        CommMetadata<DefaultChan, ResponseLbl>,
        HelloMsg,
        TestEnd,
        BiDirectionalAction,
    >;

    let _recv: TestRecv = EpChanRecv::new();
    // Verify it compiles and type checks
}

#[test]
fn test_ep_chan_choice_creation() {
    type TestEnd = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;
    type TestSend = EpChanSend<
        TestIO,
        CommMetadata<DefaultChan, RequestLbl>,
        HelloMsg,
        TestEnd,
        BiDirectionalAction,
    >;
    type TestChoice = EpChanChoice<
        TestIO,
        CommMetadata<DefaultChan, RequestLbl>,
        TestSend,
        TestEnd,
        BiDirectionalAction,
    >;

    let _choice: TestChoice = EpChanChoice::new();
    // Verify it compiles and type checks
}

#[test]
fn test_ep_chan_offer_creation() {
    type TestEnd = EpChanEnd<TestIO, CommMetadata<DefaultChan, ResponseLbl>, BiDirectionalAction>;
    type TestRecv = EpChanRecv<
        TestIO,
        CommMetadata<DefaultChan, ResponseLbl>,
        HelloMsg,
        TestEnd,
        BiDirectionalAction,
    >;
    type TestOffer = EpChanOffer<
        TestIO,
        CommMetadata<DefaultChan, ResponseLbl>,
        TestRecv,
        TestEnd,
        BiDirectionalAction,
    >;

    let _offer: TestOffer = EpChanOffer::new();
    // Verify it compiles and type checks
}

#[test]
fn test_ep_chan_par_creation() {
    type TestEnd = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;
    type TestSend = EpChanSend<
        TestIO,
        CommMetadata<DefaultChan, RequestLbl>,
        HelloMsg,
        TestEnd,
        BiDirectionalAction,
    >;
    type TestPar = EpChanPar<
        TestIO,
        CommMetadata<DefaultChan, RequestLbl>,
        TestSend,
        TestEnd,
        DisjointMarker,
        BiDirectionalAction,
    >;

    let _par: TestPar = EpChanPar::new();
    // Verify it compiles and type checks
}

#[test]
fn test_ep_chan_end_creation() {
    type TestEnd = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;

    let _end: TestEnd = EpChanEnd::new();
    // Verify it compiles and type checks
}

#[test]
fn test_ep_chan_start_creation() {
    type TestEnd = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;
    type TestStart =
        EpChanStart<TestIO, CommMetadata<DefaultChan, RequestLbl>, TestEnd, BiDirectionalAction>;

    let _start: TestStart = EpChanStart::new();
    // Verify it compiles and type checks
}

#[test]
fn test_simple_type_aliases() {
    // Test simple type aliases work correctly
    type TestEnd = SimpleEpEnd<TestIO>;
    type TestSend = SimpleEpSend<TestIO, HelloMsg, TestEnd>;
    type TestRecv = SimpleEpRecv<TestIO, HelloMsg, TestEnd>;

    let _end: TestEnd = EpChanEnd::new();
    let _send: TestSend = EpChanSend::new();
    let _recv: TestRecv = EpChanRecv::new();
}

#[test]
fn test_local_protocol_trait() {
    // Verify all endpoint types implement LocalProtocol
    type TestEnd = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;
    type TestSend = EpChanSend<
        TestIO,
        CommMetadata<DefaultChan, RequestLbl>,
        HelloMsg,
        TestEnd,
        BiDirectionalAction,
    >;
    type TestRecv = EpChanRecv<
        TestIO,
        CommMetadata<DefaultChan, ResponseLbl>,
        HelloMsg,
        TestEnd,
        BiDirectionalAction,
    >;

    fn requires_local_protocol<T: LocalProtocol>(_: &T) {}

    let end: TestEnd = EpChanEnd::new();
    let send: TestSend = EpChanSend::new();
    let recv: TestRecv = EpChanRecv::new();

    requires_local_protocol(&end);
    requires_local_protocol(&send);
    requires_local_protocol(&recv);
}

#[test]
fn test_io_capability_constraints() {
    // Test that endpoints enforce IO capability constraints
    type ValidSend = EpChanSend<
        TestIO,
        CommMetadata<DefaultChan, RequestLbl>,
        HelloMsg,
        SimpleEpEnd<TestIO>,
        BiDirectionalAction,
    >;
    type ValidRecv = EpChanRecv<
        TestIO,
        CommMetadata<DefaultChan, ResponseLbl>,
        HelloMsg,
        SimpleEpEnd<TestIO>,
        BiDirectionalAction,
    >;

    let _send: ValidSend = EpChanSend::new();
    let _recv: ValidRecv = EpChanRecv::new();

    // These should compile because TestIO supports BiDirectionalAction
}

#[test]
fn test_complex_endpoint_composition() {
    // Test complex compositions of endpoint types
    type Step1 = SimpleEpEnd<TestIO>;
    type Step2 = SimpleEpRecv<TestIO, HelloMsg, Step1>;
    type Step3 = SimpleEpSend<TestIO, HelloMsg, Step2>;
    type Protocol = SimpleEpChoice<TestIO, Step3, Step1>;

    let _protocol: Protocol = EpChanChoice::new();
    // Verify complex compositions work
}

#[test]
fn test_different_action_io_types() {
    // Test endpoints with different ActionIOTMarker types
    type InputEnd = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, InputAction>;
    type OutputEnd = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, OutputAction>;
    type BiDirEnd = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;

    let _input: InputEnd = EpChanEnd::new();
    let _output: OutputEnd = EpChanEnd::new();
    let _bidir: BiDirEnd = EpChanEnd::new();

    // These should all be different types at compile time
}
