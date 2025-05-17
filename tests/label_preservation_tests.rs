//! Tests for label preservation during projection
//!
//! This file contains tests to verify that labels are correctly preserved
//! when projecting from global to local types.

use besedarium::*;
use besedarium::GetLocalLabel;

// --- Custom Label Types for Testing ---
struct TestLabel1;
struct TestLabel2; 
struct TestLabel3;
impl ProtocolLabel for TestLabel1 {}
impl ProtocolLabel for TestLabel2 {}
impl ProtocolLabel for TestLabel3 {}

// --- Custom Roles for Testing ---
struct Alice;
struct Bob;
struct Charlie;
impl Role for Alice {}
impl Role for Bob {}
impl Role for Charlie {}

// --- Role equality implementations ---
impl RoleEq<Alice> for Alice {
    type Output = True;
}
impl RoleEq<Bob> for Alice {
    type Output = False;
}
impl RoleEq<Charlie> for Alice {
    type Output = False;
}

impl RoleEq<Alice> for Bob {
    type Output = False;
}
impl RoleEq<Bob> for Bob {
    type Output = True;
}
impl RoleEq<Charlie> for Bob {
    type Output = False;
}

impl RoleEq<Alice> for Charlie {
    type Output = False;
}
impl RoleEq<Bob> for Charlie {
    type Output = False;
}
impl RoleEq<Charlie> for Charlie {
    type Output = True;
}

// --- Message Types for Testing ---
struct Message;
struct Response;

// --- IO Types for Testing ---
struct Http;

// --- Tests for Label Preservation ---
#[cfg(test)]
mod label_preservation_tests {
    use super::*;
    
    #[test]
    fn test_preserved_label_in_end() {
        // Define a TEnd protocol with TestLabel1
        type GlobalProtocol = TEnd<Http, TestLabel1>;
        
        // Project onto Alice
        type AliceLocal = <() as ProjectRole<Alice, Http, GlobalProtocol>>::Out;
        
        // Expected: EpEnd with preserved label
        assert_type_eq!(AliceLocal, EpEnd<Http, TestLabel1, Alice>);
        
        // Verify the label is preserved using GetLocalLabel
        type PreservedLabel = <AliceLocal as GetLocalLabel>::Label;
        assert_type_eq!(PreservedLabel, TestLabel1);
    }
    
    #[test]
    fn test_preserved_label_in_interaction() {
        // Define a global protocol with TestLabel1
        type GlobalProtocol = TInteract<Http, TestLabel1, Alice, Message, TEnd<Http, TestLabel2>>;
        
        // Project onto Alice (sender)
        type AliceLocal = <() as ProjectRole<Alice, Http, GlobalProtocol>>::Out;
        
        // Expected: EpSend with preserved labels
        assert_type_eq!(
            AliceLocal,
            EpSend<Http, TestLabel1, Alice, Message, EpEnd<Http, TestLabel2, Alice>>
        );
        
        // Verify the label is preserved using GetLocalLabel
        type PreservedLabel = <AliceLocal as GetLocalLabel>::Label;
        assert_type_eq!(PreservedLabel, TestLabel1);
        
        // Project onto Bob (receiver)
        type BobLocal = <() as ProjectRole<Bob, Http, GlobalProtocol>>::Out;
        
        // Expected: EpRecv with preserved labels
        assert_type_eq!(
            BobLocal,
            EpRecv<Http, TestLabel1, Bob, Message, EpEnd<Http, TestLabel2, Bob>>
        );
        
        // Verify the label is preserved using GetLocalLabel
        type PreservedLabelBob = <BobLocal as GetLocalLabel>::Label;
        assert_type_eq!(PreservedLabelBob, TestLabel1);
    }
    
