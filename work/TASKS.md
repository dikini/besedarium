# Project Tasks: Besedarium

This task list is re-structured from `work/Plan.md` to prioritize core library
implementation, followed by testing, advanced features, and documentation,
incorporating the development approach notes below.

## Development Approach Notes

- **Priority on Core Implementation**: Focus on correctly implementing library features, especially changes related to `docs/duality.md` (structs, traits, interfaces).
- **Doccomments**: Defer extensive doccomments until core functionality is stable.
- **Unit Tests**: Implement unit tests in parallel with feature development. Existing unit tests may need significant rework or replacement.
- **Integration Tests**: Plan to redo integration tests. Consider removing existing ones initially to avoid churn.
- **Documentation**: Address full documentation after the implementation and testing phases are mature.

## Phase 1: Core Protocol & Duality Implementation (Guided by `docs/duality.md`)

This phase focuses on implementing the foundational elements of the library,
with a strong emphasis on the concepts outlined in `docs/duality.md`.

**Note:** Implementation prompts for all Task 1.1 subtasks have been created:

- Implementation prompts: `work/prompts/1.1.1-foundation-types.md` through `1.1.5-projection-trait.md`
- Verification prompt: `work/prompts/1.1-verification-comprehensive.md`
- Debugging prompt: `work/prompts/1.1-debugging-comprehensive.md`
- Execution plan: `work/prompts/1.1-implementation-execution.md`

**✅ Legacy Code Cleanup Completed:**

- Removed conflicting files: `src/types.rs`, `src/protocol/base.rs`, `src/protocol/global.rs`, `src/protocol/local.rs`, `src/protocol/transforms/`
- Disabled test files using legacy patterns: All `tests/*.rs` and `tests/protocols/*.rs` files using old protocol types
- Temporarily disabled introspection and utilities: `src/introspection.rs`, `src/protocol/utils.rs`, `src/protocol/test_overrides.rs`
- Created modular directory structure: `src/protocol/{foundation,global,local,duality,projection}/`
- Library compiles successfully with clean foundation for Task 1.1 implementation

