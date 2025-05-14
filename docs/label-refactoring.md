# Label Parameter Refactoring Strategy

This document outlines a comprehensive strategy for refactoring the label parameter naming across session type combinators in the Besedarium library. The goal is to standardize on using `Lbl` instead of `L` for label parameters throughout all session type combinators and related projection machinery.

## Current State Analysis

Currently, the codebase exhibits inconsistent naming conventions for label parameters:

1. **Inconsistent Parameter Naming**:
   - `TEnd<IO, L>` - Uses `L` as the label parameter
   - `TInteract<IO, L, R, H, T>` - Uses `L` as the label parameter
   - `TRec<IO, L, S>` - Uses `L` as the label parameter
   - `TChoice<IO, Lbl, L, R>` - Uses `Lbl` as the label parameter, `L` for left branch
   - `TPar<IO, Lbl, L, R, IsDisjoint>` - Uses `Lbl` as the label parameter, `L` for left branch

2. **Label Non-Preservation**:
   - Labels are not preserved when projecting from global to local types
   - Local endpoint types (`EpSend`, `EpRecv`, etc.) don't include label parameters
   - This limits traceability between global and local views of protocols

3. **Usage Patterns**:
   - Labels are primarily used for documentation and debugging purposes
   - Labels are preserved during composition operations via `TSession::Compose`
   - Labels are potentially used in introspection via the `LabelsOf` trait
   - Labels may be checked for uniqueness in certain contexts

## Comprehensive Parameter Usage Audit

This section provides a detailed audit of all label parameter usages throughout the codebase, which will inform our refactoring strategy.

### Core Type Definitions

| Type | Parameter Name | Parameter Constraint | File Location |
|------|---------------|---------------------|--------------|
| `TEnd<IO, L>` | `L` | No constraints (default: `EmptyLabel`) | `protocol.rs:64` |
| `TInteract<IO, L, R, H, T>` | `L` | `L: types::ProtocolLabel` | `protocol.rs:82` |
| `TRec<IO, L, S>` | `L` | `L: types::ProtocolLabel` | `protocol.rs:102` |
| `TChoice<IO, Lbl, L, R>` | `Lbl` | `Lbl: types::ProtocolLabel` | `protocol.rs:116` |
| `TPar<IO, Lbl, L, R, IsDisjoint>` | `Lbl` | `Lbl: types::ProtocolLabel` | `protocol.rs:148` |

### Trait Implementations

#### TSession Implementations

Each combinator implements `TSession<IO>`, preserving the label parameter in the `Compose` associated type:

```rust
// TEnd
impl<IO, L> TSession<IO> for TEnd<IO, L> {
    type Compose<Rhs: TSession<IO>> = Rhs;  // Label not preserved (end is replaced)
    const IS_EMPTY: bool = true;
}

// TInteract
impl<IO, L: types::ProtocolLabel, R, H, T: TSession<IO>> TSession<IO> 
    for TInteract<IO, L, R, H, T> 
{
    type Compose<Rhs: TSession<IO>> = TInteract<IO, L, R, H, T::Compose<Rhs>>;
    // Label L preserved ^
    const IS_EMPTY: bool = false;
}

// TRec
impl<IO, L: types::ProtocolLabel, S: TSession<IO>> TSession<IO> for TRec<IO, L, S> {
    type Compose<Rhs: TSession<IO>> = TRec<IO, L, S::Compose<Rhs>>;
    // Label L preserved ^
    const IS_EMPTY: bool = false;
}

// TChoice
impl<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>> TSession<IO> 
    for TChoice<IO, Lbl, L, R> 
{
    type Compose<Rhs: TSession<IO>> = TChoice<IO, Lbl, L::Compose<Rhs>, R::Compose<Rhs>>;
    // Label Lbl preserved ^
    const IS_EMPTY: bool = false;
}

// TPar
impl<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>, IsDisjoint> TSession<IO> 
    for TPar<IO, Lbl, L, R, IsDisjoint> 
{
    type Compose<Rhs: TSession<IO>> = TPar<IO, Lbl, L::Compose<Rhs>, R::Compose<Rhs>, IsDisjoint>;
    // Label Lbl preserved ^
    const IS_EMPTY: bool = false;
}
```

#### Default Label Usage in ToTChoice and ToTPar

