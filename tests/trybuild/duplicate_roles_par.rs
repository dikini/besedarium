use besedarium::*;

type DupRolePar = tpar!(Http;
    TInteract<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>,
    TInteract<Http, EmptyLabel, TClient, Publish, TEnd<Http, EmptyLabel>>
);
assert_disjoint!(par DupRolePar);