- [x] **Task 1.1**: Implement Core Types and Traits based on `duality.md`
  - [x] **Task 1.1.1**: Define/Update `CommMetadata` (including `ChanId`, `MsgLbl`, and `ActionIOType` concepts).
    - [x] **Task 1.1.1a**: Implement foundation trait definitions (`Role`, `Message`, `GlobalProtocol`, `LocalProtocol`) as defined in updated `duality.md`.
    - [x] **Task 1.1.1b**: Implement concrete `CommMetadata<C, L>` struct with `new()` method.
    - [x] **Task 1.1.1c**: Implement `ChanId` and `MsgLbl` traits with example types (`DefaultChan`, `HandshakeChan`, etc.).
    - [x] **Task 1.1.1d**: Implement `ActionIOTMarker` trait and standard action I/O types (`InputAction`, `OutputAction`, `BiDirectionalAction`).
    - [x] **Task 1.1.1e**: Implement `SupportsActionIO<AIO>` trait for capability verification.
  - [x] **Task 1.1.2**: Implement Global Protocol Types (e.g., `TChanSend`, `TChanRecv`, `TChanOffer`, `TChanChoice`, `TChanPar`, `TChanRec`, `TChanVar`, `TChanEnd`, `TChanContinue`, `TChanStart`) incorporating `CommMetadata` and `ActionIOType`.
  - [x] **Task 1.1.3**: Implement Local Endpoint Types (e.g., `EpSend`, `EpRecv`, `EpOffer`, `EpChoice`, etc.) ensuring consistent `IO` parameter handling and `SupportsActionIO` trait integration.
  - [x] **Task 1.1.4**: Implement the `IsDual` predicate/trait for verifying duality between protocol specifications, considering `CommMetadata`, message types, and `IO` consistency.
  - [x] **Task 1.1.5**: Implement the `Project<P, Role>` trait for projecting Global Protocols to Local Endpoint Types, ensuring `SupportsActionIO` checks. (**COMPLETED** - migrated to foundation types with role-based dispatch)
    - [x] **Task 1.1.5a**: Implement core `Project<P, R>` trait with `Output: LocalProtocol` associated type.
    - [x] **Task 1.1.5b**: Implement projection for `TChanSend` to `EpSend`/`EpRecv` based on role involvement.
    - [x] **Task 1.1.5c**: Implement projection for `TChanChoice` to `EpChoice`/`EpOffer` constructs.
    - [x] **Task 1.1.5d**: Implement projection for `TChanPar`, `TChanRec`, `TChanEnd` constructs.
    - [x] **Task 1.1.5e**: Implement helper traits (`ProjectChoices`, `Not<Role>`, `ValidProjection`) for complex projections.
    - [x] **Task 1.1.5f**: Implement `ProjectionError` type and validation mechanisms. (**COMPLETED**)
  - [ ] **Task 1.1.6**: Code Quality Improvements
    - [x] **Task 1.1.6a**: Fix critical clippy warnings (empty line after doc comment, unused Sealed trait). (**COMPLETED** - 2025-05-24)
    - [x] **Task 1.1.6b**: Implement `Default` trait for all protocol types with `new()` methods to improve API ergonomics and satisfy clippy suggestions. (**COMPLETED** - 2025-05-24)
      - [x] Global Protocol Types (7): TChanSend, TChanRecv, TChanChoice, TChanOffer, TChanPar, TChanEnd, TChanStart ✅
      - [x] Local Protocol Types (7): EpChanSend, EpChanRecv, EpChanOffer, EpChanChoice, EpChanPar, EpChanEnd, EpChanStart ✅
      - **Benefit**: Allows `Default::default()` usage for cleaner API and better integration with generic code ✅
      - **Status**: All 14 clippy suggestions resolved, auto-generated implementations working correctly ✅
    - [x] **Task 1.1.6c**: Module Structure Compliance (High Priority - guideline violations identified) (**COMPLETED** - 2025-05-24)
      - [x] **Task 1.1.6c.1**: Extract embedded tests from duality and projection modules (immediate) ✅ COMPLETED
        - [x] Extract 94-line test module from `duality/mod.rs` (lines 476-569) to `duality/tests.rs` ✅
        - [x] Extract 69-line test module from `projection/mod.rs` (lines 460-528) to `projection/tests.rs` ✅
        - [x] Update module declarations from embedded `mod tests {` to `mod tests;` ✅
        - **Result**: Reduced duality from 569→476 lines, projection from 528→460 lines ✅
      - [ ] **Task 1.1.6c.2**: Restructure oversized modules to meet 300-line guideline (advanced)
        - [ ] Restructure `duality/` module (475 lines after test extraction)
          - [ ] Extract helper traits to `duality/helpers.rs` (~50 lines)
          - [ ] Extract global protocol implementations to `duality/global_impls.rs` (~150 lines)
          - [ ] Extract local endpoint implementations to `duality/local_impls.rs` (~100 lines)
          - [ ] Keep core trait and validation in `duality/mod.rs` (~175 lines)
        - [ ] Restructure `projection/` module (459 lines after test extraction)
          - [ ] Extract error types to `projection/errors.rs` (~50 lines)
          - [ ] Extract validation logic to `projection/validation.rs` (~100 lines)
          - [ ] Extract projection implementations to `projection/impls.rs` (~150 lines)
          - [ ] Keep core trait in `projection/mod.rs` (~159 lines)
        - [ ] Restructure `local/` module (448 lines)
          - [ ] Extract endpoint type definitions to `local/endpoints.rs` (~200 lines)
          - [ ] Extract trait implementations to `local/impls.rs` (~150 lines)
          - [ ] Keep core types in `local/mod.rs` (~98 lines)
        - [ ] Restructure `global/` module (434 lines)
          - [ ] Extract protocol type definitions to `global/protocols.rs` (~200 lines)
          - [ ] Extract trait implementations to `global/impls.rs` (~150 lines)
          - [ ] Keep core types in `global/mod.rs` (~84 lines)
      - [x] **Task 1.1.6c.3**: Validation and compliance verification ✅ COMPLETED
        - [x] Verify all tests still pass: `cargo test` ✅ (35 tests pass)
        - [x] Verify formatting: `cargo fmt --all -- --check` ✅
        - [x] Verify linting: `cargo clippy` ✅
        - [x] Verify compilation: `cargo build` ✅
        - [x] Confirm test extraction successful ✅ (duality: 569→476 lines, projection: 528→460 lines)
        - [ ] Confirm all modules now meet 300-line guideline (2 modules still exceed: local=448, global=434)
        - [ ] Confirm implementation/test ratios meet 2:1 guideline (current: duality=5.1:1, projection=6.8:1)
      - **Justification**: 4 out of 5 protocol modules exceed 300-line size guideline. Poor test organization with embedded tests in duality/projection modules. Non-compliant implementation/test ratios.
      - **Priority**: High - affects maintainability and adherence to established coding standards
