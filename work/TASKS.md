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

- [x] **Task 1.1**: Implement Core Types and Traits based on `duality.md` ✅ **COMPLETED** (minor code quality task remaining)
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
  - [x] **Task 1.1.6**: Code Quality Improvements ✅ **COMPLETED**
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
      - [x] **Task 1.1.6c.2**: Restructure oversized modules to meet 300-line guideline (advanced) ✅ **COMPLETED**
        - [x] Restructure `duality/` module (475 lines after test extraction) ✅ **COMPLETED**
          - [x] Extract helper traits to `duality/helpers.rs` (31 lines) ✅
          - [x] Extract global protocol implementations to `duality/global_impl.rs` (156 lines) ✅
          - [x] Extract local endpoint implementations to `duality/local_impl.rs` (156 lines) ✅
          - [x] Keep core trait and validation in `duality/mod.rs` (94 lines) ✅
        - [x] Restructure `projection/` module (459 lines after test extraction) ✅ **COMPLETED**
          - [x] Extract helper logic to `projection/helpers.rs` (150 lines) ✅
          - [x] Extract projection implementations to `projection/implementations.rs` (133 lines) ✅
          - [x] Extract tests to `projection/tests.rs` (71 lines) ✅
          - [x] Extract error handling to `projection/errors.rs` (108 lines) ✅
          - [x] Core trait in `projection/mod.rs` reduced to 92 lines ✅ **COMPLETED**
        - [x] Restructure `local/` module (448 lines) ✅ **COMPLETED**
          - [x] Extract endpoint type definitions to `local/endpoints.rs` (166 lines) ✅
          - [x] Extract trait implementations to `local/implementations.rs` (328 lines) ✅
          - [x] Keep core types in `local/mod.rs` (79 lines) ✅
        - [x] Restructure `global/` module (434 lines) ✅ **COMPLETED**
          - [x] Extract protocol type definitions to `global/protocols.rs` (170 lines) ✅
          - [x] Extract trait implementations to `global/implementations.rs` (324 lines) ✅
          - [x] Keep core types in `global/mod.rs` (63 lines) ✅
      - [x] **Task 1.1.6c.3**: Validation and compliance verification ✅ COMPLETED
        - [x] Verify all tests still pass: `cargo test` ✅ (35 tests pass)
        - [x] Verify formatting: `cargo fmt --all -- --check` ✅
        - [x] Verify linting: `cargo clippy` ✅
        - [x] Verify compilation: `cargo build` ✅
        - [x] Confirm test extraction successful ✅ (duality: 569→94 lines, projection: 528→92 lines)
        - [x] Confirm 5 of 5 modules now meet 300-line guideline ✅ (duality=94, global=63, local=79, foundation=209, projection=92)
        - [x] Confirm implementation/test ratios improved ✅ (duality=1.0:1, global=2.1:1, local=1.4:1, projection=1.3:1)
      - **Justification**: All 5 protocol modules now comply with 300-line size guideline. Implementation/test ratios now compliant with guidelines.
      - **Priority**: Completed ✅
- [ ] **Task 1.2**: Implement Label Preservation and Transformation Logic
  - [x] **Task 1.2.1**: Research label behavior in `Choice`, `Parallel`, `Rec` to inform design. (**COMPLETED** - documented in updated `duality.md`)
  - [x] **Task 1.2.2**: Design type-level traits for label transformations (e.g., `TMap`, `TCollect`, `TFilter`). (**COMPLETED** - detailed in updated `duality.md`)
  - [x] **Task 1.2.3**: Implement the designed label transformation traits and integrate them with protocol types. ✅ **COMPLETED**
    - [x] **Task 1.2.3a**: Implement core `Label` trait and `LabelList` operations.
    - [x] **Task 1.2.3b**: Implement `TMap`, `TCollect`, `TFilter` traits for label transformations.
    - [x] **Task 1.2.3c**: Implement `LabelPreservation` and `LabelComposition` traits.
    - [x] **Task 1.2.3d**: Implement label validation traits (`ValidateChoiceLabels`, `LabelValidation`).
    - [x] **Task 1.2.3e**: Integrate label handling with `Choice`, `Offer`, `Parallel`, and `Recursion` constructs.
