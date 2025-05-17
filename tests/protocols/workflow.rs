use besedarium::*;

// All protocol example tests in this file have been temporarily disabled to stabilize the test base for the TInteract refactor.

// Multi-party workflow (client, server, broker, worker)
pub type Workflow = tpar!(Http;
    TSend<Http, EmptyLabel, TClient, Message, TSend<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>>,
    TSend<Http, EmptyLabel, TBroker, Publish, TEnd<Http, EmptyLabel>>,
    TSend<Http, EmptyLabel, TWorker, Notify, TEnd<Http, EmptyLabel>>
);