- [ ] **Task 1.2**: Implement Label Preservation and Transformation Logic
  - [x] **Task 1.2.1**: Research label behavior in `Choice`, `Parallel`, `Rec` to inform design. (**COMPLETED** - documented in updated `duality.md`)
  - [x] **Task 1.2.2**: Design type-level traits for label transformations (e.g., `TMap`, `TCollect`, `TFilter`). (**COMPLETED** - detailed in updated `duality.md`)
  - [ ] **Task 1.2.3**: Implement the designed label transformation traits and integrate them with protocol types.
    - [ ] **Task 1.2.3a**: Implement core `Label` trait and `LabelList` operations.
    - [ ] **Task 1.2.3b**: Implement `TMap`, `TCollect`, `TFilter` traits for label transformations.
    - [ ] **Task 1.2.3c**: Implement `LabelPreservation` and `LabelComposition` traits.
    - [ ] **Task 1.2.3d**: Implement label validation traits (`ValidateChoiceLabels`, `LabelValidation`).
    - [ ] **Task 1.2.3e**: Integrate label handling with `Choice`, `Offer`, `Parallel`, and `Recursion` constructs.
- [ ] **Task 1.3**: Implement Basic Runtime Components
  - [ ] **Task 1.3.1**: Design a foundational runtime state machine for protocol execution.
  - [ ] **Task 1.3.2**: Implement basic channel communication logic for sending/receiving typed messages according to protocol specifications.
  - [ ] **Task 1.3.3**: Define core error types for protocol violations and communication errors at runtime.
- [ ] **Task 1.4**: Research and Formalize Duality Concepts
  - [x] **Task 1.4.1**: Further research formal definitions of duality for all implemented primitives. (**COMPLETED** - enhanced in `duality.md`)
  - [x] **Task 1.4.2**: Investigate and document how duality is checked at the type level within the Rust implementation. (**COMPLETED** - documented in `duality.md`)
  - [ ] **Task 1.4.3**: Explore and document potential for generating dual protocols or verifying compatibility automatically.

## Documentation Enhancement Summary (2025-05-24)

**Recent Updates to `docs/duality.md`:**

- [x] **Added Foundation Types Section**: Concrete Rust implementations for all core traits (`Role`, `Message`, `GlobalProtocol`, `LocalProtocol`, `CommMetadata`, `ActionIOTMarker`, `SupportsActionIO`)
- [x] **Added Projection Implementation Details**: Complete `Project<P, Role>` trait implementations with examples for all major protocol constructs
- [x] **Added Label Transformation Logic**: Comprehensive type-level label operations (`TMap`, `TCollect`, `TFilter`) and validation mechanisms
- [x] **Enhanced with Practical Examples**: Concrete types and implementation patterns ready for direct use in implementation

## Phase 2: Core Feature Testing

This phase focuses on ensuring the correctness and robustness of the core library
features implemented in Phase 1. Unit tests should be developed in parallel with
Phase 1 tasks.

