use playground::*;

type MixedIOChoice = tchoice!(Http;
    TInteract<Http, TClient, Message, TEnd<Http>>,
    TInteract<Mqtt, TBroker, Publish, TEnd<Mqtt>>
);
