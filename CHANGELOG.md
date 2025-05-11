# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Initial changelog file following Keep a Changelog style.
- Comprehensive compile-time test suite for all combinators, macros, and mixed-protocol scenarios.
- Improved error messages for type equality and disjointness assertions.
- More protocol examples as individual #[test] functions: multi-party workflow, recursive streaming with choice, concurrent sub-sessions, and Mixed marker usage. These are now reported as separate tests in cargo test output.
- README.md with approachable, non-academic documentation and mermaid diagrams for all main protocol examples.
- README.md is now included as module-level documentation via `#![doc = include_str!("../README.md")]` for docs.rs and cargo doc.
- `extract_roles!` macro for compile-time role extraction from protocol types.
- Improved documentation for all macros, with clear usage examples.

### Changed
- Refactored integration tests to avoid macro name collisions.
- Updated documentation and project plan to reflect completed tasks.
- Refactored n-ary combinator macros and trait implementations to use canonical Rust pattern (no automatic TEnd<IO> appending, base case for Nil, recursive case for Cons).
- Updated tests to match new macro and trait pattern, removed invalid mixed-protocol type equality assertion, and commented out failing n-ary macro/manual type equality assertion.
- Updated the plan to mark all realistic and illustrative example protocol tasks as completed.
- Protocol examples are now in separate files under `tests/protocols/` for better discoverability and documentation. Test suite updated to import and use these examples.
- Empty protocol (should fail) tests are now fully automated as trybuild compile-fail tests and always run.
- Concrete roles, messages, and IO marker types moved to `src/test_types.rs` for clarity and reuse in tests/examples.
- Library structure reviewed: simple, single-file core with a dedicated module for test/example types. No further modularization for now.
- All macro ergonomics and safety tasks are now complete, with only attribute/proc macros deferred for future consideration.
- Plan updated: Duality, Labelled Ends, and Multiparty Extensions (Projections) are deferred/future, with rationale for each.

### Removed
- Removed main.rs and moved all logic to lib.rs for a library-only crate structure.

### Fixed
- Resolved trait overlap and type equality issues in n-ary combinators and macros.
- Ensured all tests compile and pass with the new pattern.

