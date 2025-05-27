use super::*;
use crate::protocol::foundation::{
    BiDirectionalAction, CommMetadata, DefaultChan, InputAction, Message, OutputAction, RequestLbl,
    Role, SupportsActionIO,
};
use crate::protocol::global::{
    TChanChoice, TChanEnd, TChanOffer, TChanPar, TChanRecv, TChanSend, TChanStart,
};
use crate::protocol::local::{
    EpChanChoice, EpChanEnd, EpChanOffer, EpChanPar, EpChanRecv, EpChanSend, EpChanStart,
};

// Define test roles
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Alice;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bob;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Carol;

impl Role for Alice {}
impl Role for Bob {}
impl Role for Carol {}

// Define test messages
#[derive(Debug, Clone)]
struct HelloMsg;
#[derive(Debug, Clone)]
struct AckMsg;
#[derive(Debug, Clone)]
struct DataMsg;

impl Message for HelloMsg {}
impl Message for AckMsg {}
impl Message for DataMsg {}

// Define test IO capabilities
#[derive(Debug, Clone)]
struct TestIO;
impl SupportsActionIO<BiDirectionalAction> for TestIO {}
impl SupportsActionIO<InputAction> for TestIO {}
impl SupportsActionIO<OutputAction> for TestIO {}

// =============================================================================
// GLOBAL PROTOCOL DUALITY TESTS
// =============================================================================

