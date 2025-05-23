# Status Summary: Besedarium Session Types Library

This document provides a status overview of the Besedarium session types library, focusing on the core type-level components and ignoring runtime aspects.

## 1. Global Protocol Types Status

### 1.1 Implemented Features

- **Core Combinators**: 
  - `TStart<IO, Lbl, S>`: Protocol entry point with label Lbl and continuation S - ✅ (Fully implemented with projection & tests)
  - `TSend<IO, Lbl, R, H, T>`: Send action from role R with message H and label Lbl
  - `TRecv<IO, Lbl, R, H, T>`: Receive action for role R with message H and label Lbl
  - `TChoice<IO, Lbl, L, R>`: Binary choice between two protocol branches
  - `TPar<IO, Lbl, L, R, IsDisjoint>`: Binary parallel composition with disjointness checking
  - `TEnd<IO, Lbl>`: Protocol termination
  - `TRec<IO, L>`: Basic recursion support

- **N-ary Extensions**: 
  - `ToTChoice` trait and `tchoice!` macro for n-ary choices
  - `ToTPar` trait and `tpar!` macro for n-ary parallel composition

- **Type-Level Properties**:
  - Disjointness checking for `TPar` branches
  - Label parameters for all combinators
  - Type-level role extraction and containment checking

### 1.2 Missing Features and Work in Progress

- **Advanced Recursion**:
  - No explicit recursion variables (`TMu`/`TVar` style)
  - Limited support for mutual recursion
  - No scoped recursion blocks

- **Protocol Refinements**:
  - No constraints on message types
  - No time-based constraints or timeouts

- **Channel Specification**:
  - Limited medium/channel specification capabilities
  - No support for specifying communication properties

## 2. Local (Endpoint) Protocol Types Status

### 2.1 Implemented Features

- **Core Endpoint Types**:
  - `EpSend<IO, Lbl, R, H, T>`: Send action for role R with label preservation
  - `EpRecv<IO, Lbl, R, H, T>`: Receive action for role R with label preservation
  - `EpChoice<IO, Lbl, R, L, R>`: Local choice/branch with label preservation
  - `EpPar<IO, Lbl, R, L, R>`: Local parallel composition with label preservation
  - `EpEnd<IO, Lbl, R>`: Local protocol termination with label preservation
  - `EpSkip<IO, Lbl, R>`: No-op for uninvolved roles with label preservation

- **Type-Level Properties**:
  - Role-based typing
  - Sequential composition
  - Basic branching and parallelism
  - Label preservation from global types

### 2.2 Missing Features

- **Enhanced Role Types**:
  - No distinction between internal choice (decides) and external choice (offers)
  - Limited role metadata

- **Advanced Local Features**:
  - No explicit support for channel delegation
  - No explicit recursion variables
  - Limited local control flow beyond global structure

## 3. Projection from Global to Local Types

### 3.1 Implemented Features

- **Core Projection Machinery**:
  - `ProjectRole<Me, IO, G>` trait for projecting global type G to role Me
  - Helper traits for specific combinators (`ProjectSend`, `ProjectRecv`, `ProjectChoice`, `ProjectPar`)
  - Type-level role equality (`RoleEq`) for determining send/receive actions
  - Projection cases for combinators:
    - **TSend/ TRecv**:
      - Role matches sender/receiver → Project as `EpSend`/`EpRecv` for that role
      - Role not involved → Project as `EpSkip`
    - **TPar**: 
      - Role in left branch only → Project left branch directly (preserving labels)
      - Role in right branch only → Project right branch directly (preserving labels)
      - Role in neither branch → `EpSkip` with parent label
      - Note: Due to the disjointness constraint, a role cannot appear in both branches
    - **TChoice**: 
      - Role in both branches → `EpChoice` with both branches projected
      - Role in only one branch → Project that branch with appropriate context
      - Role in neither branch → `EpSkip` with parent label
    - **TRec**: 
      - Project the body of recursion and wrap result in `EpRec` (preserving labels)
  - **Label preservation**: All endpoint types preserve the label from the corresponding global combinator, ensuring traceability.
  - **Test Base Stabilization**: All tests and doctests now pass or are properly documented/disabled if blocked by Rust limitations. The new combinator system is fully adopted.

- **Handling of Edge Cases**:
  - Proper handling of empty protocols
  - Skip composition for uninvolved roles
  - Role presence detection

- **Composition Support**:
  - Projection of nested global types
  - Handling of binary choices and parallel composition
  - Label preservation from global to local types

### 3.2 Missing Features

- **Advanced Projection**:
  - Limited support for projecting complex recursive structures
  - No merging of equivalent branches in choice projections
  - Limited static guarantees for projection correctness

