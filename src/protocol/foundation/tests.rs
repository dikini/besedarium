use super::*;
use crate::protocol::global::{
    TChanChoice, TChanEnd, TChanOffer, TChanPar, TChanRecv, TChanSend, TChanStart,
};
use crate::protocol::local::{
    EpChanChoice, EpChanEnd, EpChanOffer, EpChanPar, EpChanRecv, EpChanSend, EpChanStart,
};

// ============================================================================
// Test Infrastructure
// ============================================================================

// Test roles
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Alice;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bob;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Carol;

impl Role for Alice {}
impl Role for Bob {}
impl Role for Carol {}

// Test messages
#[derive(Debug, Clone)]
struct HelloMsg(String);
#[derive(Debug, Clone)]
struct AckMsg;
#[derive(Debug, Clone)]
struct DataMsg(Vec<u8>);

impl Message for HelloMsg {}
impl Message for AckMsg {}
impl Message for DataMsg {}

// Test I/O capability
#[derive(Debug, Clone)]
struct TestIO;

impl SupportsActionIO<InputAction> for TestIO {}
impl SupportsActionIO<OutputAction> for TestIO {}
impl SupportsActionIO<BiDirectionalAction> for TestIO {}

// Additional test channel and label types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct AuthChan;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct DataChan;

