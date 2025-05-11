use besedarium::*;

type TrailingCommaChoice = tchoice!(Http;
    TInteract<Http, TClient, Message, TEnd<Http>>,
    TInteract<Http, TServer, Response, TEnd<Http>>,
);

type TrailingCommaPar = tpar!(Http;
    TInteract<Http, TClient, Message, TEnd<Http>>,
    TInteract<Http, TServer, Response, TEnd<Http>>,
);
