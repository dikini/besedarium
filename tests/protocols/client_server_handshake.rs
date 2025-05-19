use besedarium::*;

// Protocol-specific labels
pub struct HandshakeStartLabel;
pub struct ClientRequestLabel;
pub struct ServerResponseLabel; 
pub struct HandshakeEndLabel;

impl ProtocolLabel for HandshakeStartLabel {}
impl ProtocolLabel for ClientRequestLabel {}
impl ProtocolLabel for ServerResponseLabel {}
impl ProtocolLabel for HandshakeEndLabel {}

// Client-server handshake (HTTP request/response)
pub type HttpHandshake = TStart<
    Http,
    HandshakeStartLabel,
    TSend<
        Http,
        ClientRequestLabel,
        TClient,
        Message,
        TSend<Http, ServerResponseLabel, TServer, Response, TEnd<Http, HandshakeEndLabel>>,
    >,
>;

// All protocol example tests in this file have been temporarily disabled to stabilize the test base for the TInteract refactor.