impl ChanId for AuthChan {}
impl ChanId for DataChan {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct LoginLbl;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct StatusLbl;

impl MsgLbl for LoginLbl {}
impl MsgLbl for StatusLbl {}

// ============================================================================
// CommMetadata Tests (8 tests)
// ============================================================================

#[test]
fn test_comm_metadata_creation() {
    let metadata = CommMetadata::new(DefaultChan, RequestLbl);
    assert_eq!(metadata.chan_id, DefaultChan);
    assert_eq!(metadata.msg_lbl, RequestLbl);
}

#[test]
fn test_comm_metadata_field_access() {
    let metadata = CommMetadata::new(HandshakeChan, ResponseLbl);
    assert_eq!(metadata.chan_id, HandshakeChan);
    assert_eq!(metadata.msg_lbl, ResponseLbl);
}

#[test]
fn test_comm_metadata_trait_implementations() {
    let meta1 = CommMetadata::new(DefaultChan, RequestLbl);
    let meta2 = CommMetadata::new(DefaultChan, RequestLbl);

    // Test Clone
    let cloned = meta1.clone();
    assert_eq!(meta1, cloned);

    // Test PartialEq and Eq
    assert_eq!(meta1, meta2);

    // Test Debug (just ensure it doesn't panic)
    let _ = format!("{:?}", meta1);


    // Test Hash (put in a hash set to verify)
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(meta1);
    set.insert(meta2); // Should not increase size
    assert_eq!(set.len(), 1);
}

#[test]
fn test_metadata_trait_implementation() {
    let metadata = CommMetadata::new(AuthChan, LoginLbl);

    // Test Metadata trait methods using the trait methods
    assert_eq!(
        *<CommMetadata<AuthChan, LoginLbl> as Metadata>::chan_id(&metadata),
        AuthChan
    );
    assert_eq!(
        *<CommMetadata<AuthChan, LoginLbl> as Metadata>::msg_lbl(&metadata),
        LoginLbl
    );

#[test]
fn test_comm_metadata_different_channel_types() {
    type Meta1 = CommMetadata<DefaultChan, RequestLbl>;
    type Meta2 = CommMetadata<HandshakeChan, RequestLbl>;

    let meta1 = Meta1::new(DefaultChan, RequestLbl);
    let meta2 = Meta2::new(HandshakeChan, RequestLbl);

    // These are different types, test each separately
    assert_eq!(meta1.chan_id, DefaultChan);
    assert_eq!(meta2.chan_id, HandshakeChan);
}

#[test]
fn test_comm_metadata_trait_implementation() {
    let metadata = CommMetadata::new(DataChan, StatusLbl);

    // Test CommMetadataTrait methods
    assert_eq!(*CommMetadataTrait::chan_id(&metadata), DataChan);
    assert_eq!(*CommMetadataTrait::msg_lbl(&metadata), StatusLbl);

    // Test construction via trait
    let new_metadata =
        <CommMetadata<DataChan, StatusLbl> as CommMetadataTrait>::new(DataChan, StatusLbl);
    assert_eq!(new_metadata, metadata);
}

#[test]
fn test_extensible_metadata_pattern() {
    // This test verifies that the metadata system is extensible
    // by using multiple metadata types
    let std_metadata = CommMetadata::new(DefaultChan, RequestLbl);
    let auth_metadata = CommMetadata::new(AuthChan, LoginLbl);

    // Function that accepts any metadata implementing the trait
    fn process_metadata<M: Metadata>(meta: &M) -> (String, String) {
        (
            format!("{:?}", meta.chan_id()),
            format!("{:?}", meta.msg_lbl()),
        )
    }

    let (chan1, lbl1) = process_metadata(&std_metadata);
    let (chan2, lbl2) = process_metadata(&auth_metadata);

    assert!(chan1.contains("DefaultChan"));
    assert!(lbl1.contains("RequestLbl"));
    assert!(chan2.contains("AuthChan"));
    assert!(lbl2.contains("LoginLbl"));
}

#[test]
fn test_comm_metadata_hash_consistency() {
    use std::collections::HashMap;

    // Test with homogeneous metadata types
    let mut map: HashMap<CommMetadata<DefaultChan, RequestLbl>, &str> = HashMap::new();

    let key1 = CommMetadata::new(DefaultChan, RequestLbl);
    let key2 = CommMetadata::new(DefaultChan, RequestLbl);

    map.insert(key1, "first");
    map.insert(key2, "second"); // Should overwrite since keys are equal

    assert_eq!(map.len(), 1);
    assert_eq!(
        map.get(&CommMetadata::new(DefaultChan, RequestLbl)),
        Some(&"second")
    );
}

// ============================================================================
// Global Protocol Types Testing (12 tests)
// ============================================================================

#[test]
fn test_tchan_send_creation() {
    type TestSend = TChanSend<
        Alice,
        Bob,
        DefaultChan,
        RequestLbl,
        HelloMsg,
        TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;

    // Verify it implements GlobalProtocol
    fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_global_protocol(std::marker::PhantomData::<TestSend>);
}

#[test]
fn test_tchan_recv_creation() {
    type TestRecv = TChanRecv<
        Alice,
        Bob,
        DefaultChan,
        RequestLbl,
        HelloMsg,
        TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;

    // Verify it implements GlobalProtocol
    fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_global_protocol(std::marker::PhantomData::<TestRecv>);
}

#[test]
fn test_tchan_choice_creation() {
    type LeftChoice = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type RightChoice = TChanEnd<HandshakeChan, ResponseLbl, BiDirectionalAction>;
    type TestChoice =
        TChanChoice<Alice, DefaultChan, RequestLbl, LeftChoice, RightChoice, BiDirectionalAction>;

    // Verify it implements GlobalProtocol
    fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_global_protocol(std::marker::PhantomData::<TestChoice>);
}

#[test]
fn test_tchan_offer_creation() {
    type LeftOffer = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type RightOffer = TChanEnd<HandshakeChan, ResponseLbl, BiDirectionalAction>;
    type TestOffer =
        TChanOffer<Alice, DefaultChan, RequestLbl, LeftOffer, RightOffer, BiDirectionalAction>;

    // Verify it implements GlobalProtocol
    fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_global_protocol(std::marker::PhantomData::<TestOffer>);
}

#[test]
fn test_tchan_par_creation() {
    type LeftProt = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type RightProt = TChanEnd<HandshakeChan, ResponseLbl, BiDirectionalAction>;
    // Need to provide a disjoint marker (using unit type for simplicity)
    type DisjointMarker = ();
    type TestPar =
        TChanPar<DefaultChan, RequestLbl, LeftProt, RightProt, DisjointMarker, BiDirectionalAction>;

    // Verify it implements GlobalProtocol
    fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_global_protocol(std::marker::PhantomData::<TestPar>);
}

#[test]
fn test_tchan_end_creation() {
    type TestEnd = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;

  // Verify it implements GlobalProtocol
    fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_global_protocol(std::marker::PhantomData::<TestEnd>);
}

#[test]
fn test_tchan_start_creation() {
    type StartProtocol = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type TestStart = TChanStart<DefaultChan, RequestLbl, StartProtocol, BiDirectionalAction>;

    // Verify it implements GlobalProtocol
    fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_global_protocol(std::marker::PhantomData::<TestStart>);
}

#[test]
fn test_global_protocol_composition() {
    // Test composing multiple global protocol types
    type SendHello = TChanSend<
        Alice,
        Bob,
        DefaultChan,
        RequestLbl,
        HelloMsg,
        TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type RecvAck = TChanRecv<
        Bob,
        Alice,
        DefaultChan,
        ResponseLbl,
        AckMsg,
        TChanEnd<DefaultChan, ResponseLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;

    fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_global_protocol(std::marker::PhantomData::<SendHello>);
    requires_global_protocol(std::marker::PhantomData::<RecvAck>);
}

#[test]
fn test_global_protocol_trait_bounds() {
    // Verify that GlobalProtocol types satisfy their trait bounds
    fn check_global_bounds<T: GlobalProtocol + Send + Sync + 'static + Debug>(
        _: std::marker::PhantomData<T>,
    ) {
    }

    type TestGlobal = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    check_global_bounds(std::marker::PhantomData::<TestGlobal>);
}

#[test]
fn test_global_type_distinction() {
    // Test that different global types are indeed different
    type SendType = TChanSend<
        Alice,
        Bob,
        DefaultChan,
        RequestLbl,
        HelloMsg,
        TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type RecvType = TChanRecv<
        Alice,
        Bob,
        DefaultChan,
        RequestLbl,
        HelloMsg,
        TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;

    // These should be different types (verified at compile time)
    fn accepts_send(_: std::marker::PhantomData<SendType>) {}
    fn accepts_recv(_: std::marker::PhantomData<RecvType>) {}

    accepts_send(std::marker::PhantomData::<SendType>);
    accepts_recv(std::marker::PhantomData::<RecvType>);
}

#[test]
fn test_global_metadata_integration() {
    // Test that global types properly integrate with CommMetadata
    type TestMetadata = CommMetadata<DefaultChan, RequestLbl>;
    type TestGlobal = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;

    let metadata = TestMetadata::new(DefaultChan, RequestLbl);

    // Function that requires both metadata and global protocol
    fn check_integration<M, G>(_meta: M, _global: std::marker::PhantomData<G>)
    where
        M: Metadata,
        G: GlobalProtocol,
    {
        // Type constraints verified at compile time
    }

    check_integration(metadata, std::marker::PhantomData::<TestGlobal>);
}

#[test]
fn test_global_action_io_compatibility() {
    // Test that global types work with different action I/O types
    type InputGlobal = TChanEnd<DefaultChan, RequestLbl, InputAction>;
    type OutputGlobal = TChanEnd<DefaultChan, RequestLbl, OutputAction>;
    type BiGlobal = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;

    fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_global_protocol(std::marker::PhantomData::<InputGlobal>);
    requires_global_protocol(std::marker::PhantomData::<OutputGlobal>);
    requires_global_protocol(std::marker::PhantomData::<BiGlobal>);
}

// ============================================================================
// Local Endpoint Types Testing (12 tests)
// ============================================================================

#[test]
fn test_epchan_send_creation() {
    type TestSend = EpChanSend<
        TestIO,
        CommMetadata<DefaultChan, RequestLbl>,
        HelloMsg,
        EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>,
        BiDirectionalAction,
    >;

    // Verify it implements LocalProtocol
    fn requires_local_protocol<T: LocalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_local_protocol(std::marker::PhantomData::<TestSend>);
}

#[test]
fn test_epchan_recv_creation() {
    type TestRecv = EpChanRecv<
        TestIO,
        CommMetadata<DefaultChan, RequestLbl>,
        HelloMsg,
        EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>,
        BiDirectionalAction,
    >;

    // Verify it implements LocalProtocol
    fn requires_local_protocol<T: LocalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_local_protocol(std::marker::PhantomData::<TestRecv>);
}

#[test]
fn test_epchan_choice_creation() {
    type LeftChoice = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;
    type RightChoice =
        EpChanEnd<TestIO, CommMetadata<HandshakeChan, ResponseLbl>, BiDirectionalAction>;
    type TestChoice = EpChanChoice<
        TestIO,
        CommMetadata<DefaultChan, RequestLbl>,
        LeftChoice,
        RightChoice,
        BiDirectionalAction,
    >;

    // Verify it implements LocalProtocol
    fn requires_local_protocol<T: LocalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_local_protocol(std::marker::PhantomData::<TestChoice>);
}

#[test]
fn test_epchan_offer_creation() {
    type LeftOffer = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;
    type RightOffer =
        EpChanEnd<TestIO, CommMetadata<HandshakeChan, ResponseLbl>, BiDirectionalAction>;
    type TestOffer = EpChanOffer<
        TestIO,
        CommMetadata<DefaultChan, RequestLbl>,
        LeftOffer,
        RightOffer,
        BiDirectionalAction,
    >;

    // Verify it implements LocalProtocol
    fn requires_local_protocol<T: LocalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_local_protocol(std::marker::PhantomData::<TestOffer>);
}

#[test]
fn test_epchan_par_creation() {
    type LeftEp = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;
    type RightEp = EpChanEnd<TestIO, CommMetadata<HandshakeChan, ResponseLbl>, BiDirectionalAction>;
    // Need disjoint marker and correct metadata parameter
    type DisjointMarker = ();
    type TestPar = EpChanPar<
        TestIO,
        CommMetadata<DefaultChan, RequestLbl>,
        LeftEp,
        RightEp,
        DisjointMarker,
        BiDirectionalAction,
    >;

    // Verify it implements LocalProtocol
    fn requires_local_protocol<T: LocalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_local_protocol(std::marker::PhantomData::<TestPar>);
}

#[test]
fn test_epchan_end_creation() {
    type TestEnd = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;

  // Verify it implements LocalProtocol
    fn requires_local_protocol<T: LocalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_local_protocol(std::marker::PhantomData::<TestEnd>);
}

#[test]
fn test_epchan_start_creation() {
    type StartProtocol =
        EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;
    type TestStart = EpChanStart<
        TestIO,
        CommMetadata<DefaultChan, RequestLbl>,
        StartProtocol,
        BiDirectionalAction,
    >;
    // Verify it implements LocalProtocol
    fn requires_local_protocol<T: LocalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_local_protocol(std::marker::PhantomData::<TestStart>);
}

#[test]
fn test_local_protocol_composition() {
    // Test composing multiple local endpoint types
    type SendHello = EpChanSend<
        TestIO,
        CommMetadata<DefaultChan, RequestLbl>,
        HelloMsg,
        EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type RecvAck = EpChanRecv<
        TestIO,
        CommMetadata<DefaultChan, ResponseLbl>,
        AckMsg,
        EpChanEnd<TestIO, CommMetadata<DefaultChan, ResponseLbl>, BiDirectionalAction>,
        BiDirectionalAction,
    >;

    fn requires_local_protocol<T: LocalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_local_protocol(std::marker::PhantomData::<SendHello>);
    requires_local_protocol(std::marker::PhantomData::<RecvAck>);
}

#[test]
fn test_local_protocol_trait_bounds() {
    // Verify that LocalProtocol types satisfy their trait bounds
    fn check_local_bounds<T: LocalProtocol + Send + Sync + 'static + Debug>(
        _: std::marker::PhantomData<T>,
    ) {
    }

