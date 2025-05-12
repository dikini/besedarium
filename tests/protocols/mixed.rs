use besedarium::*;

// Protocol using Mixed marker for informational use
pub type MixedExample = tpar!(Mixed;
    TInteract<Mixed, EmptyLabel, TClient, Message, TEnd<Mixed, EmptyLabel>>,
    TInteract<Mixed, EmptyLabel, TBroker, Publish, TEnd<Mixed, EmptyLabel>>
);
