use super::*;
use crate::protocol::foundation::{
    BiDirectionalAction, ChanId, DefaultChan, Message, RequestLbl, SupportsActionIO,
};
use crate::protocol::global::{TChanEnd, TChanSend, TChanStart};
use crate::protocol::local::{EpChanEnd, EpChanSend, EpChanStart};

// Define test roles
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Alice;
impl Role for Alice {}
impl SupportsActionIO<BiDirectionalAction> for Alice {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bob;
impl Role for Bob {}
impl SupportsActionIO<BiDirectionalAction> for Bob {}

// Define test message
#[derive(Debug, Clone)]
struct TestMsg;
impl Message for TestMsg {}

// Define test IO type that supports BiDirectionalAction and ChanId
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TestIO;
impl SupportsActionIO<BiDirectionalAction> for TestIO {}
impl ChanId for TestIO {}

#[test]
fn test_project_tsend_sender_role() {
    // Test that TChanSend projects to EpChanSend when role is the sender
    type SendProto = TChanSend<
        Alice,
        Bob,
        DefaultChan,
        RequestLbl,
        TestMsg,
        TChanEnd<TestIO, RequestLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >;
    type AliceProjection = <() as Project<SendProto, Alice>>::Output;

    // Should be EpChanSend
    let _: AliceProjection = EpChanSend::new();
}

#[test]
fn test_project_tend() {
    // Test that TChanEnd projects to EpChanEnd
    type EndProto = TChanEnd<TestIO, RequestLbl, BiDirectionalAction>;
    type AliceProjection = <() as Project<EndProto, Alice>>::Output;

    // Should be EpChanEnd
    let _: AliceProjection = EpChanEnd::new();
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