    type TestLocal = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;
    check_local_bounds(std::marker::PhantomData::<TestLocal>);
}

#[test]
fn test_local_io_capability_constraints() {
    // Test that local endpoints properly constrain I/O capabilities
    fn check_io_constraints<IO, AIO>(
        phantom_io: std::marker::PhantomData<IO>,
        phantom_aio: std::marker::PhantomData<AIO>,
    ) where
        IO: SupportsActionIO<AIO>,
        AIO: ActionIOTMarker,
    {
        // Constraint verification happens at compile time
        let _ = phantom_io;
        let _ = phantom_aio;
    }

    check_io_constraints(
        std::marker::PhantomData::<TestIO>,
        std::marker::PhantomData::<InputAction>,
    );
    check_io_constraints(
        std::marker::PhantomData::<TestIO>,
        std::marker::PhantomData::<OutputAction>,
    );
    check_io_constraints(
        std::marker::PhantomData::<TestIO>,
        std::marker::PhantomData::<BiDirectionalAction>,
    );
}

#[test]
fn test_local_metadata_integration() {
    // Test that local endpoints properly integrate with CommMetadata
    type TestMetadata = CommMetadata<DefaultChan, RequestLbl>;
    type TestLocal = EpChanEnd<TestIO, TestMetadata, BiDirectionalAction>;

    let metadata = TestMetadata::new(DefaultChan, RequestLbl);

    // Function that requires both metadata and local protocol
    fn check_integration<M, L>(_meta: M, _local: std::marker::PhantomData<L>)
    where
        M: Metadata,
        L: LocalProtocol,
    {
        // Type constraints verified at compile time
    }

  check_integration(metadata, std::marker::PhantomData::<TestLocal>);
}

