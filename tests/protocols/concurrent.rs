use playground::*;

// Protocol with concurrency (parallel downloads)
pub type ParallelDownloads = tpar!(Http;
    TInteract<Http, TClient, Message, TEnd<Http>>,
    TInteract<Http, TClient, Publish, TEnd<Http>>
);
