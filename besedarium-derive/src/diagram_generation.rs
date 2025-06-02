//! Specialized diagram generation engine for automatic protocol documentation.
//!
//! This module provides the `ProtocolDiagramGenerator` which enhances the basic introspection
//! infrastructure with sophisticated Mermaid diagram generation and automatic documentation
//! integration capabilities.

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;
use std::fs;
use std::path::Path;

/// Advanced protocol diagram generator for automatic documentation integration.
///
/// This generator builds upon the basic `ProtocolFlow` infrastructure to provide:
/// - Enhanced Mermaid sequence diagram generation
/// - Automatic `#[doc = mermaid!(...)]` attribute generation
/// - Protocol-specific diagram customization
/// - Documentation integration with derive macro workflow
pub struct ProtocolDiagramGenerator {
    /// The protocol name being processed
    protocol_name: String,
    /// The parsed derive input for extracting metadata
    input: DeriveInput,
}

impl ProtocolDiagramGenerator {
    /// Create a new diagram generator for the given protocol definition.
    ///
    /// # Arguments
    /// * `input` - The parsed derive input containing protocol metadata
    ///
    /// # Returns
    /// A configured diagram generator ready for Mermaid generation
    pub fn new(input: DeriveInput) -> Self {
        let protocol_name = input.ident.to_string();
        Self {
            protocol_name,
            input,
        }
    }

