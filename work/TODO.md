# Protocol Module Refactoring

## Overview

This document outlines the plan for refactoring `src/protocol.rs` into a more maintainable structure with multiple submodules. The file has grown too large and is difficult to edit.

We will follow a layer-based separation approach, organizing code by abstraction layers.

## Structure

The new structure will be organized into these files:

```
src/protocol/
├── mod.rs              # Re-exports and documentation
├── base.rs             # Foundation: Nil, Cons, base traits
├── global.rs           # Global protocol types: TEnd, TInteract, TChoice, TPar, TRec
├── local.rs            # Local protocol types: EpSend, EpRecv, etc.
├── transforms.rs       # Transformations: ProjectRole, etc.
└── utils.rs            # Helper traits and type-level operations
```

## Tasks

### Phase 1: Preparation
- [x] Create git branch `refactor/protocol-modules`
- [x] Create TODO.md with detailed tasks
- [x] Create a GitHub issue documenting the need for refactoring (Issue #11)
- [x] Create a draft PR linked to the issue and branch (PR #12)

### Phase 2: File Structure Setup
- [x] Create the `src/protocol` directory
- [x] Create an initial `mod.rs` file with basic re-exports
- [x] Set up empty module files with documentation headers

### Phase 3: Code Migration
- [ ] Move type-level foundational code to `base.rs` (Nil, Cons, etc.)
  - [ ] Update imports
  - [ ] Run tests to verify correctness
- [ ] Move global protocol types to `global.rs` (TSession, TEnd, TInteract, etc.)
  - [ ] Update imports
  - [ ] Run tests to verify correctness
- [ ] Move local protocol types to `local.rs` (EpSession, EpSend, EpRecv, etc.)
  - [ ] Update imports
  - [ ] Run tests to verify correctness
- [ ] Move projection and transformation logic to `transforms.rs`
  - [ ] Update imports
  - [ ] Run tests to verify correctness
- [ ] Move helper traits and utilities to `utils.rs`
  - [ ] Update imports
  - [ ] Run tests to verify correctness

### Phase 4: Cleanup and Verification
- [ ] Update any remaining import paths across the codebase
- [ ] Run comprehensive test suite
- [ ] Run cargo clippy to ensure code quality
- [ ] Run cargo fmt to ensure consistent formatting
- [ ] Update documentation references if needed

### Phase 5: Finalization
- [ ] Convert PR from draft to ready for review
- [ ] Address review feedback
- [ ] Merge to main branch

## Expected Timeline
- Phase 1-2: 1 day
- Phase 3: 2-3 days
- Phase 4-5: 1 day

Total estimated time: 4-5 days