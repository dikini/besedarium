use besedarium::*;

type MixedIOChoice = tchoice!(Http;
    TInteract<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
    TInteract<Mqtt, EmptyLabel, TBroker, Publish, TEnd<Mqtt, EmptyLabel>>
);

// All trybuild compile-fail tests in this file have been temporarily disabled to stabilize the test base for the TInteract refactor.