When constructing `TChoice` and `TPar` from type lists using the `ToTChoice` and `ToTPar` traits, an `EmptyLabel` is used as the default label:

```rust
// In ToTChoice implementation
impl<IO, H: TSession<IO>, T: ToTChoice<IO>> ToTChoice<IO> for Cons<H, T> {
    type Output = TChoice<IO, types::EmptyLabel, H, <T as ToTChoice<IO>>::Output>;
    //                       ^^^^^^^^^^^^^^^ Default label used here
}

// In ToTPar implementation
impl<IO, H: TSession<IO>, T: ToTPar<IO>> ToTPar<IO> for Cons<H, T> {
    type Output = TPar<IO, types::EmptyLabel, H, <T as ToTPar<IO>>::Output, types::False>;
    //                    ^^^^^^^^^^^^^^^ Default label used here
}
```

### Projection Machinery

The projection machinery uses label parameters in trait bounds but does not preserve them in the resulting endpoint types:

```rust
// Base case: projecting end-of-session yields EpEnd
impl<Me, IO, L> ProjectRole<Me, IO, TEnd<IO, L>> for ()
where
    Me: Role,
{
    type Out = EpEnd<IO, Me>;  // Label L not preserved
}

// Projection for single interaction
impl<Me, IO, L, R, H, T> ProjectRole<Me, IO, TInteract<IO, L, R, H, T>> for ()
where
    Me: Role,
    L: types::ProtocolLabel,  // Label L used as a bound
    R: Role,
    T: TSession<IO>,
    Me: RoleEq<R>,
    <Me as RoleEq<R>>::Output: types::Bool,
    (): ProjectInteract<<Me as RoleEq<R>>::Output, Me, IO, R, H, T>,
{
    type Out = <() as ProjectInteract<<Me as RoleEq<R>>::Output, Me, IO, R, H, T>>::Out;
    // Label L not included in ProjectInteract call
}
```

### Introspection Usage

The `introspection.rs` module extensively uses label parameters for collecting and manipulating protocol metadata:

```rust
// RolesOf implementation for TInteract
impl<IO, L: types::ProtocolLabel, R, H, T: protocol::TSession<IO> + RolesOf> RolesOf
    for protocol::TInteract<IO, L, R, H, T>
{
    type Roles = /* ... */  // Uses label type L in constraints
}

// LabelsOf implementation for TInteract
impl<IO, L: types::ProtocolLabel, R, H, T: protocol::TSession<IO> + LabelsOf> LabelsOf
    for protocol::TInteract<IO, L, R, H, T>
{
    type Labels = Cons<L, <T as LabelsOf>::Labels>;
    //             ^^^^^^ Label L used here
}

// Similar implementations for TChoice, TPar, TRec
```

### Label Uniqueness Checking

The codebase contains utilities for checking label uniqueness at the type level:

```rust
pub trait UniqueList {}
impl UniqueList for Nil {}
impl<H, T> UniqueList for Cons<H, T> where T: NotInList<H> + UniqueList {}

// Used in assert_unique_labels! macro
#[macro_export]
macro_rules! assert_unique_labels {
    ($T:ty) => {
        const _: fn() = || {
            fn _assert_unique_labels()
            where
                <$T as $crate::LabelsOf>::Labels: $crate::UniqueList,
            {
            }
        };
    };
}
```

### Local Endpoint Types

The endpoint types representing local projections lack label parameters entirely:

```rust
pub struct EpSend<IO, R, H, T>(PhantomData<(IO, R, H, T)>);
pub struct EpRecv<IO, R, H, T>(PhantomData<(IO, R, H, T)>);
pub struct EpEnd<IO, R>(PhantomData<(IO, R)>);
pub struct EpChoice<IO, Me, L, R>(PhantomData<(IO, Me, L, R)>);
pub struct EpPar<IO, Me, L, R>(PhantomData<(IO, Me, L, R)>);
pub struct EpSkip<IO, R>(PhantomData<(IO, R)>);
```

### Macro System

Labels are used in macros that construct protocol combinations:

```rust
// In tchoice! macro from lib.rs
#[macro_export]
macro_rules! tchoice {
    ($io:ty; $($branch:ty),+ $(,)?) => {
        <tlist!($($branch),*) as ToTChoice<$io>>::Output
    };
}

// In tpar! macro from lib.rs
#[macro_export]
macro_rules! tpar {
    ($io:ty; $($branch:ty),* $(,)?) => {
        <tlist!($($branch),*) as ToTPar<$io>>::Output
    };
}
```