- [ ] **Task 2.1**: Develop Unit Tests for Core Types and Duality Logic
  - [ ] **Task 2.1.1**: Write unit tests for `CommMetadata`, Global Types, and Local Endpoint Types.
  - [ ] **Task 2.1.2**: Write unit tests for the `IsDual` predicate/trait, covering various protocol constructs.
  - [ ] **Task 2.1.3**: Write unit tests for the `Project<P, Role>` trait.
  - [ ] **Task 2.1.4**: Write unit tests for `SupportsActionIO` and `ActionIOType` integration.
- [ ] **Task 2.2**: Develop Unit Tests for Label Logic
  - [ ] **Task 2.2.1**: Write unit tests for label transformation traits (`TMap`, `TCollect`, `TFilter`).
- [ ] **Task 2.3**: Develop Unit Tests for Basic Runtime Components
  - [ ] **Task 2.3.1**: Write unit tests for the runtime state machine.
  - [ ] **Task 2.3.2**: Write unit tests for channel communication logic.
  - [ ] **Task 2.3.3**: Write unit tests for error handling.
- [ ] **Task 2.4**: Develop Integration Tests (Complex Protocol Examples)
  - [ ] **Task 2.4.1**: Remove or overhaul existing integration tests.
  - [ ] **Task 2.4.2**: Implement multi-party protocol examples as integration tests.
  - [ ] **Task 2.4.3**: Implement examples with complex data serialization as integration tests.
  - [ ] **Task 2.4.4**: Implement examples integrating with async runtimes as integration tests.

## Phase 3: Advanced Features, Optimizations & Tooling

With a tested core library, this phase focuses on enhancements, performance, and
developer experience.

- [ ] **Task 3.1**: Enhance Runtime Checks and Error Handling
  - (Further develop tasks from 1.3, adding more sophisticated mechanisms)
- [ ] **Task 3.2**: Investigate Performance and Optimization Opportunities
  - [ ] **Task 3.2.1**: Profile compile times of type-level constructs.
  - [ ] **Task 3.2.2**: Benchmark runtime performance for common protocol operations.
  - [ ] **Task 3.2.3**: Explore and implement optimization techniques for type-level logic and runtime.
- [ ] **Task 3.3**: Explore Macro-Based DSL for Protocol Definition
- [ ] **Task 3.4**: Consider Integration with Existing Actor Frameworks
- [ ] **Task 3.5**: Develop Visualization Tools for Protocols (Optional/Future)

## Phase 4: Comprehensive Documentation

This phase focuses on creating user-facing documentation and in-code doccomments
once the library features are stable and well-tested.

- [ ] **Task 4.1**: Create Core Concept Documentation
  - [ ] **Task 4.1.1**: Create `docs/Projections.md` (detailing `Project<P, Role>`).
  - [ ] **Task 4.1.2**: Create `docs/recursion.md` (detailing `Rec<P>`, `Var<N>`).
  - [ ] **Task 4.1.3**: Update/Refine `docs/duality.md` with implementation insights.
- [ ] **Task 4.2**: Write Comprehensive Usage Examples
  - [ ] **Task 4.2.1**: Create `docs/protocol-examples.md` (leveraging integration tests from Task 2.4).
- [ ] **Task 4.3**: Update Project README
  - [ ] **Task 4.3.1**: Refresh `README.md` with current status, features, and quick start.
- [ ] **Task 4.4**: Add Doccomments
  - [ ] **Task 4.4.1**: Add comprehensive doccomments to all public types, traits, and functions in the library.
- [ ] **Task 4.5**: Review and Update Internal Documentation
  - [ ] **Task 4.5.1**: Review and update `work/learnings.md` with insights from all phases.

## Ongoing Tasks

- [ ] **Refactoring**: Continuously refactor code for clarity, efficiency, and maintainability.
  - [ ] Refactor `Label` and `Labelled` types (as part of Task 1.2 or ongoing).
  - [ ] Standardize error handling (as part of Task 1.3, 3.1 or ongoing).
  - [ ] Improve `Projection` trait (as part of Task 1.1.5 or ongoing).
- [ ] **Learning & Research**: Stay updated with advancements in type-level programming, session types, and formal methods.
- [ ] **Changelog & Status**: Maintain `CHANGELOG.md` and `work/Status.md`.
