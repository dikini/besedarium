//! Tests for projection traits
//!
//! This file contains tests to verify the behavior of projection traits
//! that generate endpoint (local) session types from global session types.

use besedarium::*;

// --- Custom Label Types for Testing ---
struct L1;
struct L2;
struct L3;
impl ProtocolLabel for L1 {}
impl ProtocolLabel for L2 {}
impl ProtocolLabel for L3 {}

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

// --- Tests for ProjectRole trait ---
#[cfg(test)]
mod project_role_tests {
    use super::*;

    // Test projection of TEnd
    #[test]
    fn test_projection_of_tend() {
        // Define a TEnd protocol
        type GlobalProtocol = TEnd<Http, L1>;

        // Project onto Alice
        type AliceLocal = <() as ProjectRole<Alice, Http, GlobalProtocol>>::Out;

        // Expected: EpEnd<Http, Alice>
        assert_type_eq!(AliceLocal, EpEnd<Http, Alice>);
    }

    // Test projection of TInteract where the role is the sender
    #[test]
    fn test_projection_of_tinteract_as_sender() {
        // Define a global protocol where Alice sends a message
        type GlobalProtocol = TInteract<Http, L1, Alice, Message, TEnd<Http, L2>>;

        // Project onto Alice
        type AliceLocal = <() as ProjectRole<Alice, Http, GlobalProtocol>>::Out;

        // Expected: EpSend<Http, Alice, Message, EpEnd<Http, Alice>>
        assert_type_eq!(
            AliceLocal,
            EpSend<Http, Alice, Message, EpEnd<Http, Alice>>
        );
    }

    // Test projection of TInteract where the role is the receiver
    #[test]
    fn test_projection_of_tinteract_as_receiver() {
        // Define a global protocol where Alice sends a message
        type GlobalProtocol = TInteract<Http, L1, Alice, Message, TEnd<Http, L2>>;

        // Project onto Bob
        type BobLocal = <() as ProjectRole<Bob, Http, GlobalProtocol>>::Out;

        // Expected: EpRecv<Http, Bob, Message, EpEnd<Http, Bob>>
        assert_type_eq!(
            BobLocal,
            EpRecv<Http, Bob, Message, EpEnd<Http, Bob>>
        );
    }

    // Test projection of a more complex protocol with multiple interactions
    #[test]
    fn test_projection_of_complex_protocol() {
        // Define a global protocol with multiple interactions:
        // 1. Alice sends Message to Bob
        // 2. Bob sends Response to Alice
        type GlobalProtocol =
            TInteract<Http, L1, Alice, Message, TInteract<Http, L2, Bob, Response, TEnd<Http, L3>>>;

        // Project onto Alice
        type AliceLocal = <() as ProjectRole<Alice, Http, GlobalProtocol>>::Out;

        // Expected: Alice sends Message then receives Response
        assert_type_eq!(
            AliceLocal,
            EpSend<
                Http,
                Alice,
                Message,
                EpRecv<Http, Alice, Response, EpEnd<Http, Alice>>
            >
        );

        // Project onto Bob
        type BobLocal = <() as ProjectRole<Bob, Http, GlobalProtocol>>::Out;

        // Expected: Bob receives Message then sends Response
        assert_type_eq!(
            BobLocal,
            EpRecv<
                Http,
                Bob,
                Message,
                EpSend<Http, Bob, Response, EpEnd<Http, Bob>>
            >
        );
    }

    // Test projection of a protocol where a role is not involved
    #[test]
    fn test_projection_of_uninvolved_role() {
        // Define a global protocol with interactions only between Alice and Bob
        type GlobalProtocol =
            TInteract<Http, L1, Alice, Message, TInteract<Http, L2, Bob, Response, TEnd<Http, L3>>>;

        // Project onto Charlie who is not involved
        type CharlieLocal = <() as ProjectRole<Charlie, Http, GlobalProtocol>>::Out;

        // Expected: Charlie receives both messages as they're not the sender
        assert_type_eq!(
            CharlieLocal,
            EpRecv<
                Http,
                Charlie,
                Message,
                EpRecv<Http, Charlie, Response, EpEnd<Http, Charlie>>
            >
        );
    }

    // Test that ProjectInteract correctly dispatches based on role equality
    #[test]
    fn test_project_interact_dispatch() {
        // When role is sender (flag = True)
        type SenderOut =
            <() as ProjectInteract<True, Alice, Http, Alice, Message, TEnd<Http, L1>>>::Out;

        assert_type_eq!(
            SenderOut,
            EpSend<Http, Alice, Message, EpEnd<Http, Alice>>
        );

        // When role is not sender (flag = False)
        type ReceiverOut =
            <() as ProjectInteract<False, Bob, Http, Alice, Message, TEnd<Http, L1>>>::Out;

        assert_type_eq!(
            ReceiverOut,
            EpRecv<Http, Bob, Message, EpEnd<Http, Bob>>
        );
    }
}

// --- Tests for helper traits used in projection ---
#[cfg(test)]
mod projection_helper_tests {
    use super::*;

    // Test IsEpSkipVariant trait
    #[test]
    fn test_is_ep_skip_variant() {
        // EpSkip should be identified as skip type
        assert_type_eq!(IsSkip<EpSkip<Http, Alice>, Http, Alice>, True);

        // Other endpoint types should not be identified as skip
        assert_type_eq!(IsSkip<EpEnd<Http, Alice>, Http, Alice>, False);
        assert_type_eq!(
            IsSkip<EpSend<Http, Alice, Message, EpEnd<Http, Alice>>, Http, Alice>,
            False
        );
    }

    // Test IsEpEndVariant trait
    #[test]
    fn test_is_ep_end_variant() {
        // EpEnd should be identified as end type
        assert_type_eq!(IsEnd<EpEnd<Http, Alice>, Http, Alice>, True);

        // Other endpoint types should not be identified as end
        assert_type_eq!(IsEnd<EpSkip<Http, Alice>, Http, Alice>, False);
        assert_type_eq!(
            IsEnd<EpSend<Http, Alice, Message, EpEnd<Http, Alice>>, Http, Alice>,
            False
        );
    }

    // Test ProjectParBranch based on role presence
    #[test]
    fn test_project_par_branch() {
        // Role is present in branch (flag = True)
        type RolePresent = <() as ProjectParBranch<
            True,
            Alice,
            Http,
            TInteract<Http, L1, Alice, Message, TEnd<Http, L2>>,
        >>::Out;
        assert_type_eq!(
            RolePresent,
            EpSend<Http, Alice, Message, EpEnd<Http, Alice>>
        );

        // Role is not present in branch (flag = False)
        type RoleNotPresent = <() as ProjectParBranch<
            False,
            Alice,
            Http,
            TInteract<Http, L1, Bob, Message, TEnd<Http, L2>>,
        >>::Out;
        assert_type_eq!(RoleNotPresent, EpSkip<Http, Alice>);
    }
}
