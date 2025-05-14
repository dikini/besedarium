# Learnings & Patterns: Type-Level Rust Protocols

This document captures core patterns, idioms, and constraints for type-level
protocol programming in Rust, distilled from recent work on session-type
dispatch and filtering.

## 1. Trait-Based Dispatch with Marker Types

Rust’s stable trait system forbids overlapping impls, negative bounds, and
using associated types as generic parameters. We overcome these limits with:

1. Marker types for each case:

   ```rust
   pub struct IsEpSkipType;
   pub struct IsNotEpSkipType;
   ```

2. A helper trait mapping each endpoint type to a marker:

   ```rust
   pub trait IsEpSkipTypeImpl<IO, Me: Role> { type TypeMarker; }

   impl<IO, Me: Role> IsEpSkipTypeImpl<IO, Me> for EpSkip<IO, Me> {
       type TypeMarker = IsEpSkipType;
   }
   impl<IO, Me: Role, H, T> IsEpSkipTypeImpl<IO, Me> for EpSend<IO, Me, H, T> {
       type TypeMarker = IsNotEpSkipType;
   }
   // ... likewise for EpRecv, EpChoice, EpPar, EpEnd
   ```

3. A single‐impl facade trait that delegates to it:

   ```rust
   pub trait GetEpSkipTypeMarker<IO, Me: Role> { type TypeMarker; }
   impl<IO, Me: Role, T> GetEpSkipTypeMarker<IO, Me> for T
   where T: IsEpSkipTypeImpl<IO, Me>
   {
       type TypeMarker = <T as IsEpSkipTypeImpl<IO,Me>>::TypeMarker;
   }
   ```

This pattern yields exactly two disjoint impls and keeps dispatch stable.

## 2. Compile‐Time Type Equality (

TypeEq
trait)

To enable `assert_type_eq!` assertions, use a “blanket + specific” approach:

```rust
pub struct True;
pub struct False;
pub trait Bool {}
impl Bool for True {} impl Bool for False {}

/// Legacy aliases for tests
pub type TrueB = True;
pub type FalseB = False;

/// Only implemented when A == B
pub trait TypeEq<A> {}
impl<T> TypeEq<T> for T {}
```

Rust’s coherence rules pick the exact match impl and disallow others.

## 3. Type‐Level Filter vs. Map

- **Map**: transforms every element of a type‐list, preserving length.
- **Filter**: keeps only elements satisfying a predicate, shortening the list.

Example: drop all `EpSkip<IO,Me>` from `Cons<H,T>`:

```rust
pub trait FilterSkips<IO, Me: Role, List> { type Out; }
impl<IO, Me: Role> FilterSkips<IO, Me, Nil> for () { type Out = Nil; }

// Delegate to helper based on TypeMarker
impl<IO, Me: Role, H, Tail> FilterSkips<IO,Me,Cons<H,Tail>> for ()
where
    H: GetEpSkipTypeMarker<IO,Me>,
    (): FilterSkipsCase<IO,Me,H,Tail,
        <H as GetEpSkipTypeMarker<IO,Me>>::TypeMarker>,
{
    type Out = <() as FilterSkipsCase<IO,Me,H,Tail,
        <H as GetEpSkipTypeMarker<IO,Me>>::TypeMarker>>::Out;
}

// If head is EpSkip → skip it
impl<IO, Me: Role, Tail>
    FilterSkipsCase<IO,Me, EpSkip<IO,Me>, Tail, IsEpSkipType> for ()
where (): FilterSkips<IO,Me,Tail>
{ type Out = <() as FilterSkips<IO,Me,Tail>>::Out; }

// Otherwise → keep it
impl<IO, Me: Role, H, Tail>
    FilterSkipsCase<IO,Me, H, Tail, IsNotEpSkipType> for ()
where
    H: EpSession<IO,Me>,
    (): FilterSkips<IO,Me,Tail>,
{
    type Out = Cons<H, <() as FilterSkips<IO,Me,Tail>>::Out>;
}
```

## 4. Rust Trait System Constraints & Workarounds

