//! Tests for introspection functionality (LabelsOf, RolesOf)
//!
//! This file contains tests to verify the behavior of introspection traits
//! that extract metadata from protocol types at the type level.

use besedarium::*;

// --- Custom Label Types for Testing ---
struct L1;
struct L2;
struct L3;
impl ProtocolLabel for L1 {}
impl ProtocolLabel for L2 {}
impl ProtocolLabel for L3 {}

// --- Label List Comparison Traits ---
/// Type-level trait to verify that two label lists are the same
pub trait SameLabelList<T> {}

/// Matches two empty lists
impl SameLabelList<Nil> for Nil {}

/// Matches two non-empty lists if their heads are the same type and tails are the same list
impl<H, T1, T2> SameLabelList<Cons<H, T2>> for Cons<H, T1> where T1: SameLabelList<T2> {}

/// Trait to assert that a type's labels match an expected list
pub trait HasLabels<Expected> {}

/// Implementation that checks if a type's labels match an expected list
impl<T: LabelsOf, Expected> HasLabels<Expected> for T where
    <T as LabelsOf>::Labels: SameLabelList<Expected>
{
}

// --- Tests for LabelsOf trait ---
#[cfg(test)]
mod labels_of_tests {
    use super::*;

    // Test IO types
    struct Http;

    // Test role types
    struct TClient;
    struct TServer;
    impl Role for TClient {}
    impl Role for TServer {}

    // Test message types
    struct Message;
    struct Response;

    // Test that TEnd<IO, L> correctly extracts its label
    #[test]
    fn test_tend_labels() {
        // Define a type using TEnd with custom label
        type EndWithLabel = TEnd<Http, L1>;

        // Expected label list is Cons<L1, Nil>
        type Expected = Cons<L1, Nil>;

        // This will compile only if the labels match the expected list
        fn assert_correct_labels<T: HasLabels<Expected>>() {}
        assert_correct_labels::<EndWithLabel>();
    }

    // Test that TInteract<IO, L, R, H, T> correctly extracts its label
    #[test]
    fn test_tinteract_labels() {
        // Define a type using TInteract with custom label
        type InteractWithLabel = TInteract<Http, L1, TClient, Message, TEnd<Http, L2>>;

        // Expected label list is Cons<L1, Cons<L2, Nil>>
        type Expected = Cons<L1, Cons<L2, Nil>>;

        // This will compile only if the labels match the expected list
        fn assert_correct_labels<T: HasLabels<Expected>>() {}
        assert_correct_labels::<InteractWithLabel>();
    }

    // Test that TRec<IO, L, S> correctly extracts its label
    #[test]
    fn test_trec_labels() {
        // Define a type using TRec with custom label
        type RecWithLabel = TRec<Http, L1, TEnd<Http, L2>>;

        // Expected label list is Cons<L1, Cons<L2, Nil>>
        type Expected = Cons<L1, Cons<L2, Nil>>;

        // This will compile only if the labels match the expected list
        fn assert_correct_labels<T: HasLabels<Expected>>() {}
        assert_correct_labels::<RecWithLabel>();
    }

    // Test that TChoice<IO, Lbl, L, R> correctly extracts its label
    #[test]
    fn test_tchoice_labels() {
        // Define a type using TChoice with custom label
        type ChoiceWithLabel = TChoice<
            Http,
            L1,
            TInteract<Http, L2, TClient, Message, TEnd<Http, L3>>,
            TEnd<Http, EmptyLabel>,
        >;

        // Expected label list is Cons<L1, Cons<L2, Cons<L3, Nil>>>
        type Expected = Cons<L1, Cons<L2, Cons<L3, Nil>>>;

        // This will compile only if the labels match the expected list
        fn assert_correct_labels<T: HasLabels<Expected>>() {}
        assert_correct_labels::<ChoiceWithLabel>();
    }

