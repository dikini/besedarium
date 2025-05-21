# Besedarium: Type-Level Protocol Programming Patterns

This document distills essential patterns for implementing type-level session types in Rust
without relying on unstable features.

## Core Type-Level Programming Patterns

### Pattern: Marker Type Dispatch

**Problem:** Rust lacks specialization, making it difficult to provide different implementations based on specific types.

**Solution:** Use marker types to represent cases and delegate implementations:

```rust
// Define marker types
pub struct IsEpSkipType;
pub struct IsNotEpSkipType;

// Helper trait mapping types to markers
pub trait IsEpSkipTypeImpl<IO, Me: Role> { type TypeMarker; }
impl<IO, Me: Role> IsEpSkipTypeImpl<IO, Me> for EpSkip<IO, Me> { 
    type TypeMarker = IsEpSkipType; 
}

// Facade trait with single implementation
pub trait GetEpSkipTypeMarker<IO, Me: Role> { type TypeMarker; }
impl<IO, Me: Role, T> GetEpSkipTypeMarker<IO, Me> for T
where T: IsEpSkipTypeImpl<IO, Me>
{
    type TypeMarker = <T as IsEpSkipTypeImpl<IO,Me>>::TypeMarker;
}
```

**When to use:** For protocol combinators that need different behavior based on endpoint types.

### Pattern: Helper Trait Case Analysis

**Problem:** Protocol projection requires different behavior based on multiple computed properties.

**Solution:** Create helper traits with specialized implementations for each case combination:

```rust
// Main trait delegates to case-specific helper
impl<Me, IO, Lbl, L, R> ProjectRole<Me, IO, TChoice<IO, Lbl, L, R>> for ()
where
    (): ProjectChoiceCase<
        Me, IO, L, R,
        <L as ContainsRole<Me>>::Output, // Concrete type parameters 
        <R as ContainsRole<Me>>::Output  // prevent implementation conflicts
    >,
{
    type Out = <() as ProjectChoiceCase</*...*/>::Out;
}

// Implementations for each distinct case
impl<Me, IO, L, R> ProjectChoiceCase<Me, IO, L, R, types::True, types::True> for () {
    // Case: Role appears in both branches
    type Out = EpChoice</*...*/>;
}
```

**When to use:** For protocol projection and transformations with complex case analysis.

### Pattern: Recursive Type Structure Traversal

**Problem:** Protocol types have nested, recursive structures that must be analyzed.

**Solution:** Use recursive trait implementations with proper base cases:

```rust
pub trait ContainsRole<R> {
    type Output: types::Bool;
}

// Base case: Role not in TEnd
impl<IO, Lbl, R> ContainsRole<R> for TEnd<IO, Lbl> {
    type Output = types::False;
}

// Recursive case: Check current role and continue traversal
impl<IO, Lbl, H, T, R1, R2> ContainsRole<R2> for TInteract<IO, Lbl, R1, H, T> {
    // Combine results with boolean operations
    type Output = types::Or<
        <R1 as RoleEq<R2>>::Output, 
        <T as ContainsRole<R2>>::Output
    >;
}
```

**When to use:** For analyzing complex protocol structures like role presence, label uniqueness, or type equality.

### Pattern: Type-Level Boolean Logic

**Problem:** Protocol analysis requires combining multiple type-level conditions.

**Solution:** Implement boolean operators as traits with associated types:

```rust
// Boolean OR operator
pub trait BoolOr<B> { type Output: Bool; }

impl BoolOr<True> for True { type Output = True; }
impl BoolOr<False> for True { type Output = True; }
impl BoolOr<True> for False { type Output = True; }
impl BoolOr<False> for False { type Output = False; }

// Type-level function for convenience
pub type Or<A, B> = <A as BoolOr<B>>::Output;
```

**When to use:** For complex conditions in protocol safety checks and transformations.

## Protocol-Specific Patterns

### Pattern: Global-to-Local Projection

**Problem:** Converting a global protocol (choreography) to local endpoint behavior.

**Solution:** Use role-based projection with specialized handling based on endpoint involvement:

