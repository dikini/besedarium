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

## Protocol Label Invariant (2025-05-17)

**Invariant:**

- All protocol combinators (TEnd, TSend, TRecv, TChoice, TPar, TRec, etc.) must have a label parameter of type `ProtocolLabel`.
- The trait `GetProtocolLabel` must be implemented for all protocol combinators.
- This ensures that label metadata is always available for introspection, projection, and compile-time checks (e.g., uniqueness, traceability, debugging).
- All combinators must preserve and propagate label information through type-level composition and projection.

**Rationale:**

- Uniform label access enables generic tooling, macros, and compile-time assertions (e.g., `assert_unique_labels!`).
- Consistent labeling across all combinators simplifies protocol analysis, code generation, and debugging.
- This invariant must be maintained as the protocol system evolves. Any new combinator must follow this rule.

**Pattern:**

```rust
// Example: All combinators carry a label and implement GetProtocolLabel
pub trait GetProtocolLabel {
    type Label: ProtocolLabel;
}

impl<IO, Lbl: ProtocolLabel, ...> GetProtocolLabel for TEnd<IO, Lbl> { type Label = Lbl; }
impl<IO, Lbl: ProtocolLabel, ...> GetProtocolLabel for TSend<IO, Lbl, ...> { type Label = Lbl; }
impl<IO, Lbl: ProtocolLabel, ...> GetProtocolLabel for TRecv<IO, Lbl, ...> { type Label = Lbl; }
impl<IO, Lbl: ProtocolLabel, ...> GetProtocolLabel for TChoice<IO, Lbl, ...> { type Label = Lbl; }
impl<IO, Lbl: ProtocolLabel, ...> GetProtocolLabel for TPar<IO, Lbl, ...> { type Label = Lbl; }
impl<IO, Lbl: ProtocolLabel, ...> GetProtocolLabel for TRec<IO, Lbl, ...> { type Label = Lbl; }
// ...and so on for any new combinator
```

**Implications:**

- When adding, removing, or modifying combinators, always update their label parameter and `GetProtocolLabel` implementation.
- When refactoring, describe explicitly which traits, structures, or impls are affected.
- This invariant is enforced by convention, code review, and compile-time tests.

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

---

*This knowledge base distills the core patterns for implementing session types in Rust. Reference when implementing protocol-related functionality.*