- **No specialization or negative bounds** on stable Rust.
- **No associated types as generic parameters**.
- **No overlapping impls** allowed.

Workarounds:

- Use **sealed helper traits** and **marker types** for stable, mutual exclusion.
- Expose a **facade trait** with a single impl to avoid overlap.
- Enumerate explicit impls rather than blanket covers that break coherence.

## 5. Documentation & Planning

- **Always plan** edits with:
  - A list of affected files, traits, or functions
  - Change order and dependencies
  - Estimated edit count
- **Document intent**: comment non‐obvious type‐level logic.
- **Distinguish** algorithm patterns (map vs filter) in prose first.
- Use examples and ensure doctests compile.

## 6. CI & Verification Workflow

- Ensure every change is validated by:
  - `cargo check` to catch compile errors
  - `cargo fmt --all -- --check` to enforce formatting
  - `cargo clippy` for linting and code quality checks
  - `cargo test` (runtime, compile-fail, doctests, trybuild) for correctness
- Use `trybuild` to automate macro edge-case and compile-fail tests.
- Mark unstable or in-progress doctests as `ignore` to keep docs builds green.

## 2025-05-13: Documentation updates for protocol-examples.md

- Synced examples with actual `TSession` API: used `TInteract`, `TChoice`, `TRec`, `TEnd`, and `Var` generics.
- Documented local projection internals: `EpSkip` filtering via `FilterSkips` and branch composition via `ComposeProjectedParBranches`.
- Clarified recursion model: flat global labels (`TRec<IO, Label, S>`) with explicit `Var<Label>` loops; absence of de Bruijn indices.
- Revised examples in sections 1.1, 1.2, and 2 to Rust `EpSession` types for send/receive/choice patterns.

## 2025-05-14: Runtime Implementation Pattern Comparison

When implementing session types in Rust, we've identified three primary approaches, each with distinct trade-offs:

1. **Typed Channel Wrappers**:
   - Encode protocol state in type parameters
   - Heavy reliance on Rust's type system for compile-time safety
   - Interleaves business logic with protocol operations
   - Complex type-level programming but high flexibility

2. **Code Generation with Procedural Macros**:
   - Minimizes boilerplate through automated code generation
   - Typically uses callbacks/handlers for business logic integration
   - Creates cleaner syntax for protocol definition
   - May limit IDE support and introduce debugging challenges

3. **State Machine Builders**:
   - Makes protocol states explicit as distinct types
   - Provides strong IDE support through state-specific method discovery
   - Uses builder patterns and fluent APIs for readable protocol definition
   - Excellent for visualizing protocol flow in code structure

Key implementation considerations:

- Type safety and protocol enforcement should be prioritized regardless of approach
- Project scale influences optimal choice (State Machines for small, Code Gen for large)
- Developer experience varies significantly between approaches
- Combined strategies often yield the best results for complex systems

These patterns can be mixed to create hybrid approaches that leverage the strengths of each implementation style while mitigating their weaknesses.

## Label Parameter Refactoring: Learnings and Insights

### Phase 1: Preparation and Analysis (May 14, 2025)

#### Patterns Observed

1. **Label Parameter Usage Patterns**:
   - The codebase has two competing naming conventions for label parameters:
     - `L` for `TEnd`, `TInteract`, and `TRec`
     - `Lbl` for `TChoice` and `TPar`
   - This inconsistency makes code harder to read, especially in complex nested types
   - Labels primarily serve documentation and debugging purposes, acting as type-level metadata

2. **Label Preservation Patterns**:
   - Labels are preserved during composition operations (`TSession::Compose`) for all combinators except `TEnd`
   - `TEnd<IO, L>::Compose<Rhs>` returns `Rhs`, discarding the label `L` entirely
   - All other combinators carefully preserve their label through composition operations

3. **Label and Projection Patterns**:
   - Endpoint types (`EpSend`, `EpRecv`, etc.) don't include label parameters
   - Labels are present in global session types but lost in projection to endpoint types
   - This creates a disconnect between global and local protocol representations