#[test]
fn test_local_global_protocol_integration() {
    // Test that local and global protocols can be used together
    type TestGlobal = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type TestLocal = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;

  fn check_protocol_bounds<G, L>()
    where
        G: GlobalProtocol,
        L: LocalProtocol,
    {
        // Type constraints verified at compile time
    }

  check_protocol_bounds::<TestGlobal, TestLocal>();
}

// ============================================================================
// Action I/O System Testing (6 tests)
// ============================================================================

#[test]
fn test_action_io_markers() {
    let input = InputAction;
    let output = OutputAction;
    let bidirectional = BiDirectionalAction;

    // Test that they implement ActionIOTMarker
    fn requires_marker<T: ActionIOTMarker>(_: T) {}
    requires_marker(input);
    requires_marker(output);
    requires_marker(bidirectional);
}

#[test]
fn test_supports_action_io_tcp() {

  // Test TcpOnlySessionIO supports all action types
    assert!(<TcpOnlySessionIO as SupportsActionIO<InputAction>>::supports_action_io());
    assert!(<TcpOnlySessionIO as SupportsActionIO<OutputAction>>::supports_action_io());
    assert!(<TcpOnlySessionIO as SupportsActionIO<
        BiDirectionalAction,
    >>::supports_action_io());
}

#[test]
fn test_supports_action_io_http() {
    // Test HttpOnlySessionIO supports output and bidirectional only
    assert!(<HttpOnlySessionIO as SupportsActionIO<OutputAction>>::supports_action_io());
    assert!(<HttpOnlySessionIO as SupportsActionIO<
        BiDirectionalAction,
    >>::supports_action_io());

    // Note: HttpOnlySessionIO doesn't implement SupportsActionIO<InputAction>
    // This is verified by the type system - if it did implement it, this would compile:
    // assert!(<HttpOnlySessionIO as SupportsActionIO<InputAction>>::supports_action_io());
}