```rust
// Core projection trait
pub trait ProjectRole<Me, IO, S: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}

// TInteract projection changes based on whether endpoint is sender, receiver, or not involved
impl<Me, IO, Lbl, R1, H, T> ProjectRole<Me, IO, TInteract<IO, Lbl, R1, H, T>> for ()
where
    // Role equality check determines projection behavior
    R1: RoleEq<Me>,
    T: TSession<IO>,
    (): ProjectInteract<Me, IO, Lbl, R1, H, T, <R1 as RoleEq<Me>>::Output>,
{
    // Delegate to specialized helper based on role equality
    type Out = <() as ProjectInteract<Me, IO, Lbl, R1, H, T, <R1 as RoleEq<Me>>::Output>>::Out;
}
```

**When to use:** For implementing endpoint view derivation from choreographies.

### Pattern: Protocol Composition

**Problem:** Safely combining protocol fragments while preserving session guarantees.

**Solution:** Define composition operations with safety checks:

```rust
// Protocol composition with continuation
pub trait Compose<S: TSession<IO>> {
    type Output: TSession<IO>;
}

// Implementation with safety checks and constraints
impl<IO, L, R, T> Compose<T> for TChoice<IO, L, R>
where
    // Ensure labels are unique in the composition
    T: TSession<IO> + LabelsOf,
    Self: LabelsOf,
    <Self as LabelsOf>::Labels: DisjointFrom<<T as LabelsOf>::Labels>,
{
    type Output = /* Composed protocol */;
}
```

**When to use:** For building complex protocols from simpler building blocks.

## Protocol Label Invariant

**Invariant:**

- All protocol combinators (TSend, TRecv, TEnd, TChoice, TPar, TRec, TInteract, etc.) must have a label parameter and implement the GetProtocolLabel trait.
- The invariant must be enforced in both code and documentation. This includes:
  - Type definitions: All combinators must have a label parameter.
  - Trait implementations: All combinators must implement GetProtocolLabel.
  - Documentation: The invariant must be stated in module-level and trait-level docs, and code examples must use combinators with label parameters.
- The review process involves:
  1. Auditing all combinators for label and trait coverage.
  2. Adding/correcting missing label parameters or trait implementations.
  3. Updating documentation and code examples to reflect the invariant.
  4. Running all tests and checks to ensure correctness and no regressions.
- Rationale: This invariant ensures that protocol labels are always available for type-level reasoning, protocol projection, and endpoint generation. It also improves maintainability and future extensibility.
- Pattern: When adding a new protocol combinator, always include a label parameter and implement GetProtocolLabel. Document this requirement in both code and project planning.
- Implications: Contributors must be aware of this invariant and check for it during code review. Automated tests and linting should be used to catch violations.

## Project Architecture Insights

### Layer-Based Protocol System

The protocol system follows a layered architecture:

1. **Base Layer** (`base.rs`): Type-level programming foundations
2. **Global Layer** (`global.rs`): Multi-party choreography types
3. **Local Layer** (`local.rs`): Endpoint behavior types
4. **Transforms Layer** (`transforms.rs`): Projection machinery
5. **Utils Layer** (`utils.rs`): General helpers and type operations

This separation enables independent evolution of protocol components while maintaining a coherent system.

### Rust Trait System Constraints

Key limitations in stable Rust that affect protocol implementation:

1. **No specialization**: Cannot provide specialized implementations for subsets
2. **No negative bounds**: Cannot constrain generics by what they are not
3. **No associated types as generic parameters**: Types must be direct
4. **No overlapping impls**: Must have disjoint implementation sets

### Runtime Implementation Approaches

Three proven approaches for implementing session types at runtime:

1. **Typed Channel Wrappers**: Protocol state encoded in type parameters
2. **Code Generation**: Using procedural macros for generating boilerplate
3. **State Machine Builders**: Explicitly modeling protocol states as types

## Critical Insights

1. **Type-Level Dispatch** is fundamental for handling different protocol cases without specialization.

2. **Helper Traits** resolve implementation conflicts through indirection.

3. **Recursive Type Traversal** requires careful handling of base cases and conditionals.

4. **Edge Case Testing** reveals subtle protocol implementation issues before they become problems.

5. **Protocol Projection** decisions must account for role presence in multiple communication paths.

6. **Compositional Design** with small, focused traits improves modularity and evolution.

7. **Type Safety at Compile Time** is achievable through proper trait bounds and assertions.

## Documentation Tooling

