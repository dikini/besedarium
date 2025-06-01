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

    // ============================================================================
    // Comprehensive Diagram Generation Test Suite (Task 3.5.2 completion)
    // ============================================================================

    #[test]
    fn test_mermaid_syntax_validation() {
        // Test that generated diagrams follow proper Mermaid syntax
        let diagram = DiagramTestProtocol::generate_diagram();
        
        // Must start with sequenceDiagram declaration
        assert!(diagram.starts_with("sequenceDiagram"), "Must start with 'sequenceDiagram'");
        
        // Should not contain syntax errors common in generated code
        assert!(!diagram.contains(";;"), "Should not have double semicolons");
        assert!(!diagram.contains("participant ;"), "Should not have malformed participants");
        
        // Check for proper line structure
        let lines: Vec<&str> = diagram.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                assert_eq!(*line, "sequenceDiagram", "First line must be 'sequenceDiagram'");
            } else if !line.trim().is_empty() {
                // Non-empty lines should be valid Mermaid syntax
                let trimmed = line.trim();
                assert!(
                    trimmed.starts_with("participant ") || 
                    trimmed.contains("->>") || 
                    trimmed.starts_with("note ") ||
                    trimmed.starts_with("alt ") ||
                    trimmed.starts_with("end") ||
                    trimmed.starts_with("loop ") ||
                    trimmed.starts_with("opt ") ||
                    trimmed.starts_with("par ") ||
                    trimmed.starts_with("%% "), // Comments
                    "Line '{}' should be valid Mermaid syntax", trimmed
                );
            }
        }
    }

    #[test] 
    fn test_protocol_flow_trait_implementation() {
        use besedarium::protocol::introspection::ProtocolFlow;

        // Test that all derived protocols properly implement ProtocolFlow
        let name = DiagramTestProtocol::get_protocol_name();
        assert_eq!(name, "DiagramTestProtocol");

        let roles = DiagramTestProtocol::get_roles();
        assert!(!roles.is_empty(), "Protocol should have at least one role");
        assert_eq!(roles.len(), 2, "DiagramTestProtocol should have exactly 2 roles");
        assert_eq!(roles, vec!["Role1", "Role2"]);

        let steps = DiagramTestProtocol::generate_sequence_steps();
        assert!(!steps.is_empty(), "Protocol should have at least one sequence step");
        assert_eq!(steps.len(), 2, "DiagramTestProtocol should have exactly 2 steps");

        // Test diagram configuration
        let config = DiagramTestProtocol::get_diagram_config();
        assert_eq!(config.title, None); // Default configuration
    }

    #[test]
    fn test_generates_diagram_trait_implementation() {
        use besedarium::protocol::introspection::GeneratesDiagram;

        // Test marker trait implementation
        fn requires_generates_diagram<T: GeneratesDiagram>() {}
        requires_generates_diagram::<DiagramTestProtocol>();
        requires_generates_diagram::<CustomerAgencyProtocol>();

        // The trait is a marker trait, so just ensuring compilation is sufficient
    }

    #[test]
    fn test_diagram_content_accuracy() {
        // Test that diagram content accurately reflects protocol structure
        let diagram = CustomerAgencyProtocol::generate_diagram();
        
        // Should contain protocol name in diagram
        assert!(diagram.contains("CustomerAgencyProtocol"), "Should contain protocol name as comment or title");
        
        // Verify basic diagram structure is present
        let lines: Vec<&str> = diagram.lines().collect();
        assert!(lines.len() >= 3, "Should have header plus content lines");
        
        // Should not be just a placeholder
        let content_lines = lines.iter().skip(1).filter(|l| !l.trim().is_empty()).count();
        assert!(content_lines > 0, "Should have actual content beyond header");
    }

    #[test]
    fn test_diagram_generation_consistency() {
        // Test that repeated calls generate identical diagrams (deterministic)
        let diagram1 = DiagramTestProtocol::generate_diagram();
        let diagram2 = DiagramTestProtocol::generate_diagram();
        
        assert_eq!(diagram1, diagram2, "Diagram generation should be deterministic");
    }

    #[test]
    fn test_error_handling_in_diagram_generation() {
        // Test that diagram generation doesn't panic on edge cases
        
        // These should all complete without panicking
        let _ = DiagramTestProtocol::generate_diagram();
        let _ = CustomerAgencyProtocol::generate_diagram();
        
        // Test multiple rapid generations
        for _ in 0..10 {
            let _ = DiagramTestProtocol::generate_diagram();
        }
    }

    #[test]
    fn test_diagram_feature_integration() {
        // Test integration with the derive feature system
        
        // This test verifies that the GenerateDiagram derive macro
        // properly integrates with the protocol derive system
        let diagram = DiagramTestProtocol::generate_diagram();
        
        // Basic sanity checks that derive integration worked
        assert!(!diagram.is_empty(), "Generated diagram should not be empty");
        assert!(diagram.len() > 20, "Generated diagram should have reasonable content");
        
        // Verify that trait bounds are properly enforced (excluding generated methods)
        fn check_core_traits<T>()
        where 
            T: besedarium::protocol::foundation::GlobalProtocol,
            T: besedarium::protocol::introspection::ProtocolFlow,
            T: besedarium::protocol::introspection::GeneratesDiagram,
        {
            let _ = T::get_protocol_name();
            let _ = T::get_roles();
            let _ = T::generate_sequence_steps();
            let _ = T::generate_mermaid_diagram(); // This is from the trait
        }
        
        check_core_traits::<DiagramTestProtocol>();
        check_core_traits::<CustomerAgencyProtocol>();
        
        // Test the generated method directly (not through generic bounds)
        let _ = DiagramTestProtocol::generate_diagram();
        let _ = CustomerAgencyProtocol::generate_diagram();
    }
}
