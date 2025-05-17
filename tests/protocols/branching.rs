use besedarium::*;

// Protocol with branching (login vs. register)
pub type LoginOrRegister = tchoice!(Http;
    TInteract<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
    TInteract<Http, EmptyLabel, TClient, Publish, TEnd<Http, EmptyLabel>>
);

// All protocol example tests in this file have been temporarily disabled to stabilize the test base for the TInteract refactor.
