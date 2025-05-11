use playground::*;

// Protocol using Mixed marker for informational use
pub type MixedExample = tpar!(Mixed;
    TInteract<Mixed, TClient, Message, TEnd<Mixed>>,
    TInteract<Mixed, TBroker, Publish, TEnd<Mixed>>
);
