# Task List: Label Parameter Standardization (Issue #9)

The goal is to standardize on using `Lbl` instead of `L` for label parameters throughout all session type combinators and related projection machinery.

- [x] Update work/TASKS.md as progress is made
- [x] Update CHANGELOG.md to record these changes

## Main Tasks

- [x] Create test infrastructure for label parameter testing
- [x] Establish test metrics for label-related functionality
- [x] Create a detailed mapping of label usage throughout codebase
- [x] Update `TEnd<IO, L>` to `TEnd<IO, Lbl>` (Phase 2.1)
  - [x] Add pre-implementation tests for TEnd behavior
  - [x] Update type definition
  - [x] Update trait implementations
  - [x] Verify behavior with tests
- [x] Update `TInteract<IO, L, R, H, T>` to `TInteract<IO, Lbl, R, H, T>` (Phase 2.2)
  - [x] Update type definition
  - [x] Update trait implementations
  - [x] Update test implementation
  - [x] Verify behavior with tests
- [x] Update `TRec<IO, L, S>` to `TRec<IO, Lbl, S>` (Phase 2.3)
  - [x] Update type definition
  - [x] Update trait implementations
  - [x] Update test implementation
  - [x] Verify behavior with tests
- [x] Update projection and supporting code (Phase 3)
  - [x] Check projection machinery (already using `Lbl`)
  - [x] Check introspection traits (already using `Lbl`)
- [x] Update utility traits and test cases (Phase 3)
  - [x] Verify utility traits (already using `Lbl`)
- [x] Run full test suite for verification (Phase 4)
  - [x] Run `cargo test` to verify all tests pass
  - [x] Run `cargo clippy` to check for any linting issues
- [x] Update documentation and examples (Phase 5)
  - [x] Update review document with standardized parameter names
  - [x] Verify README.md examples
  - [x] Update CHANGELOG.md to record these changes
- [x] Verify examples and doctests (Phase 6)
  - [x] Run `cargo test` to verify all doctests pass
  - [x] Ensure consistent parameter naming in all examples

## Completed Changes

- Created `tests/label_tests.rs` with comprehensive test suite for label parameter behavior
- Updated `TEnd<IO, L>` to `TEnd<IO, Lbl>` in type definition and implementations
- Updated `TInteract<IO, L, R, H, T>` to `TInteract<IO, Lbl, R, H, T>` in type definition and implementations
- Updated `TRec<IO, L, S>` to `TRec<IO, Lbl, S>` in type definition and implementations
- Verified projection machinery and introspection traits are already using standardized `Lbl` parameter
- Added label extraction traits for testing
- Verified behavior with test suite
- Ran comprehensive test suite and clippy checks, all passing
- Updated documentation files to use standardized parameter names
- Updated CHANGELOG.md to reflect all our changes
- Completed all phases of the label parameter standardization work

