//! Label Testing Infrastructure
//!
//! This module provides utilities and baseline tests for label behavior in session types.
//! It includes helper traits for comparing and extracting label types, as well as
//! assertion macros for label-specific testing.

use besedarium::*;

// --- Custom Label Types for Testing ---
// These label types are used to test label parameter behavior consistently
struct L1;
struct L2;
struct L3;
impl ProtocolLabel for L1 {}
impl ProtocolLabel for L2 {}
impl ProtocolLabel for L3 {}

// --- Label Extraction Traits ---
/// Extracts the label type from a session type.
/// This trait is used to verify label preservation during operations.
pub trait ExtractLabel<IO> {
    /// The extracted label type
    type Label;
}

// Implement ExtractLabel for TEnd
impl<IO, L> ExtractLabel<IO> for TEnd<IO, L> {
    type Label = L;
}

// Implement ExtractLabel for TInteract
impl<IO, L, R, H, T> ExtractLabel<IO> for TInteract<IO, L, R, H, T>
where
    L: ProtocolLabel,
    T: TSession<IO>,
{
    type Label = L;
}

// Implement ExtractLabel for TRec
impl<IO, L, S> ExtractLabel<IO> for TRec<IO, L, S>
where
    L: ProtocolLabel,
    S: TSession<IO>,
{
    type Label = L;
}

// Implement ExtractLabel for TChoice
impl<IO, Lbl, L, R> ExtractLabel<IO> for TChoice<IO, Lbl, L, R>
where
    Lbl: ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
{
    type Label = Lbl;
}

// Implement ExtractLabel for TPar
impl<IO, Lbl, L, R, IsDisjoint> ExtractLabel<IO> for TPar<IO, Lbl, L, R, IsDisjoint>
where
    Lbl: ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
{
    type Label = Lbl;
}

// --- Label Composition Verification ---
/// Verifies that the label is preserved after composition
pub trait VerifyLabelPreservation<IO, Rhs, Expected> {
    type Result;
}

impl<IO, S, Rhs, L> VerifyLabelPreservation<IO, Rhs, L> for S
where
    S: TSession<IO>,
    Rhs: TSession<IO>,
    S::Compose<Rhs>: ExtractLabel<IO>,
    <S::Compose<Rhs> as ExtractLabel<IO>>::Label: Same<L>,
{
    type Result = ();
}

/// Type-level equality check for labels
pub trait Same<T> {}
impl<T> Same<T> for T {}

// --- Label Preservation Test Cases ---
#[cfg(test)]
mod label_preservation_tests {
    use super::*;

    // Define test IO type
    pub struct Http;

    // Define test role types
    pub struct TClient;
    pub struct TServer;
    impl Role for TClient {}
    impl Role for TServer {}

    // Define test message types
    pub struct Message;
    pub struct Response;

    // Test that TEnd's label is not preserved in composition (replaced by Rhs)
    #[test]
    fn test_tend_label_replaced() {
        type LabeledEnd = TEnd<Http, L1>;
        type Continuation = TInteract<Http, L2, TClient, Message, TEnd<Http, L3>>;
        type Composed = <LabeledEnd as TSession<Http>>::Compose<Continuation>;

        // End is replaced by Rhs completely, so expected label is L2
        fn assert_label_is_l2<T: ExtractLabel<Http>>()
        where
            <T as ExtractLabel<Http>>::Label: Same<L2>,
        {
        }

        // This should compile successfully - verifying the extracted label is L2
        assert_label_is_l2::<Composed>();
    }

    // Test that TInteract's label is preserved in composition
    #[test]
    fn test_tinteract_label_preserved() {
        type LabeledInteract = TInteract<Http, L1, TClient, Message, TEnd<Http, EmptyLabel>>;
        type Continuation = TInteract<Http, L2, TServer, Response, TEnd<Http, EmptyLabel>>;
        type Composed = <LabeledInteract as TSession<Http>>::Compose<Continuation>;

        fn assert_label_is_l1<T: ExtractLabel<Http>>()
        where
            <T as ExtractLabel<Http>>::Label: Same<L1>,
        {
        }

        // This should compile successfully - verifying the extracted label is L1
        assert_label_is_l1::<Composed>();
    }