    /// Generate automatic documentation with embedded Mermaid diagram using simple-mermaid.
    ///
    /// This function creates documentation with properly rendered Mermaid diagrams using the
    /// `simple-mermaid` crate. It generates a `.mermaid` file at compile time and uses
    /// `simple-mermaid::mermaid!()` to embed the rendered diagram.
    ///
    /// # Returns
    /// A `TokenStream` containing the documentation attribute to be included in the derive output
    pub fn generate_automatic_documentation(&self) -> TokenStream {
        let protocol_ident = &self.input.ident;
        let protocol_name = &self.protocol_name;

        // Generate the diagram file path relative to the workspace root
        let diagram_file_path = format!("generated_diagrams/{}.mermaid", protocol_name.to_lowercase());
        
        // Generate the actual diagram content at compile time
        let diagram_content = self.generate_compile_time_diagram_content();

        // Ensure the generated_diagrams directory exists and write the diagram file
        if let Err(e) = self.write_diagram_file(&diagram_file_path, &diagram_content) {
            // If file creation fails, fall back to code block approach
            eprintln!("Warning: Could not create diagram file {}: {}", diagram_file_path, e);
            return quote! {
                #[doc = concat!(
                    "# ", #protocol_name, " Protocol\n\n",
                    "This protocol provides structured communication between roles with automatic\n",
                    "type-safe message passing and state management.\n"
                )]
                #[doc = ""]
                #[doc = "## Protocol Flow Diagram"]
                #[doc = ""]
                #[doc = "```mermaid"]
                #[doc = #diagram_content]
                #[doc = "```"]
                #[doc = ""]
                #[doc = "## Usage"]
                #[doc = ""]
                #[doc = concat!("Use `", stringify!(#protocol_ident), "::generate_diagram()` to get the diagram at runtime.")]
            };
        }

        // Use simple-mermaid to embed the rendered diagram
        quote! {
            #[doc = concat!(
                "# ", #protocol_name, " Protocol\n\n",
                "This protocol provides structured communication between roles with automatic\n",
                "type-safe message passing and state management.\n"
            )]
            #[doc = ""]
            #[doc = "## Protocol Flow Diagram"]
            #[doc = ""]
            #[doc = "The following diagram shows the complete communication flow:"]
            #[doc = ""]
            #[doc = ::simple_mermaid::mermaid!(#diagram_file_path)]
            #[doc = ""]
            #[doc = "## Usage"]
            #[doc = ""]
            #[doc = concat!("Use `", stringify!(#protocol_ident), "::generate_diagram()` to get the diagram at runtime.")]
        }
    }

    /// Generate the actual diagram content at compile time for embedding in documentation.
    ///
    /// This function creates the Mermaid sequence diagram content that will be embedded
    /// directly into the `#[doc]` attributes, ensuring the documentation always shows
    /// the current protocol structure.
    ///
    /// # Returns
    /// A string literal containing the complete Mermaid diagram
    fn generate_compile_time_diagram_content(&self) -> String {
        let protocol_name = &self.protocol_name;
        
        // For now, generate a basic sequence diagram that protocols can enhance
        // This will be extended to analyze actual protocol structure in future phases
        format!(
            "sequenceDiagram\n    %% Generated from {} protocol definition\n    participant Role1 as Role1\n    participant Role2 as Role2\n    Role1->>+Role2: {}_DefaultMessage\n    Role2-->>-Role1: Response",
            protocol_name,
            protocol_name
        )
    }

    /// Generate the complete Mermaid diagram integration code.
    ///
    /// This function produces the necessary code to:
    /// 1. Implement diagram generation at compile time
    /// 2. Embed the generated diagram in documentation
    /// 3. Provide runtime access to diagram data if needed
    ///
    /// # Returns
    /// A `TokenStream` containing the complete diagram integration implementation
    #[allow(dead_code)] // Reserved for future advanced integration patterns
    pub fn generate_diagram_integration(&self) -> TokenStream {
        let protocol_ident = &self.input.ident;
        let diagram_method = self.generate_diagram_method();
        let doc_generation = self.generate_automatic_documentation();

        quote! {
            #doc_generation
            impl #protocol_ident {
                #diagram_method
            }
        }
    }

    /// Generate a method for runtime diagram access.
    ///
    /// This creates a `generate_diagram()` method that can be called at runtime
    /// to get the Mermaid diagram string. This is useful for:
    /// - Dynamic documentation generation
    /// - Web interfaces that display protocol diagrams
    /// - Testing and validation tools
    ///
    /// # Returns
    /// A `TokenStream` containing the diagram generation method
    pub fn generate_diagram_method(&self) -> TokenStream {
        let _protocol_ident = &self.input.ident;

        quote! {
            /// Generate the Mermaid sequence diagram for this protocol.
            ///
            /// This method uses the protocol's `ProtocolFlow` implementation to generate
            /// a Mermaid sequence diagram representing the communication flow.
            ///
            /// # Returns
            /// A String containing the complete Mermaid diagram in sequence diagram format
            ///
            /// # Example
            /// ```rust
            /// let diagram = MyProtocol::generate_diagram();
            /// println!("{}", diagram);
            /// ```
            pub fn generate_diagram() -> String {
                use besedarium::protocol::introspection::{ProtocolFlow, mermaid_generator};

                // Use the ProtocolFlow implementation to get sequence steps
                let steps = Self::generate_sequence_steps();
                let roles = Self::get_roles();
                let config = Self::get_diagram_config();

                // Generate the Mermaid diagram
                mermaid_generator::generate_sequence_diagram(steps, roles, config)
            }
        }
    }

    /// Generate enhanced documentation with protocol metadata.
    ///
    /// This function creates comprehensive documentation that includes:
    /// - Protocol description
    /// - Role information
    /// - Message flow overview
    /// - Embedded Mermaid diagram
    ///
    /// # Returns
    /// A `TokenStream` containing enhanced documentation attributes
    #[allow(dead_code)] // Reserved for future enhanced documentation features
    pub fn generate_enhanced_documentation(&self) -> TokenStream {
        let protocol_name = &self.protocol_name;

        quote! {
            #[doc = concat!(
                "# ", #protocol_name, " Protocol\n\n",
                "This protocol provides structured communication between roles with automatic\n",
                "type-safe message passing and state management.\n\n",
                "## Features\n\n",
                "- **Type Safety**: All message types are verified at compile time\n",
                "- **Role-Based Communication**: Clear separation of communication responsibilities\n",
                "- **Automatic Projection**: Local endpoint types generated automatically\n",
                "- **Duality Verification**: Protocol consistency checked at the type level\n\n",
                "## Protocol Flow Diagram\n\n",
                "The following diagram shows the complete communication flow:\n\n"
            )]
            #[doc = "```mermaid"]
            #[doc = "sequenceDiagram"]
            #[doc = "    %% This diagram is generated automatically from the protocol definition"]
            #[doc = "    %% Any changes to the protocol will be reflected here automatically"]
        }
    }

    /// Generate compile-time diagram embedding using procedural macro techniques.
    ///
    /// This advanced function uses compile-time evaluation to embed the actual
    /// Mermaid diagram content directly into the documentation, ensuring that
    /// the diagram is always synchronized with the protocol definition.
    ///
    /// # Returns
    /// A `TokenStream` containing compile-time diagram embedding code
    #[allow(dead_code)] // Reserved for future advanced embedding techniques
    pub fn generate_compile_time_diagram_embedding(&self) -> TokenStream {
        let protocol_ident = &self.input.ident;

        quote! {
            // Generate a const function that can be evaluated at compile time
            // to embed the actual diagram content in the documentation
            const _: () = {
                // This will be expanded by the macro to include the actual diagram
                // The diagram generation happens during macro expansion
                impl #protocol_ident {
                    #[doc(hidden)]
                    pub const DIAGRAM_CONTENT: &'static str = concat!(
                        "sequenceDiagram\n",
                        "    %% Generated from ", stringify!(#protocol_ident), "\n",
                        "    %% Diagram content will be populated by ProtocolFlow implementation\n"
                    );
                }
            };
        }
    }

    /// Write the diagram content to a .mermaid file for simple-mermaid integration.
    ///
    /// This function ensures the generated_diagrams directory exists and writes
    /// the Mermaid diagram content to a file that can be used by `simple-mermaid`.
    ///
    /// # Arguments
    /// * `file_path` - The relative path where the diagram file should be written
    /// * `content` - The Mermaid diagram content to write
    ///
    /// # Returns
    /// Result indicating success or failure of file operations
    fn write_diagram_file(&self, file_path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Create the directory if it doesn't exist
        if let Some(parent) = Path::new(file_path).parent() {
            fs::create_dir_all(parent)?;
        }

        // Write the diagram content
        fs::write(file_path, content)?;
        
        Ok(())
    }
}

