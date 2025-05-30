//! Tests for declarative macro infrastructure
//!
//! This module contains comprehensive tests for all declarative macros
//! to ensure they work correctly with the foundation type system.

use crate::protocol::foundation::*;

// Import macros explicitly for testing
use crate::{impl_traits_for_label, define_role, define_message, messages, define_protocol};

#[cfg(test)]
mod label_tests {
    use super::*;

    #[test]
    fn test_impl_traits_for_label_basic() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        struct TestLabel;
        impl_traits_for_label!(TestLabel);
        
        let label = TestLabel;
        assert_eq!(format!("{}", label), "TestLabel");
        assert_eq!(format!("{:?}", label), "TestLabel");
    }

    #[test]
    fn test_impl_traits_for_label_with_dual() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        struct RequestLabel;
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        struct ResponseLabel;
        
        impl_traits_for_label!(RequestLabel, ResponseLabel);
        impl_traits_for_label!(ResponseLabel, RequestLabel);
        
        let request = RequestLabel;
        let response = ResponseLabel;
        
        assert_eq!(format!("{}", request), "RequestLabel");
        assert_eq!(format!("{}", response), "ResponseLabel");
    }

    #[test]
    fn test_label_implements_msglbl() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        struct TestLabel;
        impl_traits_for_label!(TestLabel);
        
        // Test that the label implements MsgLbl trait
        fn accepts_msglbl<T: MsgLbl>(_label: T) {}
        accepts_msglbl(TestLabel);
    }
}

#[cfg(test)]
mod role_tests {
    use super::*;

    #[test]
    fn test_define_role_basic() {
        define_role!(TestRole);
        
        let role = TestRole;
        assert_eq!(format!("{}", role), "TestRole");
        assert_eq!(format!("{:?}", role), "TestRole");
        
        // Test default implementation
        let default_role = TestRole::default();
        assert_eq!(role, default_role);
    }

    #[test]
    fn test_define_role_with_display_name() {
        define_role!(ServerRole, "Server Node");
        
        let server = ServerRole;
        assert_eq!(format!("{}", server), "Server Node");
        assert_eq!(format!("{:?}", server), "Server Node");
    }

    #[test]
    fn test_role_implements_role_trait() {
        define_role!(Alice);
        define_role!(Bob);
        
        // Test that roles implement the Role trait
        fn accepts_role<R: Role>(_role: R) {}
        
        accepts_role(Alice);
        accepts_role(Bob);
    }

    #[test]
    fn test_role_equality_and_ordering() {
        define_role!(TestRole);
        
        let role_a1 = TestRole;
        let role_a2 = TestRole;
        
        assert_eq!(role_a1, role_a2);
        
        // Test that roles can be used in hash collections
        use std::collections::HashSet;
        let mut role_set = HashSet::new();
        role_set.insert(role_a1);
        role_set.insert(role_a2);
        assert_eq!(role_set.len(), 1); // Same role type, so only one entry
    }
}

#[cfg(test)]
mod message_tests {
    use super::*;

    #[test]
    fn test_define_message_simple() {
        define_message!(Ping);
        
        let ping = Ping;
        assert_eq!(format!("{}", ping), "Ping");
        assert_eq!(format!("{:?}", ping), "Ping");
        
        // Test default implementation
        let default_ping = Ping::default();
        assert_eq!(ping, default_ping);
    }

    #[test]
    fn test_define_message_with_fields() {
        define_message!(Login {
            username: String,
            password: String,
        });
        
        let login = Login {
            username: "alice".to_string(),
            password: "secret".to_string(),
        };
        
        assert_eq!(login.username, "alice");
        assert_eq!(login.password, "secret");
        assert_eq!(format!("{}", login), "Login");
    }

    #[test]
    fn test_define_message_implements_message_trait() {
        define_message!(TestMessage);
        define_message!(ComplexMessage {
            id: u64,
            data: Vec<u8>,
        });
        
        // Test that messages implement the Message trait
        fn accepts_message<M: Message>(_msg: M) {}
        
        accepts_message(TestMessage);
        accepts_message(ComplexMessage {
            id: 42,
            data: vec![1, 2, 3],
        });
    }

    #[test]
    fn test_messages_batch_simple() {
        messages!(
            Start,
            Stop,
            Restart,
        );
        
        let start = Start;
        let stop = Stop;
        let restart = Restart;
        
        assert_eq!(format!("{}", start), "Start");
        assert_eq!(format!("{}", stop), "Stop");
        assert_eq!(format!("{}", restart), "Restart");
    }

