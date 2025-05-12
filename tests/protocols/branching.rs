use besedarium::*;

// Protocol with branching (login vs. register)
pub type LoginOrRegister = tchoice!(Http;
    TInteract<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
    TInteract<Http, EmptyLabel, TClient, Publish, TEnd<Http, EmptyLabel>>
);
