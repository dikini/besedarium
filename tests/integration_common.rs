//! Integration Tests for Besedarium Session Types Library
//!
//! This module contains integration tests that demonstrate complex protocol scenarios
//! using the modern foundation types and architecture.

use besedarium::protocol::foundation::*;
use besedarium::protocol::projection::{False, RoleEq};

// Test infrastructure: Common types used across integration tests

// === Test Roles ===
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Alice;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bob;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Charlie;

impl Role for Alice {}
impl Role for Bob {}
impl Role for Charlie {}

// === Role SupportsActionIO Implementations ===
impl SupportsActionIO<InputAction> for Alice {}
impl SupportsActionIO<OutputAction> for Alice {}
impl SupportsActionIO<BiDirectionalAction> for Alice {}

impl SupportsActionIO<InputAction> for Bob {}
impl SupportsActionIO<OutputAction> for Bob {}
impl SupportsActionIO<BiDirectionalAction> for Bob {}

impl SupportsActionIO<InputAction> for Charlie {}
impl SupportsActionIO<OutputAction> for Charlie {}
impl SupportsActionIO<BiDirectionalAction> for Charlie {}

// === Role Equality Implementations ===
impl RoleEq<Bob> for Alice {
    type Output = False;
}

impl RoleEq<Alice> for Bob {
    type Output = False;
}

impl RoleEq<Charlie> for Alice {
    type Output = False;
}

impl RoleEq<Alice> for Charlie {
    type Output = False;
}

impl RoleEq<Charlie> for Bob {
    type Output = False;
}

impl RoleEq<Bob> for Charlie {
    type Output = False;
}

// === Test Channels ===
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthChan;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataChan;

impl ChanId for AuthChan {}
impl ChanId for DataChan {}

// === Test Message Labels ===
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoginLbl;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AckLbl;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataLbl;

impl MsgLbl for LoginLbl {}
impl MsgLbl for AckLbl {}
impl MsgLbl for DataLbl {}

// === Test Messages ===
#[derive(Debug, Clone)]
pub struct LoginMsg {
    #[allow(dead_code)]
    pub username: String,
    #[allow(dead_code)]
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct AckMsg {
    pub success: bool,
    #[allow(dead_code)]
    pub session_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DataMsg {
    #[allow(dead_code)]
    pub payload: Vec<u8>,
}

impl Message for LoginMsg {}
impl Message for AckMsg {}
impl Message for DataMsg {}

// === Test I/O Types ===
#[derive(Debug)]
#[allow(dead_code)]
pub struct TestNetworkIO;

impl SupportsActionIO<InputAction> for TestNetworkIO {}
impl SupportsActionIO<OutputAction> for TestNetworkIO {}
impl SupportsActionIO<BiDirectionalAction> for TestNetworkIO {}

// Common type aliases for cleaner test code
pub type AuthMeta = CommMetadata<AuthChan, LoginLbl>;
pub type DataMeta = CommMetadata<DataChan, DataLbl>;
#[allow(dead_code)]
pub type AckMeta = CommMetadata<AuthChan, AckLbl>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_infrastructure() {
        // Verify our test infrastructure compiles and works
        let auth_meta = AuthMeta::new(AuthChan, LoginLbl);
        let _data_meta = DataMeta::new(DataChan, DataLbl);

        // Test role instances
        let alice = Alice;
        let _bob = Bob;

        // Test message instances
        let _login = LoginMsg {
            username: "alice".to_string(),
            password: "secret".to_string(),
        };

        let ack = AckMsg {
            success: true,
            session_token: Some("token123".to_string()),
        };

        // Verify types work as expected
        assert_eq!(auth_meta.chan_id, AuthChan);
        assert_eq!(auth_meta.msg_lbl, LoginLbl);
        assert_eq!(alice, Alice);
        assert!(ack.success);
    }
}