#[test]
fn test_custom_io_capability() {
    // Test our TestIO supports all actions
    assert!(<TestIO as SupportsActionIO<InputAction>>::supports_action_io());
    assert!(<TestIO as SupportsActionIO<OutputAction>>::supports_action_io());
    assert!(<TestIO as SupportsActionIO<BiDirectionalAction>>::supports_action_io());
}

#[test]
fn test_action_io_integration_with_protocols() {
    // Test that action I/O types integrate properly with protocol types
    type InputLocal = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, InputAction>;
    type OutputLocal = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, OutputAction>;
    type BiLocal = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;

    fn requires_local_protocol<T: LocalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_local_protocol(std::marker::PhantomData::<InputLocal>);
    requires_local_protocol(std::marker::PhantomData::<OutputLocal>);
    requires_local_protocol(std::marker::PhantomData::<BiLocal>);
}

#[test]
fn test_action_io_constraint_verification() {
    // Test compile-time verification of I/O capability constraints
    fn verify_io_constraint<IO, AIO>()
    where
        IO: SupportsActionIO<AIO>,
        AIO: ActionIOTMarker,
    {
        // Constraint verified at compile time
    }

    // These should all compile successfully
    verify_io_constraint::<TcpOnlySessionIO, InputAction>();
    verify_io_constraint::<TcpOnlySessionIO, OutputAction>();
    verify_io_constraint::<TcpOnlySessionIO, BiDirectionalAction>();
    verify_io_constraint::<HttpOnlySessionIO, OutputAction>();
    verify_io_constraint::<HttpOnlySessionIO, BiDirectionalAction>();
}

