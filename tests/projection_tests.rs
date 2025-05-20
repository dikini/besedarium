//! Tests for projection traits
//!
//! This file contains tests to verify the behavior of projection traits
//! that generate endpoint (local) session types from global session types.
//!
//! TODO: Create a comprehensive test suite for projections that accounts for
//! the refactored TSend/TRecv model. The current version has been temporarily
//! disabled due to the transition from TInteract to TSend/TRecv pattern.
//! Future tests should cover single interactions, nested interactions,
//! choice operators, parallel composition, and recursive protocols.

// The following tests have been temporarily disabled while the projection implementation
// is being updated to work with the TSend/TRecv model instead of the previous TInteract model.

/*
use besedarium::*;
use besedarium::global::{TRec, TContinue, TSend, TRecv, TEnd, TChoice, TPar};
use besedarium::local::{EpRec, EpContinue, EpSend, EpRecv, EpEnd, EpSkip, EpChoice};
use besedarium::ProjectRole;
use besedarium::{Role, ProtocolLabel};
use besedarium::{True, False};

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
use besedarium::Http;

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

        // Expected: EpEnd<Http, L1, Alice> (with preserved label)
        assert_type_eq!(AliceLocal, EpEnd<Http, L1, Alice>);
    }

    // Future tests will be added here once the projection mechanism is fully refactored
    // to support the TSend/TRecv model.
}
*/
