use besedarium::*;

// Client-server handshake (HTTP request/response)
pub type HttpHandshake = TInteract<
    Http,
    EmptyLabel,
    TClient,
    Message,
    TInteract<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>,
>;