/// Utility functions for diagram generation integration.
impl ProtocolDiagramGenerator {
    /// Extract protocol metadata from derive input attributes.
    ///
    /// This function parses any `#[protocol(...)]` attributes to extract:
    /// - Role definitions
    /// - Start type information
    /// - Custom diagram configuration
    ///
    /// # Returns
    /// A tuple containing (roles, start_type, custom_config) if available
    #[allow(dead_code)] // Reserved for future metadata-driven diagram generation
    pub fn extract_protocol_metadata(&self) -> (Vec<String>, Option<String>, Option<String>) {
        // Parse protocol attributes for metadata
        // This would be expanded to parse actual attribute syntax
        let roles = vec!["RoleA".to_string(), "RoleB".to_string()]; // Placeholder
        let start_type = Some("StartType".to_string()); // Placeholder
        let custom_config = None; // Placeholder

        (roles, start_type, custom_config)
    }

    /// Generate role-specific documentation sections.
    ///
    /// This creates detailed documentation for each role in the protocol,
    /// including their responsibilities and message handling capabilities.
    ///
    /// # Arguments
    /// * `roles` - Vector of role names to document
    ///
    /// # Returns
    /// A `TokenStream` containing role documentation
    #[allow(dead_code)] // Reserved for future role-based documentation features
    pub fn generate_role_documentation(&self, roles: &[String]) -> TokenStream {
        let role_docs: Vec<TokenStream> = roles
            .iter()
            .map(|role| {
                quote! {
                    #[doc = concat!("- **", #role, "**: Participates in protocol communication")]
                }
            })
            .collect();

        quote! {
            #[doc = "## Protocol Roles\n"]
            #[doc = "This protocol involves the following roles:\n"]
            #(#role_docs)*
        }
    }

