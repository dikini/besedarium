use besedarium::*;

type MixedIOChoice = tchoice!(Http;
    TInteract<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
    TInteract<Mqtt, EmptyLabel, TBroker, Publish, TEnd<Mqtt, EmptyLabel>>
);
