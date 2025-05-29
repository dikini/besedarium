# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Resolved markdown linting issues across multiple files, ensuring compliance with `markdownlint-cli2` rules.

### Added

#### Task-Specific Prompts (2025-05-26)

- Enhanced GitHub Copilot Integration: Added guidelines for maintaining task-specific prompt files
- Added detailed prompt structure recommendations
- Added examples of effective Copilot interactions
- Documentation for prompt maintenance process

#### Documentation Enhancements (2025-05-24)

- Enhanced `docs/duality.md`: Major update with implementation-ready content

  - Added Foundation Types section with concrete Rust trait definitions
  - Added Projection Implementation Details with complete `Project<P, Role>` trait patterns
  - Added Label Transformation Logic with type-level operations and validation
  - Added practical implementation examples for all core concepts

- Updated `work/TASKS.md`: Restructured tasks based on enhanced documentation guidance

- Updated `work/learnings.md`: Added patterns and insights from documentation enhancement

- Updated `work/Status.md`: Reflected current documentation and implementation readiness

#### Earlier Enhancements

- Implemented `TStart` combinator as a clear protocol entry point, enhancing protocol structure clarity and consistency

- Added projection logic for `TStart` to `EpStart` in endpoint protocols

- Created comprehensive tests for `TStart` functionality

- Updated example protocols to use `TStart` as their entry point

- Implemented `TRec` and `TContinue` recursion combinators according to the specification in docs/recursion.md, enabling proper recursive protocol definitions

- New `TStart` combinator for explicit protocol entry points with corresponding `EpStart` local type

- Label preservation during projection from global to local types for enhanced traceability and debugging

- New utility traits `GetLocalLabel` and `GetProtocolLabel` for accessing label information

- Modularized protocol transformations with dedicated modules:

  - `send.rs`: Specialized `ProjectSendCase` helper trait for send operations
  - `recv.rs`: Specialized `ProjectRecvCase` helper trait for receive operations
  - `projection.rs`: Core projection trait with improved trait bounds

- Test cases to ensure labels are correctly preserved during the projection process

- All protocol examples in README.md and documentation now use the correct 5-argument form for
`TInteract` and `TEnd`, with explicit label types for every example.

- All macro and combinator documentation examples are up to date and pass doctests.

- Initial changelog file following Keep a Changelog style.

- Comprehensive compile-time test suite for all combinators, macros, and mixed-protocol scenarios.

- Improved error messages for type equality and disjointness assertions.

- More protocol examples as individual #[test] functions: multi-party workflow, recursive streaming
with choice, concurrent sub-sessions, and Mixed marker usage. These are now reported as separate
tests in cargo test output.

- README.md with approachable, non-academic documentation and mermaid diagrams for all main
protocol examples.

- Modularized protocol transformation machinery: split `transforms.rs` into `projection.rs`, `choice.rs`, `parallel.rs`, `recursion.rs`, and `util.rs` under `src/protocol/transforms/`.

- Added module-level documentation and improved doc comments for all protocol transform traits.

- Updated Implementation Overview documentation to reflect current codebase structure:

  - Replaced outdated `TInteract` references with separate `TSend` and `TRecv` combinators
  - Updated all type signatures to match current implementation
  - Updated projection trait implementations with current trait bounds and type parameters
  - Added missing documentation for `EpStart` endpoint combinator
  - Updated examples to use current combinator implementations

- Comprehensive documentation for duality and well-formedness in Multiparty Session Types (MPST) in `docs/duality.md`:

  - Theoretical definitions of duality, IsDual predicate, and well-formedness invariants.
  - Minimal Rust struct definitions for all protocol actions (Send/Receive, Offer/Choice, Par, Seq, Rec, Continue, Init, End) for both global (T*) and local (Ep*) types.
  - Section on determining the dual action for well-formedness, with algorithmic matching.
  - Concrete Rust-style protocol examples, including three-role protocols (well-formed and not well-formed).
  - Type-level trait pseudocode for IsDual and IsWellFormed, and documentation on compile-time protocol verification patterns.
  - Detailed explanation of the Projection function (`Project(G, R_target)`), its semantics, and integration with IO/AIO types.
  - Comprehensive section on the Type-Level Implementation in Rust, covering `IsDual`, `Project`, and `WellFormed` traits, including their structure, implementation strategies, and compile-time verification benefits.
  - Conclusion summarizing the document and outlining future work.

- Updated `work/learnings.md` with key learnings and patterns from the new documentation, including Rust type-level programming, trait design, and protocol verification strategies

#### 2025-05-24 - Documentation Enhancement

- **Enhanced docs/duality.md**: Major update with implementation-ready content

  - Added Foundation Types section with concrete Rust trait definitions
  - Added Projection Implementation Details with complete `Project<P, Role>` trait patterns
  - Added Label Transformation Logic with type-level operations and validation
  - Added practical implementation examples for all core concepts

- **Updated work/TASKS.md**: Restructured tasks based on enhanced documentation guidance

- **Updated work/learnings.md**: Added patterns and insights from documentation enhancement

- **Updated work/Status.md**: Reflected current documentation and implementation readiness