    // Additional test for TInteract with L2 label type
    #[test]
    fn test_tinteract_l2_label_preserved() {
        type LabeledInteract = TInteract<Http, L2, TClient, Message, TEnd<Http, EmptyLabel>>;
        type Continuation = TInteract<Http, L3, TServer, Response, TEnd<Http, EmptyLabel>>;
        type Composed = <LabeledInteract as TSession<Http>>::Compose<Continuation>;

        fn assert_label_is_l2<T: ExtractLabel<Http>>()
        where
            <T as ExtractLabel<Http>>::Label: Same<L2>,
        {
        }

        // This should compile successfully - verifying the extracted label is L2
        assert_label_is_l2::<Composed>();
    }

    // Additional test for TInteract with L3 label type
    #[test]
    fn test_tinteract_l3_label_preserved() {
        type LabeledInteract = TInteract<Http, L3, TClient, Message, TEnd<Http, EmptyLabel>>;
        type Continuation = TInteract<Http, L1, TServer, Response, TEnd<Http, EmptyLabel>>;
        type Composed = <LabeledInteract as TSession<Http>>::Compose<Continuation>;

        fn assert_label_is_l3<T: ExtractLabel<Http>>()
        where
            <T as ExtractLabel<Http>>::Label: Same<L3>,
        {
        }

        // This should compile successfully - verifying the extracted label is L3
        assert_label_is_l3::<Composed>();
    }

    // Test that TRec's label is preserved in composition
    #[test]
    fn test_trec_label_preserved() {
        type LabeledRec = TRec<Http, L1, TEnd<Http, EmptyLabel>>;
        type Continuation = TInteract<Http, L2, TServer, Response, TEnd<Http, EmptyLabel>>;
        type Composed = <LabeledRec as TSession<Http>>::Compose<Continuation>;

        fn assert_label_is_l1<T: ExtractLabel<Http>>()
        where
            <T as ExtractLabel<Http>>::Label: Same<L1>,
        {
        }

        // This should compile successfully - verifying the extracted label is L1
        assert_label_is_l1::<Composed>();
    }

    // Additional test for TRec with L2 label type
    #[test]
    fn test_trec_l2_label_preserved() {
        type LabeledRec = TRec<Http, L2, TEnd<Http, EmptyLabel>>;
        type Continuation = TInteract<Http, L3, TServer, Response, TEnd<Http, EmptyLabel>>;
        type Composed = <LabeledRec as TSession<Http>>::Compose<Continuation>;

        fn assert_label_is_l2<T: ExtractLabel<Http>>()
        where
            <T as ExtractLabel<Http>>::Label: Same<L2>,
        {
        }

        // This should compile successfully - verifying the extracted label is L2
        assert_label_is_l2::<Composed>();
    }

    // Additional test for TRec with L3 label type
    #[test]
    fn test_trec_l3_label_preserved() {
        type LabeledRec = TRec<Http, L3, TEnd<Http, EmptyLabel>>;
        type Continuation = TInteract<Http, L1, TServer, Response, TEnd<Http, EmptyLabel>>;
        type Composed = <LabeledRec as TSession<Http>>::Compose<Continuation>;

        fn assert_label_is_l3<T: ExtractLabel<Http>>()
        where
            <T as ExtractLabel<Http>>::Label: Same<L3>,
        {
        }

        // This should compile successfully - verifying the extracted label is L3
        assert_label_is_l3::<Composed>();
    }

    // Test that TChoice's label is preserved in composition
    #[test]
    fn test_tchoice_label_preserved() {
        type LabeledChoice = TChoice<
            Http,
            L1,
            TInteract<Http, L2, TClient, Message, TEnd<Http, EmptyLabel>>,
            TInteract<Http, L3, TServer, Response, TEnd<Http, EmptyLabel>>,
        >;
        type Continuation = TInteract<Http, L2, TServer, Response, TEnd<Http, EmptyLabel>>;
        type Composed = <LabeledChoice as TSession<Http>>::Compose<Continuation>;

        fn assert_label_is_l1<T: ExtractLabel<Http>>()
        where
            <T as ExtractLabel<Http>>::Label: Same<L1>,
        {
        }

        // This should compile successfully - verifying the extracted label is L1
        assert_label_is_l1::<Composed>();
    }

