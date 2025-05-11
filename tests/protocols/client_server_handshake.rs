use playground::*;

// Client-server handshake (HTTP request/response)
pub type HttpHandshake = TInteract<Http, TClient, Message, TInteract<Http, TServer, Response, TEnd<Http>>>;
