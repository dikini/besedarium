//! # GlobalProtocol Derive Macro Implementation
//!
//! This module implements the `#[derive(GlobalProtocol)]` procedural macro for
//! automatically implementing the `GlobalProtocol` trait from the Besedarium foundation.
//!
//! Additionally, this module implements attribute macros for protocol specification:
//! - `#[protocol]` - Transform protocol specifications into session types
//! - `#[endpoint]` - Enhanced endpoint behavior specification
//! - `#[session_type]` - Session type metadata and validation

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    DeriveInput, Ident, Result, Token, Type,
};

use crate::diagram_generation::ProtocolDiagramGenerator;
use crate::dual_generation::{generate_dual_protocol_code, DualGenerator};
use crate::utils::{basic_trait_impl, get_type_name, handle_result, is_enum, is_struct};

/// Protocol specification AST structures
#[derive(Clone)]
pub struct ProtocolSpec {
    pub name: Ident,
    pub attributes: ProtocolAttributes,
    pub roles: Vec<Ident>,
    // Replaced messages with flows to support advanced constructs
    pub flows: Vec<ProtocolFlow>,
}

/// Enhanced protocol flow representation supporting advanced DSL features
#[derive(Debug, Clone)]
pub enum ProtocolFlow {
    /// Simple message flow: Sender -> Receiver: Message;
    MessageFlow(MessageFlow),
    /// Choice construct: match { ... }
    Choice(ChoiceFlow),
    /// Loop construct: loop { ... }
    Loop(LoopFlow),
    /// Conditional construct: if condition { ... } else { ... }
    Conditional(ConditionalFlow),
    /// Parallel construct: par { ... }
    Parallel(ParallelFlow),
    /// Protocol termination
    End,
    /// Continue statement for loops
    Continue,
}

/// Choice/branching flow representation
#[derive(Debug, Clone)]
pub struct ChoiceFlow {
    pub sender: Ident,
    pub receiver: Ident,
    pub message: ChoiceMessage,
    pub branches: Vec<ChoiceBranch>,
}

/// Choice message with multiple variants
#[derive(Debug, Clone)]
pub struct ChoiceMessage {
    pub name: Ident,
    pub variants: Vec<ChoiceVariant>,
}

/// Individual choice variant
#[derive(Debug, Clone)]
pub struct ChoiceVariant {
    pub name: Ident,
    pub fields: Vec<MessageField>,
}

/// Branch handling for choice variants
#[derive(Debug, Clone)]
pub struct ChoiceBranch {
    pub variant: Ident,
    pub bound_fields: Vec<Ident>,
    pub continuation: Vec<ProtocolFlow>,
}

/// Loop flow representation
#[derive(Debug, Clone)]
pub struct LoopFlow {
    pub body: Vec<ProtocolFlow>,
}

/// Conditional flow representation
#[derive(Debug, Clone)]
pub struct ConditionalFlow {
    pub condition: Ident, // For now, just reference a boolean field
    pub if_branch: Vec<ProtocolFlow>,
    pub else_branch: Option<Vec<ProtocolFlow>>,
}

/// Parallel flow representation
#[derive(Debug, Clone)]
pub struct ParallelFlow {
    pub branches: Vec<Vec<ProtocolFlow>>,
}

#[derive(Debug, Default, Clone)]
pub struct ProtocolAttributes {
    pub io_type: Option<String>,
    pub metadata_type: Option<String>,
    pub buffer_size: Option<usize>,
    pub timeout_ms: Option<u64>,
    pub serialization: Option<String>,
    pub validation: Option<bool>,
    pub concurrent: Option<bool>,
    pub reliability: Option<String>,
    // Dual Protocol Generation Attributes
    pub generate_dual: bool,
    pub dual_name: Option<String>,
    pub verify_duality: bool,
    pub dual_documentation: bool,
}

#[derive(Debug, Clone)]
pub struct MessageFlow {
    pub sender: Ident,
    pub receiver: Ident,
    pub message: MessageSpec,
    pub properties: MessageProperties,
}

#[derive(Debug, Clone)]
pub enum MessageSpec {
    Simple {
        name: Ident,
        fields: Vec<MessageField>,
    },
    Choice(ChoiceMessage),
}

impl MessageSpec {
    pub fn name(&self) -> &Ident {
        match self {
            MessageSpec::Simple { name, .. } => name,
            MessageSpec::Choice(choice_msg) => &choice_msg.name,
        }
    }

    pub fn fields(&self) -> Vec<MessageField> {
        match self {
            MessageSpec::Simple { fields, .. } => fields.clone(),
            MessageSpec::Choice(_) => {
                // Choice messages don't have simple fields like regular messages
                // Return an empty vector for compatibility
                Vec::new()
            }
        }
    }
}

#[derive(Clone)]
pub struct MessageField {
    pub name: Ident,
    pub field_type: Type,
}

impl std::fmt::Debug for MessageField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageField")
            .field("name", &self.name)
            .field("field_type", &format!("{}", quote::quote!(self.field_type)))
            .finish()
    }
}

#[derive(Debug, Default, Clone)]
pub struct MessageProperties {
    pub timeout: Option<String>,
    pub priority: Option<u8>,
    pub retry_count: Option<u32>,
    pub reliable: Option<bool>,
    pub ordered: Option<bool>,
    pub duplicate_detection: Option<bool>,
}

// Protocol specification parsing implementations
impl Parse for ProtocolSpec {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse: protocol ProtocolName { ... }
        let protocol_kw = input.parse::<Ident>()?;
        if protocol_kw != "protocol" {
            return Err(syn::Error::new_spanned(protocol_kw, "Expected 'protocol'"));
        }

        let name = input.parse::<Ident>()?;

        let content;
        syn::braced!(content in input);

        // Parse roles declaration
        let roles_kw = content.parse::<Ident>()?;
        if roles_kw != "roles" {
            return Err(syn::Error::new_spanned(roles_kw, "Expected 'roles'"));
        }
        content.parse::<Token![:]>()?;