### Test and Example Usage

Labels are used extensively in tests and examples:

```rust
// Example from a protocol test
type Global = TInteract<
    Http,
    EmptyLabel,  // Using default empty label
    Alice,
    Message,
    TInteract<Http, EmptyLabel, Bob, Response, TEnd<Http, EmptyLabel>>
>;
```

### Impact Analysis

Based on this audit, here's the impact of the proposed refactoring:

1. **Core Type Definitions**: 3 types will need parameter renaming (`TEnd`, `TInteract`, `TRec`)
2. **Trait Implementations**: At least 7 trait implementations will need updates
3. **Projection Code**: 5+ projection-related traits and implementations will need updates
4. **Introspection Code**: 10+ introspection trait implementations will need updates
5. **Test and Examples**: Numerous test cases and examples may need updates

The most complex part will be ensuring that all interdependent traits and implementations are updated consistently, especially where label parameters are used in associated types.

## Test Suite Audit

This section analyzes the current test suite with a focus on label parameter usage and how the proposed refactoring would impact the tests.

### Test Structure Overview

The test suite is organized into several components:

1. **Protocol-Specific Tests** (`tests/protocols/`):
   - `branching.rs` - Tests for protocol branching with `TChoice`
   - `client_server_handshake.rs` - Tests for request-response patterns
   - `concurrent.rs` - Tests for parallel protocols with `TPar`
   - `pubsub.rs` - Tests for publish-subscribe patterns
   - `streaming.rs` - Tests for recursive protocols with `TRec`
   - `workflow.rs` - Tests for multi-party workflows

2. **Compile-Time Tests** (`tests/compile.rs`):
   - Type equality assertions (`assert_type_eq!`) 
   - Disjointness assertions (`assert_disjoint!`)
   - Label uniqueness tests (`assert_unique_labels!`)

3. **Compile-Fail Tests** (`tests/trybuild/`):
   - Negative tests that should fail to compile
   - Includes tests for duplicate labels, duplicate roles, etc.

### Label Usage Patterns in Tests

1. **Default Labels**:
   - The majority of tests use `EmptyLabel` as a default label parameter
   - Example: `TInteract<Http, EmptyLabel, TClient, Message, TEnd<Http, EmptyLabel>>`

2. **Custom Labels**:
   - Several tests define custom label types (e.g., `struct L1; impl ProtocolLabel for L1 {}`)
   - Used primarily in label uniqueness tests

3. **Label Uniqueness Testing**:
   - Positive test: `label_uniqueness_positive` in `compile.rs`
   - Negative test: `duplicate_labels_choice.rs` in `trybuild/`

4. **Role Types Used as Labels**:
   - Some test cases implement `ProtocolLabel` for role types:
   - `impl ProtocolLabel for TClient {}`

### Impact of Refactoring on Tests

1. **Protocol-Specific Tests**:
   - Most protocol tests use macros (`tchoice!`, `tpar!`) that introduce `EmptyLabel`
   - Direct updates needed for tests using explicit type definitions:
     - `TInteract<IO, L, R, H, T>` → `TInteract<IO, Lbl, R, H, T>`
     - `TRec<IO, L, S>` → `TRec<IO, Lbl, S>`
     - `TEnd<IO, L>` → `TEnd<IO, Lbl>`

2. **Compile-Time Tests**:
   - Numerous tests with explicit type definitions that will need updating
   - Label uniqueness tests will need special attention

3. **Compile-Fail Tests**:
   - Error messages and expected outcomes may change
   - `.stderr` files may need updating to match new error messages

### Test Coverage Analysis

1. **Well-Tested Areas**:
   - Basic label usage with `EmptyLabel`
   - Label uniqueness checking
   - Disjointness checks for parallel composition

2. **Coverage Gaps**:
   - Limited testing of custom labels beyond uniqueness checks
   - No tests explicitly verifying label preservation during composition
   - No tests for label introspection functionality
   - Limited tests for interaction between labels and projection

### Test Metrics for Label-Related Functionality

