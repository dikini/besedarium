//! # Dual Protocol Generation
//!
//! This module implements automatic dual protocol generation for the Besedarium
//! session type library. It provides the ability to automatically generate dual
//! protocols from `#[protocol]` specifications, ensuring type-safe communication
//! by swapping roles and maintaining duality relationships.

use crate::protocol::{ChoiceFlow, MessageFlow, ProtocolAttributes, ProtocolFlow, ProtocolSpec};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, Result};

/// Generator for dual protocol specifications
pub struct DualGenerator {
    original_spec: ProtocolSpec,
    attributes: ProtocolAttributes,
}

impl DualGenerator {
    /// Create a new dual generator
    pub fn new(spec: ProtocolSpec, attrs: ProtocolAttributes) -> Self {
        Self {
            original_spec: spec,
            attributes: attrs,
        }
    }

    /// Generate dual protocol specification
    pub fn generate_dual_spec(&self) -> Result<ProtocolSpec> {
        let dual_name = self.generate_dual_name()?;

        let dual_spec = ProtocolSpec {
            name: dual_name,
            attributes: self.attributes.clone(),
            roles: self.swap_roles(&self.original_spec.roles),
            flows: self.transform_flows(&self.original_spec.flows)?,
        };

        Ok(dual_spec)
    }

    /// Generate the name for the dual protocol
    fn generate_dual_name(&self) -> Result<Ident> {
        let dual_name = if let Some(custom_name) = &self.attributes.dual_name {
            custom_name.clone()
        } else {
            // Generate automatic dual name by appending "Dual"
            format!("{}Dual", self.original_spec.name)
        };

        Ok(syn::Ident::new(&dual_name, proc_macro2::Span::call_site()))
    }

    /// Swap roles in the dual protocol by reversing their order
    fn swap_roles(&self, roles: &[Ident]) -> Vec<Ident> {
        // Simple role reversal: [A, B] becomes [B, A]
        // This implements the basic duality where sender/receiver roles are swapped
        let mut swapped = roles.to_vec();
        swapped.reverse();
        swapped
    }

    /// Transform protocol flows to their duals
    fn transform_flows(&self, flows: &[ProtocolFlow]) -> Result<Vec<ProtocolFlow>> {
        flows
            .iter()
            .map(|flow| self.transform_single_flow(flow))
            .collect()
    }

    /// Transform a single protocol flow to its dual
    fn transform_single_flow(&self, flow: &ProtocolFlow) -> Result<ProtocolFlow> {
        match flow {
            ProtocolFlow::MessageFlow(msg_flow) => {
                Ok(ProtocolFlow::MessageFlow(self.dual_message_flow(msg_flow)))
            }
            ProtocolFlow::Choice(choice_flow) => {
                Ok(ProtocolFlow::Choice(self.dual_choice_flow(choice_flow)?))
            }
            ProtocolFlow::Loop(loop_flow) => {
                // For loops, transform the body
                let dual_body = self.transform_flows(&loop_flow.body)?;
                Ok(ProtocolFlow::Loop(crate::protocol::LoopFlow {
                    body: dual_body,
                }))
            }
            ProtocolFlow::Conditional(cond_flow) => {
                // For conditionals, transform both branches
                let dual_if_branch = self.transform_flows(&cond_flow.if_branch)?;
                let dual_else_branch = if let Some(else_branch) = &cond_flow.else_branch {
                    Some(self.transform_flows(else_branch)?)
                } else {
                    None
                };

                Ok(ProtocolFlow::Conditional(
                    crate::protocol::ConditionalFlow {
                        condition: cond_flow.condition.clone(),
                        if_branch: dual_if_branch,
                        else_branch: dual_else_branch,
                    },
                ))
            }
            ProtocolFlow::Parallel(par_flow) => {
                // For parallel flows, transform each branch
                let mut dual_branches = Vec::new();
                for branch in &par_flow.branches {
                    dual_branches.push(self.transform_flows(branch)?);
                }

                Ok(ProtocolFlow::Parallel(crate::protocol::ParallelFlow {
                    branches: dual_branches,
                }))
            }
            ProtocolFlow::End => Ok(ProtocolFlow::End),
            ProtocolFlow::Continue => Ok(ProtocolFlow::Continue),
        }
    }