        let roles: Punctuated<Ident, Token![,]> = Punctuated::parse_terminated(&content)?;
        content.parse::<Token![;]>()?;

        // Parse message flows
        let mut flows = Vec::new();
        while !content.is_empty() {
            flows.push(content.parse::<ProtocolFlow>()?);
        }

        Ok(ProtocolSpec {
            name,
            attributes: ProtocolAttributes::default(),
            roles: roles.into_iter().collect(),
            flows,
        })
    }
}

impl Parse for ProtocolFlow {
    fn parse(input: ParseStream) -> Result<Self> {
        // Check for different protocol flow patterns
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident) {
            // Look ahead to determine the pattern
            let fork = input.fork();
            let first_ident = fork.parse::<Ident>()?;

            if fork.peek(Token![->]) {
                // Message flow: Sender -> Receiver: MessageName(field: Type);
                let sender = input.parse::<Ident>()?;
                input.parse::<Token![->]>()?;
                let receiver = input.parse::<Ident>()?;
                input.parse::<Token![:]>()?;

                let message = input.parse::<MessageSpec>()?;
                input.parse::<Token![;]>()?;

                Ok(ProtocolFlow::MessageFlow(MessageFlow {
                    sender,
                    receiver,
                    message,
                    properties: MessageProperties::default(),
                }))
            } else if first_ident == "match" {
                // Choice flow: match { variant1(bound_fields) => { flows... }, variant2(bound_fields) => { flows... } }
                let _match_kw = input.parse::<Ident>()?; // "match"

                let content;
                syn::braced!(content in input);

                // Parse choice branches: variant(fields) => { flows... },
                let mut branches = Vec::new();
                while !content.is_empty() {
                    branches.push(content.parse::<ChoiceBranch>()?);

                    // Handle optional trailing comma
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }

                // Create placeholder sender/receiver and message for the choice flow
                // These will be determined from context when generating session types
                Ok(ProtocolFlow::Choice(ChoiceFlow {
                    sender: syn::Ident::new("ChoiceSender", proc_macro2::Span::call_site()),
                    receiver: syn::Ident::new("ChoiceReceiver", proc_macro2::Span::call_site()),
                    message: ChoiceMessage {
                        name: syn::Ident::new("Choice", proc_macro2::Span::call_site()),
                        variants: Vec::new(), // Will be populated from the preceding message
                    },
                    branches,
                }))
            } else if first_ident == "loop" {
                // Loop flow: loop { ... }
                let _loop_kw = input.parse::<Ident>()?; // "loop"

                let content;
                syn::braced!(content in input);

                let mut body_flows = Vec::new();
                while !content.is_empty() {
                    body_flows.push(content.parse::<ProtocolFlow>()?);
                }

                Ok(ProtocolFlow::Loop(LoopFlow { body: body_flows }))
            } else if first_ident == "if" {
                // Conditional flow: if condition { ... } else { ... }
                let _if_kw = input.parse::<Ident>()?; // "if"
                let condition = input.parse::<Ident>()?;

                let if_content;
                syn::braced!(if_content in input);

                let mut if_flows = Vec::new();
                while !if_content.is_empty() {
                    if_flows.push(if_content.parse::<ProtocolFlow>()?);
                }

                let else_branch = if input.peek(Ident) {
                    let fork = input.fork();
                    if let Ok(else_kw) = fork.parse::<Ident>() {
                        if else_kw == "else" {
                            let _else_kw = input.parse::<Ident>()?; // "else"
                            let else_content;
                            syn::braced!(else_content in input);

                            let mut else_flows = Vec::new();
                            while !else_content.is_empty() {
                                else_flows.push(else_content.parse::<ProtocolFlow>()?);
                            }

                            Some(else_flows)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                Ok(ProtocolFlow::Conditional(ConditionalFlow {
                    condition,
                    if_branch: if_flows,
                    else_branch,
                }))
            } else if first_ident == "par" {
                // Parallel flow: par { flow1; flow2; flow3; }
                let _par_kw = input.parse::<Ident>()?; // "par"

                let content;
                syn::braced!(content in input);

                // Parse flows directly within the par block
                let mut flows = Vec::new();
                while !content.is_empty() {
                    flows.push(content.parse::<ProtocolFlow>()?);
                }

                // For now, treat all flows as a single parallel branch
                // This may need refinement based on the actual semantics
                Ok(ProtocolFlow::Parallel(ParallelFlow {
                    branches: vec![flows],
                }))
            } else if first_ident == "end" {
                let _end_kw = input.parse::<Ident>()?; // "end"
                input.parse::<Token![;]>()?;
                Ok(ProtocolFlow::End)
            } else if first_ident == "continue" {
                let _continue_kw = input.parse::<Ident>()?; // "continue"
                input.parse::<Token![;]>()?;
                Ok(ProtocolFlow::Continue)
            } else {
                Err(lookahead.error())
            }
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for MessageSpec {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;

        if input.peek(syn::token::Paren) {
            // Regular message: MessageName(field1: Type1, field2: Type2)
            let content;
            syn::parenthesized!(content in input);

            let field_list: Punctuated<MessageField, Token![,]> =
                Punctuated::parse_terminated(&content)?;
            let fields = field_list.into_iter().collect();

            Ok(MessageSpec::Simple { name, fields })
        } else if input.peek(syn::token::Brace) {
            // Choice message: MessageName { Variant1(fields), Variant2(fields) }
            let content;
            syn::braced!(content in input);

            let mut variants = Vec::new();
            while !content.is_empty() {
                variants.push(content.parse::<ChoiceVariant>()?);
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }

            Ok(MessageSpec::Choice(ChoiceMessage { name, variants }))
        } else {
            // Simple message without fields or braces
            Ok(MessageSpec::Simple {
                name,
                fields: Vec::new(),
            })
        }
    }
}

impl Parse for ChoiceMessage {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;

        let mut variants = Vec::new();
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);

            while !content.is_empty() {
                variants.push(content.parse::<ChoiceVariant>()?);
                if !content.is_empty() {
                    content.parse::<Token![,]>()?;
                }
            }
        }

        Ok(ChoiceMessage { name, variants })
    }
}

impl Parse for ChoiceVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;

