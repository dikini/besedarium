//! # Protocol Introspection Infrastructure
//!
//! This module provides the core infrastructure for extracting protocol flow information
//! from type-level protocol definitions. It enables automatic diagram generation by
//! analyzing protocol structure through trait-based introspection.
//!
//! ## Key Components
//!
//! - **ProtocolFlow**: Core trait for extracting sequence steps from protocols
//! - **SequenceStep**: Enum representing different types of protocol actions
//! - **ProtocolAnalyzer**: Helper for protocol structure traversal
//! - **Type-Level Extraction**: Utilities for working within Rust stable constraints

use crate::protocol::foundation::{GlobalProtocol, Message, Role};
use std::fmt::Debug;

// ============================================================================
// Core Protocol Introspection Traits
// ============================================================================

/// Main trait for extracting protocol flow information from type definitions
///
/// This trait enables automatic diagram generation by providing a way to extract
/// the sequence of protocol actions from type-level protocol definitions.
///
/// # Design Principles
///
/// - **Type-Level Analysis**: Extract information from the type system
/// - **Derive Macro Compatible**: Designed to work with proc macro code generation
/// - **Stable Rust**: Works within stable Rust constraints (no specialization)
///
/// # Example
///
/// ```rust,ignore
/// #[derive(Protocol, GenerateDiagram)]
/// pub struct CustomerAgencyProtocol;
///
/// impl ProtocolFlow for CustomerAgencyProtocol {
///     fn generate_sequence_steps() -> Vec<SequenceStep> {
///         vec![
///             SequenceStep::Send {
///                 from: "Customer".to_string(),
///                 to: "Agency".to_string(),
///                 message: "Order".to_string(),
///             },
///             // ... more steps
///         ]
///     }
/// }
/// ```
pub trait ProtocolFlow {
    /// Generate a sequence of protocol steps for visualization
    fn generate_sequence_steps() -> Vec<SequenceStep>;

    /// Get all roles involved in this protocol
    fn get_roles() -> Vec<String>;

    /// Get the protocol name for diagram titles
    fn get_protocol_name() -> String;

    /// Get optional diagram configuration
    fn get_diagram_config() -> DiagramConfig {
        DiagramConfig::default()
    }
}

/// Represents a single step in a protocol sequence
///
/// This enum captures the different types of actions that can occur
/// in a protocol, suitable for generating sequence diagrams.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SequenceStep {
    /// A message being sent from one role to another
    Send {
        from: String,
        to: String,
        message: String,
    },

    /// A message being received (typically same as Send, but explicit)
    Receive {
        from: String,
        to: String,
        message: String,
    },

    /// A choice point where a role decides between options
    Choice {
        role: String,
        options: Vec<ChoiceOption>,
    },

    /// Parallel composition of multiple protocol branches
    Parallel { branches: Vec<Vec<SequenceStep>> },

    /// Start of a recursive block
    RecursionStart { label: String },

    /// Reference to a recursive label (loop back)
    RecursionVar { label: String },

    /// End of the protocol
    End,

    /// Continue to next protocol phase
    Continue { next_protocol: String },
}

/// Represents a choice option in a protocol
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChoiceOption {
    pub label: String,
    pub steps: Vec<SequenceStep>,
}

/// Configuration for diagram generation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramConfig {
    pub title: Option<String>,
    pub theme: DiagramTheme,
    pub show_activations: bool,
    pub show_notes: bool,
}

impl Default for DiagramConfig {
    fn default() -> Self {
        Self {
            title: None,
            theme: DiagramTheme::Default,
            show_activations: true,
            show_notes: false,
        }
    }
}

/// Available diagram themes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagramTheme {
    Default,
    Dark,
    Neutral,
    Base,
}

// ============================================================================
// Protocol Analysis Helper Traits
// ============================================================================

/// Helper trait for analyzing protocol structure in a type-safe way
///
/// This trait provides utilities for traversing protocol types and extracting
/// information needed for diagram generation. It's designed to work with
/// derive macro code generation.
pub trait ProtocolAnalyzer<P: GlobalProtocol> {
    /// Extract the starting protocol type for analysis
    type StartType: GlobalProtocol;

    /// Analyze the protocol structure and generate steps
    fn analyze() -> Vec<SequenceStep>;

    /// Extract all roles from the protocol
    fn extract_roles() -> Vec<String>;

    /// Get the protocol name from type information
    fn get_name() -> String;
}

/// Marker trait for protocols that support automatic diagram generation
///
/// This trait is intended to be derived automatically and indicates that
/// a protocol has the necessary introspection implementation.
pub trait GeneratesDiagram: GlobalProtocol + ProtocolFlow {
    /// Generate a Mermaid sequence diagram for this protocol
    fn generate_mermaid_diagram() -> String {
        let steps = Self::generate_sequence_steps();
        let roles = Self::get_roles();
        let config = Self::get_diagram_config();

        mermaid_generator::generate_sequence_diagram(steps, roles, config)
    }

    /// Generate documentation with embedded diagram
    fn generate_doc_comment() -> String {
        let diagram = Self::generate_mermaid_diagram();
        format!(
            "//! # {}\n//!\n#[doc = mermaid!(\nr#\"{}\n\"#\n)]",
            Self::get_protocol_name(),
            diagram
        )
    }
}

// ============================================================================
// Type-Level Protocol Introspection Utilities
// ============================================================================

/// Utility functions for extracting type information within stable Rust constraints
pub mod type_utils {
    use super::*;

    /// Extract type name from a type (for use in derive macros)
    pub fn extract_type_name<T>() -> String {
        std::any::type_name::<T>()
            .split("::")
            .last()
            .unwrap_or("Unknown")
            .to_string()
    }

