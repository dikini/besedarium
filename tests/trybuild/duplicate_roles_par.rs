use besedarium::*;

type DupRolePar = tpar!(Http;
    TInteract<Http, TClient, Message, TEnd<Http>>,
    TInteract<Http, TClient, Publish, TEnd<Http>>
);
assert_disjoint!(par DupRolePar);