        let mut fields = Vec::new();
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);

            let field_list: Punctuated<MessageField, Token![,]> =
                Punctuated::parse_terminated(&content)?;
            fields = field_list.into_iter().collect();
        }

        Ok(ChoiceVariant { name, fields })
    }
}

impl Parse for ChoiceBranch {
    fn parse(input: ParseStream) -> Result<Self> {
        let variant = input.parse::<Ident>()?;

        // Parse optional bound fields: variant(field1, field2) => { ... }
        let mut bound_fields = Vec::new();
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);

            let bound_list: Punctuated<Ident, Token![,]> = Punctuated::parse_terminated(&content)?;
            bound_fields = bound_list.into_iter().collect();
        }

        // Parse => arrow
        input.parse::<Token![=>]>()?;

        // Parse continuation block
        let content;
        syn::braced!(content in input);

        let mut continuation = Vec::new();
        while !content.is_empty() {
            continuation.push(content.parse::<ProtocolFlow>()?);
        }

        Ok(ChoiceBranch {
            variant,
            bound_fields,
            continuation,
        })
    }
}

impl Parse for MessageField {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let field_type = input.parse::<Type>()?;

        Ok(MessageField { name, field_type })
    }
}

/// Implementation of the `#[derive(GlobalProtocol)]` macro
pub fn derive_global_protocol_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    handle_result(derive_global_protocol_inner(&input))
}

/// Internal implementation logic for the GlobalProtocol derive
fn derive_global_protocol_inner(input: &DeriveInput) -> Result<TokenStream2> {
    let type_name = get_type_name(input);

    // GlobalProtocol trait can be derived for structs and enums
    if !is_struct(input) && !is_enum(input) {
        return Err(syn::Error::new_spanned(
            input,
            "#[derive(GlobalProtocol)] can only be derived for structs and enums",
        ));
    }

    // Check for protocol-specific attributes
    let _protocol_attrs = crate::utils::parse_derive_attributes(input, "protocol");

    // Generate the GlobalProtocol trait implementation
    let trait_impl = generate_global_protocol_impl(type_name);

    Ok(trait_impl)
}

/// Generate the actual GlobalProtocol trait implementation
fn generate_global_protocol_impl(type_name: &syn::Ident) -> TokenStream2 {
    // GlobalProtocol is a marker trait with no required methods
    // It requires: Send + Sync + 'static + Debug
    let trait_path = quote! { ::besedarium::protocol::foundation::GlobalProtocol };

    basic_trait_impl(type_name, trait_path, None)
}

/// Implementation of the `#[protocol]` attribute macro
pub fn protocol_attribute_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse protocol attributes from args
    let protocol_attrs = match parse_protocol_args(args) {
        Ok(attrs) => attrs,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };

    let input = parse_macro_input!(input as syn::ItemStruct);
    let struct_name = &input.ident;

    // Try to parse protocol specification from doc comments
    let protocol_spec = match parse_protocol_from_struct(&input) {
        Ok(spec) => spec,
        Err(e) => {
            // If parsing fails, generate a basic protocol with just the struct
            let _error_msg = format!("Failed to parse protocol specification: {}", e);
            let expanded = quote! {
                // Original struct
                #input

                // Add basic GlobalProtocol implementation with error comment
                impl ::besedarium::protocol::foundation::GlobalProtocol for #struct_name {
                }

                // TODO: Parse protocol specification from doc comments
            };
            return TokenStream::from(expanded);
        }
    };

    // Generate the protocol implementation based on parsed specification
    match generate_protocol_implementation(protocol_spec, protocol_attrs) {
        Ok(tokens) => TokenStream::from(tokens),
        Err(e) => {
            let _error_msg = format!("Failed to generate protocol implementation: {}", e);
            let expanded = quote! {
                // Original struct
                #input

                // Add basic GlobalProtocol implementation with error comment
                impl ::besedarium::protocol::foundation::GlobalProtocol for #struct_name {
                }

                // TODO: Generate protocol implementation from parsed specification
            };
            TokenStream::from(expanded)
        }
    }
}

/// Parse protocol specification from doc comments in the struct
fn parse_protocol_from_struct(input: &syn::ItemStruct) -> Result<ProtocolSpec> {
    // Try to parse from doc comments first
    if let Ok(spec) = parse_protocol_from_doc_comments(input) {
        return Ok(spec);
    }

    // Try to parse from struct attributes
    if let Ok(spec) = parse_protocol_from_attributes(input) {
        return Ok(spec);
    }

    // Fallback: Create a basic protocol specification with the struct name
    let basic_spec = ProtocolSpec {
        name: input.ident.clone(),
        attributes: ProtocolAttributes::default(),
        roles: vec![
            syn::Ident::new("Client", proc_macro2::Span::call_site()),
            syn::Ident::new("Server", proc_macro2::Span::call_site()),
        ],
        flows: vec![ProtocolFlow::MessageFlow(MessageFlow {
            sender: syn::Ident::new("Client", proc_macro2::Span::call_site()),
            receiver: syn::Ident::new("Server", proc_macro2::Span::call_site()),
            message: MessageSpec::Simple {
                name: syn::Ident::new("Hello", proc_macro2::Span::call_site()),
                fields: vec![],
            },
            properties: MessageProperties::default(),
        })],
    };

    Ok(basic_spec)
}

/// Parse protocol specification from doc comments
fn parse_protocol_from_doc_comments(input: &syn::ItemStruct) -> Result<ProtocolSpec> {
    let mut doc_content = String::new();

    // Collect all doc comments
    for attr in &input.attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) = &meta.value
                {
                    let line = lit_str.value();
                    let trimmed = line.trim();

                    // Skip empty lines and standard doc formatting
                    if !trimmed.is_empty() && !trimmed.starts_with('#') {
                        doc_content.push_str(trimmed);
                        doc_content.push('\n');
                    }
                }
            }
        }
    }

    if doc_content.trim().is_empty() {
        return Err(syn::Error::new_spanned(
            input,
            "No protocol specification found in doc comments",
        ));
    }

    // Try to parse protocol specification from collected doc content
    parse_protocol_spec_from_text(&doc_content, &input.ident)
}

