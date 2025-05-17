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
}