    #[test]
    fn test_preserved_label_in_choice() {
        // Define a global protocol with choices
        type LeftBranch = TInteract<Http, TestLabel2, Alice, Message, TEnd<Http, TestLabel3>>;
        type RightBranch = TInteract<Http, TestLabel2, Bob, Response, TEnd<Http, TestLabel3>>;
        type GlobalProtocol = TChoice<Http, TestLabel1, LeftBranch, RightBranch>;
        
        // Project onto Alice
        type AliceLocal = <() as ProjectRole<Alice, Http, GlobalProtocol>>::Out;
        
        // Expected: EpChoice with preserved labels
        assert_type_eq!(
            AliceLocal, 
            EpChoice<
                Http, 
                TestLabel1, 
                Alice,
                EpSend<Http, TestLabel2, Alice, Message, EpEnd<Http, TestLabel3, Alice>>,
                EpRecv<Http, TestLabel2, Alice, Response, EpEnd<Http, TestLabel3, Alice>>
            >
        );
        
        // Verify the label is preserved using GetLocalLabel
        type PreservedLabel = <AliceLocal as GetLocalLabel>::Label;
        assert_type_eq!(PreservedLabel, TestLabel1);
    }
    
    #[test]
    fn test_preserved_label_in_parallel() {
        // Define a global protocol with parallel composition
        type LeftBranch = TInteract<Http, TestLabel2, Alice, Message, TEnd<Http, TestLabel3>>;
        type RightBranch = TInteract<Http, TestLabel2, Bob, Response, TEnd<Http, TestLabel3>>;
        type GlobalProtocol = TPar<Http, TestLabel1, LeftBranch, RightBranch, ()>;
        
        // Project onto Alice (only in left branch)
        type AliceLocal = <() as ProjectRole<Alice, Http, GlobalProtocol>>::Out;
        
        // Expected: EpSend with preserved labels (since Alice is only in left branch)
        assert_type_eq!(
            AliceLocal,
            EpSend<Http, TestLabel2, Alice, Message, EpEnd<Http, TestLabel3, Alice>>
        );
        
        // Project onto Charlie (not in either branch)
        type CharlieLocal = <() as ProjectRole<Charlie, Http, GlobalProtocol>>::Out;
        
        // Expected: EpSkip with preserved label
        assert_type_eq!(CharlieLocal, EpSkip<Http, TestLabel1, Charlie>);
        
        // Verify the label is preserved using GetLocalLabel
        type PreservedLabel = <CharlieLocal as GetLocalLabel>::Label;
        assert_type_eq!(PreservedLabel, TestLabel1);
    }
    
    #[test]
    fn test_complex_protocol_label_preservation() {
        // Define a more complex protocol with multiple interactions and choices
        type InnerChoice = TChoice<
            Http, 
            TestLabel3,
            TInteract<Http, TestLabel2, Alice, Message, TEnd<Http, TestLabel3>>,
            TInteract<Http, TestLabel2, Bob, Response, TEnd<Http, TestLabel3>>
        >;
        
        type GlobalProtocol = TInteract<
            Http, 
            TestLabel1, 
            Alice, 
            Message, 
            TInteract<Http, TestLabel2, Bob, Response, InnerChoice>
        >;
        
        // Project onto Alice
        type AliceLocal = <() as ProjectRole<Alice, Http, GlobalProtocol>>::Out;
        
        // Expected: Complex endpoint type with preserved labels
        assert_type_eq!(
            AliceLocal,
            EpSend<
                Http, 
                TestLabel1, 
                Alice, 
                Message, 
                EpRecv<
                    Http, 
                    TestLabel2, 
                    Alice, 
                    Response, 
                    EpChoice<
                        Http, 
                        TestLabel3, 
                        Alice,
                        EpSend<Http, TestLabel2, Alice, Message, EpEnd<Http, TestLabel3, Alice>>,
                        EpRecv<Http, TestLabel2, Alice, Response, EpEnd<Http, TestLabel3, Alice>>
                    >
                >
            >
        );
        
        // Verify the label is preserved at the top level using GetLocalLabel
        type PreservedLabel = <AliceLocal as GetLocalLabel>::Label;
        assert_type_eq!(PreservedLabel, TestLabel1);
    }
}