    // Test that TPar's label is preserved in composition
    #[test]
    fn test_tpar_label_preserved() {
        type LabeledPar = TPar<
            Http,
            L1,
            TInteract<Http, L2, TClient, Message, TEnd<Http, EmptyLabel>>,
            TInteract<Http, L3, TServer, Response, TEnd<Http, EmptyLabel>>,
            FalseB,
        >;
        type Continuation = TInteract<Http, L2, TServer, Response, TEnd<Http, EmptyLabel>>;
        type Composed = <LabeledPar as TSession<Http>>::Compose<Continuation>;

        fn assert_label_is_l1<T: ExtractLabel<Http>>()
        where
            <T as ExtractLabel<Http>>::Label: Same<L1>,
        {
        }

        // This should compile successfully - verifying the extracted label is L1
        assert_label_is_l1::<Composed>();
    }
}

#[cfg(test)]
mod label_edge_cases {
    use super::*;

    // Define test IO type
    pub struct Http;

    // Define test role types
    pub struct TClient;
    pub struct TServer;
    impl Role for TClient {}
    impl Role for TServer {}

    // Define test message types
    pub struct Message;
    pub struct Response;

    // Edge Case 1: Nested composition with multiple label types
    // Test complex nesting of session types and verify label propagation
    #[test]
    fn test_nested_composition_label_preservation() {
        // Create a deeply nested protocol with different labels at each level
        type InnerProtocol = TInteract<Http, L1, TClient, Message, TEnd<Http, EmptyLabel>>;
        type MiddleProtocol = TRec<Http, L2, InnerProtocol>;
        type OuterProtocol = TChoice<Http, L3, MiddleProtocol, TEnd<Http, EmptyLabel>>;
        
        // Create a simple continuation
        type Continuation = TInteract<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>;
        
        // Compose protocols
        type Composed = <OuterProtocol as TSession<Http>>::Compose<Continuation>;

        // Verify that the outermost label (L3) is preserved
        fn assert_label_is_l3<T: ExtractLabel<Http>>()
        where
            <T as ExtractLabel<Http>>::Label: Same<L3>,
        {
        }

        // This should compile successfully - verifying the extracted label is L3
        assert_label_is_l3::<Composed>();
    }

    // Edge Case 2: Mixed combinator interactions
    // Test how different combinators interact when composed together
    #[test]
    fn test_mixed_combinator_interactions() {
        // Create a protocol mixing TPar and TChoice
        type LeftBranch = TInteract<Http, L1, TClient, Message, TEnd<Http, EmptyLabel>>;
        type RightBranch = TChoice<
            Http,
            L2,
            TInteract<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>,
            TEnd<Http, EmptyLabel>,
        >;
        
        type MixedProtocol = TPar<Http, L3, LeftBranch, RightBranch, FalseB>;
        type Continuation = TRec<Http, EmptyLabel, TEnd<Http, EmptyLabel>>;
        
        // Compose protocols
        type Composed = <MixedProtocol as TSession<Http>>::Compose<Continuation>;

        // Verify that the outermost label (L3) is preserved
        fn assert_label_is_l3<T: ExtractLabel<Http>>()
        where
            <T as ExtractLabel<Http>>::Label: Same<L3>,
        {
        }

        // This should compile successfully - verifying the extracted label is L3
        assert_label_is_l3::<Composed>();
    }

