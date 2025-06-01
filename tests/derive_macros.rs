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
    #[derive(Debug, Clone, besedarium_derive::GlobalProtocol)]
    struct SimpleProtocol;

    #[derive(Debug, Clone, besedarium_derive::GlobalProtocol)]
    enum ProtocolState {
        Initial,
        Connected,
        Closed,
    }

    // Test GenerateDiagram derive
    #[derive(
        Debug, Clone, besedarium_derive::GlobalProtocol, besedarium_derive::GenerateDiagram,
    )]
    struct DiagramTestProtocol;

    #[derive(
        Debug, Clone, besedarium_derive::GlobalProtocol, besedarium_derive::GenerateDiagram,
    )]
    struct CustomerAgencyProtocol;

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
    fn test_generate_diagram_traits() {
        use besedarium::protocol::introspection::{GeneratesDiagram, ProtocolFlow};

        // Test that derived types implement ProtocolFlow trait
        fn requires_protocol_flow<T: ProtocolFlow>() {}
        requires_protocol_flow::<DiagramTestProtocol>();
        requires_protocol_flow::<CustomerAgencyProtocol>();

        // Test that derived types implement GeneratesDiagram trait
        fn requires_generates_diagram<T: GeneratesDiagram>() {}
        requires_generates_diagram::<DiagramTestProtocol>();
        requires_generates_diagram::<CustomerAgencyProtocol>();

        // Test protocol flow functionality
        assert_eq!(
            DiagramTestProtocol::get_protocol_name(),
            "DiagramTestProtocol"
        );
        assert_eq!(
            CustomerAgencyProtocol::get_protocol_name(),
            "CustomerAgencyProtocol"
        );

        let roles = DiagramTestProtocol::get_roles();
        assert_eq!(roles.len(), 2);
        assert_eq!(roles, vec!["Role1", "Role2"]);

        let steps = DiagramTestProtocol::generate_sequence_steps();
        assert_eq!(steps.len(), 2); // Default implementation has Send + End
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
        fn check_role_bounds<
            T: Role + Send + Sync + Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash,
        >(
            _: T,
        ) {
        }
        check_role_bounds(Client);

        // MsgLbl must be Send + Sync + Clone + Debug + PartialEq + Eq + Hash
        fn check_msg_lbl_bounds<
            T: MsgLbl + Send + Sync + Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash,
        >(
            _: T,
        ) {
        }
        check_msg_lbl_bounds(RequestLabel);

        // GlobalProtocol must be Send + Sync + Debug
        fn check_global_protocol_bounds<T: GlobalProtocol + Send + Sync + std::fmt::Debug>(_: T) {}
        check_global_protocol_bounds(SimpleProtocol);
    }

    #[test]
    fn test_generate_diagram_derive() {
        // Test that protocols with GenerateDiagram derive can generate diagrams
        let diagram = DiagramTestProtocol::generate_diagram();

        // Verify the output is a Mermaid sequence diagram
        assert!(diagram.starts_with("sequenceDiagram"));
        assert!(diagram.contains("DiagramTestProtocol"));

        let customer_diagram = CustomerAgencyProtocol::generate_diagram();

        // Verify the output is valid Mermaid syntax
        assert!(customer_diagram.starts_with("sequenceDiagram"));
        assert!(customer_diagram.contains("CustomerAgencyProtocol"));
    }

    #[test]
    fn test_diagram_generation_structure() {
        // Test that generated diagrams have proper structure
        let diagram = DiagramTestProtocol::generate_diagram();

        // Check for required Mermaid elements
        assert!(
            diagram.contains("sequenceDiagram"),
            "Should start with sequenceDiagram"
        );

        // Verify that the diagram contains protocol-specific information
        assert!(
            diagram.contains("DiagramTestProtocol"),
            "Should contain protocol name"
        );

        // Check that diagram is not empty beyond the header
        let lines: Vec<&str> = diagram.lines().collect();
        assert!(
            lines.len() >= 2,
            "Should have more than just the header line"
        );
    }

    #[test]
    fn test_multiple_protocol_diagram_generation() {
        // Test that different protocols generate different diagrams
        let diagram1 = DiagramTestProtocol::generate_diagram();
        let diagram2 = CustomerAgencyProtocol::generate_diagram();

        // Diagrams should be different (contain different protocol names)
        assert_ne!(
            diagram1, diagram2,
            "Different protocols should generate different diagrams"
        );
        assert!(diagram1.contains("DiagramTestProtocol"));
        assert!(diagram2.contains("CustomerAgencyProtocol"));
    }

    #[test]
    fn test_diagram_generation_method_signature() {
        // Test that the generated method has the correct signature
        fn check_diagram_method<F>(_: F)
        where
            F: Fn() -> String,
        {
        }

        check_diagram_method(DiagramTestProtocol::generate_diagram);
    }
}