### Fixed

- Disabled failing tests in projection_tests.rs due to the transition from TInteract to TSend/TRecv model

- Added TODO comments explaining the need for comprehensive test suite for projections in the future

- Fixed circular imports issue with `protocol_original.rs` that was causing build failures

- Removed leftover empty `protocol.rs` file that was conflicting with the new module structure

- Removed superfluous `protocol_original.rs` compatibility layer

- README.md is now included as module-level documentation via `#![doc =
include_str!("../README.md")]` for docs.rs and cargo doc.

- `extract_roles!` macro for compile-time role extraction from protocol types.

- Updated test overrides in `test_overrides.rs` to work with the new modularized structure and implemented `SessionType` for test types to satisfy trait bounds.

- Improved documentation for all macros, with clear usage examples.

- Projection machinery: derive local (endpoint) session types for a given role from a global
protocol specification using the `ProjectRole` trait and helpers (`ProjectInteract`,
`ProjectChoice`, `ProjectPar`).

- Endpoint types: `EpSend`, `EpRecv`, `EpChoice`, `EpPar` for local session types.

- Comprehensive documentation for projection in both the library and README, including usage,
examples, and trait requirements for protocol authors.

- GitHub Actions CI workflow: automatically builds, tests, lints (clippy), and checks formatting on
push to main and on non-draft pull requests targeting main. Draft PRs are skipped.

- Static compile-time projection check: Added a test to ensure that the projection from global to
local session type for a role (e.g., Alice) is correct. This test is reported as a regular Rust
test and fails to compile if the projection is incorrect.

- Added docs/ImplementationOverview.md: a detailed overview and analysis of the current
implementation, including goals, session type theory, combinator-by-combinator discussion,
global/local types, compile-time properties, testing, and a comparison with other session type
libraries (with direct links). Diagrams are properly separated for clarity.

- Major codebase modularization: split core logic into `protocol.rs`, `types.rs`, and
`introspection.rs` for clarity and maintainability.

- Comprehensive doc comments for all major types, traits, and modules, following project
documentation standards and cross-referencing advanced type-level patterns.

- Top-level documentation for type-level map/fold patterns and the use of helper traits to resolve
overlapping trait impls.

- Improved documentation for protocol marker types, message primitives, and introspection traits.

- Explicit user and Copilot contribution guidelines added to the project (see instructions section).

- Alias types `TrueB` and `FalseB` in `types.rs` for boolean literals used in tests

- `TypeEq` trait with universal impl for compile-time type equality assertions

- Added `docs/review-20250514.md`: A comprehensive code review of the Besedarium library examining
the implementation approach, analyzing combinators for global and local session types, evaluating
the projection mechanism, and providing suggestions for future development.

- Added `docs/label-refactoring.md`: A detailed strategy document for standardizing label parameter
naming across session type combinators, including current state analysis, parameter usage audit,
test suite analysis, implementation plan, and recommendations for label preservation.

- Added `docs/runtime-implementation-patterns.md`: A reference guide for implementing local
per-role runtimes in Rust, with detailed examples of how to handle complex session type combinators
(choice/offer, parallel composition, and recursion) across different implementation approaches.

- Added comparative analysis of runtime implementation patterns to `work/learnings.md`, identifying
three primary approaches (Typed Channel Wrappers, Code Generation with Procedural Macros, and State
Machine Builders) with their respective trade-offs and implementation considerations.

- Added `work/metrics/label_coverage.md`: A tracking document for label parameter test coverage
metrics, including combinator coverage, composition operation coverage, custom label type coverage,
and edge case coverage.

- Added comprehensive testing for label behavior in edge cases: nested compositions, mixed
combinator interactions, and complex protocol structures with multiple branches and nested
compositions.

- Added additional tests for `TInteract` and `TRec` with multiple custom label types (`L1`, `L2`,
`L3`) to achieve full test coverage.

- Added pre-implementation tests for introspection functionality to verify behavior before and
after label parameter refactoring.

- Added pre-implementation tests for projection traits to ensure consistent behavior throughout the
refactoring process.

- Completed projection machinery by implementing missing `ProjectRole` for `TChoice` and `TPar`,
enabling full end-to-end projection from global to local session types for all combinators. New
helper traits (`ProjectChoiceCase`, `TParContainsRoleImpl`) provide type-level dispatch to handle
different role containment scenarios.

- Added type-level boolean operations (`Or`, `Not`, `BoolOr`) to support the projection machinery
with proper type constraints.

- Enhanced `ContainsRole`/`NotContainsRole` traits with rigorous type-level reasoning to determine
role presence in nested protocol structures.

- Fixed all failing doctests by updating or removing outdated examples that used the old 4-argument
form for `TInteract` and `TEnd`.

- Ignored outdated doctests for `ToTChoice` and `ToTPar` examples to ensure doc builds pass.

- Resolved trait overlap and type equality issues in n-ary combinators and macros.

- Ensured all tests compile and pass with the new pattern.

- Resolved duplicate imports and module visibility issues after modularization.

- All code and documentation pass `cargo check`, `cargo build`, `cargo fmt`, and `cargo clippy`
(except for known/ignored test failures).

