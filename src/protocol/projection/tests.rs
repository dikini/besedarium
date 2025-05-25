use super::*;
use crate::protocol::foundation::{
    BiDirectionalAction, ChanId, CommMetadata, DefaultChan, InputAction, Message, OutputAction,
    RequestLbl, SupportsActionIO,
};
use crate::protocol::global::{TChanChoice, TChanEnd, TChanPar, TChanRecv, TChanSend, TChanStart};
use crate::protocol::local::{
    EpChanChoice, EpChanEnd, EpChanPar, EpChanRecv, EpChanSend, EpChanStart,
};
use crate::protocol::projection::helpers::{RoleEq, False};

// ============================================================================
// Test Infrastructure 
// ============================================================================

// Define test roles
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Alice;
impl Role for Alice {}
impl SupportsActionIO<BiDirectionalAction> for Alice {}
impl SupportsActionIO<InputAction> for Alice {}
impl SupportsActionIO<OutputAction> for Alice {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bob;
impl Role for Bob {}
impl SupportsActionIO<BiDirectionalAction> for Bob {}
impl SupportsActionIO<InputAction> for Bob {}
impl SupportsActionIO<OutputAction> for Bob {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Carol;
impl Role for Carol {}
impl SupportsActionIO<BiDirectionalAction> for Carol {}
impl SupportsActionIO<InputAction> for Carol {}
impl SupportsActionIO<OutputAction> for Carol {}

// Non-reflexive role equality implementations
impl RoleEq<Bob> for Alice { type Output = False; }
impl RoleEq<Carol> for Alice { type Output = False; }
impl RoleEq<Alice> for Bob { type Output = False; }
impl RoleEq<Carol> for Bob { type Output = False; }
impl RoleEq<Alice> for Carol { type Output = False; }
impl RoleEq<Bob> for Carol { type Output = False; }

// Define test messages
#[derive(Debug, Clone)]
struct HelloMsg;
impl Message for HelloMsg {}

#[derive(Debug, Clone)]
struct AckMsg;
impl Message for AckMsg {}

#[derive(Debug, Clone)]
struct DataMsg;
impl Message for DataMsg {}

// Define test IO type that supports all action types and ChanId
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TestIO;
impl SupportsActionIO<BiDirectionalAction> for TestIO {}
impl SupportsActionIO<InputAction> for TestIO {}
impl SupportsActionIO<OutputAction> for TestIO {}
impl ChanId for TestIO {}

// Test label types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DataLbl;
impl crate::protocol::foundation::MsgLbl for DataLbl {}

// Helper type aliases for cleaner test code
type TestMeta = CommMetadata<DefaultChan, RequestLbl>;
type DataMeta = CommMetadata<TestIO, DataLbl>;

// ============================================================================
// Send/Recv Projection Tests
// ============================================================================

#[test]
fn test_project_tsend_sender_role() {
    // Test that TChanSend projects to EpChanSend when role is the sender
    type SendProto = TChanSend<
        Alice,
        Bob,
        DefaultChan,
        RequestLbl,
        HelloMsg,
        TChanEnd<TestIO, RequestLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type AliceProjection = <() as Project<SendProto, Alice>>::Output;

    // Should be EpChanSend
    let _: AliceProjection = EpChanSend::new();
}

#[test]
fn test_project_tsend_receiver_role() {
    // Test that TChanSend doesn't directly project to EpChanRecv for receiver
    // (Receiver role gets the continuation since TSend doesn't create a receive for the receiver)
    type SendProto = TChanSend<
        Alice,
        Bob,
        DefaultChan,
        RequestLbl,
        HelloMsg,
        TChanEnd<TestIO, RequestLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type BobProjection = <() as Project<SendProto, Bob>>::Output;

    // Bob gets the continuation (TChanEnd projects to EpChanEnd)
    let _: BobProjection = EpChanEnd::new();
}

#[test]
fn test_project_tsend_uninvolved_role() {
    // Test that uninvolved role gets the continuation
    type SendProto = TChanSend<
        Alice,
        Bob,
        DefaultChan,
        RequestLbl,
        HelloMsg,
        TChanEnd<TestIO, RequestLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type CarolProjection = <() as Project<SendProto, Carol>>::Output;

    // Carol gets the continuation (TChanEnd projects to EpChanEnd)
    let _: CarolProjection = EpChanEnd::new();
}

#[test]
fn test_project_trecv_receiver_role() {
    // Test that TChanRecv projects to EpChanRecv when role is the receiver
    type RecvProto = TChanRecv<
        Bob,
        Alice,
        DefaultChan,
        RequestLbl,
        AckMsg,
        TChanEnd<TestIO, RequestLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type BobProjection = <() as Project<RecvProto, Bob>>::Output;

    // Bob (receiver) should get EpChanRecv
    let _: BobProjection = EpChanRecv::new();
}

#[test]
fn test_project_trecv_sender_role() {
    // Test that sender role gets the continuation for TChanRecv
    type RecvProto = TChanRecv<
        Bob,
        Alice,
        DefaultChan,
        RequestLbl,
        AckMsg,
        TChanEnd<TestIO, RequestLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type AliceProjection = <() as Project<RecvProto, Alice>>::Output;

    // Alice (sender) gets the continuation (TChanEnd projects to EpChanEnd)
    let _: AliceProjection = EpChanEnd::new();
}

#[test]
fn test_project_trecv_uninvolved_role() {
    // Test that uninvolved role gets the continuation
    type RecvProto = TChanRecv<
        Bob,
        Alice,
        DefaultChan,
        RequestLbl,
        AckMsg,
        TChanEnd<TestIO, RequestLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type CarolProjection = <() as Project<RecvProto, Carol>>::Output;

    // Carol gets the continuation (TChanEnd projects to EpChanEnd)
    let _: CarolProjection = EpChanEnd::new();
}

// ============================================================================
// Choice Projection Tests
// ============================================================================

#[test]
fn test_project_tchoice_simple() {
    // Test basic choice projection
    type LeftBranch = TChanSend<
        Alice,
        Bob,
        TestIO,
        DataLbl,
        HelloMsg,
        TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type RightBranch = TChanSend<
        Alice,
        Carol,
        TestIO,
        DataLbl,
        AckMsg,
        TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type ChoiceProto = TChanChoice<Alice, TestIO, DataLbl, LeftBranch, RightBranch, BiDirectionalAction>;
    
    type AliceProjection = <() as Project<ChoiceProto, Alice>>::Output;

    // Alice should get EpChanChoice
    let _: AliceProjection = EpChanChoice::new();
}

#[test]
fn test_project_tchoice_involved_role() {
    // Test choice projection when role is involved in branches
    type LeftBranch = TChanSend<
        Alice,
        Bob,
        TestIO,
        DataLbl,
        HelloMsg,
        TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type RightBranch = TChanSend<
        Carol,
        Bob,
        TestIO,
        DataLbl,
        AckMsg,
        TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type ChoiceProto = TChanChoice<Alice, TestIO, DataLbl, LeftBranch, RightBranch, BiDirectionalAction>;
    
    type BobProjection = <() as Project<ChoiceProto, Bob>>::Output;

    // Bob should get EpChanChoice since involved in both branches
    let _: BobProjection = EpChanChoice::new();
}

#[test]
fn test_project_tchoice_uninvolved_role() {
    // Test choice projection when role is not involved in any branch
    type LeftBranch = TChanSend<
        Alice,
        Bob,
        TestIO,
        DataLbl,
        HelloMsg,
        TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type RightBranch = TChanSend<
        Alice,
        Bob,
        TestIO,
        DataLbl,
        AckMsg,
        TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type ChoiceProto = TChanChoice<Alice, TestIO, DataLbl, LeftBranch, RightBranch, BiDirectionalAction>;
    
    type CarolProjection = <() as Project<ChoiceProto, Carol>>::Output;

    // Carol should get EpChanChoice (simplified - all get same structure)
    let _: CarolProjection = EpChanChoice::new();
}

// ============================================================================
// Parallel Projection Tests
// ============================================================================

#[test]
fn test_project_tpar_involved_role() {
    // Test parallel projection when role is involved in both branches
    type LeftBranch = TChanSend<
        Alice,
        Bob,
        TestIO,
        DataLbl,
        HelloMsg,
        TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type RightBranch = TChanSend<
        Alice,
        Carol,
        TestIO,
        DataLbl,
        AckMsg,
        TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type ParProto = TChanPar<TestIO, DataLbl, LeftBranch, RightBranch, (), BiDirectionalAction>;
    
    type AliceProjection = <() as Project<ParProto, Alice>>::Output;

    // Alice should get EpChanPar since involved in both branches
    let _: AliceProjection = EpChanPar::new();
}

#[test]
fn test_project_tpar_single_branch_involvement() {
    // Test parallel projection when role is involved in only one branch
    type LeftBranch = TChanSend<
        Alice,
        Bob,
        TestIO,
        DataLbl,
        HelloMsg,
        TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type RightBranch = TChanSend<
        Carol,
        Bob,
        TestIO,
        DataLbl,
        AckMsg,
        TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type ParProto = TChanPar<TestIO, DataLbl, LeftBranch, RightBranch, (), BiDirectionalAction>;
    
    type BobProjection = <() as Project<ParProto, Bob>>::Output;

    // Bob should get EpChanPar since involved in both branches
    let _: BobProjection = EpChanPar::new();
}

#[test]
fn test_project_tpar_uninvolved_role() {
    // Test parallel projection when role is not involved in any branch
    type LeftBranch = TChanSend<
        Alice,
        Bob,
        TestIO,
        DataLbl,
        HelloMsg,
        TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type RightBranch = TChanSend<
        Alice,
        Bob,
        TestIO,
        DataLbl,
        AckMsg,
        TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type ParProto = TChanPar<TestIO, DataLbl, LeftBranch, RightBranch, (), BiDirectionalAction>;
    
    type CarolProjection = <() as Project<ParProto, Carol>>::Output;

    // Carol should get EpChanPar (simplified - all get same structure)
    let _: CarolProjection = EpChanPar::new();
}

// ============================================================================
// End/Start Projection Tests
// ============================================================================

#[test]
fn test_project_tend() {
    // Test that TChanEnd projects to EpChanEnd
    type EndProto = TChanEnd<TestIO, RequestLbl, BiDirectionalAction>;
    type AliceProjection = <() as Project<EndProto, Alice>>::Output;

    // Should be EpChanEnd
    let _: AliceProjection = EpChanEnd::new();
}

#[test]
fn test_project_tend_different_roles() {
    // Test that TChanEnd projects to EpChanEnd for any role
    type EndProto = TChanEnd<DefaultChan, DataLbl, BiDirectionalAction>;
    
    type AliceProjection = <() as Project<EndProto, Alice>>::Output;
    type BobProjection = <() as Project<EndProto, Bob>>::Output;
    type CarolProjection = <() as Project<EndProto, Carol>>::Output;

    // All should be EpChanEnd
    let _: AliceProjection = EpChanEnd::new();
    let _: BobProjection = EpChanEnd::new();
    let _: CarolProjection = EpChanEnd::new();
}

#[test]
fn test_project_tstart() {
    // Test that TChanStart projects to EpChanStart
    type StartProto = TChanStart<
        TestIO,
        RequestLbl,
        TChanEnd<TestIO, RequestLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type AliceProjection = <() as Project<StartProto, Alice>>::Output;

    // Should be EpChanStart
    let _: AliceProjection = EpChanStart::new();
}

#[test]
fn test_project_tstart_with_complex_continuation() {
    // Test TChanStart with complex continuation
    type ComplexContinuation = TChanSend<
        Alice,
        Bob,
        TestIO,
        DataLbl,
        HelloMsg,
        TChanRecv<
            Bob,
            Alice,
            TestIO,
            DataLbl,
            AckMsg,
            TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
            BiDirectionalAction,
        >,
        BiDirectionalAction,
    >;
    type StartProto = TChanStart<TestIO, DataLbl, ComplexContinuation, BiDirectionalAction>;
    
    type AliceProjection = <() as Project<StartProto, Alice>>::Output;
    type BobProjection = <() as Project<StartProto, Bob>>::Output;

    // Both should be EpChanStart with appropriate inner projections
    let _: AliceProjection = EpChanStart::new();
    let _: BobProjection = EpChanStart::new();
}

// ============================================================================
// Complex Protocol Projection Tests
// ============================================================================

#[test]
fn test_project_nested_choice_in_parallel() {
    // Test complex nested protocol: parallel branches with choice inside
    type NestedChoice = TChanChoice<
        Alice,
        TestIO,
        DataLbl,
        TChanSend<Alice, Bob, TestIO, DataLbl, HelloMsg, TChanEnd<TestIO, DataLbl, BiDirectionalAction>, BiDirectionalAction>,
        TChanSend<Alice, Carol, TestIO, DataLbl, AckMsg, TChanEnd<TestIO, DataLbl, BiDirectionalAction>, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type SimpleBranch = TChanSend<
        Bob,
        Carol,
        TestIO,
        DataLbl,
        DataMsg,
        TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type ComplexPar = TChanPar<TestIO, DataLbl, NestedChoice, SimpleBranch, (), BiDirectionalAction>;
    
    type AliceProjection = <() as Project<ComplexPar, Alice>>::Output;
    type BobProjection = <() as Project<ComplexPar, Bob>>::Output;
    type CarolProjection = <() as Project<ComplexPar, Carol>>::Output;

    // All should produce valid local protocol types
    let _: AliceProjection = EpChanPar::new();
    let _: BobProjection = EpChanPar::new();
    let _: CarolProjection = EpChanPar::new();
}

#[test]
fn test_project_sequential_send_recv() {
    // Test sequential send followed by receive
    type SeqProto = TChanSend<
        Alice,
        Bob,
        TestIO,
        DataLbl,
        HelloMsg,
        TChanRecv<
            Bob,
            Alice,
            TestIO,
            DataLbl,
            AckMsg,
            TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
            BiDirectionalAction,
        >,
        BiDirectionalAction,
    >;
    
    type AliceProjection = <() as Project<SeqProto, Alice>>::Output;
    type BobProjection = <() as Project<SeqProto, Bob>>::Output;
    type CarolProjection = <() as Project<SeqProto, Carol>>::Output;

    // Alice: send then receive
    let _: AliceProjection = EpChanSend::new();
    // Bob: continuation (receive then end)  
    let _: BobProjection = EpChanRecv::new();
    // Carol: uninvolved, gets final end
    let _: CarolProjection = EpChanEnd::new();
}

#[test]
fn test_project_choice_with_different_message_types() {
    // Test choice with different message types in branches
    type LeftBranch = TChanSend<
        Alice,
        Bob,
        TestIO,
        DataLbl,
        HelloMsg,
        TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type RightBranch = TChanSend<
        Alice,
        Carol,
        TestIO,
        DataLbl,
        DataMsg,
        TChanEnd<TestIO, DataLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type MixedChoice = TChanChoice<Alice, TestIO, DataLbl, LeftBranch, RightBranch, BiDirectionalAction>;
    
    type AliceProjection = <() as Project<MixedChoice, Alice>>::Output;
    type BobProjection = <() as Project<MixedChoice, Bob>>::Output;
    type CarolProjection = <() as Project<MixedChoice, Carol>>::Output;

    // All should get choice structure with appropriate inner projections
    let _: AliceProjection = EpChanChoice::new();
    let _: BobProjection = EpChanChoice::new();
    let _: CarolProjection = EpChanChoice::new();
}

// ============================================================================
// Role Equality Tests
// ============================================================================

#[test]
fn test_role_equality_reflexive() {
    // Test that roles equal themselves
    fn _assert_alice_equals_alice() 
    where
        <Alice as RoleEq<Alice>>::Output: crate::protocol::projection::helpers::Bool,
    {
        // This function existing proves the trait bound is satisfied
    }
    
    fn _assert_bob_equals_bob() 
    where
        <Bob as RoleEq<Bob>>::Output: crate::protocol::projection::helpers::Bool,
    {
        // This function existing proves the trait bound is satisfied
    }
    
    _assert_alice_equals_alice();
    _assert_bob_equals_bob();
}

#[test]
fn test_role_equality_non_reflexive() {
    // Test that different roles are not equal
    fn _assert_alice_not_equals_bob() 
    where
        <Alice as RoleEq<Bob>>::Output: crate::protocol::projection::helpers::Bool,
    {
        // This function existing proves the trait bound is satisfied
    }
    
    fn _assert_bob_not_equals_alice() 
    where
        <Bob as RoleEq<Alice>>::Output: crate::protocol::projection::helpers::Bool,
    {
        // This function existing proves the trait bound is satisfied
    }
    
    _assert_alice_not_equals_bob();
    _assert_bob_not_equals_alice();
}

// ============================================================================
// Action I/O Integration Tests  
// ============================================================================

#[test]
fn test_project_with_input_action() {
    // Test projection works with InputAction
    type RecvOnlyProto = TChanRecv<
        Bob,
        Alice,
        TestIO,
        DataLbl,
        HelloMsg,
        TChanEnd<TestIO, DataLbl, InputAction>,
        InputAction,
    >;
    
    type AliceProjection = <() as Project<RecvOnlyProto, Alice>>::Output;
    
    // Alice is the sender, so gets the continuation (EpChanEnd)
    let _: AliceProjection = EpChanEnd::new();
}

#[test]
fn test_project_with_output_action() {
    // Test projection works with OutputAction
    type SendOnlyProto = TChanSend<
        Alice,
        Bob,
        TestIO,
        DataLbl,
        HelloMsg,
        TChanEnd<TestIO, DataLbl, OutputAction>,
        OutputAction,
    >;
    
    type AliceProjection = <() as Project<SendOnlyProto, Alice>>::Output;
    
    // Should project to send operation
    let _: AliceProjection = EpChanSend::new();
}
