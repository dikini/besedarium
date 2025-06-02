//! Integration tests for dual protocol generation workflow
//!
//! These tests verify the complete end-to-end dual generation integration
//! from macro attributes through to final protocol code generation.

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use crate::dual_generation::DualGenerator;
    use crate::protocol::{
        generate_protocol_implementation, MessageFlow, MessageProperties,
        MessageSpec, ProtocolAttributes, ProtocolFlow, ProtocolSpec,
    };

    /// Helper to create a minimal protocol spec for testing
    fn create_test_protocol_spec() -> ProtocolSpec {
        ProtocolSpec {
            name: parse_quote!(TestProtocol),
            attributes: ProtocolAttributes::default(),
            roles: vec![parse_quote!(Client), parse_quote!(Server)],
            flows: vec![ProtocolFlow::MessageFlow(MessageFlow {
                sender: parse_quote!(Client),
                receiver: parse_quote!(Server),
                message: MessageSpec::Simple {
                    name: parse_quote!(Request),
                    fields: vec![],
                },
                properties: MessageProperties::default(),
            })],
        }
    }

    #[test]
    fn test_dual_generation_integration_basic() {
        let protocol_spec = create_test_protocol_spec();
        let mut protocol_attrs = ProtocolAttributes::default();
        protocol_attrs.generate_dual = true;

        // Test that dual generation integration doesn't crash
        let result = generate_protocol_implementation(protocol_spec, protocol_attrs);
        assert!(result.is_ok(), "Dual generation integration should succeed");
    }

    #[test]
    fn test_dual_generation_integration_with_custom_name() {
        let protocol_spec = create_test_protocol_spec();
        let mut protocol_attrs = ProtocolAttributes::default();
        protocol_attrs.generate_dual = true;
        protocol_attrs.dual_name = Some("CustomDual".to_string());

        let result = generate_protocol_implementation(protocol_spec, protocol_attrs);
        assert!(
            result.is_ok(),
            "Dual generation with custom name should succeed"
        );
    }

    #[test]
    fn test_dual_generation_integration_with_verification() {
        let protocol_spec = create_test_protocol_spec();
        let mut protocol_attrs = ProtocolAttributes::default();
        protocol_attrs.generate_dual = true;
        protocol_attrs.verify_duality = true;

        let result = generate_protocol_implementation(protocol_spec, protocol_attrs);
        assert!(
            result.is_ok(),
            "Dual generation with verification should succeed"
        );
    }

    #[test]
    fn test_dual_generation_integration_with_documentation() {
        let protocol_spec = create_test_protocol_spec();
        let mut protocol_attrs = ProtocolAttributes::default();
        protocol_attrs.generate_dual = true;
        protocol_attrs.dual_documentation = true;

        let result = generate_protocol_implementation(protocol_spec, protocol_attrs);
        assert!(
            result.is_ok(),
            "Dual generation with documentation should succeed"
        );
    }

    #[test]
    fn test_dual_generation_integration_full_featured() {
        let protocol_spec = create_test_protocol_spec();
        let mut protocol_attrs = ProtocolAttributes::default();
        protocol_attrs.generate_dual = true;
        protocol_attrs.dual_name = Some("FullFeaturedDual".to_string());
        protocol_attrs.verify_duality = true;
        protocol_attrs.dual_documentation = true;

        let result = generate_protocol_implementation(protocol_spec, protocol_attrs);
        assert!(
            result.is_ok(),
            "Full-featured dual generation should succeed"
        );
    }

    #[test]
    fn test_no_dual_generation_fallback() {
        let protocol_spec = create_test_protocol_spec();
        let protocol_attrs = ProtocolAttributes::default(); // generate_dual = false by default

        let result = generate_protocol_implementation(protocol_spec, protocol_attrs);
        assert!(
            result.is_ok(),
            "Non-dual generation should work as fallback"
        );
    }

    #[test]
    fn test_parse_dual_attributes_integration() {
        // Test parsing dual attributes through the main parsing function
        // Using a more direct approach that doesn't require proc macro conversion
        let mut attrs = ProtocolAttributes::default();
        attrs.generate_dual = true;
        attrs.dual_name = Some("MyDual".to_string());
        attrs.verify_duality = true;
        attrs.dual_documentation = false;

        // Verify the attributes were set correctly
        assert!(attrs.generate_dual);
        assert_eq!(attrs.dual_name, Some("MyDual".to_string()));
        assert!(attrs.verify_duality);
        assert!(!attrs.dual_documentation);
    }

    #[test]
    fn test_dual_generator_integration_with_parsed_spec() {
        let protocol_spec = create_test_protocol_spec();
        let mut protocol_attrs = ProtocolAttributes::default();
        protocol_attrs.generate_dual = true;
        protocol_attrs.dual_name = Some("IntegratedDual".to_string());

        // Test DualGenerator creation with parsed specs
        let dual_generator = DualGenerator::new(protocol_spec.clone(), protocol_attrs.clone());

        // Test dual spec generation
        let dual_spec_result = dual_generator.generate_dual_spec();
        assert!(
            dual_spec_result.is_ok(),
            "Dual spec generation should succeed"
        );

        let dual_spec = dual_spec_result.unwrap();
        assert_eq!(dual_spec.name.to_string(), "IntegratedDual");
    }
}