- **Label and Metadata Handling**:
  - ~~Labels from global protocols are not preserved during projection~~ (Implemented)
  - ~~Loss of traceability between global and local protocol points~~ (Fixed with label preservation)

- **Performance and Optimization**:
  - Potential for optimization in nested choice projection
  - Complex projections may be verbose and inefficient

## 4. Known Limitations and Future Work

### 4.1 Theoretical Limitations

- **Role-Disjoint Parallel Composition Only**:
  - Current implementation strictly enforces that parallel branches must have disjoint sets of roles
  - This prevents certain valid protocols with controlled role overlap
  - Mutual recursion via Par+Rec patterns becomes impossible

- **Flat Label Namespace**:
  - Labels exist in a single global namespace
  - No scoped recursion or shadowing
  - Limited expressiveness for certain advanced protocol patterns

### 4.2 Implementation Limitations

- **Rust Trait System Constraints**:
  - No specialization
  - No negative bounds
  - No associated types as generic parameters 
  - No overlapping impls

- **Type Identity in Doctests**:
  - Rust's type identity rules prevent strict type equality assertions in doctests, even when types are structurally identical. This affects `assert_type_eq!` and similar macros in documentation examples.
  - Macro visibility in doctests is limited; macros like `tlist!`, `tchoice!`, and `tpar!` may not be available in doctest context, causing failures for illustrative code blocks.

### 4.3 Priority Areas for Future Work

- ~~**Label preservation** during projection for better traceability and debugging~~ (Completed)
- ~~**TSend/TRecv combinator migration**: Remove legacy `TInteract` and update all protocol logic~~ (Completed)
- **Enhanced recursion support** with explicit variables and potential for mutual recursion
- **Branch merging** for optimized choice projection
- **Internal/external choice distinction** for clearer protocol semantics
- **Protocol verification tools** for static analysis of deadlock freedom and progress
- **Init** Global session combinator that projects to all local roles. Signifies protocol initialisation. Possibly tied to runtime channels.
- **Metadata** type parameter. A reader-like, configuration type parameter, there to supply common configuration to all. Should be projected to local roles, either as a whole, or could be projected to piece-wise to specific roles.

## 5. Current Project Status

### 5.1 Work in Progress

- Protocol transform modularization: **completed** implementing `ProjectSendCase` and `ProjectRecvCase` in their own modules
- Next step: Update tests in `test_overrides.rs` to work with the new modular structure
- Finalizing documentation for core concepts:
  - `docs/duality.md` (Duality, Well-Formedness, Projection, and Type-Level Implementation) is now complete.

## Current Work In Progress

- **Phase 1: Core Protocol & Duality Implementation**
  - **Task 1.1.1**: Define/Update `CommMetadata` (including `ChanId`, `MsgLbl`, and `ActionIOType` concepts) based on `docs/duality.md`. (wip)
    - Status: Analyzing `docs/duality.md` for requirements.
    - Next Steps: Propose data structures and traits for `CommMetadata` and related concepts.

### 5.2 Current Issues

- Tests in `test_overrides.rs` need updates to work with the new modular structure
- Need to review documentation consistency across the modularized files
- See GitHub issues for any remaining edge cases or documentation improvements

### 5.3 Current PRs

- [Modularize protocol transforms and update references](https://github.com/YOUR_REPO/besedarium/pull/XX) (pending review)
- [New implementation of recursive combinators](https://github.com/YOUR_REPO/besedarium/pull/19) (draft, resolves issue #19)

### 5.4 Current Tasks

- See [work/TASKS.md](TASKS.md)

### 5.5 Current Learnings

- See [work/learnings.md](learnings.md)

### 5.6 Structure, Tests, Documentation

- Core protocol transform code is now completely modularized:
  - `ProjectSendCase` trait fully implemented in `src/protocol/transforms/send.rs`
    - True case: When Me == RSender → Project as `EpSend`
    - False case: When Me != RSender → Project continuation only
  - `ProjectRecvCase` trait fully implemented in `src/protocol/transforms/recv.rs`
    - True case: When Me == RReceiver → Project as `EpRecv`
    - False case: When Me != RReceiver → Project continuation only
  - `ProjectRole` implementations updated in `projection.rs` with:
    - Proper trait bounds for helper traits
    - Role equality checks for dispatch
    - Type-level boolean flags to select appropriate implementation
  - Other helper traits already modularized in their respective files
- Library code compiles successfully with `cargo check --lib`
- Some tests need updates due to module changes, especially in `test_overrides.rs`
- Documentation and code comments are comprehensive with proper module-level documentation
