# Learnings & Patterns: Type-Level Rust Protocols

This document captures core patterns, idioms, and constraints for type-level
protocol programming in Rust, distilled from work on session types implementation.

## 1. Type-Level Programming Patterns

### Type-Level Dispatch Patterns

#### Marker Type Dispatch

```rust
// 1. Define marker types for each case
pub struct IsEpSkipType;
pub struct IsNotEpSkipType;

// 2. Helper trait mapping types to markers
pub trait IsEpSkipTypeImpl<IO, Me: Role> { type TypeMarker; }

impl<IO, Me: Role> IsEpSkipTypeImpl<IO, Me> for EpSkip<IO, Me> {
    type TypeMarker = IsEpSkipType;
}
impl<IO, Me: Role, H, T> IsEpSkipTypeImpl<IO, Me> for EpSend<IO, Me, H, T> {
    type TypeMarker = IsNotEpSkipType;
}
// ... other types

// 3. Single-impl facade trait that delegates
pub trait GetEpSkipTypeMarker<IO, Me: Role> { type TypeMarker; }
impl<IO, Me: Role, T> GetEpSkipTypeMarker<IO, Me> for T
where T: IsEpSkipTypeImpl<IO, Me>
{
    type TypeMarker = <T as IsEpSkipTypeImpl<IO,Me>>::TypeMarker;
}
```

This pattern enables non-overlapping trait implementations where naive bounds would conflict.

#### Helper Trait Specialization

When trait implementations would overlap, use helper traits with concrete type parameters:

```rust
// Main trait delegates to specialized helper using concrete types
impl<Me, IO, Lbl, L, R> ProjectRole<Me, IO, TChoice<IO, Lbl, L, R>> for ()
where
    // ...constraints...
    (): ProjectChoiceCase<
        Me, IO, L, R,
        <L as ContainsRole<Me>>::Output, // Concrete type parameter
        <R as ContainsRole<Me>>::Output  // Concrete type parameter
    >,
{
    type Out = <() as ProjectChoiceCase</*...*/>::Out;
}

// Helper trait with specialized implementations for each case
pub trait ProjectChoiceCase<Me, IO, L: TSession<IO>, R: TSession<IO>, LContainsMe, RContainsMe> {
    type Out: EpSession<IO, Me>;
}

// Non-overlapping implementations
impl<Me, IO, L, R> ProjectChoiceCase<Me, IO, L, R, types::True, types::True> for () {
    // Both branches contain role
    type Out = EpChoice</*...*/>;
}

impl<Me, IO, L, R> ProjectChoiceCase<Me, IO, L, R, types::True, types::False> for () {
    // Only left branch contains role
    type Out = <() as ProjectRole<Me, IO, L>>::Out;
}

// Additional non-overlapping cases...
```

This approach:

- Avoids trait implementation conflicts
- Provides precise control over implementation selection
- Improves maintainability by separating distinct cases
- Enables type-level dispatch without specialization or negative bounds

### Type-Level Boolean Operations

To support complex type-level decisions:

```rust
// Boolean OR type-level function
pub type Or<A, B> = <A as BoolOr<B>>::Output;

// Helper trait for boolean OR
pub trait BoolOr<B> {
    type Output: Bool;
}

impl BoolOr<True> for True { type Output = True; }
impl BoolOr<False> for True { type Output = True; }
impl BoolOr<True> for False { type Output = True; }
impl BoolOr<False> for False { type Output = False; }

// Boolean NOT type-level function
pub trait Not {
    type Output: Bool;
}

impl Not for True { type Output = False; }
impl Not for False { type Output = True; }
```

These operations enable:

- Complex type-level conditions
- Composition of multiple boolean results
- Rich type-level protocol analysis

### Type-Level Maps and Folds

Two fundamental patterns for type-level list processing:

- **Map**: Transforms every element of a type list, preserving length
- **Filter**: Removes elements failing a predicate, potentially shortening the list

Example: Filter implementation to drop all `EpSkip<IO,Me>` instances:

```rust
pub trait FilterSkips<IO, Me: Role, List> { type Out; }

// Base case: empty list
impl<IO, Me: Role> FilterSkips<IO, Me, Nil> for () { type Out = Nil; }

// Recursive case with dispatch on marker type
impl<IO, Me: Role, H, T> FilterSkips<IO,Me,Cons<H,T>> for ()
where
    H: GetEpSkipTypeMarker<IO,Me>,
    (): FilterSkipsCase<IO,Me,H,T,
        <H as GetEpSkipTypeMarker<IO,Me>>::TypeMarker>,
{
    type Out = <() as FilterSkipsCase<IO,Me,H,T,
        <H as GetEpSkipTypeMarker<IO,Me>>::TypeMarker>>::Out;
}

// Skip EpSkip elements
impl<IO, Me: Role, T> FilterSkipsCase<IO,Me,EpSkip<IO,Me>,T,IsEpSkipType> for ()
where (): FilterSkips<IO,Me,T>
{
    type Out = <() as FilterSkips<IO,Me,T>>::Out;
}

// Keep other elements
impl<IO, Me: Role, H, T> FilterSkipsCase<IO,Me,H,T,IsNotEpSkipType> for ()
where H: EpSession<IO,Me>, (): FilterSkips<IO,Me,T>
{
    type Out = Cons<H, <() as FilterSkips<IO,Me,T>>::Out>;
}
```

### Type-Level Equality

Essential for compile-time type assertions:

```rust
pub trait TypeEq<A> {}
impl<T> TypeEq<T> for T {}
```

Used for assertions like `assert_type_eq!(A, B)` which fail if A ≠ B.

### Role Containment Checking

For determining if a protocol branch contains a specific role:

```rust
pub trait ContainsRole<R> {
    type Output: types::Bool;
}

// Base case: End contains no roles
impl<IO, Lbl, R> ContainsRole<R> for TEnd<IO, Lbl> {
    type Output = types::False;
}

// Recursive case: TInteract contains role if role matches or continuation does
impl<IO, Lbl, H, T, R1, R2> ContainsRole<R2> for TInteract<IO, Lbl, R1, H, T>
where
    // ...constraints...
    <R1 as RoleEq<R2>>::Output: types::BoolOr<<T as ContainsRole<R2>>::Output>,
{
    type Output = types::Or<<R1 as RoleEq<R2>>::Output, <T as ContainsRole<R2>>::Output>;
}
```

This pattern enables:

- Recursive traversal of nested protocol structures
- Boolean composition of role presence
- Precise typing for projection decisions

### Case-Based Composition Pattern

For handling different combinations of endpoint types:

```rust
pub trait ComposeProjectedParBranches<IO, Me: Role, L, R> /* constraints */ {
    type Out: EpSession<IO, Me>;
}

impl<IO, Me: Role, L, R> ComposeProjectedParBranches<IO, Me, L, R> for ()
where
    // ...constraints...
    (): ComposeProjectedParBranchesCase<
        IsSkip<L, IO, Me>,  // Type-level query
        IsSkip<R, IO, Me>,  // Type-level query
        IsEnd<L, IO, Me>,   // Type-level query
        IsEnd<R, IO, Me>,   // Type-level query
        IO, Me, L, R,
    >,
{
    type Out = <() as ComposeProjectedParBranchesCase</*...*/>::Out;
}

// Multiple specialized implementations for different cases
```

## 2. Rust Trait System Constraints & Workarounds

### Key Limitations in Stable Rust

- **No specialization**: Cannot provide specialized implementations for subsets
- **No negative bounds**: Cannot constrain generics by what they are not
- **No associated types as generic parameters**: Types must be direct
- **No overlapping impls**: Must have disjoint implementation sets

### Effective Workarounds

- Use **sealed helper traits** with marker types for mutual exclusion
- Create a **facade trait** with a single impl to avoid overlap
- Use **explicit enumeration** rather than blanket implementations
- Prefer **concrete type parameters** over complex bounds
- Leverage **type-level booleans** for conditional logic

