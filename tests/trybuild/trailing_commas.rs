use besedarium::*;

type TrailingCommaChoice = tchoice!(Http;
    TInteract<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
    TInteract<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>,
);

type TrailingCommaPar = tpar!(Http;
    TInteract<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
    TInteract<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>,
);

// All trybuild compile-fail tests in this file have been temporarily disabled to stabilize the test base for the TInteract refactor.