    // Edge Case 3: Complex structure with multiple label dependencies
    // Test a complex protocol structure with multiple label interactions
    #[test]
    fn test_complex_protocol_structure() {
        // Create a complex protocol with multiple branches and nested structures
        type Branch1 = TInteract<Http, L1, TClient, Message, TEnd<Http, EmptyLabel>>;
        type Branch2 = TRec<
            Http, 
            L2, 
            TInteract<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>
        >;
        
        type ComplexProtocol = TPar<
            Http,
            L3,
            Branch1,
            TChoice<
                Http,
                L2,
                Branch2,
                TInteract<Http, L1, TClient, Message, TEnd<Http, EmptyLabel>>,
            >,
            FalseB,
        >;
        
        // When composed with a continuation, the outer label should be preserved
        type Continuation = TInteract<Http, EmptyLabel, TServer, Response, TEnd<Http, EmptyLabel>>;
        type Composed = <ComplexProtocol as TSession<Http>>::Compose<Continuation>;

        // Verify that the outermost label (L3) is preserved
        fn assert_label_is_l3<T: ExtractLabel<Http>>()
        where
            <T as ExtractLabel<Http>>::Label: Same<L3>,
        {
        }

        // This should compile successfully - verifying the extracted label is L3
        assert_label_is_l3::<Composed>();
    }
}

// --- Label Test Coverage Tracking ---
#[doc(hidden)]
pub mod test_coverage {
    use super::*;

    // Trait to track which combinators have been tested with custom labels
    pub trait TestedWithCustomLabel {}

    // Mark combinators as tested as we create tests for them
    impl TestedWithCustomLabel for TEnd<Http, L1> {}
    // Mark TInteract as tested with all three custom label types
    impl TestedWithCustomLabel for TInteract<Http, L1, TClient, Message, TEnd<Http, EmptyLabel>> {}
    impl TestedWithCustomLabel for TInteract<Http, L2, TClient, Message, TEnd<Http, EmptyLabel>> {}
    impl TestedWithCustomLabel for TInteract<Http, L3, TClient, Message, TEnd<Http, EmptyLabel>> {}
    // Mark TRec as tested with all three custom label types
    impl TestedWithCustomLabel for TRec<Http, L1, TEnd<Http, EmptyLabel>> {}
    impl TestedWithCustomLabel for TRec<Http, L2, TEnd<Http, EmptyLabel>> {}
    impl TestedWithCustomLabel for TRec<Http, L3, TEnd<Http, EmptyLabel>> {}
    impl TestedWithCustomLabel for TChoice<Http, L1, TEnd<Http, EmptyLabel>, TEnd<Http, EmptyLabel>> {}
    impl TestedWithCustomLabel
        for TPar<Http, L1, TEnd<Http, EmptyLabel>, TEnd<Http, EmptyLabel>, FalseB>
    {
    }

    // Trait to track which composition operations have been tested for label preservation
    pub trait TestedLabelPreservation {}

    // Mark compositions as tested as we create tests for them
    impl TestedLabelPreservation for TInteract<Http, L1, TClient, Message, TEnd<Http, EmptyLabel>> {}
    impl TestedLabelPreservation for TInteract<Http, L2, TClient, Message, TEnd<Http, EmptyLabel>> {}
    impl TestedLabelPreservation for TInteract<Http, L3, TClient, Message, TEnd<Http, EmptyLabel>> {}
    impl TestedLabelPreservation for TRec<Http, L1, TEnd<Http, EmptyLabel>> {}
    impl TestedLabelPreservation for TRec<Http, L2, TEnd<Http, EmptyLabel>> {}
    impl TestedLabelPreservation for TRec<Http, L3, TEnd<Http, EmptyLabel>> {}
    impl TestedLabelPreservation for TChoice<Http, L1, TEnd<Http, EmptyLabel>, TEnd<Http, EmptyLabel>> {}
    impl TestedLabelPreservation
        for TPar<Http, L1, TEnd<Http, EmptyLabel>, TEnd<Http, EmptyLabel>, FalseB>
    {
    }

    // Test coverage statistics (manually updated)
    #[derive(Debug)]
    pub struct LabelTestCoverage {
        pub combinators_with_custom_labels: usize,
        pub total_combinators: usize,
        pub composition_operations_tested: usize,
        pub total_composition_operations: usize,
        pub custom_label_types_tested: usize,
        pub target_custom_label_types: usize,
        pub edge_cases_tested: usize,
        pub target_edge_cases: usize,
    }