To ensure thorough testing of label-related functionality during the refactoring process, we should establish specific test metrics. These metrics will help track test coverage and identify areas that need additional testing.

#### Proposed Test Metrics

1. **Label Parameter Coverage**:
   - Percentage of session combinator types with dedicated label parameter tests
   - Target: 100% of session combinators have tests specifically verifying label behavior

2. **Label Operation Coverage**:
   - Percentage of label-related operations with dedicated tests
   - Operations include: composition, introspection, uniqueness checking, projection

3. **Custom Label Type Coverage**:
   - Number of tests using custom label types vs. `EmptyLabel`
   - Target: Each combinator should have tests with at least 2 different custom label types

4. **Label Preservation Verification**:
   - Percentage of composition operations with label preservation tests
   - Target: 100% of composition paths should verify label preservation

5. **Edge Case Coverage**:
   - Number of edge cases tested (nested compositions, complex structures, etc.)
   - Target: At least 3 edge cases for each combinator type

#### Implementation Approaches

1. **Tracking Framework**:

   ```rust
   // Example: Label Test Tracking with type-level assertions
   mod label_test_tracker {
       use super::*;
       
       // Track which combinators have been tested with custom labels
       trait TestedWithCustomLabel {}
       impl TestedWithCustomLabel for TEnd<Http, CustomLabel1> {}
       impl TestedWithCustomLabel for TInteract<Http, CustomLabel2, TClient, Message, TEnd<Http, EmptyLabel>> {}
       // ... add implementations as tests are created
       
       // Can use compile-time assertions to ensure coverage
       const _: () = {
           // This will fail to compile until TRec has been tested with a custom label
           fn assert_trec_tested_with_custom_label() where TRec<Http, CustomLabel3, TEnd<Http>>: TestedWithCustomLabel {}
       };
   }
   ```

2. **Manual Checklist**:

   Create a checklist in the test file comments:

   ```rust
   // Label Test Coverage Checklist:
   // - [x] TEnd with custom label
   // - [x] TInteract with custom label
   // - [ ] TRec with custom label
   // - [x] TChoice with custom label
   // - [ ] TPar with custom label
   // - [x] Label preservation in TEnd.Compose
   // - [ ] Label preservation in TInteract.Compose
   // ...etc
   ```

3. **Test Matrix Approach**:

   Create a test matrix that systematically combines combinators with different label types:

   ```rust
   mod label_test_matrix {
       // Define a variety of label types for testing
       struct L1; impl ProtocolLabel for L1 {}
       struct L2; impl ProtocolLabel for L2 {}
       struct L3; impl ProtocolLabel for L3 {}
       
       // Test matrix: combinators × labels
       #[test]
       fn test_tend_with_l1() {
           type T = TEnd<Http, L1>;
           // Test assertions
       }
       
       #[test]
       fn test_tend_with_l2() {
           type T = TEnd<Http, L2>;
           // Test assertions
       }
       
       // Continue with all combinations
   }
   ```

4. **Coverage Report Generation**:

   Add a test module that runs at compile time to generate coverage statistics:

   ```rust
   mod label_coverage_stats {
       use super::*;
       
       // Use type traits to track test coverage
       trait Tested {}
       
       // Mark types as tested
       impl Tested for TEnd<Http, CustomLabel> {}
       // ...other implementations
       
       // Compile-time counter
       struct Counter<const N: usize>;
       
       // Count tested combinators
       type TestedCount = /* type-level counting mechanism */;
       
       // This will output coverage percentage in compile error
       // Only for development use
       // const COVERAGE: f32 = 100.0 * (TestedCount::VALUE as f32) / (TotalCombinators::VALUE as f32);
   }
   ```

#### Example Test Implementation for Label Metrics

