# Besedarium: Type-Level Protocol Programming Patterns

This document distills essential patterns for implementing type-level session types in Rust
without relying on unstable features.

## Recent Achievements (2025-05-24)

### Task 1.1.3 Successfully Completed: Local Endpoint Types

**Implementation Success:** Successfully implemented all local endpoint types using the extensible metadata pattern:

**Types Implemented:**

- `EpChanSend<IO, M, Msg, P, AIO>` - Local endpoint for sending messages  
- `EpChanRecv<IO, M, Msg, P, AIO>` - Local endpoint for receiving messages
- `EpChanOffer<IO, M, Left, Right, AIO>` - Local endpoint for offering choices
- `EpChanChoice<IO, M, Left, Right, AIO>` - Local endpoint for making choices  
- `EpChanPar<IO, M, Left, Right, IsDisjoint, AIO>` - Local endpoint for parallel composition
- `EpChanEnd<IO, M, AIO>` - Local endpoint for protocol termination
- `EpChanStart<IO, M, Start, AIO>` - Local endpoint for protocol initialization

**Key Implementation Insights:**

1. **Extensible Metadata Success**: The `CommMetadataTrait` approach enabled using `M: CommMetadataTrait` bounds while maintaining compatibility with `CommMetadata<C, L>` instances

2. **Type Signature Consistency**: Successfully established consistent pattern:

   ```rust
   // 5 parameters for action types
   EpChanSend<IO, M, Msg, P, AIO>
   // 3 parameters for termination  
   EpChanEnd<IO, M, AIO>
   // 6 parameters for parallel (adds IsDisjoint)
   EpChanPar<IO, M, Left, Right, IsDisjoint, AIO>
   ```

3. **Test Infrastructure**: Created comprehensive test suite covering basic type creation, type aliases, trait implementations, IO constraints, and complex compositions

4. **Compilation Success**: All tests pass and project compiles with proper error checking

**Next Priority:** Task 1.1.5 (Project trait) to complete the core protocol system.

### Task 1.1.4 Successfully Completed: IsDual Predicate Implementation

**Implementation Success:** Successfully implemented comprehensive duality checking system using type-level programming patterns:

**Core Trait System:**

- `IsDual<P, Q>` trait with `Output: Bool` associated type for compile-time duality verification
- Helper traits: `EqualsTrue`, `EqualsFalse`, `DualityCheck` for type-level constraints
- Type alias: `IsDualOutput<P, Q>` for convenience

**Duality Rules Implemented:**

1. **Send/Recv Duality**: `TChanSend<S,R>` ↔ `TChanRecv<R,S>` (role-swapped)
2. **Choice/Offer Duality**: `TChanChoice` ↔ `TChanOffer` (branching duality)
3. **Self-Duality**: `TChanEnd` ↔ `TChanEnd`, `TChanPar` ↔ `TChanPar`
4. **Local Endpoint Duality**: All local endpoint types with appropriate role/IO constraints
5. **Recursive Duality**: Proper handling of nested protocol structures

**Key Implementation Insights:**

1. **Type-Level Boolean Logic**: Successfully used `True`/`False` types for compile-time duality decisions
2. **Role Swapping Pattern**: Implemented role inversion for send/recv duality relationships
3. **Default False Implementation**: Blanket implementation ensuring non-dual types return `False`
4. **Assertion Macros**: Created `assert_dual!` and `assert_not_dual!` for compile-time verification
5. **Comprehensive Testing**: Full test suite covering all protocol type combinations

**Tool Usage Pattern**: Effective use of `semantic_search` to discover existing infrastructure (type-level booleans) before implementing new functionality.

### Task 1.1.5 Successfully Completed: Project Trait Implementation

**Implementation Success:** Successfully implemented comprehensive protocol projection system with robust error handling and role-based dispatch:

**Core Projection System:**

- `Project<P, R>` trait for mapping global protocols to local endpoint types
- `ProjectOutput<P, R>` type alias for convenience  
- `ProjectionError` enum with comprehensive error variants for validation
- `ValidateProjection<P, R>` trait for compile-time validation
- `ProjectionValidator` trait with `DefaultProjectionValidator` implementation

**Helper Trait System:**

- `Bool`, `True`, `False` types for type-level boolean logic
- `RoleEq<Other>` trait for role equality checking at type level
- `ProjectSendCase<...>` and `ProjectRecvCase<...>` for role-based dispatch
- Role-based conditional projection using type-level case analysis

**Key Implementation Insights:**