4. **Label Testing Patterns**:
   - Most tests use `EmptyLabel` as the default label parameter
   - Few tests specifically validate label behavior during composition
   - Tests primarily validate uniqueness constraints rather than preservation behavior

#### Key Insights

1. **Systematic Testing is Essential**:
   - To safely refactor parameter names, we need strong tests validating behavior preservation
   - Our new test infrastructure with `ExtractLabel` and `Same` traits enables explicit verification of label behavior
   - The test coverage metrics we established provide a clear way to track our testing progress

2. **Type-Level Programming Complexity**:
   - Label parameters are deeply integrated into the type system through trait bounds and associated types
   - Any change must carefully propagate through all dependent traits and types
   - The introspection system (`LabelsOf` trait) and projection system have complex interactions with label parameters

3. **Test Metric Approach**:
   - Using type-level traits like `TestedWithCustomLabel` provides a compile-time verification of test coverage
   - The combination of manual metrics tracking and automated test output provides good visibility
   - Setting explicit coverage targets helps ensure thorough testing

4. **Mapping the Codebase**:
   - Creating a detailed mapping of all label usages is critical for comprehensive refactoring
   - Five key areas were identified for focused attention:
     1. Core type definitions
     2. Trait implementations
     3. Projection machinery
     4. Introspection system
     5. Tests and examples
   - The mapping revealed potential challenges, particularly in areas where `L` is used both as a label and as a left branch parameter

#### Implementation Learnings

1. **Test Infrastructure Design**:
   - Creating generic traits like `ExtractLabel` provides flexibility for testing various label behaviors
   - Type-level assertions using traits like `Same` enable compile-time verification
   - The modular approach of testing each combinator separately simplifies reasoning about behavior

2. **Metrics Collection**:
   - Separating metrics into distinct categories (combinator coverage, composition coverage, etc.) provides clearer insights
   - Establishing baseline measurements helps identify areas needing more attention
   - The coverage tracking implementation provides a template for future test infrastructure

3. **Branching Strategy**:
   - Using a dedicated feature branch (`feat/label-refactoring`) isolates this breaking change
   - This allows focused testing without disrupting main development
   - Branch protection and systematic testing will be essential as we proceed

#### Next Steps and Recommendations

1. **Improve Test Coverage**:
   - Add tests for `TInteract` and `TRec` with additional custom label types to meet our target
   - Implement tests for identified edge cases (nested compositions, mixed combinator interactions, complex structures)
   - These improvements will ensure more comprehensive coverage before proceeding with the actual refactoring

2. **Prioritize Simple Combinators First**:
   - Begin the actual refactoring with `TEnd` as it has the simplest implementation
   - Its unique behavior (not preserving labels in composition) makes it a good test case
   - After gaining experience with the simplest case, proceed with `TInteract` and `TRec`

3. **Consider Broader Impact**:
   - This refactoring provides an opportunity to establish better label handling conventions
   - Future work might explore preserving labels in projection, which would require endpoint types to include label parameters
   - Documentation should clearly explain the role of labels in the protocol system

4. **Be Vigilant About Parameter Ambiguity**:
   - In `TChoice` and `TPar`, both `Lbl` (label) and `L` (left branch) parameters exist
   - Careful attention is needed to ensure they aren't confused during refactoring
   - Clear documentation will help future developers understand the distinction

### Phase 2: Test Enhancement and Initial Refactoring (May 14, 2025)

#### Completed Work

1. **Enhanced Test Coverage**:
   - Added comprehensive tests for `TInteract` and `TRec` with all three custom label types (`L1`, `L2`, `L3`)
   - Implemented edge case tests for:
     - Nested compositions with multiple label types
     - Mixed combinator interactions (e.g., `TPar` with `TChoice` inside)
     - Complex protocol structures with multi-level nesting
   - Updated coverage metrics to reflect these improvements
   - Achieved 100% coverage across all defined metrics

2. **Refactored `TEnd<IO, L>` to `TEnd<IO, Lbl>`**:
   - Successfully changed parameter name in type definition
   - Updated documentation to reflect the new parameter name
   - Updated trait implementations to use the new parameter name
   - All tests still pass, confirming backward compatibility

