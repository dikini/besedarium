use besedarium::*;

// Recursive/streaming protocol
pub type Streaming = TRec<Http, TInteract<Http, TClient, Message, TEnd<Http>>>;