    /// Helper for role name extraction in derive macros
    pub fn role_name<R: Role>() -> String {
        extract_type_name::<R>()
    }

    /// Helper for message name extraction in derive macros  
    pub fn message_name<M: Message>() -> String {
        extract_type_name::<M>()
    }
}

// ============================================================================
// Mermaid Generation Module
// ============================================================================

/// Mermaid diagram generation utilities
pub mod mermaid_generator {
    use super::*;

    /// Generate a Mermaid sequence diagram from protocol steps
    pub fn generate_sequence_diagram(
        steps: Vec<SequenceStep>,
        roles: Vec<String>,
        config: DiagramConfig,
    ) -> String {
        let mut diagram = String::new();

        // Add title if specified
        if let Some(title) = config.title {
            diagram.push_str(&format!("title {}\n", title));
        }

        // Start sequence diagram
        diagram.push_str("sequenceDiagram\n");

        // Add participants
        for role in roles {
            diagram.push_str(&format!("    participant {} as {}\n", role, role));
        }

        // Add steps
        for step in steps {
            match step {
                SequenceStep::Send { from, to, message } => {
                    diagram.push_str(&format!("    {}->>+{}: {}\n", from, to, message));
                }
                SequenceStep::Receive { from, to, message } => {
                    diagram.push_str(&format!("    {}-->>-{}: {}\n", from, to, message));
                }
                SequenceStep::Choice { role: _, options } => {
                    if options.len() >= 2 {
                        diagram.push_str(&format!("    alt {}\n", options[0].label));
                        for option_step in &options[0].steps {
                            diagram.push_str(&format!(
                                "        {}\n",
                                format_step_for_choice(option_step)
                            ));
                        }

                        for option in &options[1..] {
                            diagram.push_str(&format!("    else {}\n", option.label));
                            for option_step in &option.steps {
                                diagram.push_str(&format!(
                                    "        {}\n",
                                    format_step_for_choice(option_step)
                                ));
                            }
                        }
                        diagram.push_str("    end\n");
                    }
                }
                SequenceStep::Parallel { branches } => {
                    diagram.push_str("    par\n");
                    if let Some(first_branch) = branches.first() {
                        for step in first_branch {
                            diagram
                                .push_str(&format!("        {}\n", format_step_for_choice(step)));
                        }
                    }

                    for branch in &branches[1..] {
                        diagram.push_str("    and\n");
                        for step in branch {
                            diagram
                                .push_str(&format!("        {}\n", format_step_for_choice(step)));
                        }
                    }
                    diagram.push_str("    end\n");
                }
                SequenceStep::RecursionStart { label } => {
                    diagram.push_str(&format!("    loop {}\n", label));
                }
                SequenceStep::RecursionVar { label: _ } => {
                    diagram.push_str("    end\n");
                }
                SequenceStep::End => {
                    // End is implicit in mermaid
                }
                SequenceStep::Continue { next_protocol } => {
                    diagram.push_str(&format!("    Note over: Continue to {}\n", next_protocol));
                }
            }
        }

        diagram
    }

    /// Format a step for use inside choice or parallel blocks
    fn format_step_for_choice(step: &SequenceStep) -> String {
        match step {
            SequenceStep::Send { from, to, message } => {
                format!("{}->>+{}: {}", from, to, message)
            }
            SequenceStep::Receive { from, to, message } => {
                format!("{}-->>-{}: {}", from, to, message)
            }
            SequenceStep::End => "Note: End".to_string(),
            _ => {
                format!("Note: {:?}", step)
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_step_creation() {
        let step = SequenceStep::Send {
            from: "Customer".to_string(),
            to: "Agency".to_string(),
            message: "Order".to_string(),
        };

        match step {
            SequenceStep::Send { from, to, message } => {
                assert_eq!(from, "Customer");
                assert_eq!(to, "Agency");
                assert_eq!(message, "Order");
            }
            _ => panic!("Wrong step type"),
        }
    }

    #[test]
    fn test_choice_option_creation() {
        let option = ChoiceOption {
            label: "Accept".to_string(),
            steps: vec![SequenceStep::End],
        };

        assert_eq!(option.label, "Accept");
        assert_eq!(option.steps.len(), 1);
    }

    #[test]
    fn test_diagram_config_default() {
        let config = DiagramConfig::default();

        assert!(config.title.is_none());
        assert_eq!(config.theme, DiagramTheme::Default);
        assert!(config.show_activations);
        assert!(!config.show_notes);
    }

    #[test]
    fn test_mermaid_generation_simple() {
        let steps = vec![
            SequenceStep::Send {
                from: "Customer".to_string(),
                to: "Agency".to_string(),
                message: "Order".to_string(),
            },
            SequenceStep::Receive {
                from: "Agency".to_string(),
                to: "Customer".to_string(),
                message: "Quote".to_string(),
            },
        ];

        let roles = vec!["Customer".to_string(), "Agency".to_string()];
        let config = DiagramConfig::default();

        let diagram = mermaid_generator::generate_sequence_diagram(steps, roles, config);

        assert!(diagram.contains("sequenceDiagram"));
        assert!(diagram.contains("Customer->>+Agency: Order"));
        assert!(diagram.contains("Agency-->>-Customer: Quote"));
        assert!(diagram.contains("participant Customer as Customer"));
        assert!(diagram.contains("participant Agency as Agency"));
    }

    #[test]
    fn test_type_name_extraction() {
        let name = type_utils::extract_type_name::<String>();
        assert_eq!(name, "String");
    }
}