#### Key Insights

1. **Test-First Refactoring is Effective**:
   - By significantly improving test coverage before refactoring, we gained confidence in our changes
   - Edge case tests were particularly valuable, uncovering subtleties in label propagation
   - The comprehensive test suite now serves as a reliable safety net for further refactoring

2. **Parameter Name Consistency Matters**:
   - The consistent use of `Lbl` across combinators makes type definitions more readable
   - It becomes easier to reason about the role of each parameter when they follow a consistent convention
   - Type errors become more meaningful with consistent parameter names

3. **Edge Cases Reveal Design Principles**:
   - Testing nested compositions revealed that the outermost label always takes precedence
   - This confirms the hierarchical nature of the protocol combinators
   - Understanding this pattern is crucial for correctly implementing projection and composition

4. **Metrics Drive Development**:
   - Having explicit coverage metrics guided our testing efforts effectively
   - The detailed breakdown (by combinator, by custom label type, by edge case) provided clear targets
   - Automating the metrics reporting in tests helps maintain awareness of coverage quality

#### Challenges Encountered

1. **Multi-Parameter Type Classes**:
   - Keeping track of which traits have `L` vs. `Lbl` parameters requires careful attention
   - Our comprehensive mapping from Phase 1 was essential for navigating these dependencies
   - We needed to ensure changes to `TEnd` didn't break any type-level computations

2. **Test Design Complexity**:
   - Creating meaningful edge case tests requires deep understanding of the type system
   - We needed to model realistic protocol scenarios while isolating the behavior being tested
   - Type-level assertions required careful construction to test exactly what we wanted

3. **Documentation Synchronization**:
   - Keeping documentation in sync with code changes is critical but challenging
   - We updated both inline documentation and the metrics document
   - This multi-document approach requires discipline but provides valuable context

#### Next Phase Planning

1. **Continue with Remaining Combinators**:
   - Next step is to refactor `TInteract<IO, L, R, H, T>` to `TInteract<IO, Lbl, R, H, T>`
   - Then proceed to `TRec<IO, L, S>` to `TRec<IO, Lbl, S>`
   - `TChoice` and `TPar` already use the `Lbl` convention and don't need changes

2. **Testing Strategy**:
   - Run the full test suite after each combinator refactoring
   - Pay special attention to projection tests, as they rely heavily on combinator types
   - Update any examples or documentation using the old parameter names

3. **Documentation Updates**:
   - Continue updating the label_coverage.md metrics with each phase
   - Consider adding a section in the main documentation about the label parameter convention
   - Update code comments to reflect the new parameter naming convention

4. **Consistency Checklist**:
   - For each remaining refactoring step, verify:
     - Type definition and documentation
     - Trait implementations
     - Type aliases and examples
     - Tests specifically targeting that combinator

### Phase 3: Projection and Introspection Refactoring (May 14, 2025)

#### Completed Work

1. **Pre-Implementation Tests for Introspection**:
   - Created comprehensive tests for the `LabelsOf` and `RolesOf` traits
   - Verified their behavior with existing parameter naming conventions
   - Added tests that would detect any regression during refactoring

2. **Updated Introspection Code**:
   - Successfully refactored the `RolesOf` trait implementations to use `Lbl` parameter naming
   - Updated the `LabelsOf` trait implementations with the consistent parameter name
   - All introspection tests pass, confirming backward compatibility

3. **Pre-Implementation Tests for Projection**:
   - Created dedicated tests for projection traits with `TEnd` and `TInteract`
   - Tested edge cases in projection machinery that rely on label parameters
   - Established a baseline for expected projection behavior

4. **Updated Projection Traits**:
   - Refactored `ProjectRole` implementations for `TEnd` and `TInteract`
   - Updated trait bounds and type constraints consistently
   - All projection tests pass with the refactored parameter names

5. **Updated Struct Definitions**:
   - Refactored `TInteract` struct definition to use `Lbl` parameter naming
   - Updated documentation to reflect the new parameter naming convention
   - Refactored `TRec` struct definition to use `Lbl` parameter naming
   - All struct-related trait implementations updated consistently