1. **Role-Based Dispatch Success**: Implemented sophisticated role equality checking system:

   ```rust
   // Type-level role equality
   trait RoleEq<Other: Role>: Role {
       type Output: Bool; // True if equal, False otherwise  
   }
   ```

2. **IO Parameter Resolution**: Successfully resolved critical IO parameter issues:
   - **Problem**: Unconstrained `IO` parameters in projection implementations
   - **Solution**: Added `Me: Role + SupportsActionIO<AIO>` bounds to ensure the role can handle the required I/O actions
   - **Result**: EpChanSend/EpChanRecv now properly use role types that implement SupportsActionIO

3. **Type Signature Corrections**: Fixed all global protocol type signatures to match definitions:
   - **TChanEnd**: Corrected to 3 parameters `<C, L, AIO>`
   - **TChanChoice**: Corrected to 6 parameters `<R, C, Lbl, Left, Right, AIO>`  
   - **TChanPar**: Corrected to 6 parameters `<C, Lbl, Left, Right, IsDisjoint, AIO>`
   - **TChanStart**: Corrected to 4 parameters `<C, L, Start, AIO>`

4. **Comprehensive Error Handling**: Implemented production-ready error system:

   ```rust
   enum ProjectionError {
       RoleNotInvolved { role: String, protocol_step: String },
       InvalidProjection { reason: String, protocol_type: String, target_role: String },
       ActionIOCapabilityMismatch { required_capability: String, actual_capability: String },
       InvalidMetadata { description: String },
   }
   ```

5. **Test Infrastructure Success**: Created comprehensive test suite with proper trait derives and type signatures

**Critical Resolution:** Successfully resolved E0761 module conflicts from previous sessions by removing empty legacy files and fixed all duality module issues.

**Compilation Status:** ✅ All 35 tests passing, project builds successfully with only dead code warnings

### Task 1.1 Implementation Prompts and Documentation

**Comprehensive Implementation Strategy:** Created complete set of LLM prompts and enhanced documentation for implementing all Task 1.1 subtasks, including foundation types, global protocol types, projection mechanisms, and duality checking with concrete trait definitions and implementation patterns.

## Core Type-Level Programming Patterns

### Extensible Metadata Pattern

**Key Insight:** The specification intentionally uses extensible compound data structures rather than individual parameters to enable downstream extensions and protocol evolution.

**Implementation Strategy:**

```rust
// EXTENSIBLE: Allows downstream implementations to extend metadata
pub struct EpChanSend<IO, M, Msg, P, AIO> 
where 
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait, // trait that CommMetadata<C, L> implements
    Msg: Message,
    P: LocalProtocol,
    AIO: ActionIOTMarker,

// VS. NON-EXTENSIBLE: Fixed individual parameters  
pub struct EpChanSend<IO, C, L, Msg, P, AIO>
where
    IO: SupportsActionIO<AIO>, 
    C: ChanId,
    L: MsgLbl,
    // Cannot be extended by downstream implementations
```

**Benefits:**

- **Downstream Extensions**: Add timestamps, priorities, routing info, etc.
- **Compound Data Structure**: Groups related metadata for easier manipulation
- **Type System Flexibility**: Metadata variants while maintaining type safety
- **Protocol Evolution**: Metadata evolution without breaking type signatures

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
    T: TSession<IO> + LabelsOf,
    Self: LabelsOf,
    <Self as LabelsOf>::Labels: DisjointFrom<<T as LabelsOf>::Labels>,
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

**Invariant:** All protocol combinators (TSend, TRecv, TEnd, TChoice, TPar, TRec, TInteract, etc.) must have a label parameter and implement the GetProtocolLabel trait.

**Enforcement:**

- Type definitions: All combinators must have a label parameter
- Trait implementations: All combinators must implement GetProtocolLabel
- Documentation: The invariant must be stated in module-level and trait-level docs
- Code examples must use combinators with label parameters

**Rationale:** This invariant ensures that protocol labels are always available for type-level reasoning, protocol projection, and endpoint generation. It also improves maintainability and future extensibility.

**Pattern:** When adding a new protocol combinator, always include a label parameter and implement GetProtocolLabel. Document this requirement in both code and project planning.

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

## Critical Implementation Insights

1. **Type-Level Dispatch** is fundamental for handling different protocol cases without specialization.

2. **Helper Traits** resolve implementation conflicts through indirection.

3. **Recursive Type Traversal** requires careful handling of base cases and conditionals.

4. **Edge Case Testing** reveals subtle protocol implementation issues before they become problems.

5. **Protocol Projection** decisions must account for role presence in multiple communication paths.

