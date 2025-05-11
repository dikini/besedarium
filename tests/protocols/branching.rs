use playground::*;

// Protocol with branching (login vs. register)
pub type LoginOrRegister = tchoice!(Http;
    TInteract<Http, TClient, Message, TEnd<Http>>,
    TInteract<Http, TClient, Publish, TEnd<Http>>
);