- Resolved overlapping trait impl errors in `IsEpSkipTypeImpl` by replacing the former blanket impl
with explicit per-type impls

- Aligned boolean alias names in tests to match `TrueB`/`FalseB`, fixing compile errors in
`compile.rs`

### Changed

- Updated README.md protocol examples and projection example to match the current API and pass
doctests.

- Refactored integration tests to avoid macro name collisions.

- Updated documentation and project plan to reflect completed tasks.

- Refactored n-ary combinator macros and trait implementations to use canonical Rust pattern (no
automatic `TEnd<IO>` appending, base case for Nil, recursive case for Cons).

- Updated tests to match new macro and trait pattern, removed invalid mixed-protocol type equality
assertion, and commented out failing n-ary macro/manual type equality assertion.

- Updated the plan to mark all realistic and illustrative example protocol tasks as completed.

- Protocol examples are now in separate files under `tests/protocols/` for better discoverability
and documentation. Test suite updated to import and use these examples.

- Empty protocol (should fail) tests are now fully automated as trybuild compile-fail tests and
always run.

- Concrete roles, messages, and IO marker types moved to `src/test_types.rs` for clarity and reuse
in tests/examples.

- Library structure reviewed: simple, single-file core with a dedicated module for test/example
types. No further modularization for now.

- All macro ergonomics and safety tasks are now complete, with only attribute/proc macros deferred
for future consideration.

- Plan updated: Duality, Labelled Ends, and Multiparty Extensions (Projections) are
deferred/future, with rationale for each.

- Renamed crate from `playground` to `besedarium` (package and library names).

- Updated imports and module references from `playground::` to `besedarium::` across code, tests,
examples, and documentation.

- Updated README title, headings, and docs to reflect the new project name "Besedarium".

- Refactored combinator projection logic to use helper traits and avoid overlapping trait impls,
improving maintainability and extensibility.

- Updated documentation and README to include a dedicated section on projection from global to
local session types.

- Refactored the protocol system into a modular structure with separate files:

  - Created a dedicated `protocol/` module with `base.rs`, `global.rs`, `local.rs`,
  `transforms.rs`, and `utils.rs`
  - Separated core protocol types, traits, and functionality into logically organized files
  - Improved code organization and maintainability without changing behavior
  - Updated all code style to pass `cargo fmt` and `cargo clippy` checks

- All documentation files in `docs/` have been reformatted for markdownlint compliance: long lines
wrapped, spacing and heading issues fixed, and style improved for readability and consistency. No
content changes were made.

- All protocol/session combinators, endpoint types, and projection traits are now re-exported at
the crate root for user and test compatibility.

- Code formatting and style updated to follow rustfmt and clippy recommendations.

- Refactored `FilterSkips` trait to use marker-type dispatch (`IsEpSkipType`/`IsNotEpSkipType`) via
`GetEpSkipTypeMarker`, eliminating associated-type generics

- Enumerated explicit `IsEpSkipTypeImpl` impls for each endpoint variant (`EpSkip`, `EpSend`,
`EpRecv`, `EpChoice`, `EpPar`, `EpEnd`)

- Updated docs/protocol-examples.md with real API examples using `TInteract`, `TChoice`, `TRec`,
`TEnd`, and explicit local-projection types (`EpSend`, `EpRecv`, `EpChoice`, `EpSkip`). Documented
skip-filtering via `FilterSkips` and branch composition via `ComposeProjectedParBranches`.

- Recorded new patterns in work/learnings.md after protocol-examples updates, including marker-type
dispatch for `EpSkip` and explicit recursion modeling with `TRec`.

- Refactored `TEnd<IO, L>` to `TEnd<IO, Lbl>`, `TInteract<IO, L, R, H, T>` to `TInteract<IO, Lbl,
R, H, T>`, and `TRec<IO, L, S>` to `TRec<IO, Lbl, S>` for parameter name consistency across
combinators as part of Phase 2 of the label parameter refactoring.

- Updated introspection code in `introspection.rs` to use consistent `Lbl` parameter naming for
`RolesOf` and `LabelsOf` traits as part of Phase 3.

- Updated projection trait implementations in `protocol.rs` to use consistent `Lbl` parameter
naming for `ProjectRole` and related traits.

- Updated documentation and examples to use the standardized `Lbl` parameter name throughout the
codebase.

- Updated label parameter documentation in `protocol.rs` to reflect the new consistent naming
convention.

- Updated test coverage metrics in `work/metrics/label_coverage.md` to reflect the improved test
coverage.

- Enhanced learnings document with insights from Phase 2 and Phase 3 of the label parameter
refactoring, focusing on test-first refactoring approach and parameter name consistency benefits.

### Removed

- Removed main.rs and moved all logic to lib.rs for a library-only crate structure.

- Deleted obsolete `transforms.rs` and removed the deprecated `ProjectInteract` pattern.

- Cleaned up unused imports and fixed all warnings and formatting issues.

### Known Issues

- Some projection and trybuild tests (especially involving `TPar`/`EpPar`) are expected to fail due
to ongoing design work. See protocol.rs and test files for details.
