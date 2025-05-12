use besedarium::*;

struct L1; impl ProtocolLabel for L1 {}
struct L2; impl ProtocolLabel for L2 {}

// This should fail: duplicate label L1
// (Trybuild will check that this fails to compile)
type DuplicateLabels = TChoice<
    Http,
    L1,
    TInteract<Http, L1, TClient, Message, TEnd<Http, EmptyLabel>>,
    TInteract<Http, L1, TServer, Response, TEnd<Http, EmptyLabel>>
>;
assert_unique_labels!(DuplicateLabels);