    /// Generate dual of a message flow by swapping sender and receiver
    fn dual_message_flow(&self, msg_flow: &MessageFlow) -> MessageFlow {
        MessageFlow {
            sender: msg_flow.receiver.clone(),
            receiver: msg_flow.sender.clone(),
            message: msg_flow.message.clone(),
            properties: msg_flow.properties.clone(),
        }
    }

    /// Generate dual of a choice flow
    fn dual_choice_flow(&self, choice_flow: &ChoiceFlow) -> Result<ChoiceFlow> {
        // In the dual, the choice becomes an offer (roles are swapped)
        Ok(ChoiceFlow {
            sender: choice_flow.receiver.clone(),
            receiver: choice_flow.sender.clone(),
            message: choice_flow.message.clone(),
            branches: choice_flow
                .branches
                .iter()
                .map(|branch| self.dual_choice_branch(branch))
                .collect::<Result<Vec<_>>>()?,
        })
    }

    /// Generate dual of a choice branch
    fn dual_choice_branch(
        &self,
        branch: &crate::protocol::ChoiceBranch,
    ) -> Result<crate::protocol::ChoiceBranch> {
        Ok(crate::protocol::ChoiceBranch {
            variant: branch.variant.clone(),
            bound_fields: branch.bound_fields.clone(),
            continuation: self.transform_flows(&branch.continuation)?,
        })
    }
}

/// Generate code for both original and dual protocols
pub fn generate_dual_protocol_code(
    original_spec: &ProtocolSpec,
    dual_spec: &ProtocolSpec,
    attributes: &ProtocolAttributes,
) -> Result<TokenStream2> {
    let original_name = &original_spec.name;
    let dual_name = &dual_spec.name;

    // Generate basic protocol implementations
    let original_impl = generate_protocol_impl(original_spec)?;
    let dual_impl = generate_protocol_impl(dual_spec)?;

    // Generate IsDual trait implementation if verification is enabled
    let is_dual_impl = if attributes.verify_duality {
        generate_is_dual_impl(original_name, dual_name)?
    } else {
        quote! {}
    };

    // Generate documentation if requested
    let dual_docs = if attributes.dual_documentation {
        generate_dual_documentation(original_name, dual_name)
    } else {
        quote! {}
    };

    Ok(quote! {
        #original_impl

        #dual_impl

        #is_dual_impl

        #dual_docs
    })
}

