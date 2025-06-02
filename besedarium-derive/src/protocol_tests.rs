//! Tests for protocol attribute parsing and duplicate detection
//!
//! This module contains comprehensive tests for the protocol attribute parsing
//! functionality, with a focus on duplicate attribute detection.

#[cfg(test)]
mod tests {
    use super::super::protocol::parse_protocol_args_test;
    use proc_macro2::TokenStream;
    use std::str::FromStr;

    /// Test parsing of valid single attributes
    #[test]
    fn test_parse_single_attributes() {
        // Test io attribute
        let tokens = TokenStream::from_str("io = \"async\"").unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();
        assert_eq!(result.io_type, Some("async".to_string()));
        assert_eq!(result.metadata_type, None);

        // Test metadata attribute
        let tokens = TokenStream::from_str("metadata = \"standard\"").unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();
        assert_eq!(result.metadata_type, Some("standard".to_string()));
        assert_eq!(result.io_type, None);

        // Test buffer_size attribute
        let tokens = TokenStream::from_str("buffer_size = 1024").unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();
        assert_eq!(result.buffer_size, Some(1024));

        // Test timeout_ms attribute
        let tokens = TokenStream::from_str("timeout_ms = 5000").unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();
        assert_eq!(result.timeout_ms, Some(5000));

        // Test serialization attribute
        let tokens = TokenStream::from_str("serialization = \"json\"").unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();
        assert_eq!(result.serialization, Some("json".to_string()));

        // Test validation attribute
        let tokens = TokenStream::from_str("validation = true").unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();
        assert_eq!(result.validation, Some(true));

        // Test concurrent attribute
        let tokens = TokenStream::from_str("concurrent = false").unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();
        assert_eq!(result.concurrent, Some(false));

        // Test reliability attribute
        let tokens = TokenStream::from_str("reliability = \"at_least_once\"").unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();
        assert_eq!(result.reliability, Some("at_least_once".to_string()));
    }

    /// Test parsing of multiple valid attributes
    #[test]
    fn test_parse_multiple_attributes() {
        let tokens =
            TokenStream::from_str("io = \"async\", buffer_size = 2048, validation = true").unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();

        assert_eq!(result.io_type, Some("async".to_string()));
        assert_eq!(result.buffer_size, Some(2048));
        assert_eq!(result.validation, Some(true));
        assert_eq!(result.metadata_type, None);
        assert_eq!(result.timeout_ms, None);
    }

    /// Test duplicate io attribute detection
    #[test]
    fn test_duplicate_io_attribute() {
        let tokens = TokenStream::from_str("io = \"async\", io = \"sync\"").unwrap();
        let result = parse_protocol_args_test(tokens);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate attribute 'io'"));
        assert!(error_msg.contains("can only be specified once"));
    }

    /// Test duplicate metadata attribute detection
    #[test]
    fn test_duplicate_metadata_attribute() {
        let tokens =
            TokenStream::from_str("metadata = \"standard\", metadata = \"custom\"").unwrap();
        let result = parse_protocol_args_test(tokens);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate attribute 'metadata'"));
    }

    /// Test duplicate buffer_size attribute detection
    #[test]
    fn test_duplicate_buffer_size_attribute() {
        let tokens = TokenStream::from_str("buffer_size = 1024, buffer_size = 2048").unwrap();
        let result = parse_protocol_args_test(tokens);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate attribute 'buffer_size'"));
    }

    /// Test duplicate timeout_ms attribute detection
    #[test]
    fn test_duplicate_timeout_ms_attribute() {
        let tokens = TokenStream::from_str("timeout_ms = 1000, timeout_ms = 5000").unwrap();
        let result = parse_protocol_args_test(tokens);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate attribute 'timeout_ms'"));
    }

    /// Test duplicate serialization attribute detection
    #[test]
    fn test_duplicate_serialization_attribute() {
        let tokens =
            TokenStream::from_str("serialization = \"json\", serialization = \"binary\"").unwrap();
        let result = parse_protocol_args_test(tokens);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate attribute 'serialization'"));
    }

    /// Test duplicate validation attribute detection
    #[test]
    fn test_duplicate_validation_attribute() {
        let tokens = TokenStream::from_str("validation = true, validation = false").unwrap();
        let result = parse_protocol_args_test(tokens);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate attribute 'validation'"));
    }

    /// Test duplicate concurrent attribute detection
    #[test]
    fn test_duplicate_concurrent_attribute() {
        let tokens = TokenStream::from_str("concurrent = true, concurrent = false").unwrap();
        let result = parse_protocol_args_test(tokens);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate attribute 'concurrent'"));
    }

