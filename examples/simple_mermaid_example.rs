//! Example demonstrating simple-mermaid integration with protocol diagrams
//!
//! This example shows how to use the `simple-mermaid` crate to embed
//! rendered Mermaid diagrams directly in documentation.

use besedarium::protocol::foundation::GlobalProtocol;

/// A protocol that demonstrates simple-mermaid integration
///
/// This protocol uses a pre-generated Mermaid diagram file that is embedded
/// in the documentation using the `simple-mermaid` crate. The diagram will
/// be properly rendered in the generated rustdoc output.
#[doc = ::simple_mermaid::mermaid!("../generated_diagrams/example_protocol.mermaid")]
#[derive(Debug)]
pub struct SimpleProtocol;

impl GlobalProtocol for SimpleProtocol {}

fn main() {
    println!("Simple Mermaid Integration Example");
    println!("==================================");
    println!();
    println!("This example demonstrates how to use simple-mermaid to embed");
    println!("rendered Mermaid diagrams in protocol documentation.");
    println!();
    println!("To view the embedded diagrams:");
    println!("1. Run: cargo doc --example simple_mermaid_example --open");
    println!("2. Navigate to the SimpleProtocol struct documentation");
    println!("3. The Mermaid diagram should be properly rendered");
    println!();
    println!("The diagram file used: generated_diagrams/example_protocol.mermaid");
}