/// Parse protocol specification from struct attributes
fn parse_protocol_from_attributes(input: &syn::ItemStruct) -> Result<ProtocolSpec> {
    // Look for protocol specification in attributes like #[protocol_spec("...")]
    for attr in &input.attrs {
        if attr.path().is_ident("protocol_spec") {
            if let syn::Meta::List(meta_list) = &attr.meta {
                // Parse the content within the attribute
                if let Ok(lit) = syn::parse2::<syn::LitStr>(meta_list.tokens.clone()) {
                    return parse_protocol_spec_from_text(&lit.value(), &input.ident);
                }
            }
        }
    }

    Err(syn::Error::new_spanned(
        input,
        "No protocol specification found in struct attributes",
    ))
}

/// Parse protocol specification from text content
fn parse_protocol_spec_from_text(content: &str, struct_name: &syn::Ident) -> Result<ProtocolSpec> {
    // Look for protocol specification pattern
    let content = content.trim();

    // Simple pattern matching for basic protocol syntax
    if content.contains("roles:") && content.contains("->") {
        // Try to parse using a simple state machine approach
        parse_simple_protocol_syntax(content, struct_name)
    } else {
        Err(syn::Error::new_spanned(
            struct_name,
            format!("Invalid protocol specification format: {}", content),
        ))
    }
}

/// Parse simple protocol syntax from text
fn parse_simple_protocol_syntax(content: &str, struct_name: &syn::Ident) -> Result<ProtocolSpec> {
    let mut spec = ProtocolSpec {
        name: struct_name.clone(),
        attributes: ProtocolAttributes::default(),
        roles: Vec::new(),
        flows: Vec::new(),
    };

    let lines: Vec<&str> = content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];

        if line.starts_with("roles:") {
            // Parse roles: Client, Server
            let roles_part = line.strip_prefix("roles:").unwrap().trim();
            let roles_part = roles_part.strip_suffix(';').unwrap_or(roles_part);

            for role_name in roles_part.split(',') {
                let role_name = role_name.trim();
                if !role_name.is_empty() {
                    spec.roles
                        .push(syn::Ident::new(role_name, proc_macro2::Span::call_site()));
                }
            }
            i += 1;
        } else if line.contains("->") && line.contains(':') {
            // Parse message flow - might be multi-line for choice messages
            let mut complete_flow = String::new();
            let mut brace_count = 0;
            let mut j = i;

            // Collect all lines until we have a complete message flow
            while j < lines.len() {
                let current_line = lines[j];
                complete_flow.push_str(current_line);
                complete_flow.push(' ');

                // Count braces to detect multi-line choice constructs
                brace_count += current_line.chars().filter(|&c| c == '{').count() as i32;
                brace_count -= current_line.chars().filter(|&c| c == '}').count() as i32;

                // If we have balanced braces and a semicolon, the flow is complete
                if brace_count == 0 && current_line.contains(';') {
                    break;
                }
                j += 1;
            }

            if let Ok(message_flow) = parse_message_flow_from_text(&complete_flow.trim()) {
                // Check if this is a choice message and if the next non-empty line is a match statement
                let is_choice_message = matches!(message_flow.message, MessageSpec::Choice(_));

                if is_choice_message {
                    // Look for a following match statement
                    let mut next_idx = j + 1;
                    while next_idx < lines.len() && lines[next_idx].trim().is_empty() {
                        next_idx += 1;
                    }

                    if next_idx < lines.len() && lines[next_idx].starts_with("match") {
                        // Parse the choice flow combining the message and match
                        let (choice_flow, consumed_lines) = parse_choice_flow_with_match(
                            &message_flow,
                            &lines[next_idx..],
                            next_idx,
                        )?;
                        spec.flows.push(ProtocolFlow::Choice(choice_flow));
                        i = next_idx + consumed_lines;
                    } else {
                        // Just a choice message without match
                        spec.flows.push(ProtocolFlow::MessageFlow(message_flow));
                        i = j + 1;
                    }
                } else {
                    // Regular message flow
                    spec.flows.push(ProtocolFlow::MessageFlow(message_flow));
                    i = j + 1;
                }
            } else {
                i = j + 1;
            }
        } else {
            i += 1;
        }
    }

    // Validation
    if spec.roles.is_empty() {
        return Err(syn::Error::new_spanned(
            struct_name,
            "Protocol specification must define at least one role",
        ));
    }

    if spec.flows.is_empty() {
        return Err(syn::Error::new_spanned(
            struct_name,
            "Protocol specification must define at least one message flow",
        ));
    }

    Ok(spec)
}