1. **Markdown Linting** with markdownlint-cli2 using a standardized `.markdownlint-cli2.yaml` configuration file ensures consistent documentation formats.

2. **Line Length Standards** set to 100 characters provide a balance between readability and efficient use of screen space.

3. **List Formatting Rules** require proper indentation (2 spaces for top-level) and blank lines before and after lists.

## Doctest/Test Failure Lessons (2025-05-18)

- Rust doctests do not have access to crate macros unless explicitly exported and imported; integration tests do.
- Type-level equality assertions (e.g., `assert_type_eq!`) may fail in doctests due to Rust's type identity limitations, even if types are structurally identical.
- Macro-based protocol definitions (`tchoice!`, `tpar!`) should be tested in integration/compile-time tests, not doctests.
- To avoid CI failures, README.md inclusion is now limited to docs.rs builds, and a warning is present in the README.
- Always document these limitations for users and contributors.

## Learnings

### Protocol Transform Modularization (2025-05-18)

- Modularizing protocol transformation logic improves maintainability, discoverability, and testability.
- Rust stable type-level programming requires careful trait and module organization to avoid orphan rules and import conflicts.
- Doctests must use public re-export paths (e.g., `besedarium::ProjectChoice`) to work for both users and CI.
- Removing unused imports and running `cargo fmt`, `clippy`, and doctests is essential for a clean, CI-ready codebase.
- Documentation should always reflect the public API and module structure, with clear examples and trait explanations.
- When splitting large files, update all references and re-exports to avoid breakage in dependent modules and tests.
- Use `pub use` in `mod.rs` to provide a stable, discoverable API surface for downstream users.
- Maintain a running changelog and status file to track progress and ensure nothing is missed during large refactors.

### Implementing Helper Traits for Projection (2025-05-19)

- Implementing helper traits like `ProjectSendCase` and `ProjectRecvCase` in separate modules improves code organization and maintainability.
- When implementing these traits, proper type bounds are critical - especially ensuring `IO: SessionType` and the role-specific requirements.
- Using type-level booleans (`True`/`False`) for dispatch based on role equality is an effective pattern for handling different projection cases.
- The implementation follows the "Helper Trait Case Analysis" pattern where:
  1. The main trait (`ProjectRole`) delegates to case-specific helpers (`ProjectSendCase`, `ProjectRecvCase`)
  2. The helpers handle different cases based on type-level boolean flags (`Me == RSender` vs `Me != RSender`)
  3. Each implementation provides a different local protocol type based on role involvement
- Trait bounds in the implementation must be carefully managed:
  1. Need to explicitly declare `(): ProjectRole<Me, IO, G>` to ensure the unit type implements the recursive projection
  2. Need to include `IO: SessionType` in all implementations to maintain consistent bounds
  3. Properly reference external bounds from other modules using full paths when needed
- Code reuse across modules requires careful attention to imports and visibility
- Mirror implementations between `send.rs` and `recv.rs` help maintain consistency and symmetry
- In `ProjectRole` implementations, we need to:
  1. Include bounds for role equality: `Me: RoleEq<RSender>` and `<Me as RoleEq<RSender>>::Output: Bool`
  2. Add bounds for the helper traits: `(): ProjectSendCase<Me, IO, Lbl, RSender, P, G, <Me as RoleEq<RSender>>::Output>`
- Compiler errors provide valuable guidance for fixing trait bounds, especially the `consider extending the where clause` hints.
- When testing modularized code, `cargo check --lib` is useful for isolating library compilation from test cases.

### Test Overrides Update (2025-05-19)

- When modularizing a project, it's important to update test files that contain special case implementations
- Special case test implementations often need explicit trait bounds that weren't required before modularization
- For test types like `Http`, `Alice`, etc., we need to implement traits like `SessionType` to satisfy bounds in the new modularized structure
- After modularization, imports should reference specific module paths rather than wildcard imports (e.g., `use crate::protocol::transforms::projection::ProjectRole` instead of `use crate::protocol::transforms::*`)
- The compiler provides valuable diagnostic information about missing trait implementations which can guide the update process
- Test types defined in test files need the same trait implementations as their production counterparts to ensure type safety
- Even when the structure of implementations doesn't change, the trait bounds and import paths need to be updated to match the new module organization

### Protocol Combinator Implementation (2025-05-19)