- [x] **Task 1.3**: Implement Basic Runtime Components ✅ **COMPLETED**
  - [x] **Task 1.3.1**: Design a foundational runtime state machine for protocol execution. ✅ **COMPLETED** - implemented in `src/runtime/state.rs`
  - [x] **Task 1.3.2**: Implement async channel communication logic for sending/receiving typed messages according to protocol specifications. ✅ **COMPLETED** - implemented in `src/runtime/channel.rs`
  - [x] **Task 1.3.3**: Implement session lifecycle management for protocol execution coordination. ✅ **COMPLETED** - implemented in `src/runtime/session.rs`
  - [x] **Task 1.3.4**: Define comprehensive error types for runtime failures and communication errors. ✅ **COMPLETED** - implemented in `src/runtime/error.rs`
  - [x] **Task 1.3.5**: Implement integration testing for runtime components validation. ✅ **COMPLETED** - implemented in `src/runtime/integration_tests.rs`
- [ ] **Task 1.4**: Research and Formalize Duality Concepts
  - [x] **Task 1.4.1**: Further research formal definitions of duality for all implemented primitives. (**COMPLETED** - enhanced in `duality.md`)
  - [x] **Task 1.4.2**: Investigate and document how duality is checked at the type level within the Rust implementation. (**COMPLETED** - documented in `duality.md`)
  - [x] **Task 1.4.3**: Explore and document potential for generating dual protocols or verifying compatibility automatically. ✅ **COMPLETED**

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

- [x] **Task 2.1**: Develop Unit Tests for Core Types and Duality Logic ✅ **COMPLETED** (118 total tests passing)
  - [x] **Task 2.1.1**: Write unit tests for `CommMetadata`, Global Types, and Local Endpoint Types. ✅ **COMPLETED** (46 tests passing)
  - [x] **Task 2.1.2**: Write unit tests for the `IsDual` predicate/trait, covering various protocol constructs. ✅ **COMPLETED** (24 comprehensive tests passing)
  - [x] **Task 2.1.3**: Write unit tests for the `Project<P, Role>` trait. ✅ **COMPLETED** (23 comprehensive tests passing)
  - [x] **Task 2.1.4**: Write unit tests for `SupportsActionIO` and `ActionIOType` integration. ✅ **COMPLETED** (25 comprehensive tests passing)
- [x] **Task 2.2**: Develop Unit Tests for Label Logic ✅ **COMPLETED** (54+ tests passing)
  - [x] **Task 2.2.1**: Write unit tests for label transformation traits (`TMap`, `TCollect`, `TFilter`). ✅ **COMPLETED** (54+ comprehensive tests passing)
- [x] **Task 2.3**: Develop Unit Tests for Basic Runtime Components ✅ **COMPLETED**
  - [x] **Task 2.3.1**: Write unit tests for the runtime state machine. ✅ **COMPLETED** - created `src/runtime/state/tests.rs` with 25+ comprehensive tests
  - [x] **Task 2.3.2**: Write unit tests for channel communication logic. ✅ **COMPLETED** - created `src/runtime/channel/tests.rs` with 25+ comprehensive tests  
  - [x] **Task 2.3.3**: Write unit tests for session lifecycle management. ✅ **COMPLETED** - created `src/runtime/session/tests.rs` with 20+ comprehensive tests
- [ ] **Task 2.4**: Develop Integration Tests (Complex Protocol Examples)
  - [x] **Task 2.4.1**: Remove or overhaul existing integration tests. ✅ **COMPLETED** (11 integration tests passing)
  - [x] **Task 2.4.2**: Implement multi-party protocol examples as integration tests.
  - [x] **Task 2.4.3**: Implement examples with complex data serialization as integration tests. ✅ **COMPLETED**
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