// ============================================================================
// Foundation Traits Testing (4 tests)
// ============================================================================

#[test]
fn test_role_trait_implementation() {
    let alice = Alice;
    let bob = Bob;
    let carol = Carol;

    // Test Clone
    let alice_clone = alice.clone();
    assert_eq!(alice, alice_clone);

    // Test Debug
    let _ = format!("{:?}", alice);
    let _ = format!("{:?}", bob);
    let _ = format!("{:?}", carol);

    // Test PartialEq and Eq
    assert_eq!(alice, alice);
    // Note: Cannot compare different role types directly

    // Test Hash - use separate sets for each role type to avoid type mismatch
    use std::collections::HashSet;
    let mut alice_set = HashSet::new();
    alice_set.insert(alice);
    assert_eq!(alice_set.len(), 1);
}

#[test]
fn test_message_trait_implementation() {
    let hello = HelloMsg("hello".to_string());
    let ack = AckMsg;
    let data = DataMsg(vec![1, 2, 3]);

    // Test Clone
    let hello_clone = hello.clone();
    let ack_clone = ack.clone();
    let data_clone = data.clone();

    // Test Debug
    let _ = format!("{:?}", hello);
    let _ = format!("{:?}", ack);
    let _ = format!("{:?}", data);

    // Test that they implement Message trait
    fn requires_message<T: Message>(_: T) {}
    requires_message(hello_clone);
    requires_message(ack_clone);
    requires_message(data_clone);
}

#[test]
fn test_chanid_trait_implementation() {
    let default_chan = DefaultChan;
    let handshake_chan = HandshakeChan;
    let auth_chan = AuthChan;
    let data_chan = DataChan;

    // Test that they implement ChanId trait
    fn requires_chanid<T: ChanId>(_: T) {}
    requires_chanid(default_chan);
    requires_chanid(handshake_chan);
    requires_chanid(auth_chan);
    requires_chanid(data_chan);

    // Test Clone and PartialEq
    assert_eq!(DefaultChan, DefaultChan);
    // Note: Cannot compare different channel types directly

    // Test Debug
    let _ = format!("{:?}", DefaultChan);
    let _ = format!("{:?}", HandshakeChan);
}

#[test]
fn test_msglbl_trait_implementation() {
    let request_lbl = RequestLbl;
    let response_lbl = ResponseLbl;
    let login_lbl = LoginLbl;
    let status_lbl = StatusLbl;

    // Test that they implement MsgLbl trait
    fn requires_msglbl<T: MsgLbl>(_: T) {}
    requires_msglbl(request_lbl);
    requires_msglbl(response_lbl);
    requires_msglbl(login_lbl);
    requires_msglbl(status_lbl);

    // Test Clone and PartialEq
    assert_eq!(RequestLbl, RequestLbl);
    // Note: Cannot compare different label types directly

    // Test Debug
    let _ = format!("{:?}", RequestLbl);
    let _ = format!("{:?}", ResponseLbl);
}

