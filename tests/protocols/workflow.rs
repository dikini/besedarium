use playground::*;

// Multi-party workflow (client, server, broker, worker)
pub type Workflow = tpar!(Http;
    TInteract<Http, TClient, Message, TInteract<Http, TServer, Response, TEnd<Http>>>,
    TInteract<Http, TBroker, Publish, TEnd<Http>>,
    TInteract<Http, TWorker, Notify, TEnd<Http>>
);