```rust
// Example of systematic label testing with metrics tracking
mod label_metrics_tests {
    use super::*;
    
    // Define test labels
    struct TestLabel1; impl ProtocolLabel for TestLabel1 {}
    struct TestLabel2; impl ProtocolLabel for TestLabel2 {}
    
    // Helper trait to extract label type from a session type
    trait ExtractLabel<IO> {
        type Label;
    }
    
    impl<IO, L> ExtractLabel<IO> for TEnd<IO, L> {
        type Label = L;
    }
    
    impl<IO, L, R, H, T> ExtractLabel<IO> for TInteract<IO, L, R, H, T> 
    where T: TSession<IO> {
        type Label = L;
    }
    
    // Similar implementations for other combinators
    
    // Test that verifies label preservation in composition
    #[test]
    fn test_label_preservation_in_composition() {
        // Define types with explicit labels
        type S1 = TInteract<Http, TestLabel1, TClient, Message, TEnd<Http, EmptyLabel>>;
        type S2 = TInteract<Http, TestLabel2, TServer, Response, TEnd<Http, EmptyLabel>>;
        
        // Composed type
        type Composed = <S1 as TSession<Http>>::Compose<S2>;
        
        // Verify the first label is preserved
        // This requires custom type traits to extract the label from Composed
        type PreservedLabel = <Composed as ExtractLabel<Http>>::Label;
        
        // Static assertion (you'd need a helper trait for this)
        assert_type_eq!(PreservedLabel, TestLabel1);
    }
    
    // Track coverage with a struct
    struct CoverageTracker {
        tend_custom_label: bool,
        tinteract_custom_label: bool,
        trec_custom_label: bool,
        tchoice_custom_label: bool,
        tpar_custom_label: bool,
        
        tend_composition: bool,
        tinteract_composition: bool,
        // ...and so on
    }
    
    // This could be updated manually or semi-automatically as tests are added
    const COVERAGE: CoverageTracker = CoverageTracker {
        tend_custom_label: true,
        tinteract_custom_label: true,
        trec_custom_label: false,
        // ...rest of the fields
    };
    
    // Calculate coverage percentage (manually)
    // const COVERAGE_PERCENTAGE: f32 = /* count true values */ / /* total fields */ * 100.0;
}
```

By implementing these metrics and approaches, we can ensure systematic test coverage of label-related functionality throughout the refactoring process.

### Recommended Test Additions

Before proceeding with the refactoring, the following tests should be added:

1. **Label Composition Tests**:
   - Tests verifying that labels are preserved during `Compose` operations
   - Tests for label handling in nested compositions

2. **Label Introspection Tests**:
   - Tests for the `LabelsOf` trait to ensure it correctly extracts labels
   - Tests for label collection across complex protocol structures

3. **Projection Label Tests**:
   - Tests specifically focused on label behavior during projection
   - Will serve as a baseline for potential future enhancements to preserve labels in projection

4. **Modular Test Structure**:
   - Create dedicated test modules for label-related functionality
   - Separate label uniqueness tests, label composition tests, etc.

### Example Test Cases to Add

```rust
// Test for label preservation during composition
mod label_composition_test {
    use super::*;
    struct L1; impl ProtocolLabel for L1 {}
    struct L2; impl ProtocolLabel for L2 {}
    
    type Base = TInteract<Http, L1, TClient, Message, TEnd<Http, EmptyLabel>>;
    type Continuation = TInteract<Http, L2, TServer, Response, TEnd<Http, EmptyLabel>>;
    type Composed = <Base as TSession<Http>>::Compose<Continuation>;
    
    // Verify first label is preserved
    // This would need a helper trait to extract the label at a specific position
}

// Test for label introspection
mod label_introspection_test {
    use super::*;
    struct L1; impl ProtocolLabel for L1 {}
    struct L2; impl ProtocolLabel for L2 {}
    
    type Protocol = TInteract<
        Http, 
        L1, 
        TClient, 
        Message, 
        TInteract<Http, L2, TServer, Response, TEnd<Http, EmptyLabel>>
    >;
    
    // Use LabelsOf to verify the extracted labels match the expected list
    type ExpectedLabels = Cons<L1, Cons<L2, Cons<EmptyLabel, Nil>>>;
    // Would need a way to assert type equality between <Protocol as LabelsOf>::Labels and ExpectedLabels
}

// Test focusing on label parameter transformation during refactoring
mod label_parameter_refactoring_test {
    use super::*;
    struct TestLabel; impl ProtocolLabel for TestLabel {}
    
    // Current structure
    type BeforeRefactor = TInteract<Http, TestLabel, TClient, Message, TEnd<Http, TestLabel>>;
    
    // After refactoring (to be uncommented after refactoring)
    // type AfterRefactor = TInteract<Http, TestLabel, TClient, Message, TEnd<Http, TestLabel>>;
    
    // Test that behavior remains identical
}
```

## Refactoring Scope