## 3. Project Organization Patterns

### Documentation & Planning

- **Always plan** edits with:
  - Affected files/traits/functions
  - Change order and dependencies
  - Estimated edit count
- **Document intent** with comments for non-obvious logic
- **Distinguish algorithms** (map vs. filter) in prose first
- Use examples with doctests for clarity

### Test-Driven Parameter Refactoring

1. Create pre-implementation tests verifying current behavior
2. Make parameter changes while preserving behavior
3. Verify with post-implementation tests
4. Systematically track test coverage metrics

### Label Parameter Standardization

- Start with core types and related traits
- Move outward to dependent systems
- Finally update tests and examples
- Ensure thorough test coverage before refactoring

### Runtime Implementation Approaches

Three primary patterns for session type runtime implementation:

1. **Typed Channel Wrappers**:
   - Protocol state encoded in type parameters
   - Heavy reliance on Rust's type system
   - High flexibility but complex type-level programming

2. **Code Generation with Procedural Macros**:
   - Minimizes boilerplate through code generation
   - Uses callbacks/handlers for business logic integration
   - Cleaner syntax but may limit IDE support

3. **State Machine Builders**:
   - Explicit protocol states as distinct types
   - Strong IDE support through state method discovery
   - Excellent for visualizing protocol flow

## 4. Key Insights

1. **Type-Level Dispatch** is crucial for non-overlapping implementations and enables
pattern-matching-like behavior in the type system.

2. **Helper Traits** create indirection that resolves implementation conflicts and improves
modularity.

3. **Trait Design** should focus on composable, single-responsibility traits with clear boundaries.

4. **Recursive Type Traversal** with proper terminal cases is essential for handling nested
protocols.

5. **Case Multiplication** means implementations grow with the product of case dimensions.

6. **Type-Level Boolean Operations** enable complex decision logic that would be impossible with
simple trait bounds.

7. **Protocol Projection** decisions depend heavily on role presence, requiring sophisticated role
containment checking.

8. **Test-First Refactoring** with comprehensive metrics significantly reduces regression risk.

9. **Parameter Consistency** across the codebase improves readability and simplifies reasoning.

10. **Edge Case Testing** reveals subtle design principles that may not be apparent from basic
tests.

## 5. Code Organization Insights

### Modular Protocol System Structure

The refactoring of the protocol system into smaller, purpose-specific files revealed several
important insights:

1. **Layer-Based Organization**: Separating the protocol system into conceptual layers improves
maintainability:
   - `base.rs`: Core traits and types used across the entire system
   - `global.rs`: Global protocol type definitions and combinators
   - `local.rs`: Local endpoint type definitions and behaviors
   - `transforms.rs`: Transformation logic between global and local types
   - `utils.rs`: Helper functions and utilities

2. **Interface Stability**: By carefully designing the public module interfaces and re-exports, we
maintained full backward compatibility while significantly improving the internal organization.

3. **Documentation Cohesion**: Comments and documentation now align more closely with the code they
explain, making it easier to understand specific components without having to search through a
large file.

4. **Testing Focus**: Tests can now target specific components more precisely, making it easier to
understand test failures and add new tests for specific features.

5. **Evolution Path**: The modular structure creates a clearer path for evolving individual
components while maintaining the overall system architecture.

### Style and Documentation Standards

1. **Consistent Style**: Using `cargo fmt` and `cargo clippy` consistently ensures that the code
follows Rust's standard style guidelines.

2. **Documentation Style**: Separating markdown style fixes from code refactoring prevents scope
creep and helps maintain focus on the primary task.

3. **Progressive Enhancement**: Address foundational issues first (code structure) before tackling
more superficial concerns (documentation formatting).

4. **Technical Debt Management**: Tracking formatting issues as separate tasks helps maintain
project momentum while acknowledging work that still needs to be done.

---

*Consult this summary before any future protocol‐projection or
type‐level work to maintain stability, clarity, and correctness.*
