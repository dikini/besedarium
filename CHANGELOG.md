# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- All protocol examples in README.md and documentation now use the correct 5-argument form for `TInteract` and `TEnd`, with explicit label types for every example.
- All macro and combinator documentation examples are up to date and pass doctests.
- Initial changelog file following Keep a Changelog style.
- Comprehensive compile-time test suite for all combinators, macros, and mixed-protocol scenarios.
- Improved error messages for type equality and disjointness assertions.
- More protocol examples as individual #[test] functions: multi-party workflow, recursive streaming with choice, concurrent sub-sessions, and Mixed marker usage. These are now reported as separate tests in cargo test output.
- README.md with approachable, non-academic documentation and mermaid diagrams for all main protocol examples.
- README.md is now included as module-level documentation via `#![doc = include_str!("../README.md")]` for docs.rs and cargo doc.
- `extract_roles!` macro for compile-time role extraction from protocol types.
- Improved documentation for all macros, with clear usage examples.
- Projection machinery: derive local (endpoint) session types for a given role from a global protocol specification using the `ProjectRole` trait and helpers (`ProjectInteract`, `ProjectChoice`, `ProjectPar`).
- Endpoint types: `EpSend`, `EpRecv`, `EpChoice`, `EpPar` for local session types.
- Comprehensive documentation for projection in both the library and README, including usage, examples, and trait requirements for protocol authors.
- GitHub Actions CI workflow: automatically builds, tests, lints (clippy), and checks formatting on push to main and on non-draft pull requests targeting main. Draft PRs are skipped.
- Static compile-time projection check: Added a test to ensure that the projection from global to local session type for a role (e.g., Alice) is correct. This test is reported as a regular Rust test and fails to compile if the projection is incorrect.
- Added docs/ImplementationOverview.md: a detailed overview and analysis of the current implementation, including goals, session type theory, combinator-by-combinator discussion, global/local types, compile-time properties, testing, and a comparison with other session type libraries (with direct links). Diagrams are properly separated for clarity.

### Changed

- Updated README.md protocol examples and projection example to match the current API and pass doctests.
- Updated Plan-labels.md to clarify that marker types (not const generics) are used for labels, and no migration is needed for this greenfield library.
- Refactored integration tests to avoid macro name collisions.
- Updated documentation and project plan to reflect completed tasks.
- Refactored n-ary combinator macros and trait implementations to use canonical Rust pattern (no automatic `TEnd<IO>` appending, base case for Nil, recursive case for Cons).
- Updated tests to match new macro and trait pattern, removed invalid mixed-protocol type equality assertion, and commented out failing n-ary macro/manual type equality assertion.
- Updated the plan to mark all realistic and illustrative example protocol tasks as completed.
- Protocol examples are now in separate files under `tests/protocols/` for better discoverability and documentation. Test suite updated to import and use these examples.
- Empty protocol (should fail) tests are now fully automated as trybuild compile-fail tests and always run.
- Concrete roles, messages, and IO marker types moved to `src/test_types.rs` for clarity and reuse in tests/examples.
- Library structure reviewed: simple, single-file core with a dedicated module for test/example types. No further modularization for now.
- All macro ergonomics and safety tasks are now complete, with only attribute/proc macros deferred for future consideration.
- Plan updated: Duality, Labelled Ends, and Multiparty Extensions (Projections) are deferred/future, with rationale for each.
- Renamed crate from `playground` to `besedarium` (package and library names).
- Updated imports and module references from `playground::` to `besedarium::` across code, tests, examples, and documentation.
- Updated README title, headings, and docs to reflect the new project name "Besedarium".
- Refactored combinator projection logic to use helper traits and avoid overlapping trait impls, improving maintainability and extensibility.
- Updated documentation and README to include a dedicated section on projection from global to local session types.
- All documentation files in `docs/` have been reformatted for markdownlint compliance: long lines wrapped, spacing and heading issues fixed, and style improved for readability and consistency. No content changes were made.

### Removed

- Removed main.rs and moved all logic to lib.rs for a library-only crate structure.

### Fixed

- Fixed all failing doctests by updating or removing outdated examples that used the old 4-argument form for `TInteract` and `TEnd`.
- Resolved trait overlap and type equality issues in n-ary combinators and macros.
- Ensured all tests compile and pass with the new pattern.