/// Parse a choice flow by combining a choice message with its match statement
fn parse_choice_flow_with_match(
    message_flow: &MessageFlow,
    match_lines: &[&str],
    _start_line_idx: usize,
) -> Result<(ChoiceFlow, usize)> {
    // Extract the choice message
    let choice_message = match &message_flow.message {
        MessageSpec::Choice(choice_msg) => choice_msg.clone(),
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Expected choice message for choice flow parsing",
            ))
        }
    };

    // Parse the match statement
    let mut branches = Vec::new();
    let mut consumed_lines = 0;
    let mut brace_depth = 0;
    let mut in_match_body = false;
    let mut current_branch: Option<ChoiceBranch> = None;
    let mut current_branch_flows = Vec::new();

    for (line_idx, line) in match_lines.iter().enumerate() {
        consumed_lines = line_idx + 1;
        let line = line.trim();

        if line.starts_with("match") {
            in_match_body = true;
            continue;
        }

        if !in_match_body {
            continue;
        }

        // Count braces to track nesting
        brace_depth += line.chars().filter(|&c| c == '{').count() as i32;
        brace_depth -= line.chars().filter(|&c| c == '}').count() as i32;

        // If we're back to brace_depth 0, we've finished the match statement
        if brace_depth == 0 && line.ends_with('}') {
            // Save any pending branch
            if let Some(mut branch) = current_branch.take() {
                branch.continuation = current_branch_flows.clone();
                branches.push(branch);
                current_branch_flows.clear();
            }
            break;
        }

        // Parse branch patterns: GetData(id) => { ... }
        if line.contains("=>") && brace_depth == 1 {
            // Save previous branch if any
            if let Some(mut branch) = current_branch.take() {
                branch.continuation = current_branch_flows.clone();
                branches.push(branch);
                current_branch_flows.clear();
            }

            // Parse new branch
            let parts: Vec<&str> = line.split("=>").collect();
            if parts.len() == 2 {
                let pattern_part = parts[0].trim();
                let branch = parse_choice_branch_pattern(pattern_part)?;
                current_branch = Some(branch);
            }
        } else if brace_depth > 1 {
            // Inside a branch body - parse flow statements
            if line.contains("->") && line.contains(':') {
                if let Ok(flow) = parse_message_flow_from_text(line) {
                    current_branch_flows.push(ProtocolFlow::MessageFlow(flow));
                }
            } else if line == "end" {
                current_branch_flows.push(ProtocolFlow::End);
            } else if line == "continue" {
                current_branch_flows.push(ProtocolFlow::Continue);
            }
        }
    }

    let choice_flow = ChoiceFlow {
        sender: message_flow.sender.clone(),
        receiver: message_flow.receiver.clone(),
        message: choice_message,
        branches,
    };

    Ok((choice_flow, consumed_lines))
}

/// Parse a choice branch pattern like "GetData(id)" or "Quit"
fn parse_choice_branch_pattern(pattern: &str) -> Result<ChoiceBranch> {
    if let Some(paren_start) = pattern.find('(') {
        // Pattern with bound fields: GetData(id)
        let variant_name = pattern[..paren_start].trim();
        let fields_part = &pattern[paren_start + 1..];

        if let Some(paren_end) = fields_part.rfind(')') {
            let bound_fields_content = &fields_part[..paren_end];
            let bound_fields = if bound_fields_content.trim().is_empty() {
                Vec::new()
            } else {
                bound_fields_content
                    .split(',')
                    .map(|s| syn::Ident::new(s.trim(), proc_macro2::Span::call_site()))
                    .collect()
            };

            Ok(ChoiceBranch {
                variant: syn::Ident::new(variant_name, proc_macro2::Span::call_site()),
                bound_fields,
                continuation: Vec::new(), // Will be filled later
            })
        } else {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Unclosed parentheses in choice branch pattern: {}", pattern),
            ))
        }
    } else {
        // Simple pattern without fields: Quit
        Ok(ChoiceBranch {
            variant: syn::Ident::new(pattern.trim(), proc_macro2::Span::call_site()),
            bound_fields: Vec::new(),
            continuation: Vec::new(), // Will be filled later
        })
    }
}

/// Parse a single message flow from text like "Client -> Server: Login(username: String);"
fn parse_message_flow_from_text(text: &str) -> Result<MessageFlow> {
    let text = text.trim().trim_end_matches(';');

    // Split on "->" to get sender and receiver parts
    let parts: Vec<&str> = text.split("->").collect();
    if parts.len() != 2 {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Invalid message flow format: {}", text),
        ));
    }

    let sender = syn::Ident::new(parts[0].trim(), proc_macro2::Span::call_site());
    let receiver_and_message = parts[1].trim();

    // Split on ":" to get receiver and message
    let msg_parts: Vec<&str> = receiver_and_message.split(':').collect();
    if msg_parts.len() != 2 {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Invalid message format: {}", receiver_and_message),
        ));
    }

    let receiver = syn::Ident::new(msg_parts[0].trim(), proc_macro2::Span::call_site());
    let message_text = msg_parts[1].trim();

    let message = parse_message_spec_from_text(message_text)?;

    Ok(MessageFlow {
        sender,
        receiver,
        message,
        properties: MessageProperties::default(),
    })
}

/// Parse message specification from text
fn parse_message_spec_from_text(text: &str) -> Result<MessageSpec> {
    if let Some(brace_start) = text.find('{') {
        if let Some(brace_end) = text.rfind('}') {
            // Choice message: Request { GetData(id: u32), PostData(data: String), Quit }
            let message_name = text[..brace_start].trim();
            let variants_content = &text[brace_start + 1..brace_end];

            let name = syn::Ident::new(message_name, proc_macro2::Span::call_site());
            let variants = parse_choice_variants(variants_content)?;

            Ok(MessageSpec::Choice(ChoiceMessage { name, variants }))
        } else {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Unclosed brace in choice message: {}", text),
            ))
        }
    } else {
        // Simple message: Login(username: String) or just Login
        let (name, fields) = if let Some(paren_start) = text.find('(') {
            if let Some(paren_end) = text.rfind(')') {
                let message_name = text[..paren_start].trim();
                let fields_content = &text[paren_start + 1..paren_end];

                let name = syn::Ident::new(message_name, proc_macro2::Span::call_site());
                let fields = parse_message_fields(fields_content)?;
                (name, fields)
            } else {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Unclosed parenthesis in message: {}", text),
                ));
            }
        } else {
            // Message without fields
            let name = syn::Ident::new(text.trim(), proc_macro2::Span::call_site());
            (name, Vec::new())
        };

        Ok(MessageSpec::Simple { name, fields })
    }
}

/// Parse message fields from text like "username: String, id: u32"
fn parse_message_fields(text: &str) -> Result<Vec<MessageField>> {
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }

    let mut fields = Vec::new();
    for field_text in text.split(',') {
        let field_text = field_text.trim();
        if field_text.is_empty() {
            continue;
        }

        let parts: Vec<&str> = field_text.split(':').collect();
        if parts.len() != 2 {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Invalid field format: {}", field_text),
            ));
        }

        let name = syn::Ident::new(parts[0].trim(), proc_macro2::Span::call_site());
        let type_str = parts[1].trim();
        let field_type: Type = syn::parse_str(type_str)?;

        fields.push(MessageField { name, field_type });
    }

    Ok(fields)
}