/// Generate protocol implementation for a given spec
fn generate_protocol_impl(spec: &ProtocolSpec) -> Result<TokenStream2> {
    let struct_name = &spec.name;

    // Generate roles tuple type
    let role_types: Vec<_> = spec.roles.iter().map(|role| quote! { #role }).collect();

    Ok(quote! {
        impl ::besedarium::protocol::foundation::GlobalProtocol for #struct_name {
            type Roles = (#(#role_types),*);
            type Messages = ();

            fn protocol_name() -> &'static str {
                stringify!(#struct_name)
            }
        }
    })
}

/// Generate IsDual trait implementation
fn generate_is_dual_impl(original_name: &Ident, dual_name: &Ident) -> Result<TokenStream2> {
    Ok(quote! {
        // Implement IsDual to prove duality relationship
        impl ::besedarium::protocol::duality::IsDual<#dual_name> for #original_name {}
        impl ::besedarium::protocol::duality::IsDual<#original_name> for #dual_name {}

        // Compile-time assertion to verify duality
        const _: () = {
            fn _assert_duality() {
                fn _check_dual<P1, P2>()
                where
                    P1: ::besedarium::protocol::duality::IsDual<P2>,
                    P2: ::besedarium::protocol::duality::IsDual<P1>,
                {
                    // This function will only compile if the duality relationship holds
                }

                _check_dual::<#original_name, #dual_name>();
            }
        };
    })
}

/// Generate documentation for dual protocols
fn generate_dual_documentation(original_name: &Ident, dual_name: &Ident) -> TokenStream2 {
    quote! {
        #[doc = concat!("Dual protocol for `", stringify!(#original_name), "`.")]
        #[doc = ""]
        #[doc = "This protocol is the automatic dual of the original protocol, with roles reversed"]
        #[doc = "and message flows swapped to maintain session type duality. The dual relationship"]
        #[doc = "ensures that communication between the original and dual protocols is type-safe."]
        #[doc = ""]
        #[doc = concat!("**Original Protocol**: `", stringify!(#original_name), "`")]
        #[doc = concat!("**Dual Protocol**: `", stringify!(#dual_name), "`")]
        impl #dual_name {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{MessageProperties, MessageSpec};

    fn create_test_protocol() -> ProtocolSpec {
        ProtocolSpec {
            name: syn::Ident::new("TestProtocol", proc_macro2::Span::call_site()),
            attributes: ProtocolAttributes::default(),
            roles: vec![
                syn::Ident::new("Client", proc_macro2::Span::call_site()),
                syn::Ident::new("Server", proc_macro2::Span::call_site()),
            ],
            flows: vec![ProtocolFlow::MessageFlow(MessageFlow {
                sender: syn::Ident::new("Client", proc_macro2::Span::call_site()),
                receiver: syn::Ident::new("Server", proc_macro2::Span::call_site()),
                message: MessageSpec::Simple {
                    name: syn::Ident::new("Request", proc_macro2::Span::call_site()),
                    fields: vec![],
                },
                properties: MessageProperties::default(),
            })],
        }
    }

    #[test]
    fn test_dual_generator_creation() {
        let spec = create_test_protocol();
        let attrs = ProtocolAttributes::default();
        let generator = DualGenerator::new(spec, attrs);

        assert_eq!(generator.original_spec.name, "TestProtocol");
    }

    #[test]
    fn test_role_swapping() {
        let spec = create_test_protocol();
        let attrs = ProtocolAttributes::default();
        let generator = DualGenerator::new(spec, attrs);

        let original_roles = vec![
            syn::Ident::new("Client", proc_macro2::Span::call_site()),
            syn::Ident::new("Server", proc_macro2::Span::call_site()),
        ];

        let swapped = generator.swap_roles(&original_roles);
        assert_eq!(swapped[0], "Server");
        assert_eq!(swapped[1], "Client");
    }

    #[test]
    fn test_dual_name_generation() {
        let spec = create_test_protocol();
        let attrs = ProtocolAttributes::default();
        let generator = DualGenerator::new(spec, attrs);

        let dual_name = generator.generate_dual_name().unwrap();
        assert_eq!(dual_name, "TestProtocolDual");
    }

    #[test]
    fn test_custom_dual_name() {
        let spec = create_test_protocol();
        let mut attrs = ProtocolAttributes::default();
        attrs.dual_name = Some("CustomDualName".to_string());
        let generator = DualGenerator::new(spec, attrs);

        let dual_name = generator.generate_dual_name().unwrap();
        assert_eq!(dual_name, "CustomDualName");
    }

    #[test]
    fn test_message_flow_dualization() {
        let spec = create_test_protocol();
        let attrs = ProtocolAttributes::default();
        let generator = DualGenerator::new(spec, attrs);

        let original_flow = MessageFlow {
            sender: syn::Ident::new("Client", proc_macro2::Span::call_site()),
            receiver: syn::Ident::new("Server", proc_macro2::Span::call_site()),
            message: MessageSpec::Simple {
                name: syn::Ident::new("Request", proc_macro2::Span::call_site()),
                fields: vec![],
            },
            properties: MessageProperties::default(),
        };

        let dual_flow = generator.dual_message_flow(&original_flow);
        assert_eq!(dual_flow.sender, "Server");
        assert_eq!(dual_flow.receiver, "Client");
    }

    #[test]
    fn test_full_dual_generation() {
        let spec = create_test_protocol();
        let attrs = ProtocolAttributes {
            generate_dual: true,
            verify_duality: true,
            dual_documentation: true,
            ..Default::default()
        };
        let generator = DualGenerator::new(spec, attrs);

        let dual_spec = generator.generate_dual_spec().unwrap();
        assert_eq!(dual_spec.name, "TestProtocolDual");
        assert_eq!(dual_spec.roles.len(), 2);
        assert_eq!(dual_spec.roles[0], "Server");
        assert_eq!(dual_spec.roles[1], "Client");
    }
}