This refactoring would involve updating:

1. **Type Definitions**:
   - Update all global combinator type parameter names for consistency
   - Consider adding label parameters to local endpoint types

2. **Trait Implementations**:
   - Update `TSession` implementations for all combinators
   - Update `ProjectRole` and related projection traits
   - Update any introspection traits that work with labels

3. **Downstream Code**:
   - Update all tests and examples using these combinators
   - Update macros that might rely on specific parameter names
   - Update documentation references to these parameters

4. **Documentation**:
   - Update all doc comments and markdown documentation
   - Update code examples to reflect the new parameter naming

## Potential Dangers and Challenges

1. **Missing Usages**:
   - Failing to update all usages could lead to compilation errors
   - Type mismatches might occur in complex trait implementations

2. **Parameter Ambiguity**:
   - In `TChoice` and `TPar`, both `Lbl` and `L` exist but serve different purposes
   - Renaming could lead to confusion about which `L` is being referenced

3. **Macro Expansion Issues**:
   - Macros like `tchoice!` and `tpar!` might rely on specific parameter names
   - Changes could break macro expansion in subtle ways

4. **Documentation Synchronization**:
   - Keeping documentation in sync with implementation may be challenging
   - Code examples might become outdated

5. **Projection Correctness**:
   - Label-related logic in projection might be subtly affected
   - Edge cases in the projection machinery might be overlooked

6. **Breaking Changes**:
   - Public API changes might break downstream code
   - Type signatures would change, affecting any code using these types

7. **Test Regressions**:
   - Existing tests might fail or behave differently after refactoring
   - Compile-fail tests might start passing or fail with different errors

## Prerequisites for Safe Refactoring

Before beginning this refactoring, ensure:

1. **Comprehensive Test Coverage**:
   - All combinators and their interactions should be tested
   - Edge cases should be covered in the test suite
   - Tests should verify that the semantic behavior remains unchanged
   - Add the new test cases identified in the test suite audit section

2. **Parameter Usage Audit**:
   - Document all places where label parameters are used ✅
   - Identify all trait bounds and constraints on these parameters ✅ 
   - Map out dependencies between different parts of the system ✅

3. **Consistent Label Strategy**:
   - Decide on a uniform approach to label handling
   - Document the intended semantics and lifecycle of labels
   - Consider whether labels should be preserved in projection

4. **Version Management**:
   - Consider whether this is a breaking change requiring a major version bump
   - Plan for backward compatibility if needed

## Step-by-Step Implementation Plan

### Phase 1: Preparation and Analysis

1. **Create a dedicated branch** for the refactoring work

2. **Develop foundational test infrastructure**:
   - Create helper traits/utilities for comparing and extracting label types
   - Add assertion macros for label-specific testing if needed
   - Develop baseline tests for current label behavior

3. **Document the current label usage** across the codebase ✅

4. **Create a detailed mapping** of all places where labels appear ✅

5. **Establish test metrics** to measure test coverage of label-related functionality:
   - **Define coverage criteria**: Create specific metrics for what constitutes "complete" label testing
     - Example: "Each combinator must be tested with at least 3 different label types"
     - Example: "Each composition operation must verify label preservation"
   
   - **Create a tracking mechanism**: Implement one of the tracking approaches:
     - Type-level trait implementations to mark tested components
     - Manual checklist in test file documentation
     - Test matrix implementation that systematically tests combinations
     - Custom coverage report generator
   
   - **Set up baseline measurements**: Evaluate current test coverage against the defined metrics
     - Example: "Currently 2 of 5 combinators have custom label tests (40%)"
     - Example: "0 of 5 composition operations verify label preservation (0%)"
   
   - **Define coverage targets**: Set specific coverage targets for each metric
     - Example: "100% of combinators must have custom label tests"
     - Example: "At least 80% of edge cases must be tested"
   
   - **Implement test tracking**: Add code to track progress against metrics
     ```rust
     // Example tracking implementation in tests
     mod test_metrics {
         // Track test coverage with compile-time metrics
         #[derive(Debug)]
         struct LabelTestCoverage {
             combinators_with_custom_labels: usize,
             total_combinators: usize,
             // ...other metrics
         }
         
         // Updated each time a test is added
         const CURRENT_COVERAGE: LabelTestCoverage = LabelTestCoverage {
             combinators_with_custom_labels: 2,
             total_combinators: 5,
             // ...other metrics
         };
         
         // Calculate percentages
         const COMBINATOR_COVERAGE_PCT: f32 = 
             (CURRENT_COVERAGE.combinators_with_custom_labels as f32) /
             (CURRENT_COVERAGE.total_combinators as f32) * 100.0;
         
         // Use #[test] to output current coverage during test runs
         #[test]
         fn report_label_test_coverage() {
             println!("Label Test Coverage Report:");
             println!("Combinators with custom label tests: {}%", 
                     COMBINATOR_COVERAGE_PCT);
             // ...print other metrics
         }
     }
     ```

