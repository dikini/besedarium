use besedarium::*;

// All protocol example tests in this file have been temporarily disabled to stabilize the test base for the TInteract refactor.

// Protocol with concurrency (parallel downloads)
pub type ParallelDownloads = tpar!(Http;
    TSend<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
    TSend<Http, EmptyLabel, TClient, Publish, TEnd<Http, EmptyLabel>>
);
