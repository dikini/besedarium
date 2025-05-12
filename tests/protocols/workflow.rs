use besedarium::*;

// Multi-party workflow (client, server, broker, worker)
pub type Workflow = tpar!(Http;
    TInteract<Http, EmptyLabel, TClient, Message, TInteract<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>>,
    TInteract<Http, EmptyLabel, TBroker, Publish, TEnd<Http, EmptyLabel>>,
    TInteract<Http, EmptyLabel, TWorker, Notify, TEnd<Http, EmptyLabel>>
);
