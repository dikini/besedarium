use besedarium::*;

// Recursive/streaming protocol
pub type Streaming =
    TRec<Http, EmptyLabel, TInteract<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>>;
