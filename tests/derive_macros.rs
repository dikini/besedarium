//! # Integration Tests for Derive Macros
//!
//! These tests verify that the derive macros work correctly with the
//! foundation types from the main besedarium crate.

#[cfg(feature = "derive")]
mod tests {
    use besedarium::protocol::foundation::{GlobalProtocol, Message, MsgLbl, Role};

    // Test Message derive
    #[derive(Debug, Clone, besedarium_derive::Message)]
    struct LoginRequest {
        username: String,
        password: String,
    }

    #[derive(Debug, Clone, besedarium_derive::Message)]
    enum Response {
        Success(String),
        Error(u32),
    }

    #[derive(Debug, Clone, besedarium_derive::Message)]
    struct Ping;

    // Test Role derive  
    #[derive(Debug, Clone, PartialEq, Eq, Hash, besedarium_derive::Role)]
    struct Client;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, besedarium_derive::Role)]
    struct Server;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, besedarium_derive::Role)]
    enum Participant {
        Alice,
        Bob,
        Charlie,
    }

    // Test MsgLbl derive
    #[derive(Debug, Clone, PartialEq, Eq, Hash, besedarium_derive::MsgLbl)]
    struct RequestLabel;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, besedarium_derive::MsgLbl)]
    struct ResponseLabel;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, besedarium_derive::MsgLbl)]
    enum ProtocolLabel {
        Login,
        Data,
        Logout,
    }

    // Test GlobalProtocol derive
    #[derive(Debug, besedarium_derive::GlobalProtocol)]
    struct SimpleProtocol;

    #[derive(Debug, besedarium_derive::GlobalProtocol)]
    enum ProtocolState {
        Initial,
        Active,
        Terminated,
    }

    #[test]
    fn test_message_traits() {
        // Test that derived types implement Message trait
        fn requires_message<T: Message>(_: T) {}
        
        requires_message(LoginRequest {
            username: "user".to_string(),
            password: "pass".to_string(),
        });
        requires_message(Response::Success("ok".to_string()));
        requires_message(Ping);
    }

    #[test]
    fn test_role_traits() {
        // Test that derived types implement Role trait
        fn requires_role<T: Role>(_: T) {}
        
        requires_role(Client);
        requires_role(Server);
        requires_role(Participant::Alice);
    }

    #[test]
    fn test_msg_lbl_traits() {
        // Test that derived types implement MsgLbl trait
        fn requires_msg_lbl<T: MsgLbl>(_: T) {}
        
        requires_msg_lbl(RequestLabel);
        requires_msg_lbl(ResponseLabel);
        requires_msg_lbl(ProtocolLabel::Login);
    }

    #[test]
    fn test_global_protocol_traits() {
        // Test that derived types implement GlobalProtocol trait
        fn requires_global_protocol<T: GlobalProtocol>(_: T) {}
        
        requires_global_protocol(SimpleProtocol);
        requires_global_protocol(ProtocolState::Initial);
    }

    #[test]
    fn test_trait_bounds() {
        // Test specific trait bound requirements
        
        // Messages must be Send + Sync + Clone + Debug
        fn check_message_bounds<T: Message + Send + Sync + Clone + std::fmt::Debug>(_: T) {}
        check_message_bounds(LoginRequest {
            username: "test".to_string(),
            password: "test".to_string(),
        });

        // Roles must be Send + Sync + Clone + Debug + PartialEq + Eq + Hash
        fn check_role_bounds<T: Role + Send + Sync + Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash>(_: T) {}
        check_role_bounds(Client);

        // MsgLbl must be Send + Sync + Clone + Debug + PartialEq + Eq + Hash
        fn check_msg_lbl_bounds<T: MsgLbl + Send + Sync + Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash>(_: T) {}
        check_msg_lbl_bounds(RequestLabel);

        // GlobalProtocol must be Send + Sync + Debug
        fn check_global_protocol_bounds<T: GlobalProtocol + Send + Sync + std::fmt::Debug>(_: T) {}
        check_global_protocol_bounds(SimpleProtocol);
    }
}