#[test]
fn test_global_send_recv_duality() {
    type End = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type Send = TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>;
    type Recv = TChanRecv<Bob, Alice, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>;

    // This should compile successfully
    fn _test_dual()
    where
        (): IsDual<Send, Recv>,
        <() as IsDual<Send, Recv>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_global_recv_send_duality_symmetric() {
    type End = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type Send = TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>;
    type Recv = TChanRecv<Bob, Alice, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>;

    // Test symmetry: Recv should be dual to Send as well
    fn _test_dual()
    where
        (): IsDual<Recv, Send>,
        <() as IsDual<Recv, Send>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_global_end_self_duality() {
    type End = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;

    // End should be dual to itself
    fn _test_dual()
    where
        (): IsDual<End, End>,
        <() as IsDual<End, End>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_global_choice_offer_duality() {
    type EndType = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;

    type Choice =
        TChanChoice<Alice, DefaultChan, RequestLbl, EndType, EndType, BiDirectionalAction>;
    type Offer = TChanOffer<Alice, DefaultChan, RequestLbl, EndType, EndType, BiDirectionalAction>;

    // Choice and Offer should be dual
    fn _test_dual()
    where
        (): IsDual<Choice, Offer>,
        <() as IsDual<Choice, Offer>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_global_offer_choice_duality_symmetric() {
    type EndType = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;

    type Choice =
        TChanChoice<Alice, DefaultChan, RequestLbl, EndType, EndType, BiDirectionalAction>;
    type Offer = TChanOffer<Alice, DefaultChan, RequestLbl, EndType, EndType, BiDirectionalAction>;

    // Test symmetry: Offer should be dual to Choice as well
    fn _test_dual()
    where
        (): IsDual<Offer, Choice>,
        <() as IsDual<Offer, Choice>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_global_par_duality() {
    type End1 = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type End2 = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;

    type Par1 = TChanPar<DefaultChan, RequestLbl, End1, End2, (), BiDirectionalAction>;
    type Par2 = TChanPar<DefaultChan, RequestLbl, End1, End2, (), BiDirectionalAction>;

    // Parallel composition is dual when constituent branches are dual
    fn _test_dual()
    where
        (): IsDual<Par1, Par2>,
        <() as IsDual<Par1, Par2>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_global_start_self_duality() {
    type End = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type Start = TChanStart<DefaultChan, RequestLbl, End, BiDirectionalAction>;

    // Start should be dual to itself when the inner protocol is self-dual
    fn _test_dual()
    where
        (): IsDual<Start, Start>,
        <() as IsDual<Start, Start>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_global_complex_protocol_duality() {
    // Test duality with more complex nested protocols
    type End = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type SendAck = TChanSend<Bob, Alice, DefaultChan, RequestLbl, AckMsg, End, BiDirectionalAction>;
    type RecvAck = TChanRecv<Alice, Bob, DefaultChan, RequestLbl, AckMsg, End, BiDirectionalAction>;

    type SendHello =
        TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, SendAck, BiDirectionalAction>;
    type RecvHello =
        TChanRecv<Bob, Alice, DefaultChan, RequestLbl, HelloMsg, RecvAck, BiDirectionalAction>;

    // Complex send/receive chain should be dual
    fn _test_dual()
    where
        (): IsDual<SendHello, RecvHello>,
        <() as IsDual<SendHello, RecvHello>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_global_choice_with_different_branches() {
    type End = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type SendBranch =
        TChanSend<Alice, Bob, DefaultChan, RequestLbl, DataMsg, End, BiDirectionalAction>;
    type RecvBranch =
        TChanRecv<Bob, Alice, DefaultChan, RequestLbl, DataMsg, End, BiDirectionalAction>;

    type Choice = TChanChoice<Alice, DefaultChan, RequestLbl, SendBranch, End, BiDirectionalAction>;
    type Offer = TChanOffer<Alice, DefaultChan, RequestLbl, RecvBranch, End, BiDirectionalAction>;

    // Choice with send branch should be dual to offer with recv branch
    fn _test_dual()
    where
        (): IsDual<Choice, Offer>,
        <() as IsDual<Choice, Offer>>::Output: EqualsTrue,
    {
    }
}

// =============================================================================
// LOCAL ENDPOINT DUALITY TESTS
// =============================================================================

#[test]
fn test_local_send_recv_duality() {
    type Meta = CommMetadata<DefaultChan, RequestLbl>;
    type EndEp = EpChanEnd<TestIO, Meta, BiDirectionalAction>;
    type SendEp = EpChanSend<TestIO, Meta, HelloMsg, EndEp, BiDirectionalAction>;
    type RecvEp = EpChanRecv<TestIO, Meta, HelloMsg, EndEp, BiDirectionalAction>;

    // Local endpoints should be dual
    fn _test_dual()
    where
        (): IsDual<SendEp, RecvEp>,
        <() as IsDual<SendEp, RecvEp>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_local_recv_send_duality_symmetric() {
    type Meta = CommMetadata<DefaultChan, RequestLbl>;
    type EndEp = EpChanEnd<TestIO, Meta, BiDirectionalAction>;
    type SendEp = EpChanSend<TestIO, Meta, HelloMsg, EndEp, BiDirectionalAction>;
    type RecvEp = EpChanRecv<TestIO, Meta, HelloMsg, EndEp, BiDirectionalAction>;

    // Test symmetry: Recv should be dual to Send as well
    fn _test_dual()
    where
        (): IsDual<RecvEp, SendEp>,
        <() as IsDual<RecvEp, SendEp>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_local_end_self_duality() {
    type Meta = CommMetadata<DefaultChan, RequestLbl>;
    type EndEp = EpChanEnd<TestIO, Meta, BiDirectionalAction>;

    // Local end should be dual to itself
    fn _test_dual()
    where
        (): IsDual<EndEp, EndEp>,
        <() as IsDual<EndEp, EndEp>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_local_choice_offer_duality() {
    type Meta = CommMetadata<DefaultChan, RequestLbl>;
    type EndEp = EpChanEnd<TestIO, Meta, BiDirectionalAction>;

    type ChoiceEp = EpChanChoice<TestIO, Meta, EndEp, EndEp, BiDirectionalAction>;
    type OfferEp = EpChanOffer<TestIO, Meta, EndEp, EndEp, BiDirectionalAction>;

    // Local choice and offer should be dual
    fn _test_dual()
    where
        (): IsDual<ChoiceEp, OfferEp>,
        <() as IsDual<ChoiceEp, OfferEp>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_local_offer_choice_duality_symmetric() {
    type Meta = CommMetadata<DefaultChan, RequestLbl>;
    type EndEp = EpChanEnd<TestIO, Meta, BiDirectionalAction>;

    type ChoiceEp = EpChanChoice<TestIO, Meta, EndEp, EndEp, BiDirectionalAction>;
    type OfferEp = EpChanOffer<TestIO, Meta, EndEp, EndEp, BiDirectionalAction>;

    // Test symmetry: Offer should be dual to Choice as well
    fn _test_dual()
    where
        (): IsDual<OfferEp, ChoiceEp>,
        <() as IsDual<OfferEp, ChoiceEp>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_local_par_duality() {
    type Meta = CommMetadata<DefaultChan, RequestLbl>;
    type EndEp = EpChanEnd<TestIO, Meta, BiDirectionalAction>;

    type ParEp1 = EpChanPar<TestIO, Meta, EndEp, EndEp, (), BiDirectionalAction>;
    type ParEp2 = EpChanPar<TestIO, Meta, EndEp, EndEp, (), BiDirectionalAction>;

    // Local parallel endpoints are dual when constituent branches are dual
    fn _test_dual()
    where
        (): IsDual<ParEp1, ParEp2>,
        <() as IsDual<ParEp1, ParEp2>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_local_start_self_duality() {
    type Meta = CommMetadata<DefaultChan, RequestLbl>;
    type EndEp = EpChanEnd<TestIO, Meta, BiDirectionalAction>;
    type StartEp = EpChanStart<TestIO, Meta, EndEp, BiDirectionalAction>;

    // Local start should be dual to itself when the inner protocol is self-dual
    fn _test_dual()
    where
        (): IsDual<StartEp, StartEp>,
        <() as IsDual<StartEp, StartEp>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_local_complex_endpoint_duality() {
    // Test duality with more complex nested local endpoints
    type Meta = CommMetadata<DefaultChan, RequestLbl>;
    type EndEp = EpChanEnd<TestIO, Meta, BiDirectionalAction>;
    type SendAckEp = EpChanSend<TestIO, Meta, AckMsg, EndEp, BiDirectionalAction>;
    type RecvAckEp = EpChanRecv<TestIO, Meta, AckMsg, EndEp, BiDirectionalAction>;

    type SendHelloEp = EpChanSend<TestIO, Meta, HelloMsg, SendAckEp, BiDirectionalAction>;
    type RecvHelloEp = EpChanRecv<TestIO, Meta, HelloMsg, RecvAckEp, BiDirectionalAction>;

    // Complex local endpoint chain should be dual
    fn _test_dual()
    where
        (): IsDual<SendHelloEp, RecvHelloEp>,
        <() as IsDual<SendHelloEp, RecvHelloEp>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_local_choice_with_different_branches() {
    type Meta = CommMetadata<DefaultChan, RequestLbl>;
    type EndEp = EpChanEnd<TestIO, Meta, BiDirectionalAction>;
    type SendBranchEp = EpChanSend<TestIO, Meta, DataMsg, EndEp, BiDirectionalAction>;
    type RecvBranchEp = EpChanRecv<TestIO, Meta, DataMsg, EndEp, BiDirectionalAction>;

    type ChoiceEp = EpChanChoice<TestIO, Meta, SendBranchEp, EndEp, BiDirectionalAction>;
    type OfferEp = EpChanOffer<TestIO, Meta, RecvBranchEp, EndEp, BiDirectionalAction>;

    // Local choice with send branch should be dual to offer with recv branch
    fn _test_dual()
    where
        (): IsDual<ChoiceEp, OfferEp>,
        <() as IsDual<ChoiceEp, OfferEp>>::Output: EqualsTrue,
    {
    }
}

// =============================================================================
// EDGE CASES AND ACTION I/O CONSTRAINT TESTS
// =============================================================================

#[test]
fn test_duality_with_input_action() {
    type Meta = CommMetadata<DefaultChan, RequestLbl>;
    type EndEp = EpChanEnd<TestIO, Meta, InputAction>;
    type SendEp = EpChanSend<TestIO, Meta, HelloMsg, EndEp, InputAction>;
    type RecvEp = EpChanRecv<TestIO, Meta, HelloMsg, EndEp, InputAction>;

    // Duality should work with InputAction
    fn _test_dual()
    where
        (): IsDual<SendEp, RecvEp>,
        <() as IsDual<SendEp, RecvEp>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_duality_with_output_action() {
    type Meta = CommMetadata<DefaultChan, RequestLbl>;
    type EndEp = EpChanEnd<TestIO, Meta, OutputAction>;
    type SendEp = EpChanSend<TestIO, Meta, HelloMsg, EndEp, OutputAction>;
    type RecvEp = EpChanRecv<TestIO, Meta, HelloMsg, EndEp, OutputAction>;

    // Duality should work with OutputAction
    fn _test_dual()
    where
        (): IsDual<SendEp, RecvEp>,
        <() as IsDual<SendEp, RecvEp>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_global_par_with_different_constituents() {
    // Test parallel composition where branches are different but dual
    type End = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type Send = TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>;
    type Recv = TChanRecv<Bob, Alice, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>;

    type Par1 = TChanPar<DefaultChan, RequestLbl, Send, End, (), BiDirectionalAction>;
    type Par2 = TChanPar<DefaultChan, RequestLbl, Recv, End, (), BiDirectionalAction>;

    // Parallel compositions with dual constituents should be dual
    fn _test_dual()
    where
        (): IsDual<Par1, Par2>,
        <() as IsDual<Par1, Par2>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_local_par_with_different_constituents() {
    // Test local parallel composition where branches are different but dual
    type Meta = CommMetadata<DefaultChan, RequestLbl>;
    type EndEp = EpChanEnd<TestIO, Meta, BiDirectionalAction>;
    type SendEp = EpChanSend<TestIO, Meta, HelloMsg, EndEp, BiDirectionalAction>;
    type RecvEp = EpChanRecv<TestIO, Meta, HelloMsg, EndEp, BiDirectionalAction>;

    type ParEp1 = EpChanPar<TestIO, Meta, SendEp, EndEp, (), BiDirectionalAction>;
    type ParEp2 = EpChanPar<TestIO, Meta, RecvEp, EndEp, (), BiDirectionalAction>;

    // Local parallel compositions with dual constituents should be dual
    fn _test_dual()
    where
        (): IsDual<ParEp1, ParEp2>,
        <() as IsDual<ParEp1, ParEp2>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_nested_protocol_duality() {
    // Test deeply nested protocols
    type End = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    type InnerSend =
        TChanSend<Alice, Bob, DefaultChan, RequestLbl, AckMsg, End, BiDirectionalAction>;
    type InnerRecv =
        TChanRecv<Bob, Alice, DefaultChan, RequestLbl, AckMsg, End, BiDirectionalAction>;

    type MiddleChoice =
        TChanChoice<Alice, DefaultChan, RequestLbl, InnerSend, End, BiDirectionalAction>;
    type MiddleOffer =
        TChanOffer<Alice, DefaultChan, RequestLbl, InnerRecv, End, BiDirectionalAction>;

    type OuterSend =
        TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, MiddleChoice, BiDirectionalAction>;
    type OuterRecv =
        TChanRecv<Bob, Alice, DefaultChan, RequestLbl, HelloMsg, MiddleOffer, BiDirectionalAction>;

    // Deeply nested protocols should be dual
    fn _test_dual()
    where
        (): IsDual<OuterSend, OuterRecv>,
        <() as IsDual<OuterSend, OuterRecv>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_protocol_composition_duality_consistency() {
    // Test that duality is maintained through protocol composition
    type End = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;

    // Define base dual pairs
    type Send1 = TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>;
    type Recv1 = TChanRecv<Bob, Alice, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>;

    type Send2 = TChanSend<Bob, Alice, DefaultChan, RequestLbl, AckMsg, End, BiDirectionalAction>;
    type Recv2 = TChanRecv<Alice, Bob, DefaultChan, RequestLbl, AckMsg, End, BiDirectionalAction>;

    // Compose them in parallel
    type Par1 = TChanPar<DefaultChan, RequestLbl, Send1, Send2, (), BiDirectionalAction>;
    type Par2 = TChanPar<DefaultChan, RequestLbl, Recv1, Recv2, (), BiDirectionalAction>;

    // Parallel composition of dual pairs should be dual
    fn _test_dual()
    where
        (): IsDual<Par1, Par2>,
        <() as IsDual<Par1, Par2>>::Output: EqualsTrue,
    {
    }
}
