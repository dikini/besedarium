use super::*;
use crate::protocol::foundation::{
    BiDirectionalAction, CommMetadata, DefaultChan, InputAction, OutputAction, Message, RequestLbl,
    Role, SupportsActionIO,
};
use crate::protocol::global::{TChanChoice, TChanEnd, TChanOffer, TChanRecv, TChanSend};
use crate::protocol::local::{EpChanEnd, EpChanRecv, EpChanSend};

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

// Define test IO capabilities
#[derive(Debug, Clone)]
struct TestIO;
impl SupportsActionIO<BiDirectionalAction> for TestIO {}
impl SupportsActionIO<InputAction> for TestIO {}
impl SupportsActionIO<OutputAction> for TestIO {}

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
fn test_choice_offer_duality() {
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

// Note: Tests for non-dual types would need negative compilation tests
// which are better handled in integration tests or trybuild tests