    #[test]
    fn test_messages_batch_mixed() {
        messages!(
            SimpleMessage,
            ComplexMessage {
                field1: String,
                field2: i32,
            },
            AnotherMessage {
                data: Vec<u8>,
            },
        );
        
        let simple = SimpleMessage;
        let complex = ComplexMessage {
            field1: "test".to_string(),
            field2: 42,
        };
        let another = AnotherMessage {
            data: vec![1, 2, 3, 4],
        };
        
        assert_eq!(format!("{}", simple), "SimpleMessage");
        assert_eq!(complex.field1, "test");
        assert_eq!(complex.field2, 42);
        assert_eq!(another.data, vec![1, 2, 3, 4]);
    }
}

#[cfg(test)]
mod protocol_tests {
    use super::*;

    #[test]
    fn test_define_protocol_basic() {
        define_protocol!(TestProtocol);
        
        let protocol = TestProtocol;
        assert_eq!(format!("{}", protocol), "TestProtocol");
        assert_eq!(format!("{:?}", protocol), "TestProtocol");
    }

    #[test]
    fn test_define_protocol_with_description() {
        define_protocol!(AuthProtocol, "Authentication and authorization protocol");
        
        let protocol = AuthProtocol;
        assert_eq!(
            format!("{}", protocol),
            "AuthProtocol: Authentication and authorization protocol"
        );
    }

    #[test]
    fn test_protocol_implements_global_protocol_trait() {
        define_protocol!(MyProtocol);
        
        // Test that protocol implements GlobalProtocol trait
        fn accepts_global_protocol<P: GlobalProtocol>(_protocol: P) {}
        accepts_global_protocol(MyProtocol);
    }

    #[test]
    fn test_protocol_equality_and_hashing() {
        define_protocol!(TestProtocol);
        
        let protocol_a1 = TestProtocol;
        let protocol_a2 = TestProtocol;
        
        assert_eq!(protocol_a1, protocol_a2);
        
        // Test that protocols can be used in hash collections
        use std::collections::HashMap;
        let mut protocol_map = HashMap::new();
        protocol_map.insert(protocol_a1, "Test Protocol");
        protocol_map.insert(protocol_a2, "Test Protocol Updated");
        assert_eq!(protocol_map.len(), 1); // Same protocol type, so only one entry
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_protocol_definition() {
        // Define roles
        define_role!(Client);
        define_role!(Server);
        
        // Define messages
        messages!(
            Login {
                username: String,
                password: String,
            },
            LoginResponse {
                success: bool,
                session_token: Option<String>,
            },
            Logout,
        );
        
        // Define protocol
        define_protocol!(AuthenticationProtocol, "Client-server authentication");
        
        // Test that everything works together
        let client = Client;
        let server = Server;
        let login = Login {
            username: "alice".to_string(),
            password: "secret123".to_string(),
        };
        let response = LoginResponse {
            success: true,
            session_token: Some("abc123".to_string()),
        };
        let logout = Logout;
        let protocol = AuthenticationProtocol;
        
        // Verify types implement required traits
        fn verify_role<R: Role>(_r: R) {}
        fn verify_message<M: Message>(_m: M) {}
        fn verify_protocol<P: GlobalProtocol>(_p: P) {}
        
        verify_role(client);
        verify_role(server);
        verify_message(login.clone());
        verify_message(response.clone());
        verify_message(logout.clone());
        verify_protocol(protocol.clone());
        
        // Test display formatting
        assert_eq!(format!("{}", client), "Client");
        assert_eq!(format!("{}", server), "Server");
        assert_eq!(format!("{}", logout), "Logout");
        assert_eq!(
            format!("{}", protocol),
            "AuthenticationProtocol: Client-server authentication"
        );
    }

    #[test]
    fn test_macro_hygiene() {
        // Test that macros don't interfere with each other or user code
        define_role!(HygieneTestRole);
        define_message!(HygieneTestMessage);
        define_protocol!(HygieneTestProtocol);
        
        // Define some local variables that might conflict
        let role = "string role";
        let message = "string message";
        let protocol = "string protocol";
        
        // The macro-generated types should still work
        let macro_role = HygieneTestRole;
        let macro_message = HygieneTestMessage;
        let macro_protocol = HygieneTestProtocol;
        
        assert_eq!(role, "string role");
        assert_eq!(message, "string message");
        assert_eq!(protocol, "string protocol");
        assert_eq!(format!("{}", macro_role), "HygieneTestRole");
        assert_eq!(format!("{}", macro_message), "HygieneTestMessage");
        assert_eq!(format!("{}", macro_protocol), "HygieneTestProtocol");
    }
}