### Phase 2: Core Type Updates

#### Phase 2.1: Pre-implementation Testing for TEnd

1. **Add tests for `TEnd<IO, L>` behavior**:
   - Test label preservation in compositions
   - Test label access via introspection
   - Create tests that will verify behavior remains unchanged after parameter renaming

2. **Update `TEnd<IO, L>`** → **`TEnd<IO, Lbl>`**:
   - Update type definition
   - Update trait implementations
   - Update any direct usages
   - Run tests to verify behavior matches pre-refactor state

#### Phase 2.2: Pre-implementation Testing for TInteract

1. **Add tests for `TInteract<IO, L, R, H, T>` behavior**:
   - Test label preservation in compositions
   - Test label access via introspection
   - Test label behavior in projections
   - Create tests that will verify behavior remains unchanged after parameter renaming

2. **Update `TInteract<IO, L, R, H, T>`** → **`TInteract<IO, Lbl, R, H, T>`**:
   - Update type definition
   - Update trait implementations
   - Update projection machinery
   - Update any direct usages
   - Run tests to verify behavior matches pre-refactor state

#### Phase 2.3: Pre-implementation Testing for TRec

1. **Add tests for `TRec<IO, L, S>` behavior**:
   - Test label preservation in compositions
   - Test label access via introspection
   - Test label behavior in recursive structures
   - Create tests that will verify behavior remains unchanged after parameter renaming

2. **Update `TRec<IO, L, S>`** → **`TRec<IO, Lbl, S>`**:
   - Update type definition
   - Update trait implementations
   - Update any direct usages
   - Run tests to verify behavior matches pre-refactor state

### Phase 3: Projection and Supporting Code

#### Phase 3.1: Pre-implementation Testing for Projection Traits

1. **Add tests for projection behavior**:
   - Test how labels are handled during projection
   - Create tests that will verify behavior remains unchanged after parameter renaming
   - Test projection edge cases specifically related to label usage

2. **Update projection traits** to use the new parameter names:
   - Update `ProjectRole` implementations for all combinators
   - Update helper traits like `ProjectInteract`, `ProjectChoice`, and `ProjectPar`
   - Run tests to verify projection behavior remains unchanged

#### Phase 3.2: Pre-implementation Testing for Introspection

1. **Add tests for introspection behavior**:
   - Test `LabelsOf` trait implementation for each combinator
   - Test label collection across complex protocol structures
   - Create tests that will verify behavior remains unchanged after parameter renaming

2. **Update introspection code** that works with labels:
   - Update `LabelsOf` implementations for all combinators
   - Update any other traits that interact with labels
   - Run tests to verify introspection behavior remains unchanged

#### Phase 3.3: Update Utility Traits and Test Cases

1. **Test utility traits related to label handling**:
   - Develop tests for `UniqueList`, `NotInList`, etc.
   - Create tests that will verify behavior remains unchanged after parameter renaming

2. **Update utility traits** related to label handling:
   - Update any traits that interact with labels
   - Run tests to verify utility trait behavior remains unchanged

3. **Update test cases** to reflect the new parameter naming:
   - Update protocol-specific tests
   - Update compile-time tests
   - Run tests to verify overall system behavior remains unchanged

### Phase 4: Testing and Verification

1. **Run the full test suite** to ensure correctness:
   - Run regular tests to verify runtime behavior
   - Run compile-time tests to verify type-level behavior
   - Debug and fix any failures

2. **Add additional tests for edge cases**:
   - Test complex compositions that use multiple combinator types
   - Test boundary conditions and error cases
   - Add any missing tests identified during implementation

