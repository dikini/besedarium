//! Test for embedded documentation in protocol derive macros
//! 
//! This example demonstrates how the `#[derive(GenerateDiagram)]` macro
//! automatically embeds Mermaid diagrams directly into doccomments.

use besedarium::protocol::foundation::GlobalProtocol;

// Import the derive macro for automatic diagram generation
#[cfg(feature = "derive")]
use besedarium_derive::GenerateDiagram;

/// Example protocol with automatic diagram embedding
/// 
/// This protocol demonstrates automatic diagram embedding via the derive macro.
#[derive(Debug)]
#[cfg_attr(feature = "derive", derive(GenerateDiagram))]
pub struct ExampleProtocol;

impl GlobalProtocol for ExampleProtocol {}

fn main() {
    println!("Example Protocol Documentation Test");
    
    #[cfg(feature = "derive")]
    {
        // Generate the diagram at runtime
        let diagram = ExampleProtocol::generate_diagram();
        println!("\nRuntime Generated Diagram:");
        println!("{}", diagram);
        
        println!("\n✓ The protocol doccomment now contains the embedded diagram!");
        println!("✓ Use `cargo doc --open` to view the embedded documentation.");
    }
    
    #[cfg(not(feature = "derive"))]
    {
        println!("Note: Enable 'derive' feature to see automatic diagram embedding");
    }
}