    // Current coverage metrics
    pub const CURRENT_COVERAGE: LabelTestCoverage = LabelTestCoverage {
        combinators_with_custom_labels: 5, // TEnd, TInteract, TRec, TChoice, TPar
        total_combinators: 5,              // TEnd, TInteract, TRec, TChoice, TPar
        composition_operations_tested: 4,  // TInteract, TRec, TChoice, TPar
        total_composition_operations: 5,   // TEnd, TInteract, TRec, TChoice, TPar
        custom_label_types_tested: 5,      // TEnd with L1, TInteract with all 3, TRec with all 3, TChoice with L1, TPar with L1
        target_custom_label_types: 5,      // Each combinator should be tested with at least 1 custom label type
        edge_cases_tested: 3,              // Nested compositions, mixed combinators, complex structures
        target_edge_cases: 3,              // Nested compositions, mixed combinators, complex structures
    };

    // Calculate coverage percentages
    pub const COMBINATOR_COVERAGE_PCT: f32 = (CURRENT_COVERAGE.combinators_with_custom_labels
        as f32)
        / (CURRENT_COVERAGE.total_combinators as f32)
        * 100.0;

    pub const COMPOSITION_COVERAGE_PCT: f32 = (CURRENT_COVERAGE.composition_operations_tested
        as f32)
        / (CURRENT_COVERAGE.total_composition_operations as f32)
        * 100.0;

    pub const CUSTOM_LABEL_TYPES_PCT: f32 = (CURRENT_COVERAGE.custom_label_types_tested as f32)
        / (CURRENT_COVERAGE.target_custom_label_types as f32)
        * 100.0;

    pub const EDGE_CASES_PCT: f32 = (CURRENT_COVERAGE.edge_cases_tested as f32)
        / (CURRENT_COVERAGE.target_edge_cases as f32)
        * 100.0;

    // Print current coverage during test runs
    #[test]
    fn report_label_test_coverage() {
        println!("Label Test Coverage Report:");
        println!(
            "Combinators with custom label tests: {}/{} ({}%)",
            CURRENT_COVERAGE.combinators_with_custom_labels,
            CURRENT_COVERAGE.total_combinators,
            COMBINATOR_COVERAGE_PCT
        );
        println!(
            "Composition operations with label preservation tests: {}/{} ({}%)",
            CURRENT_COVERAGE.composition_operations_tested,
            CURRENT_COVERAGE.total_composition_operations,
            COMPOSITION_COVERAGE_PCT
        );
        println!(
            "Combinators meeting custom label type target: {}/{} ({}%)",
            CURRENT_COVERAGE.custom_label_types_tested,
            CURRENT_COVERAGE.target_custom_label_types,
            CUSTOM_LABEL_TYPES_PCT
        );
        println!(
            "Edge cases tested: {}/{} ({}%)",
            CURRENT_COVERAGE.edge_cases_tested,
            CURRENT_COVERAGE.target_edge_cases,
            EDGE_CASES_PCT
        );
    }

    // Define test types used above
    struct Http;
    struct TClient;
    impl Role for TClient {}
    struct Message;
}

// --- Assertion Macros for Label Testing ---
/// Assert that a label is preserved after composition
#[macro_export]
macro_rules! assert_label_preserved {
    ($Base:ty, $Continuation:ty, $ExpectedLabel:ty) => {
        const _: () = {
            fn assert_label_preservation()
            where
                $Base: TSession<IO>,
                $Continuation: TSession<IO>,
                <$Base as TSession<IO>>::Compose<$Continuation>: ExtractLabel<IO>,
                <<$Base as TSession<IO>>::Compose<$Continuation> as ExtractLabel<IO>>::Label:
                    Same<$ExpectedLabel>,
            {
            }
        };
    };
}

/// Assert that types have the same label type
#[macro_export]
macro_rules! assert_same_label {
    ($T1:ty, $T2:ty) => {
        const _: () = {
            fn assert_same_label()
            where
                $T1: ExtractLabel<IO>,
                $T2: ExtractLabel<IO>,
                <$T1 as ExtractLabel<IO>>::Label: Same<<$T2 as ExtractLabel<IO>>::Label>,
            {
            }
        };
    };
}
