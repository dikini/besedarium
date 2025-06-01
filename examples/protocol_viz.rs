//! Protocol Visualization Example
//!
//! This example demonstrates protocol definition with embedded Mermaid diagrams
//! for documentation. The diagrams can be rendered in documentation using mdBook
//! or other Mermaid-supporting tools.
//!
//! Run with:
//! ```bash
//! cargo doc --example protocol_viz --open
//! ```

use besedarium::protocol::foundation::{
    BiDirectionalAction, DefaultChan, GlobalProtocol, InputAction, Message, OutputAction,
    RequestLbl, ResponseLbl, Role,
};
use besedarium::protocol::global::{TChanEnd, TChanRecv, TChanSend};

/// A simple client role for demonstration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Client;

impl Role for Client {}

/// A simple server role for demonstration  
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Server;

impl Role for Server {}

/// A greeting message
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Greeting {
    pub content: String,
}

impl Message for Greeting {}

/// A response message  
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub content: String,
}

impl Message for Response {}

/// Simple client-server protocol: Client sends greeting, Server responds
///
/// This protocol represents a basic request-response pattern where:
/// 1. Client sends a greeting message to Server
/// 2. Server sends a response message back to Client  
/// 3. Protocol terminates
///
/// # Protocol Diagram
///
/// ```mermaid
/// sequenceDiagram
///     participant C as Client
///     participant S as Server
///     
///     C->>S: Greeting("Hello!")
///     S-->>C: Response("Hi there!")
///     
///     Note over C,S: Protocol Complete
/// ```
pub type SimpleClientServerProtocol = TChanSend<
    Client,
    Server,
    DefaultChan,
    RequestLbl,
    Greeting,
    TChanRecv<
        Server,
        Client,
        DefaultChan,
        ResponseLbl,
        Response,
        TChanEnd<DefaultChan, ResponseLbl, BiDirectionalAction>,
        BiDirectionalAction,
    >,
    OutputAction,
>;

/// Wrapper struct to implement GlobalProtocol trait
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleProtocol(pub SimpleClientServerProtocol);

impl GlobalProtocol for SimpleProtocol {}

/// Extended protocol with multiple message exchanges
///
/// This protocol demonstrates a more complex interaction pattern:
/// 1. Client sends initial greeting
/// 2. Server responds with acknowledgment
/// 3. Client sends a second message
/// 4. Server sends final response
/// 5. Protocol terminates
///
/// # Extended Protocol Diagram
///
/// ```mermaid
/// sequenceDiagram
///     participant C as Client  
///     participant S as Server
///     
///     C->>S: Greeting("Hello Server!")
///     S-->>C: Response("Hello Client!")
///     C->>S: Greeting("How are you?")
///     S-->>C: Response("I am doing well!")
///     
///     Note over C,S: Extended Protocol Complete
/// ```
pub type ExtendedClientServerProtocol = TChanSend<
    Client,
    Server,
    DefaultChan,
    RequestLbl,
    Greeting,
    TChanRecv<
        Server,
        Client,
        DefaultChan,
        ResponseLbl,
        Response,
        TChanSend<
            Client,
            Server,
            DefaultChan,
            RequestLbl,
            Greeting,
            TChanRecv<
                Server,
                Client,
                DefaultChan,
                ResponseLbl,
                Response,
                TChanEnd<DefaultChan, ResponseLbl, BiDirectionalAction>,
                BiDirectionalAction,
            >,
            OutputAction,
        >,
        InputAction,
    >,
    OutputAction,
>;

/// Wrapper struct to implement GlobalProtocol trait
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedProtocol(pub ExtendedClientServerProtocol);

impl GlobalProtocol for ExtendedProtocol {}

/// Utility function to export protocol diagrams to Mermaid format
pub fn export_simple_protocol_diagram() -> String {
    r#"sequenceDiagram
    participant C as Client
    participant S as Server
    
    C->>S: Greeting("Hello!")
    S-->>C: Response("Hi there!")
    
    Note over C,S: Protocol Complete"#
        .to_string()
}

/// Utility function to export extended protocol diagram to Mermaid format
pub fn export_extended_protocol_diagram() -> String {
    r#"sequenceDiagram
    participant C as Client
    participant S as Server
    
    C->>S: Greeting("Hello Server!")
    S-->>C: Response("Hello Client!")
    C->>S: Greeting("How are you?")
    S-->>C: Response("I am doing well!")
    
    Note over C,S: Extended Protocol Complete"#
        .to_string()
}

fn main() {
    println!("Protocol Visualization Example");
    println!("==============================");
    println!();
    println!("This example demonstrates protocol definition with Mermaid diagrams.");
    println!(
        "Run 'cargo doc --example protocol_viz --open' to see the diagrams in the documentation."
    );
    println!();
    println!("Protocols defined:");
    println!("- SimpleClientServerProtocol: Basic request-response");
    println!("- ExtendedClientServerProtocol: Multiple message exchanges");
    println!();
    println!("Generated Mermaid diagrams:");
    println!();
    println!("Simple Protocol:");
    println!("{}", export_simple_protocol_diagram());
    println!();
    println!("Extended Protocol:");
    println!("{}", export_extended_protocol_diagram());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_types_compile() {
        // This test verifies that our protocol types compile correctly
        let _simple: SimpleClientServerProtocol;
        let _extended: ExtendedClientServerProtocol;
    }

    #[test]
    fn test_protocol_wrappers() {
        // Test that our wrapper types implement the required traits
        let _simple_wrapper = SimpleProtocol;
        let _extended_wrapper = ExtendedProtocol;

        // Test trait bounds
        fn assert_global_protocol<T: GlobalProtocol>() {}
        assert_global_protocol::<SimpleProtocol>();
        assert_global_protocol::<ExtendedProtocol>();
    }

    #[test]
    fn test_role_traits() {
        // Test that our role types implement the required traits
        let _client = Client;
        let _server = Server;

        // Test trait bounds
        fn assert_role<T: Role>() {}
        assert_role::<Client>();
        assert_role::<Server>();
    }

    #[test]
    fn test_message_traits() {
        // Test that our message types implement the required traits
        let _greeting = Greeting {
            content: "test".to_string(),
        };
        let _response = Response {
            content: "test".to_string(),
        };

        // Test trait bounds
        fn assert_message<T: Message>() {}
        assert_message::<Greeting>();
        assert_message::<Response>();
    }

    #[test]
    fn test_diagram_export() {
        // Test that diagram export functions work
        let simple_diagram = export_simple_protocol_diagram();
        let extended_diagram = export_extended_protocol_diagram();

        assert!(simple_diagram.contains("sequenceDiagram"));
        assert!(simple_diagram.contains("Client"));
        assert!(simple_diagram.contains("Server"));

        assert!(extended_diagram.contains("sequenceDiagram"));
        assert!(extended_diagram.contains("Client"));
        assert!(extended_diagram.contains("Server"));
    }
}
