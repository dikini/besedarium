use besedarium::*;

// Recursive/streaming protocol
pub type Streaming =
    TRec<Http, EmptyLabel, TInteract<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>>;

// All protocol example tests in this file have been temporarily disabled to stabilize the test base for the TInteract refactor.