- When adding a new protocol combinator, follow these key steps:
  1. Define the struct in `global.rs` with proper generic parameters and PhantomData
  2. Implement `sealed::Sealed` to restrict trait implementations
  3. Implement `TSession<IO>` with proper composition rules
  4. Implement `GetProtocolLabel` to adhere to the protocol label invariant
  5. Implement `ProjectRole` for projection to local protocols
- All combinators must adhere to the protocol label invariant by having a label parameter and implementing GetProtocolLabel
- For projection, consider how the new combinator should translate to local protocols
- Maintain consistent pattern with existing combinators (e.g., TEnd, TSend, TRecv, TChoice)
- Carefully define composition rules in the TSession implementation to ensure proper protocol composition
- Add comprehensive documentation and test cases for the new combinator
- Update integration tests to showcase the new combinator in real protocol scenarios
- Consider the impact on existing code and maintain backward compatibility where possible

### TStart Implementation (2025-05-19)

- The `TStart` combinator provides an explicit entry point for protocols:
  1. Defined `TStart<IO, Lbl, S>` in `global.rs` with label parameter and continuation
  2. Created corresponding `EpStart<IO, Lbl, Me, T>` in `local.rs` for local protocols
  3. Implemented `ProjectRole` to correctly project global `TStart` to local `EpStart`
  4. Added label preservation via `GetProtocolLabel` implementation
- Defined composition behavior: `TStart<IO, Lbl, S>::Compose<Rhs> = TStart<IO, Lbl, S::Compose<Rhs>>`
- Added comprehensive tests:
  1. Basic type construction tests
  2. Projection correctness tests for multiple roles
  3. Label preservation tests
  4. Composition tests with other protocol combinators
- Updated docs and examples to showcase `TStart` usage in practical protocol definitions
- Protocol entry points improve protocol clarity and provide a consistent structure
- The architecture follows the protocol label invariant, ensuring all type information is preserved during transformations
- Benefits of explicit protocol entry points:
  1. Clear delineation of protocol boundaries
  2. Improved protocol readability and maintainability
  3. Consistent structure for protocol definitions
  4. Simplified debugging and error messages
  5. Better support for protocol composition and reuse

---

*This knowledge base distills the core patterns for implementing session types in Rust. Reference when implementing protocol-related functionality*

## Duality and Well-Formedness in MPST (May 2025)

### Key Concepts and Patterns

- **Duality and Well-Formedness**
  - Duality ensures that for every communication action (send/receive, offer/choice) between two roles, the actions are complementary.
  - Well-formedness requires that all pairs of communicating roles are duals for their shared actions, preventing mismatches and deadlocks.

- **Global (T*) vs Local (Ep*) Types**
  - Global types (T*) describe the protocol as a whole; local types (Ep*) are projections for each role.
  - Consistent naming and structuring clarify protocol intent and implementation.

- **Type-Level Programming Patterns in Rust**
  - Use marker types and PhantomData to encode protocol structure at the type level.
  - Traits such as `TSession`, `Project`, `Dual`, `IsDual`, and `IsWellFormed` enable compile-time protocol verification.
  - Boolean logic and recursive trait implementations are used for type-level checks.
  - Compile-time assertions (e.g., `assert_type_eq!`) help enforce protocol invariants.

- **Pairwise Duality Checking**
  - For each communicating pair, filter projections to shared actions and check duality in lockstep.
  - Algorithmic matching ensures that every send has a matching receive, and every offer has a matching choice.

- **Example-Driven Documentation**
  - Concrete Rust-style examples for two- and three-role protocols clarify both well-formed and ill-formed cases.
  - Examples illustrate projection, duality, and well-formedness checks.

- **Type-Level Well-Formedness Expression**
  - `IsWellFormed<G>` trait aggregates pairwise duality checks for all communicating pairs.
  - Uses type-level sets, filtering, and aggregation to ensure protocol safety at compile time.

### Patterns and Insights

- Prefer explicit, minimal struct definitions for each protocol action.
- Use trait-based type-level functions for protocol analysis and verification.
- Document invariants and requirements for each protocol construct.
- Maintain clear separation between global and local protocol representations.
- Use lockstep filtering and pairwise checks for robust well-formedness analysis.
- Example-driven explanations improve clarity and correctness.

---

_Last updated: 2025-05-20_