3. **Check for any subtle behavior changes** in edge cases:
   - Verify that all error messages are as expected
   - Check that all type-level computations produce the same results
   - Run memory and performance benchmarks if applicable

4. **Update any trybuild error output files** that may have changed:
   - Review stderr files for changes in error messages
   - Update any files where errors have changed due to parameter renaming

5. **Validate integration tests** that combine multiple refactored components:
   - Test interactions between different combinators with renamed parameters
   - Verify that composed protocols behave as expected

### Phase 5: Documentation and Examples

1. **Update doc comments** on type and trait definitions:
   - Update parameter names in documentation
   - Clarify any label-related functionality
   - Ensure documentation accurately reflects the refactored code

2. **Update markdown documentation** files:
   - Update any references to the old parameter names
   - Add explanations about the standardized label parameter naming
   - Update diagrams or visual representations

3. **Update code examples** in documentation:
   - Update all examples to use the standardized `Lbl` parameter
   - Add examples demonstrating proper label usage
   - Ensure examples accurately reflect the refactored code

4. **Update any diagrams or visual representations**:
   - Update any diagrams showing type structures
   - Update any sequence or protocol diagrams
   - Ensure all visual aids accurately reflect the refactored code

### Phase 6: Example and Doctest Verification

1. **Update examples in README and documentation**:
   - Verify each example compiles with the refactored code
   - Update examples to use the standardized naming
   - Add examples demonstrating proper label usage

2. **Run doctests to ensure correctness**:
   - Run `cargo test --doc` to verify all doctests pass
   - Update any failing doctests to use the new parameter naming
   - Add additional doctests to demonstrate label functionality

3. **Verify external examples**:
   - Update any standalone example projects
   - Test examples in tutorial documentation
   - Ensure all examples accurately reflect the refactored code

## Label Preservation Strategy

As part of this refactoring, consider addressing the current limitation of label non-preservation during projection:

1. **Option 1: Add Labels to Endpoint Types**
   - Modify endpoint types to include label parameters: `EpSend<IO, Lbl, R, H, T>`
   - Update projection machinery to preserve labels
   - Update all related code to handle the new parameter

2. **Option 2: Separate Label Tracking**
   - Maintain a separate mapping from endpoint types to their originating labels
   - Create utilities for retrieving labels associated with endpoint types
   - Avoid changing endpoint type signatures

3. **Option 3: Hybrid Approach**
   - Add optional label parameters to endpoint types
   - Make labels optional or use default empty labels
   - Gradually adopt label preservation where most valuable

## Recommendations

Based on the comprehensive parameter usage audit and test suite analysis, we recommend:

1. **Start with a Smaller Scope**: Begin by standardizing only the global combinator names without changing endpoint types
2. **Use a Phased Approach**: Implement changes one combinator at a time, verifying correctness at each step
3. **Prioritize Test Coverage**: Add the recommended tests for label behavior before making changes
4. **Follow Test-Driven Development**: Develop tests before implementation for each phase to ensure behavior preservation
5. **Implement Test Metrics**: Use the proposed test metrics approach to systematically track test coverage
6. **Consider Label Preservation**: Address the label loss during projection as a separate enhancement
7. **Document Label Semantics**: Clearly define the meaning and purpose of labels throughout the system
8. **Address Potential Conflicts**: Pay special attention to cases where `L` is used both as a label parameter and as a left branch parameter (in `TChoice` and `TPar`)
9. **Update Introspection First**: The `introspection.rs` module contains the most label-dependent code and should be refactored early in the process
10. **Create Test Helpers**: Develop helper traits or macros to make label-related testing easier
11. **Modularize Tests**: Restructure tests to isolate label-related functionality for easier testing and maintenance

## Next Steps

1. Create a dedicated branch for this work
2. Start with a comprehensive audit of current label usage (completed ✅)
3. Establish test metrics and implement test tracking infrastructure
4. Implement the test infrastructure and baseline tests for current behavior
5. Implement the recommended test cases from the test suite audit section
6. Begin with the simplest combinator (`TEnd`) as a proof of concept
7. Re-evaluate the plan after initial implementation experience

This refactoring, while extensive, will improve code readability, maintainability, and consistency. It also presents an opportunity to enhance the overall label handling strategy in Besedarium.

---

*Prepared by GitHub Copilot - May 14, 2025*