/// Parse choice variants from text like "GetData(id: u32), PostData(data: String), Quit"
fn parse_choice_variants(text: &str) -> Result<Vec<ChoiceVariant>> {
    let mut variants = Vec::new();

    for variant_text in text.split(',') {
        let variant_text = variant_text.trim();
        if variant_text.is_empty() {
            continue;
        }

        let variant = parse_choice_variant(variant_text)?;
        variants.push(variant);
    }

    Ok(variants)
}

/// Parse a single choice variant from text like "GetData(id: u32)" or "Quit"
fn parse_choice_variant(text: &str) -> Result<ChoiceVariant> {
    if let Some(paren_start) = text.find('(') {
        if let Some(paren_end) = text.rfind(')') {
            let variant_name = text[..paren_start].trim();
            let fields_content = &text[paren_start + 1..paren_end];

            let name = syn::Ident::new(variant_name, proc_macro2::Span::call_site());
            let fields = parse_message_fields(fields_content)?;

            Ok(ChoiceVariant { name, fields })
        } else {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Unclosed parenthesis in choice variant: {}", text),
            ))
        }
    } else {
        // Variant without fields
        let name = syn::Ident::new(text.trim(), proc_macro2::Span::call_site());
        Ok(ChoiceVariant {
            name,
            fields: Vec::new(),
        })
    }
}

/// Generate the protocol implementation code
/// Generate the complete protocol implementation with optional dual generation
pub(crate) fn generate_protocol_implementation(
    protocol_spec: ProtocolSpec,
    protocol_attrs: ProtocolAttributes,
) -> Result<TokenStream2> {
    let struct_name = &protocol_spec.name;

    // Generate basic protocol implementation
    let basic_impl = quote! {
        impl GlobalProtocol for #struct_name {
            type Roles = ();
            type Messages = ();

            fn protocol_name() -> &'static str {
                stringify!(#struct_name)
            }
        }
    };

    // If dual generation is enabled, generate both original and dual protocols
    if protocol_attrs.generate_dual {
        // Generate dual protocol using DualGenerator
        let dual_generator = DualGenerator::new(protocol_spec.clone(), protocol_attrs.clone());

        match dual_generator.generate_dual_spec() {
            Ok(dual_spec) => {
                // Generate combined original and dual protocol code
                match generate_dual_protocol_code(&protocol_spec, &dual_spec, &protocol_attrs) {
                    Ok(dual_code) => Ok(dual_code),
                    Err(e) => {
                        // Fallback to basic implementation if dual generation fails
                        let _error_msg = format!("Dual generation failed: {}", e);
                        Ok(basic_impl)
                    }
                }
            }
            Err(e) => {
                // Fallback to basic implementation if dual spec generation fails
                let _error_msg = format!("Dual spec generation failed: {}", e);
                Ok(basic_impl)
            }
        }
    } else {
        // Generate only the original protocol
        Ok(basic_impl)
    }
}

/// Implementation for the endpoint attribute macro
pub fn endpoint_attribute_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    // For now, just return the input unchanged with a comment
    let input_tokens: proc_macro2::TokenStream = input.into();
    let _args_tokens: proc_macro2::TokenStream = args.into();

    let output = quote! {
        // Endpoint attribute applied with args: #_args_tokens
        #input_tokens
    };

    output.into()
}

/// Implementation for the session_type attribute macro
pub fn session_type_attribute_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    // For now, just return the input unchanged with a comment
    let input_tokens: proc_macro2::TokenStream = input.into();
    let _args_tokens: proc_macro2::TokenStream = args.into();

    let output = quote! {
        // Session type attribute applied with args: #_args_tokens
        #input_tokens
    };

    output.into()
}

/// Parse protocol attribute arguments with duplicate detection
///
/// Parses attribute arguments like: io = "async", metadata = "standard", buffer_size = 2048
/// Returns an error if any attribute appears more than once.
#[allow(dead_code)] // Will be used when integrating with main protocol macro
pub fn parse_protocol_args(args: TokenStream) -> Result<ProtocolAttributes> {
    if args.is_empty() {
        return Ok(ProtocolAttributes::default());
    }

    let meta: syn::Meta = syn::parse(args)?;
    let mut attrs = ProtocolAttributes::default();

    match meta {
        syn::Meta::List(list) => {
            // Parse comma-separated list like: io = "async", buffer_size = 1024
            let parser = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated;
            let nested_metas = parser.parse2(list.tokens)?;

            for nested_meta in nested_metas {
                if let syn::Meta::NameValue(name_value) = nested_meta {
                    parse_single_attribute(&mut attrs, &name_value)?;
                } else {
                    return Err(syn::Error::new_spanned(
                        nested_meta,
                        "Expected attribute in the form: attribute = value",
                    ));
                }
            }
        }
        syn::Meta::NameValue(name_value) => {
            // Single attribute like: io = "async"
            parse_single_attribute(&mut attrs, &name_value)?;
        }
        _ => {
            return Err(syn::Error::new_spanned(
                meta,
                "Expected protocol attributes in the form: attribute = value",
            ));
        }
    }

    Ok(attrs)
}

