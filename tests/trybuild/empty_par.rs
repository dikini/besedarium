use besedarium::*;

type EmptyPar = tpar!(Http;);

// Test: Projecting an empty parallel composition for any role yields EpEnd
struct DummyRole;
impl besedarium::Role for DummyRole {}
impl besedarium::ProtocolLabel for DummyRole {}
impl besedarium::RoleEq<DummyRole> for DummyRole { type Output = besedarium::True; }

// Compile-time assertion: projection for DummyRole is EpEnd
assert_type_eq!(
    <() as besedarium::ProjectRole<DummyRole, Http, EmptyPar>>::Out,
    besedarium::EpEnd<Http, DummyRole>
);

// The above assertion ensures that uninvolved (silent) roles in an empty TPar project to EpEnd.