    /// Test duplicate reliability attribute detection
    #[test]
    fn test_duplicate_reliability_attribute() {
        let tokens = TokenStream::from_str(
            "reliability = \"at_least_once\", reliability = \"exactly_once\"",
        )
        .unwrap();
        let result = parse_protocol_args_test(tokens);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate attribute 'reliability'"));
    }

    /// Test duplicate generate_dual attribute detection
    #[test]
    fn test_duplicate_generate_dual_detection() {
        let tokens = TokenStream::from_str("generate_dual = true, generate_dual = false").unwrap();
        let result = parse_protocol_args_test(tokens);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate attribute 'generate_dual'"));
    }

    /// Test duplicate verify_duality attribute detection
    #[test]
    fn test_duplicate_verify_duality_detection() {
        let tokens =
            TokenStream::from_str("verify_duality = true, verify_duality = false").unwrap();
        let result = parse_protocol_args_test(tokens);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate attribute 'verify_duality'"));
    }

    /// Test duplicate dual_documentation attribute detection
    #[test]
    fn test_duplicate_dual_documentation_detection() {
        let tokens =
            TokenStream::from_str("dual_documentation = true, dual_documentation = false").unwrap();
        let result = parse_protocol_args_test(tokens);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate attribute 'dual_documentation'"));
    }

    /// Test mixed valid and duplicate attributes
    #[test]
    fn test_mixed_valid_and_duplicate_attributes() {
        let tokens = TokenStream::from_str(
            "io = \"async\", buffer_size = 1024, io = \"sync\", validation = true",
        )
        .unwrap();
        let result = parse_protocol_args_test(tokens);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate attribute 'io'"));
    }

    /// Test unknown attribute detection
    #[test]
    fn test_unknown_attribute() {
        let tokens = TokenStream::from_str("unknown_attr = \"value\"").unwrap();
        let result = parse_protocol_args_test(tokens);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Unknown protocol attribute: 'unknown_attr'"));
        assert!(error_msg.contains("Supported attributes"));
    }

    /// Test invalid value types
    #[test]
    fn test_invalid_value_types() {
        // Test string attribute with integer value
        let tokens = TokenStream::from_str("io = 123").unwrap();
        let result = parse_protocol_args_test(tokens);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Expected string literal for 'io' attribute"));

        // Test integer attribute with string value
        let tokens = TokenStream::from_str("buffer_size = \"not_a_number\"").unwrap();
        let result = parse_protocol_args_test(tokens);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Expected integer literal for 'buffer_size' attribute"));

        // Test boolean attribute with string value
        let tokens = TokenStream::from_str("validation = \"not_a_bool\"").unwrap();
        let result = parse_protocol_args_test(tokens);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Expected boolean literal for 'validation' attribute"));
    }

    /// Test empty input
    #[test]
    fn test_empty_input() {
        let tokens = TokenStream::from_str("").unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();

        // Should return default attributes
        assert_eq!(result.io_type, None);
        assert_eq!(result.metadata_type, None);
        assert_eq!(result.buffer_size, None);
        assert_eq!(result.timeout_ms, None);
        assert_eq!(result.serialization, None);
        assert_eq!(result.validation, None);
        assert_eq!(result.concurrent, None);
        assert_eq!(result.reliability, None);
        // Check dual generation defaults
        assert!(!result.generate_dual);
        assert_eq!(result.dual_name, None);
        assert!(!result.verify_duality);
        assert!(!result.dual_documentation);
    }

    /// Test comprehensive duplicate detection across all attributes
    #[test]
    fn test_comprehensive_duplicate_detection() {
        let test_cases = vec![
            ("io = \"async\", io = \"sync\"", "io"),
            ("metadata = \"std\", metadata = \"custom\"", "metadata"),
            ("buffer_size = 1024, buffer_size = 2048", "buffer_size"),
            ("timeout_ms = 1000, timeout_ms = 5000", "timeout_ms"),
            (
                "serialization = \"json\", serialization = \"binary\"",
                "serialization",
            ),
            ("validation = true, validation = false", "validation"),
            ("concurrent = true, concurrent = false", "concurrent"),
            (
                "reliability = \"once\", reliability = \"twice\"",
                "reliability",
            ),
        ];

        for (input, expected_attr) in test_cases {
            let tokens = TokenStream::from_str(input).unwrap();
            let result = parse_protocol_args_test(tokens);

            assert!(result.is_err(), "Expected error for input: {}", input);
            let error_msg = result.unwrap_err().to_string();
            assert!(
                error_msg.contains(&format!("Duplicate attribute '{}'", expected_attr)),
                "Expected duplicate error for '{}' in message: {}",
                expected_attr,
                error_msg
            );
        }
    }

