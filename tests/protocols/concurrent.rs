use besedarium::*;

// Protocol with concurrency (parallel downloads)
pub type ParallelDownloads = tpar!(Http;
    TInteract<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
    TInteract<Http, EmptyLabel, TClient, Publish, TEnd<Http, EmptyLabel>>
);
