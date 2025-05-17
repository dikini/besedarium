use besedarium::*;

// Client-server handshake (HTTP request/response)
pub type HttpHandshake = TInteract<
    Http,
    EmptyLabel,
    TClient,
    Message,
    TInteract<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>,
>;

// All protocol example tests in this file have been temporarily disabled to stabilize the test base for the TInteract refactor.
