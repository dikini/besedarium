# Status Summary: Besedarium Session Types Library

This document provides a status overview of the Besedarium session types library, focusing on the core type-level components and ignoring runtime aspects.

## 1. Global Protocol Types Status

### 1.1 Implemented Features

- **Core Combinators**: 
  - `TInteract<IO, Lbl, R, H, T>`: Interaction between roles with message passing
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

### 1.2 Missing Features

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
  - `EpSend<IO, R, H, T>`: Send action for role R
  - `EpRecv<IO, R, H, T>`: Receive action for role R
  - `EpChoice<IO, R, L, R>`: Local choice/branch
  - `EpPar<IO, R, L, R>`: Local parallel composition
  - `EpEnd<IO, R>`: Local protocol termination
  - `EpSkip<IO, R>`: No-op for uninvolved roles

- **Type-Level Properties**:
  - Role-based typing
  - Sequential composition
  - Basic branching and parallelism

### 2.2 Missing Features

- **Label Preservation**: 
  - Local types don't preserve labels from global types
  - No connection to corresponding global protocol points
  
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
  - Helper traits for specific combinators (`ProjectInteract`, `ProjectChoice`, `ProjectPar`)
  - Type-level role equality (`RoleEq`) for determining send/receive actions
  
- **Handling of Edge Cases**:
  - Proper handling of empty protocols
  - Skip composition for uninvolved roles
  - Role presence detection
  
- **Composition Support**:
  - Projection of nested global types
  - Handling of binary choices and parallel composition

### 3.2 Missing Features

- **Advanced Projection**:
  - Limited support for projecting complex recursive structures
  - No merging of equivalent branches in choice projections
  - Limited static guarantees for projection correctness
  
- **Label and Metadata Handling**:
  - Labels from global protocols are not preserved during projection
  - Loss of traceability between global and local protocol points
  
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

### 4.3 Priority Areas for Future Work

- **Label preservation** during projection for better traceability and debugging
- **Enhanced recursion support** with explicit variables and potential for mutual recursion
- **Branch merging** for optimized choice projection
- **Internal/external choice distinction** for clearer protocol semantics
- **Protocol verification tools** for static analysis of deadlock freedom and progress
- **Init** Global session combinator that project to all local roles. Signifies protocol initialisation. Possibly tied to runtime channels.
- **Metadata** type parameter. A reader-like, configuration type parameter, there to supply common configuration to all. Should be projected to local roles, either as a whole, or could be projected to piece-wise to specific roles.