6. **Compositional Design** with small, focused traits improves modularity and evolution.

7. **Type Safety at Compile Time** is achievable through proper trait bounds and assertions.

## Module Organization and File Size Management

### File Size and Module Structure Guidelines

**Keep Files Compact and Focused:**

- **Target Size**: Individual module files should stay under 300 lines including documentation
- **Test Separation**: When a module reaches ~200 lines, extract tests to separate `tests.rs` files
- **Implementation/Test Ratio**: Aim for roughly 2:1 implementation to test code ratio
- **Documentation Balance**: Include essential documentation but avoid excessive inline examples

**Module Structure Pattern for Growth:**

When modules grow beyond manageable size, follow this consistent structure:

```text
src/protocol/[module_name]/
├── mod.rs              # Core implementation (~150-250 lines)
├── tests.rs            # Unit tests (~50-150 lines)  
├── builders.rs         # Builder functions/helpers (optional)
└── examples.rs         # Extended examples (optional)
```

**Benefits:**

- **Better Organization**: Clear separation of concerns
- **Maintainability**: Easier to navigate and understand
- **Scalability**: Ready for continued development
- **Consistency**: Predictable structure across modules

**Refactoring Process Pattern:**

1. **Identify Split Points**: Look for natural boundaries (tests, type groups, helpers)
2. **Create Module Directory**: Convert `module.rs` to `module/mod.rs`
3. **Extract Tests**: Move all `#[cfg(test)]` blocks to `tests.rs`
4. **Update Imports**: Add `mod tests;` declaration in `mod.rs`
5. **Verify Compilation**: Ensure all tests still pass and no circular dependencies
6. **Update Documentation**: Reflect new structure in module docs

## Duality and Well-Formedness in MPST

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

- **Type-Level Well-Formedness Expression**
  - `IsWellFormed<G>` trait aggregates pairwise duality checks for all communicating pairs.
  - Uses type-level sets, filtering, and aggregation to ensure protocol safety at compile time.

### Implementation Patterns and Insights

- Prefer explicit, minimal struct definitions for each protocol action.
- Use trait-based type-level functions for protocol analysis and verification.
- Document invariants and requirements for each protocol construct.
- Maintain clear separation between global and local protocol representations.
- Use lockstep filtering and pairwise checks for robust well-formedness analysis.
- Example-driven explanations improve clarity and correctness.

## Development Best Practices

### Documentation and Testing

1. **Markdown Linting** with markdownlint-cli2 using a standardized `.markdownlint-cli2.yaml` configuration file ensures consistent documentation formats.

2. **Line Length Standards** set to 100 characters provide a balance between readability and efficient use of screen space.

3. **List Formatting Rules** require proper indentation (2 spaces for top-level) and blank lines before and after lists.

### Doctest and Test Management

- Rust doctests do not have access to crate macros unless explicitly exported and imported; integration tests do.
- Type-level equality assertions (e.g., `assert_type_eq!`) may fail in doctests due to Rust's type identity limitations, even if types are structurally identical.
- Macro-based protocol definitions (`tchoice!`, `tpar!`) should be tested in integration/compile-time tests, not doctests.
- Always document these limitations for users and contributors.

### Modularization Learnings

- Modularizing protocol transformation logic improves maintainability, discoverability, and testability.
- Rust stable type-level programming requires careful trait and module organization to avoid orphan rules and import conflicts.
- Doctests must use public re-export paths (e.g., `besedarium::ProjectChoice`) to work for both users and CI.
- When splitting large files, update all references and re-exports to avoid breakage in dependent modules and tests.
- Use `pub use` in `mod.rs` to provide a stable, discoverable API surface for downstream users.

### Protocol Combinator Implementation

When adding a new protocol combinator, follow these key steps:

1. Define the struct in `global.rs` with proper generic parameters and PhantomData
2. Implement `sealed::Sealed` to restrict trait implementations
3. Implement `TSession<IO>` with proper composition rules
4. Implement `GetProtocolLabel` to adhere to the protocol label invariant
5. Implement `ProjectRole` for projection to local protocols
6. Add comprehensive documentation and test cases
7. Update integration tests to showcase the new combinator in real protocol scenarios

### Helper Trait Implementation

- Implementing helper traits like `ProjectSendCase` and `ProjectRecvCase` in separate modules improves code organization and maintainability.
- Using type-level booleans (`True`/`False`) for dispatch based on role equality is an effective pattern for handling different projection cases.
- The implementation follows the "Helper Trait Case Analysis" pattern where the main trait delegates to case-specific helpers that handle different cases based on type-level boolean flags.
- Trait bounds in the implementation must be carefully managed, especially ensuring proper bounds for role equality and recursive projections.