    /// Test all attributes together (no duplicates)
    #[test]
    fn test_all_attributes_no_duplicates() {
        let tokens = TokenStream::from_str(
            "io = \"async\", metadata = \"standard\", buffer_size = 1024, timeout_ms = 5000, serialization = \"json\", validation = true, concurrent = false, reliability = \"at_least_once\""
        ).unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();

        assert_eq!(result.io_type, Some("async".to_string()));
        assert_eq!(result.metadata_type, Some("standard".to_string()));
        assert_eq!(result.buffer_size, Some(1024));
        assert_eq!(result.timeout_ms, Some(5000));
        assert_eq!(result.serialization, Some("json".to_string()));
        assert_eq!(result.validation, Some(true));
        assert_eq!(result.concurrent, Some(false));
        assert_eq!(result.reliability, Some("at_least_once".to_string()));
    }

    /// Test parsing of dual generation attributes
    #[test]
    fn test_parse_dual_generation_attributes() {
        // Test generate_dual attribute
        let tokens = TokenStream::from_str("generate_dual = true").unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();
        assert!(result.generate_dual);
        assert_eq!(result.dual_name, None);
        assert!(!result.verify_duality);
        assert!(!result.dual_documentation);

        // Test dual_name attribute
        let tokens = TokenStream::from_str("dual_name = \"CustomDual\"").unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();
        assert_eq!(result.dual_name, Some("CustomDual".to_string()));
        assert!(!result.generate_dual);

        // Test verify_duality attribute
        let tokens = TokenStream::from_str("verify_duality = true").unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();
        assert!(result.verify_duality);
        assert!(!result.generate_dual);

        // Test dual_documentation attribute
        let tokens = TokenStream::from_str("dual_documentation = true").unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();
        assert!(result.dual_documentation);
        assert!(!result.generate_dual);
    }

    /// Test parsing of multiple dual generation attributes
    #[test]
    fn test_parse_multiple_dual_attributes() {
        let tokens = TokenStream::from_str(
            "generate_dual = true, dual_name = \"ClientServerDual\", verify_duality = true, dual_documentation = true"
        ).unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();

        assert!(result.generate_dual);
        assert_eq!(result.dual_name, Some("ClientServerDual".to_string()));
        assert!(result.verify_duality);
        assert!(result.dual_documentation);
        // Check that other attributes remain at defaults
        assert_eq!(result.io_type, None);
        assert_eq!(result.buffer_size, None);
    }

    /// Test dual generation attributes with regular attributes
    #[test]
    fn test_dual_attributes_with_regular_attributes() {
        let tokens = TokenStream::from_str(
            "io = \"async\", generate_dual = true, buffer_size = 1024, dual_name = \"MyDual\"",
        )
        .unwrap();
        let result = parse_protocol_args_test(tokens).unwrap();

        // Check dual attributes
        assert!(result.generate_dual);
        assert_eq!(result.dual_name, Some("MyDual".to_string()));
        assert!(!result.verify_duality);
        assert!(!result.dual_documentation);
        // Check regular attributes
        assert_eq!(result.io_type, Some("async".to_string()));
        assert_eq!(result.buffer_size, Some(1024));
    }

    /// Test dual generation attribute error cases
    #[test]
    fn test_dual_attributes_error_cases() {
        // Test generate_dual with wrong type
        let tokens = TokenStream::from_str("generate_dual = \"not_a_bool\"").unwrap();
        let result = parse_protocol_args_test(tokens);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Expected boolean literal for 'generate_dual' attribute"));

        // Test dual_name with wrong type
        let tokens = TokenStream::from_str("dual_name = 123").unwrap();
        let result = parse_protocol_args_test(tokens);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Expected string literal for 'dual_name' attribute"));

        // Test verify_duality with wrong type
        let tokens = TokenStream::from_str("verify_duality = \"not_a_bool\"").unwrap();
        let result = parse_protocol_args_test(tokens);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Expected boolean literal for 'verify_duality' attribute"));

        // Test dual_documentation with wrong type
        let tokens = TokenStream::from_str("dual_documentation = 42").unwrap();
        let result = parse_protocol_args_test(tokens);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Expected boolean literal for 'dual_documentation' attribute"));
    }

    /// Test duplicate dual_name detection
    #[test]
    fn test_duplicate_dual_name_detection() {
        let tokens =
            TokenStream::from_str("dual_name = \"First\", dual_name = \"Second\"").unwrap();
        let result = parse_protocol_args_test(tokens);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate attribute 'dual_name'"));
    }
}