/// Parse protocol attribute arguments with duplicate detection (test version)
///
/// This version accepts proc_macro2::TokenStream for use in unit tests.
/// Parses attribute arguments like: io = "async", metadata = "standard", buffer_size = 2048
/// Returns an error if any attribute appears more than once.
#[cfg(test)]
pub fn parse_protocol_args_test(args: proc_macro2::TokenStream) -> Result<ProtocolAttributes> {
    if args.is_empty() {
        return Ok(ProtocolAttributes::default());
    }

    let mut attrs = ProtocolAttributes::default();

    // Try to parse as a single MetaNameValue first
    if let Ok(name_value) = syn::parse2::<syn::MetaNameValue>(args.clone()) {
        parse_single_attribute(&mut attrs, &name_value)?;
        return Ok(attrs);
    }

    // Try to parse as comma-separated list of MetaNameValue items
    let parser =
        syn::punctuated::Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated;
    if let Ok(name_values) = parser.parse2(args.clone()) {
        for name_value in name_values {
            parse_single_attribute(&mut attrs, &name_value)?;
        }
        return Ok(attrs);
    }

    // If neither works, try parsing as Meta for backwards compatibility
    let meta: syn::Meta = syn::parse2(args)?;
    match meta {
        syn::Meta::List(list) => {
            // Parse comma-separated list like: io = "async", buffer_size = 1024
            let parser = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated;
            let nested_metas = parser.parse2(list.tokens)?;

            for nested_meta in nested_metas {
                if let syn::Meta::NameValue(name_value) = nested_meta {
                    parse_single_attribute(&mut attrs, &name_value)?;
                } else {
                    return Err(syn::Error::new_spanned(
                        nested_meta,
                        "Expected attribute in the form: attribute = value",
                    ));
                }
            }
        }
        syn::Meta::NameValue(name_value) => {
            // Single attribute like: io = "async"
            parse_single_attribute(&mut attrs, &name_value)?;
        }
        _ => {
            return Err(syn::Error::new_spanned(
                meta,
                "Expected protocol attributes in the form: attribute = value",
            ));
        }
    }

    Ok(attrs)
}

/// Helper function to parse a single protocol attribute and check for duplicates
#[allow(dead_code)] // Will be used when integrating with main protocol macro
fn parse_single_attribute(
    attrs: &mut ProtocolAttributes,
    name_value: &syn::MetaNameValue,
) -> syn::Result<()> {
    let attr_name = name_value
        .path
        .get_ident()
        .ok_or_else(|| syn::Error::new_spanned(&name_value.path, "Invalid attribute name"))?
        .to_string();

    match attr_name.as_str() {
        "io" => {
            if attrs.io_type.is_some() {
                return Err(syn::Error::new_spanned(
                    name_value,
                    "Duplicate attribute 'io': this attribute can only be specified once",
                ));
            }
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) = &name_value.value
            {
                attrs.io_type = Some(lit_str.value());
            } else {
                return Err(syn::Error::new_spanned(
                    &name_value.value,
                    "Expected string literal for 'io' attribute",
                ));
            }
        }
        "metadata" => {
            if attrs.metadata_type.is_some() {
                return Err(syn::Error::new_spanned(
                    name_value,
                    "Duplicate attribute 'metadata': this attribute can only be specified once",
                ));
            }
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) = &name_value.value
            {
                attrs.metadata_type = Some(lit_str.value());
            } else {
                return Err(syn::Error::new_spanned(
                    &name_value.value,
                    "Expected string literal for 'metadata' attribute",
                ));
            }
        }
        "buffer_size" => {
            if attrs.buffer_size.is_some() {
                return Err(syn::Error::new_spanned(
                    name_value,
                    "Duplicate attribute 'buffer_size': this attribute can only be specified once",
                ));
            }
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(lit_int),
                ..
            }) = &name_value.value
            {
                match lit_int.base10_parse::<usize>() {
                    Ok(val) => attrs.buffer_size = Some(val),
                    Err(_) => {
                        return Err(syn::Error::new_spanned(
                            lit_int,
                            "Invalid integer value for 'buffer_size' attribute",
                        ));
                    }
                }
            } else {
                return Err(syn::Error::new_spanned(
                    &name_value.value,
                    "Expected integer literal for 'buffer_size' attribute",
                ));
            }
        }
        "timeout_ms" => {
            if attrs.timeout_ms.is_some() {
                return Err(syn::Error::new_spanned(
                    name_value,
                    "Duplicate attribute 'timeout_ms': this attribute can only be specified once",
                ));
            }
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(lit_int),
                ..
            }) = &name_value.value
            {
                match lit_int.base10_parse::<u64>() {
                    Ok(val) => attrs.timeout_ms = Some(val),
                    Err(_) => {
                        return Err(syn::Error::new_spanned(
                            lit_int,
                            "Invalid integer value for 'timeout_ms' attribute",
                        ));
                    }
                }
            } else {
                return Err(syn::Error::new_spanned(
                    &name_value.value,
                    "Expected integer literal for 'timeout_ms' attribute",
                ));
            }
        }
        "serialization" => {
            if attrs.serialization.is_some() {
                return Err(syn::Error::new_spanned(
                    name_value,
                    "Duplicate attribute 'serialization': this attribute can only be specified once",
                ));
            }
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) = &name_value.value
            {
                attrs.serialization = Some(lit_str.value());
            } else {
                return Err(syn::Error::new_spanned(
                    &name_value.value,
                    "Expected string literal for 'serialization' attribute",
                ));
            }
        }
        "validation" => {
            if attrs.validation.is_some() {
                return Err(syn::Error::new_spanned(
                    name_value,
                    "Duplicate attribute 'validation': this attribute can only be specified once",
                ));
            }
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Bool(lit_bool),
                ..
            }) = &name_value.value
            {
                attrs.validation = Some(lit_bool.value());
            } else {
                return Err(syn::Error::new_spanned(
                    &name_value.value,
                    "Expected boolean literal for 'validation' attribute",
                ));
            }
        }
        "concurrent" => {
            if attrs.concurrent.is_some() {
                return Err(syn::Error::new_spanned(
                    name_value,
                    "Duplicate attribute 'concurrent': this attribute can only be specified once",
                ));
            }
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Bool(lit_bool),
                ..
            }) = &name_value.value
            {
                attrs.concurrent = Some(lit_bool.value());
            } else {
                return Err(syn::Error::new_spanned(
                    &name_value.value,
                    "Expected boolean literal for 'concurrent' attribute",
                ));
            }
        }
        "reliability" => {
            if attrs.reliability.is_some() {
                return Err(syn::Error::new_spanned(
                    name_value,
                    "Duplicate attribute 'reliability': this attribute can only be specified once",
                ));
            }
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) = &name_value.value
            {
                attrs.reliability = Some(lit_str.value());
            } else {
                return Err(syn::Error::new_spanned(
                    &name_value.value,
                    "Expected string literal for 'reliability' attribute",
                ));
            }
        }
        "generate_dual" => {
            if attrs.generate_dual {
                return Err(syn::Error::new_spanned(
                    name_value,
                    "Duplicate attribute 'generate_dual': this attribute can only be specified once",
                ));
            }
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Bool(lit_bool),
                ..
            }) = &name_value.value
            {
                attrs.generate_dual = lit_bool.value();
            } else {
                return Err(syn::Error::new_spanned(
                    &name_value.value,
                    "Expected boolean literal for 'generate_dual' attribute",
                ));
            }
        }
        "dual_name" => {
            if attrs.dual_name.is_some() {
                return Err(syn::Error::new_spanned(
                    name_value,
                    "Duplicate attribute 'dual_name': this attribute can only be specified once",
                ));
            }
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) = &name_value.value
            {
                attrs.dual_name = Some(lit_str.value());
            } else {
                return Err(syn::Error::new_spanned(
                    &name_value.value,
                    "Expected string literal for 'dual_name' attribute",
                ));
            }
        }
        "verify_duality" => {
            if attrs.verify_duality {
                return Err(syn::Error::new_spanned(
                    name_value,
                    "Duplicate attribute 'verify_duality': this attribute can only be specified once",
                ));
            }
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Bool(lit_bool),
                ..
            }) = &name_value.value
            {
                attrs.verify_duality = lit_bool.value();
            } else {
                return Err(syn::Error::new_spanned(
                    &name_value.value,
                    "Expected boolean literal for 'verify_duality' attribute",
                ));
            }
        }
        "dual_documentation" => {
            if attrs.dual_documentation {
                return Err(syn::Error::new_spanned(
                    name_value,
                    "Duplicate attribute 'dual_documentation': this attribute can only be specified once",
                ));
            }
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Bool(lit_bool),
                ..
            }) = &name_value.value
            {
                attrs.dual_documentation = lit_bool.value();
            } else {
                return Err(syn::Error::new_spanned(
                    &name_value.value,
                    "Expected boolean literal for 'dual_documentation' attribute",
                ));
            }
        }
        _ => {
            return Err(syn::Error::new_spanned(
                &name_value.path,
                format!("Unknown protocol attribute: '{}'. Supported attributes: io, metadata, buffer_size, timeout_ms, serialization, validation, concurrent, reliability, generate_dual, dual_name, verify_duality, dual_documentation", attr_name),
            ));
        }
    }

    Ok(())
}

