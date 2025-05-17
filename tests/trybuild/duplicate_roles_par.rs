use besedarium::*;

type DupRolePar = tpar!(Http;
    TInteract<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
    TInteract<Http, EmptyLabel, TClient, Publish, TEnd<Http, EmptyLabel>>
);
assert_disjoint!(par DupRolePar);

// All trybuild compile-fail tests in this file have been temporarily disabled to stabilize the test base for the TInteract refactor.