// ============================================================================
// Legacy Tests (preserved for compatibility)
// ============================================================================

#[test]
fn test_legacy_action_io_support() {

    // Test TcpOnlySessionIO supports all action types
    assert!(<TcpOnlySessionIO as SupportsActionIO<InputAction>>::supports_action_io());
    assert!(<TcpOnlySessionIO as SupportsActionIO<OutputAction>>::supports_action_io());
    assert!(<TcpOnlySessionIO as SupportsActionIO<BiDirectionalAction>>::supports_action_io());
}

#[test]
fn test_supports_action_io_http() {
    // Test HttpOnlySessionIO supports output and bidirectional only
    assert!(<HttpOnlySessionIO as SupportsActionIO<OutputAction>>::supports_action_io());
    assert!(<HttpOnlySessionIO as SupportsActionIO<BiDirectionalAction>>::supports_action_io());
    
    // Note: HttpOnlySessionIO doesn't implement SupportsActionIO<InputAction>
    // This is verified by the type system - if it did implement it, this would compile:
    // assert!(<HttpOnlySessionIO as SupportsActionIO<InputAction>>::supports_action_io());
}

#[test]
fn test_custom_io_capability() {
    // Test our TestIO supports all actions
    assert!(<TestIO as SupportsActionIO<InputAction>>::supports_action_io());
    assert!(<TestIO as SupportsActionIO<OutputAction>>::supports_action_io());
    assert!(<TestIO as SupportsActionIO<BiDirectionalAction>>::supports_action_io());
}

#[test]
fn test_action_io_integration_with_protocols() {
    // Test that action I/O types integrate properly with protocol types
    type InputLocal = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, InputAction>;
    type OutputLocal = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, OutputAction>;
    type BiLocal = EpChanEnd<TestIO, CommMetadata<DefaultChan, RequestLbl>, BiDirectionalAction>;
    
    fn requires_local_protocol<T: LocalProtocol>(_: std::marker::PhantomData<T>) {}
    requires_local_protocol(std::marker::PhantomData::<InputLocal>);
    requires_local_protocol(std::marker::PhantomData::<OutputLocal>);
    requires_local_protocol(std::marker::PhantomData::<BiLocal>);
}

#[test]
fn test_action_io_constraint_verification() {
    // Test compile-time verification of I/O capability constraints
    fn verify_io_constraint<IO, AIO>()
    where
        IO: SupportsActionIO<AIO>,
        AIO: ActionIOTMarker,
    {
        // Constraint verified at compile time
    }
    
    // These should all compile successfully
    verify_io_constraint::<TcpOnlySessionIO, InputAction>();
    verify_io_constraint::<TcpOnlySessionIO, OutputAction>();
    verify_io_constraint::<TcpOnlySessionIO, BiDirectionalAction>();
    verify_io_constraint::<HttpOnlySessionIO, OutputAction>();
    verify_io_constraint::<HttpOnlySessionIO, BiDirectionalAction>();
}

// ============================================================================
// Foundation Traits Testing (4 tests)
// ============================================================================

#[test]
fn test_role_trait_implementation() {
    let alice = Alice;
    let bob = Bob;
    let carol = Carol;

    // Test Clone
    let alice_clone = alice.clone();
    assert_eq!(alice, alice_clone);

    // Test Debug
    format!("{:?}", alice);
    format!("{:?}", bob);
    format!("{:?}", carol);

    // Test PartialEq and Eq
    assert_eq!(alice, alice);
    // Note: Cannot compare different role types directly

    // Test Hash - use separate sets for each role type to avoid type mismatch
    use std::collections::HashSet;
    let mut alice_set = HashSet::new();
    alice_set.insert(alice);
    assert_eq!(alice_set.len(), 1);
}