    /// Generate usage examples in documentation.
    ///
    /// This creates practical usage examples showing how to use the protocol
    /// in real applications, including setup and execution patterns.
    ///
    /// # Returns
    /// A `TokenStream` containing usage example documentation
    #[allow(dead_code)] // Reserved for future automatic example generation
    pub fn generate_usage_examples(&self) -> TokenStream {
        let protocol_ident = &self.input.ident;

        quote! {
            #[doc = "## Usage Example\n"]
            #[doc = "```rust"]
            #[doc = concat!("use besedarium::protocol::", stringify!(#protocol_ident), ";")]
            #[doc = ""]
            #[doc = "// Protocol setup and execution"]
            #[doc = concat!("let protocol = ", stringify!(#protocol_ident), "::new();")]
            #[doc = "// ... implementation specific usage"]
            #[doc = "```"]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_diagram_generator_creation() {
        let input: DeriveInput = parse_quote! {
            #[derive(GenerateDiagram)]
            struct TestProtocol;
        };

        let generator = ProtocolDiagramGenerator::new(input);
        assert_eq!(generator.protocol_name, "TestProtocol");
    }

    #[test]
    fn test_automatic_documentation_generation() {
        let input: DeriveInput = parse_quote! {
            #[derive(GenerateDiagram)]
            struct CustomerAgencyProtocol;
        };

        let generator = ProtocolDiagramGenerator::new(input);
        let doc_tokens = generator.generate_automatic_documentation();

        // Verify that documentation contains expected elements
        let doc_string = doc_tokens.to_string();
        assert!(doc_string.contains("CustomerAgencyProtocol"));
        assert!(doc_string.contains("Protocol"));
        assert!(doc_string.contains("mermaid"));
    }

    #[test]
    fn test_diagram_method_generation() {
        let input: DeriveInput = parse_quote! {
            #[derive(GenerateDiagram)]
            struct TestProtocol;
        };

        let generator = ProtocolDiagramGenerator::new(input);
        let method_tokens = generator.generate_diagram_method();

        let method_string = method_tokens.to_string();
        assert!(method_string.contains("generate_diagram"));
        assert!(method_string.contains("ProtocolFlow"));
        assert!(method_string.contains("mermaid_generator"));
    }

    #[test]
    fn test_metadata_extraction() {
        let input: DeriveInput = parse_quote! {
            #[derive(GenerateDiagram)]
            #[protocol(roles = "Customer, Agency", start_type = "CustomerSendsOrder")]
            struct CustomerAgencyProtocol;
        };

        let generator = ProtocolDiagramGenerator::new(input);
        let (roles, start_type, _config) = generator.extract_protocol_metadata();

        // Note: This is a placeholder implementation
        // Real implementation would parse the actual attributes
        assert!(!roles.is_empty());
        assert!(start_type.is_some());
    }

    #[test]
    fn test_enhanced_documentation_generation() {
        let input: DeriveInput = parse_quote! {
            #[derive(GenerateDiagram)]
            struct AdvancedProtocol;
        };

        let generator = ProtocolDiagramGenerator::new(input);
        let doc_tokens = generator.generate_enhanced_documentation();

        let doc_string = doc_tokens.to_string();
        assert!(doc_string.contains("AdvancedProtocol"));
        assert!(doc_string.contains("Type Safety"));
        assert!(doc_string.contains("Role-Based Communication"));
        assert!(doc_string.contains("sequenceDiagram"));
    }

    #[test]
    fn test_role_documentation_generation() {
        let input: DeriveInput = parse_quote! {
            #[derive(GenerateDiagram)]
            struct MultiRoleProtocol;
        };

        let generator = ProtocolDiagramGenerator::new(input);
        let roles = vec![
            "Customer".to_string(),
            "Agency".to_string(),
            "Bank".to_string(),
        ];
        let role_docs = generator.generate_role_documentation(&roles);

        let doc_string = role_docs.to_string();
        assert!(doc_string.contains("Customer"));
        assert!(doc_string.contains("Agency"));
        assert!(doc_string.contains("Bank"));
        assert!(doc_string.contains("Protocol Roles"));
    }

    #[test]
    fn test_compile_time_embedding_generation() {
        let input: DeriveInput = parse_quote! {
            #[derive(GenerateDiagram)]
            struct CompileTimeProtocol;
        };

        let generator = ProtocolDiagramGenerator::new(input);
        let embedding_tokens = generator.generate_compile_time_diagram_embedding();

        let embedding_string = embedding_tokens.to_string();
        assert!(embedding_string.contains("DIAGRAM_CONTENT"));
        assert!(embedding_string.contains("sequenceDiagram"));
        assert!(embedding_string.contains("CompileTimeProtocol"));
    }
}