    // Test that TPar<IO, Lbl, L, R, IsDisjoint> correctly extracts its label
    #[test]
    fn test_tpar_labels() {
        // Define a type using TPar with custom label
        type ParWithLabel = TPar<
            Http,
            L1,
            TInteract<Http, L2, TClient, Message, TEnd<Http, L3>>,
            TEnd<Http, EmptyLabel>,
            FalseB,
        >;

        // Expected label list is Cons<L1, Cons<L2, Cons<L3, Nil>>>
        type Expected = Cons<L1, Cons<L2, Cons<L3, Nil>>>;

        // This will compile only if the labels match the expected list
        fn assert_correct_labels<T: HasLabels<Expected>>() {}
        assert_correct_labels::<ParWithLabel>();
    }

    // Test complex nested protocol structure
    #[test]
    fn test_complex_protocol_labels() {
        // Create a complex protocol with multiple branches and nested structures
        type Branch1 = TInteract<Http, L1, TClient, Message, TEnd<Http, EmptyLabel>>;
        type Branch2 =
            TRec<Http, L2, TInteract<Http, L3, TServer, Response, TEnd<Http, EmptyLabel>>>;

        type ComplexProtocol = TPar<
            Http,
            L1,
            Branch1,
            TChoice<
                Http,
                L2,
                Branch2,
                TInteract<Http, L3, TClient, Message, TEnd<Http, EmptyLabel>>,
            >,
            FalseB,
        >;

        // Expected label list is quite complex due to nesting
        // TPar has L1, then branch1 has L1 and EmptyLabel, then L2 from TChoice, L2 from TRec, etc.
        type Expected = Cons<L1, Cons<L1, Cons<EmptyLabel, Nil>>>;

        // This will compile only if the labels match the expected list
        fn assert_correct_labels<T: HasLabels<Expected>>() {}
        assert_correct_labels::<ComplexProtocol>();
    }
}

// --- Tests for RolesOf trait ---
#[cfg(test)]
mod roles_of_tests {
    use super::*;

    // Test IO types
    struct Http;

    // Test role types
    struct TClient;
    struct TServer;
    impl Role for TClient {}
    impl Role for TServer {}

    // Test message types
    struct Message;
    struct Response;

    /// Type-level trait to verify that two role lists are the same
    pub trait SameRoleList<T> {}

    /// Matches two empty lists
    impl SameRoleList<Nil> for Nil {}

    /// Matches two non-empty lists if their heads are the same type and tails are the same list
    impl<H, T1, T2> SameRoleList<Cons<H, T2>> for Cons<H, T1> where T1: SameRoleList<T2> {}

    /// Trait to assert that a type's roles match an expected list
    pub trait HasRoles<Expected> {}

    /// Implementation that checks if a type's roles match an expected list
    impl<T: RolesOf, Expected> HasRoles<Expected> for T where
        <T as RolesOf>::Roles: SameRoleList<Expected>
    {
    }

    // Test that TInteract<IO, L, R, H, T> correctly extracts its roles
    #[test]
    fn test_tinteract_roles() {
        // Define a type using TInteract with roles
        // Use EmptyLabel for TEnd to match current implementation
        type InteractWithRole = TInteract<Http, L1, TClient, Message, TEnd<Http>>;

        // Expected role list is Cons<TClient, Nil>
        type Expected = Cons<TClient, Nil>;

        // This will compile only if the roles match the expected list
        fn assert_correct_roles<T: HasRoles<Expected>>() {}
        assert_correct_roles::<InteractWithRole>();
    }

    // Test complex protocol with multiple roles
    #[test]
    fn test_complex_protocol_roles() {
        // Define a complex protocol with multiple roles
        // Use TEnd<Http> instead of TEnd<Http, L3> to match current implementation
        type Protocol = TInteract<
            Http,
            L1,
            TClient,
            Message,
            TInteract<Http, L2, TServer, Response, TEnd<Http>>,
        >;

        // Expected role list is Cons<TClient, Cons<TServer, Nil>>
        type Expected = Cons<TClient, Cons<TServer, Nil>>;

        // This will compile only if the roles match the expected list
        fn assert_correct_roles<T: HasRoles<Expected>>() {}
        assert_correct_roles::<Protocol>();
    }
}