---

*This knowledge base distills the core patterns for implementing session types in Rust. Reference when implementing protocol-related functionality.*

*Last updated: 2025-05-24*

### Task 1.1.5 Planning: Project Trait Implementation (2025-05-24)

**Current State Analysis:**

- Tasks 1.1.1-1.1.4 completed successfully with correct type signatures
- Legacy projection/mod.rs (999 lines) uses outdated type signatures incompatible with new foundation
- Need to implement new Project trait compatible with current type system:
  - Global types: `TChanSend<S,R,C,L,Msg,P,AIO>`, `TChanRecv<S,R,C,L,Msg,P,AIO>`, etc.
  - Local types: `EpChanSend<IO,M,Msg,P,AIO>`, `EpChanRecv<IO,M,Msg,P,AIO>`, etc.
- No functional transforms/ directory - was removed during cleanup

**Implementation Strategy:**

1. **Core Project Trait**: `Project<P, R>` where P: GlobalProtocol, R: Role, Output: LocalProtocol
2. **Role-Based Dispatch**: Use helper traits for TSend/TRecv to determine if role is sender/receiver
3. **Type-Level Programming**: Use Bool types (True/False) for compile-time case selection
4. **Modular Design**: Split projection logic across helper traits for maintainability

**Key Type Mappings:**

- `TChanSend<S,R,C,L,Msg,P,AIO>` + Role `S` → `EpChanSend<IO,M,Msg,ProjectedP,AIO>`
- `TChanSend<S,R,C,L,Msg,P,AIO>` + Role `R` → `EpChanRecv<IO,M,Msg,ProjectedP,AIO>`
- Metadata mapping: Global `(C,L)` → Local `CommMetadata<C,L>`

### Task 1.1.5 Successfully Completed: Project Trait Implementation

**Implementation Success:** Successfully implemented comprehensive protocol projection system with robust error handling and role-based dispatch:

**Core Projection System:**

- `Project<P, R>` trait for mapping global protocols to local endpoint types
- `ProjectOutput<P, R>` type alias for convenience  
- `ProjectionError` enum with comprehensive error variants for validation
- `ValidateProjection<P, R>` trait for compile-time validation
- `ProjectionValidator` trait with `DefaultProjectionValidator` implementation

**Helper Trait System:**

- `Bool`, `True`, `False` types for type-level boolean logic
- `RoleEq<Other>` trait for role equality checking at type level
- `ProjectSendCase<...>` and `ProjectRecvCase<...>` for role-based dispatch
- Role-based conditional projection using type-level case analysis

**Key Implementation Insights:**

1. **Role-Based Dispatch Success**: Implemented sophisticated role equality checking system:
   ```rust
   // Type-level role equality
   trait RoleEq<Other: Role>: Role {
       type Output: Bool; // True if equal, False otherwise  
   }
   ```

2. **IO Parameter Resolution**: Successfully resolved critical IO parameter issues:
   - **Problem**: Unconstrained `IO` parameters in projection implementations
   - **Solution**: Added `Me: Role + SupportsActionIO<AIO>` bounds to ensure the role can handle the required I/O actions
   - **Result**: EpChanSend/EpChanRecv now properly use role types that implement SupportsActionIO

3. **Type Signature Corrections**: Fixed all global protocol type signatures to match definitions:
   - **TChanEnd**: Corrected to 3 parameters `<C, L, AIO>`
   - **TChanChoice**: Corrected to 6 parameters `<R, C, Lbl, Left, Right, AIO>`  
   - **TChanPar**: Corrected to 6 parameters `<C, Lbl, Left, Right, IsDisjoint, AIO>`
   - **TChanStart**: Corrected to 4 parameters `<C, L, Start, AIO>`

4. **Comprehensive Error Handling**: Implemented production-ready error system:
   ```rust
   enum ProjectionError {
       RoleNotInvolved { role: String, protocol_step: String },
       InvalidProjection { reason: String, protocol_type: String, target_role: String },
       ActionIOCapabilityMismatch { required_capability: String, actual_capability: String },
       InvalidMetadata { description: String },
   }
   ```

5. **Test Infrastructure Success**: Created comprehensive test suite with proper trait derives and type signatures

**Critical Resolution:** Successfully resolved E0761 module conflicts from previous sessions by removing empty legacy files and fixed all duality module issues.

**Compilation Status:** ✅ All 35 tests passing, project builds successfully with only dead code warnings
