# Besedarium Project Tasks

## Active Tasks

- [x] **Label preservation** during projection for better traceability and debugging
  - [x] Update local endpoint type definitions to include label parameters
  - [x] Modify ProjectRole trait implementation to propagate labels
  - [x] Update helper traits to handle labels during projection
  - [x] Add utility traits for label access and comparison
  - [x] Update macros and helper functions
  - [x] Create test cases for label preservation
  - [x] Update documentation

## Planned Tasks

- [ ] **Enhanced recursion support** with explicit variables and potential for mutual recursion
  - [ ] Design explicit recursion variable representation (TMu/TVar style)
  - [ ] Implement mutual recursion capabilities
  - [ ] Support scoped recursion blocks
  - [ ] Update projection machinery for recursion variables

- [ ] **Branch merging** for optimized choice projection
  - [ ] Implement branch equivalence detection
  - [ ] Create merge algorithm for equivalent choice branches
  - [ ] Update projection to utilize branch merging

- [ ] **Internal/external choice distinction** for clearer protocol semantics
  - [ ] Define separate types for internal vs external choice
  - [ ] Update projection to distinguish choice types
  - [ ] Provide composable combinators for both choice types

- [ ] **Protocol verification tools** for static analysis
  - [ ] Deadlock freedom verification
  - [ ] Progress guarantees
  - [ ] Well-formedness checks

- [ ] **Init** Global session combinator
  - [ ] Design API for initialization combinator
  - [ ] Implement projection to all local roles
  - [ ] Consider runtime channel integration

- [ ] **Metadata** type parameter for configuration
  - [ ] Add metadata type parameter to global types
  - [ ] Implement projection strategies (whole/piece-wise)
  - [ ] Create utilities for metadata access

## Completed Tasks

- [x] Core global combinators implementation
- [x] N-ary extensions for choice and parallel composition
- [x] Core endpoint types implementation
- [x] Basic projection machinery

## Technical Improvements

- [ ] Optimize compile-time performance for complex protocols
- [ ] Improve type error messages for protocol errors
- [ ] Add more comprehensive test suite