#[test]
fn test_message_trait_implementation() {
    let hello = HelloMsg("hello".to_string());
    let ack = AckMsg;
    let data = DataMsg(vec![1, 2, 3]);

    // Test Clone
    let hello_clone = hello.clone();
    let ack_clone = ack.clone();
    let data_clone = data.clone();

    // Test Debug
    format!("{:?}", hello);
    format!("{:?}", ack);
    format!("{:?}", data);

    // Test that they implement Message trait
    fn requires_message<T: Message>(_: T) {}
    requires_message(hello_clone);
    requires_message(ack_clone);
    requires_message(data_clone);
}

#[test]
fn test_chanid_trait_implementation() {
    let default_chan = DefaultChan;
    let handshake_chan = HandshakeChan;
    let auth_chan = AuthChan;
    let data_chan = DataChan;

    // Test that they implement ChanId trait
    fn requires_chanid<T: ChanId>(_: T) {}
    requires_chanid(default_chan);
    requires_chanid(handshake_chan);
    requires_chanid(auth_chan);
    requires_chanid(data_chan);

    // Test Clone and PartialEq
    assert_eq!(DefaultChan, DefaultChan);
    // Note: Cannot compare different channel types directly

    // Test Debug
    format!("{:?}", DefaultChan);
    format!("{:?}", HandshakeChan);
}

#[test]
fn test_msglbl_trait_implementation() {
    let request_lbl = RequestLbl;
    let response_lbl = ResponseLbl;
    let login_lbl = LoginLbl;
    let status_lbl = StatusLbl;

    // Test that they implement MsgLbl trait
    fn requires_msglbl<T: MsgLbl>(_: T) {}
    requires_msglbl(request_lbl);
    requires_msglbl(response_lbl);
    requires_msglbl(login_lbl);
    requires_msglbl(status_lbl);

    // Test Clone and PartialEq
    assert_eq!(RequestLbl, RequestLbl);
    // Note: Cannot compare different label types directly

    // Test Debug
    format!("{:?}", RequestLbl);
    format!("{:?}", ResponseLbl);
}

// ============================================================================
// Legacy Tests (preserved for compatibility)
// ============================================================================

#[test]
fn test_legacy_action_io_support() {
    // Test TcpOnlySessionIO supports all action types
    assert!(<TcpOnlySessionIO as SupportsActionIO<InputAction>>::supports_action_io());
    assert!(<TcpOnlySessionIO as SupportsActionIO<OutputAction>>::supports_action_io());
    assert!(<TcpOnlySessionIO as SupportsActionIO<BiDirectionalAction>>::supports_action_io());

    // Test HttpOnlySessionIO supports output and bidirectional
    assert!(<HttpOnlySessionIO as SupportsActionIO<OutputAction>>::supports_action_io());
    assert!(<HttpOnlySessionIO as SupportsActionIO<BiDirectionalAction>>::supports_action_io());
}

// Test that example roles implement the Role trait
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TestRole;
impl Role for TestRole {}

#[test]
fn test_legacy_role_implementation() {
    let role1 = TestRole;
    let role2 = TestRole;
    assert_eq!(role1, role2);
}

// Test that example messages implement the Message trait
#[derive(Debug, Clone)]
struct TestMessage(String);
impl Message for TestMessage {}

#[test]
fn test_legacy_message_implementation() {
    let msg = TestMessage("test".to_string());
    let _clone = msg.clone();
}

// Type safety verification
#[test]
fn test_legacy_type_safety() {
    // This should work
    type ExampleMetadata = CommMetadata<DefaultChan, RequestLbl>;
    let meta = ExampleMetadata::new(DefaultChan, RequestLbl);

    // This should provide proper type safety
    fn process_metadata<C: ChanId, L: MsgLbl>(_meta: CommMetadata<C, L>) {
        // Implementation
    }

    process_metadata(meta);
}
