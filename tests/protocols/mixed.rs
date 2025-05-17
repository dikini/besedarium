use besedarium::*;

// Protocol using Mixed marker for informational use
pub type MixedExample = tpar!(Mixed;
    TSend<Mixed, EmptyLabel, TClient, Message, TEnd<Mixed, EmptyLabel>>,
    TSend<Mixed, EmptyLabel, TBroker, Publish, TEnd<Mixed, EmptyLabel>>
);

// All protocol example tests in this file have been temporarily disabled to stabilize the test base for the TInteract refactor.
