use super::*;
use crate::protocol::duality::{helpers::EqualsTrue, IsDual};
use crate::protocol::foundation::{
    BiDirectionalAction, CommMetadata, DefaultChan, Message, RequestLbl, Role, TcpOnlySessionIO,
};

// Test-specific types (not exposed in the public API)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Alice;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bob;
#[derive(Debug, Clone)]
struct HelloMsg;

impl Role for Alice {}
impl Role for Bob {}
impl Message for HelloMsg {}

type TestIO = TcpOnlySessionIO;
type TestMetadata = CommMetadata<DefaultChan, RequestLbl>;

#[test]
fn test_well_founded_verification() {
    // Test well-foundedness for simple protocol ending
    type End = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
    verify_well_founded::<End>();

    // Test well-foundedness for send followed by end
    type Send = TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>;
    verify_well_founded::<Send>();
}

#[test]
fn test_local_dual_generation() {
    // Test dual generation for termination
    type End = EpChanEnd<TestIO, TestMetadata, BiDirectionalAction>;
    verify_local_dual_generation::<End>();

    // Verify that generated dual satisfies IsDual relationship
    fn _verify_local_dual()
    where
        (): IsDual<End, LocalDual<End>>,
        <() as IsDual<End, LocalDual<End>>>::Output: EqualsTrue,
    {
    }
}

#[test]
fn test_local_send_dual_generation() {
    type End = EpChanEnd<TestIO, TestMetadata, BiDirectionalAction>;
    type Send = EpChanSend<TestIO, TestMetadata, HelloMsg, End, BiDirectionalAction>;

    // Test the verification function
    verify_local_dual_generation::<Send>();
}

#[test]
fn test_local_choice_dual_generation() {
    type End = EpChanEnd<TestIO, TestMetadata, BiDirectionalAction>;
    type Choice = EpChanChoice<TestIO, TestMetadata, End, End, BiDirectionalAction>;

    // Test the verification function
    verify_local_dual_generation::<Choice>();
}