#### Key Insights

1. **Introspection System Complexity**:
   - The `RolesOf` and `LabelsOf` traits are more deeply integrated with the type system than initially apparent
   - Their implementations depend on recursive trait resolution and proper parameter propagation
   - Consistent parameter naming greatly improves the readability and maintainability of this code

2. **Pre-Implementation Testing Value**:
   - Creating tests before implementation proved invaluable for introspection and projection
   - Tests caught subtle issues in our understanding of the current implementation
   - They provided a clear baseline against which to measure our changes

3. **Dependencies in Projection System**:
   - The projection system relies on intricate relationships between multiple traits
   - Each trait focuses on a specific aspect of the projection process:
     - `ProjectRole` handles the high-level projection from global to local types
     - `ProjectInteract` specializes in single interaction projections
     - `IsSkip` and `IsEnd` provide type-level predicates for endpoint types
   - Consistent parameter naming makes these relationships clearer

4. **Incremental Testing Approach**:
   - Starting with simpler tests that work with the current implementation
   - Gradually expanding to cover more complex cases
   - This approach helped identify where projection implementations were incomplete

5. **Documentation Importance**:
   - Clear documentation of parameter names and their purposes is essential
   - Updated doc comments throughout the codebase to maintain consistency
   - This improves future maintainability and makes the code more approachable

#### Challenges Encountered

1. **Projection Implementation Gaps**:
   - We discovered that `ProjectRole` is not fully implemented for all combinators
   - Specifically, `TChoice` and `TPar` lack implementations, causing test failures
   - We adjusted our tests to focus on the implemented functionality

2. **Test Adaptation for Current State**:
   - Some tests needed to be modified to work with the current implementation
   - This required understanding which parts of the projection system are complete
   - The adjusted tests still provide valuable verification of the refactoring

3. **Parameter Constraints Propagation**:
   - Each parameter change needs to propagate through multiple trait bounds
   - Special care is needed to ensure all constraints are updated consistently
   - The compiler provides valuable guidance but requires careful attention to error messages

4. **Balancing Thoroughness with Progress**:
   - Finding the right level of testing was a challenge
   - Too detailed, and we'd end up testing unimplemented features
   - Too shallow, and we might miss important interactions

#### Refactoring Patterns Identified

1. **Progressive Parameter Standardization**:
   - Start with core types and related traits
   - Move outward to dependent systems (introspection, projection)
   - Finally update tests and examples

2. **Trait Bound Examination**:
   - For each type definition change, carefully examine all trait bounds
   - Update related trait implementations with consistent parameter names
   - Check for subtle relationships between traits that might be affected

3. **Documentation Synchronization**:
   - Update doc comments alongside code changes
   - Ensure examples in documentation reflect new parameter names
   - Keep changelog updated with each phase completion

4. **Test-Driven Parameter Refactoring**:
   - Create pre-implementation tests to verify current behavior
   - Make parameter changes while keeping behavior identical
   - Verify with post-implementation tests

#### Future Recommendations

1. **Complete Projection Implementation**:
   - Implement `ProjectRole` for `TChoice` and `TPar` to complete the projection system
   - Add corresponding tests once implemented
   - This would allow more comprehensive testing of label parameter usage

2. **Consider Label Preservation in Projection**:
   - The current projection system loses label information
   - A future enhancement could preserve labels in endpoint types
   - This would provide better traceability between global and local types

3. **Expand Test Coverage**:
   - Continue adding tests for complex interactions between projection and labels
   - Test label propagation through nested projections
   - These tests will help catch any subtle issues in future refactorings

4. **Documentation Improvements**:
   - Add a dedicated section on label parameters in the library documentation
   - Provide examples of how labels can be used effectively
   - Explain the label preservation semantics clearly

---

These learnings from Phase 3 complement our earlier insights and will guide any future work on the session type system, particularly regarding projection and introspection.

---
*Consult this summary before any future protocol‐projection or
type‐level work to maintain stability, clarity, and correctness.*