/// Implementation of the `#[derive(GenerateDiagram)]` macro
pub fn derive_generate_diagram_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    handle_result(derive_generate_diagram_inner(&input))
}

/// Internal implementation logic for the GenerateDiagram derive
fn derive_generate_diagram_inner(input: &DeriveInput) -> Result<TokenStream2> {
    let type_name = get_type_name(input);

    // GenerateDiagram trait can be derived for structs and enums
    if !is_struct(input) && !is_enum(input) {
        return Err(syn::Error::new_spanned(
            input,
            "#[derive(GenerateDiagram)] can only be derived for structs and enums",
        ));
    }

    // Parse protocol-specific attributes for diagram generation
    let protocol_attrs = crate::utils::parse_derive_attributes(input, "protocol");

    // Generate the ProtocolFlow trait implementation
    let trait_impl = generate_protocol_flow_impl(type_name, &protocol_attrs);

    Ok(trait_impl)
}

/// Generate the ProtocolFlow trait implementation for diagram generation
fn generate_protocol_flow_impl(type_name: &syn::Ident, _attrs: &[syn::Attribute]) -> TokenStream2 {
    // For now, generate a basic implementation that protocols can override
    // This will be enhanced in future phases to extract from protocol structure

    let protocol_name = type_name.to_string();

    // Create a dummy DeriveInput for diagram generator
    let dummy_input = syn::DeriveInput {
        attrs: Vec::new(),
        vis: syn::Visibility::Public(syn::token::Pub::default()),
        ident: type_name.clone(),
        generics: syn::Generics::default(),
        data: syn::Data::Struct(syn::DataStruct {
            struct_token: syn::token::Struct::default(),
            fields: syn::Fields::Unit,
            semi_token: Some(syn::token::Semi::default()),
        }),
    };

    // Create diagram generator for enhanced documentation
    let diagram_generator = ProtocolDiagramGenerator::new(dummy_input);
    let automatic_docs = diagram_generator.generate_automatic_documentation();
    let diagram_method = diagram_generator.generate_diagram_method();

    quote! {
        #automatic_docs
        impl ::besedarium::protocol::introspection::ProtocolFlow for #type_name {
            fn generate_sequence_steps() -> Vec<::besedarium::protocol::introspection::SequenceStep> {
                // Default implementation - protocols should override this
                // In future phases, this will be generated from protocol structure analysis
                vec![
                    ::besedarium::protocol::introspection::SequenceStep::Send {
                        from: "Role1".to_string(),
                        to: "Role2".to_string(),
                        message: format!("{}_DefaultMessage", #protocol_name),
                    },
                    ::besedarium::protocol::introspection::SequenceStep::End,
                ]
            }

            fn get_roles() -> Vec<String> {
                // Default implementation - extract from protocol definition in future phases
                vec!["Role1".to_string(), "Role2".to_string()]
            }

            fn get_protocol_name() -> String {
                stringify!(#type_name).to_string()
            }

            fn get_diagram_config() -> ::besedarium::protocol::introspection::DiagramConfig {
                ::besedarium::protocol::introspection::DiagramConfig::default()
            }
        }

        impl ::besedarium::protocol::introspection::GeneratesDiagram for #type_name {}

        impl #type_name {
            #diagram_method
        }
    }
}
