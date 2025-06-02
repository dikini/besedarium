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
- [x] **Task 1.2**: Implement Label Preservation and Transformation Logic ✅ **COMPLETED**
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
- [x] **Task 1.4**: Research and Formalize Duality Concepts
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
- [x] **Task 2.4**: Develop Integration Tests (Complex Protocol Examples) ✅ **COMPLETED**
  - [x] **Task 2.4.1**: Remove or overhaul existing integration tests. ✅ **COMPLETED** (11 integration tests passing)
  - [x] **Task 2.4.2**: Implement multi-party protocol examples as integration tests. ✅ **COMPLETED** (covered by existing integration tests in `tests/client_server_integration.rs`)
  - [x] **Task 2.4.3**: Implement examples with complex data serialization as integration tests. ✅ **COMPLETED**
  - [x] **Task 2.4.4**: Implement examples integrating with async runtimes as integration tests. ✅ **COMPLETED**
- [x] **Task 2.5**: Address Remaining TODO Comments in Integration Tests ✅ **COMPLETED**
  - [x] **Task 2.5.1**: Implement missing error propagation test in `src/runtime/integration_tests.rs` ✅ **COMPLETED**
    - **Description**: Implemented comprehensive test in `test_error_propagation_integration()` using `StateValidator` and `validated_transition()` API
    - **Implementation**: Tests channel timeouts, state validation errors, and session error propagation with proper error handling
    - **Status**: Fully implemented and passing - covers error propagation across protocol boundaries
  - [x] **Task 2.5.2**: Implement multi-session concurrent execution test in `src/runtime/integration_tests.rs` ✅ **COMPLETED**
    - **Description**: Implemented comprehensive test in `test_multi_session_integration()` leveraging `SessionManager<P, R, AIO>` for coordinating multiple concurrent sessions
    - **Implementation**: Tests concurrent session lifecycle management, bulk operations, and resource tracking
    - **Status**: Fully implemented and passing - validates concurrent session behavior
  - [x] **Task 2.5.3**: Implement concurrent state and channel operations test in `src/runtime/integration_tests.rs` ✅ **COMPLETED**
    - **Description**: Implemented comprehensive test in `test_concurrent_state_channel_integration()` combining state machine transitions with channel communication under concurrent load
    - **Implementation**: Uses `Arc<Barrier>`, `Arc<AtomicUsize>`, and proper mutex guards with async safety for thread-safety testing
    - **Status**: Fully implemented and passing - validates thread-safety and concurrent access patterns
  - **Result**: All 12 integration tests pass successfully with no remaining TODO items. Comprehensive coverage of runtime components working together.

## Phase 3: Advanced Features, Optimizations & Tooling

With a tested core library, this phase focuses on enhancements, performance, and
developer experience.

- [x] **Task 3.1**: Enhance Runtime Checks and Error Handling (✅ **COMPLETED**)
  - [x] **Task 3.1.1**: Improve `RuntimeError::Serialization` to include an optional `source` field for better debugging of serialization issues. (✅ **COMPLETED**)
  - [x] **Task 3.1.2**: Implement Advanced State Transition Validation and Deadlock/Livelock Detection. (✅ **COMPLETED**)
    - [x] **Task 3.1.2a**: Enhanced error system with `DeadlockError`, `LivelockError`, and `StateValidationError` types.
    - [x] **Task 3.1.2b**: Comprehensive validation framework with configurable detection mechanisms.
    - [x] **Task 3.1.2c**: State machine integration with optional validation support.
    - [x] **Task 3.1.2d**: Resource allocation graph for deadlock detection.
    - [x] **Task 3.1.2e**: Progress tracking for livelock detection.
    - [x] **Task 3.1.2f**: Comprehensive test suite with 18 tests covering all validation scenarios.
  - [x] **Task 3.1.3**: Implement Robust Channel Communication with Timeouts and Enhanced Error Reporting. (✅ **COMPLETED**)
    - [x] **Task 3.1.3a**: Enhanced `CommunicationError` enum with detailed error variants (`ChannelOperationFailed`, `ChannelTimeout`, `DeserializationFailed`, `SerializationFailed`).
    - [x] **Task 3.1.3b**: Complete robust channel implementation with health monitoring, timeout management, and configurable timeout durations.
    - [x] **Task 3.1.3c**: Comprehensive serialization framework integration with serde for all channel communication components.
    - [x] **Task 3.1.3d**: Manual serde implementations for `CommMetadata<C, L>` with proper trait bounds and lifetime management.
    - [x] **Task 3.1.3e**: Foundation type updates with serde derives for concrete channel and label types.
    - [x] **Task 3.1.3f**: Enhanced channel methods with serialization trait bounds for type-safe communication.
    - [x] **Task 3.1.3g**: Critical bug fixes: Display implementation for `ChannelOperation`, Copy derive, dyn Role compatibility fixes.
    - [x] **Task 3.1.3h**: Comprehensive test suite verification with 98 foundation tests and channel-specific tests passing.
    - [x] **Task 3.1.3i**: Async health tracking race condition fix: Fixed timeout tests by making health failure recording synchronous to prevent race conditions between error reporting and health counter verification. (✅ **COMPLETED** - 2025-05-31)
  - [x] **Task 3.1.4**: Enhance Session Lifecycle Management with Graceful Shutdown and Resource Leak Detection. (✅ **COMPLETED**)
    - [x] **Task 3.1.4a**: Implement graceful shutdown mechanisms with configurable timeouts and signal-based coordination.
    - [x] **Task 3.1.4b**: Implement resource tracking and leak detection for channels, tasks, and other session-managed resources.
    - [x] **Task 3.1.4c**: Implement consistent termination handling with proper status management and cleanup.
    - [x] **Task 3.1.4d**: Implement comprehensive logging and reporting for lifecycle events and leak detection.
    - [x] **Task 3.1.4e**: Implement session manager capabilities for multi-session lifecycle management and bulk operations.
    - [x] **Task 3.1.4f**: Complete test suite with 16 comprehensive tests covering all lifecycle scenarios including timeout handling.
  - [x] **Task 3.1.5**: Refine Runtime Error Hierarchy and Contextual Information (✅ **COMPLETED**)
    - [x] **Task 3.1.5a**: Add error severity levels and categorization system for better error handling. (✅ **COMPLETED**)
    - [x] **Task 3.1.5b**: Enhance RuntimeError variants with more detailed contextual information. (✅ **COMPLETED**)
    - [x] **Task 3.1.5c**: Improve error display and debug formatting with structured output. (✅ **COMPLETED**)
    - [x] **Task 3.1.5d**: Add error recovery suggestions to error types. (✅ **COMPLETED**)
    - [x] **Task 3.1.5e**: Enhance source error chaining and causality tracking. (✅ **COMPLETED**)
    - [x] **Task 3.1.5f**: Update error creation patterns throughout runtime module for consistency. (✅ **COMPLETED**)
    - [x] **Task 3.1.5g**: Add comprehensive error tests covering all scenarios. (✅ **COMPLETED**)
    - [x] **Task 3.1.5h**: Update error documentation and usage examples. (✅ **COMPLETED**)
  - [x] **Task 3.1.6**: Fix GitHub PR 54 Review Comments (✅ **COMPLETED** - 2025-05-30)
    - [x] **Task 3.1.6a**: Fix critical validation error handling bug where iterator was consumed during logging, leaving no errors for return statement. (✅ **COMPLETED**)
    - **Fix Details**: Changed error handling logic to collect errors into Vec first, log all errors comprehensively, then return first error while preserving error data.
    - **Impact**: Ensures validation failures are both logged AND properly propagated as errors, making validation system effective.
    - **Testing**: All 221 tests continue passing with fix applied.
- [ ] **Task 3.2**: Investigate Performance and Optimization Opportunities (postponed to after release)
  - [ ] **Task 3.2.1**: Profile compile times of type-level constructs. (postponed to after release)
  - [ ] **Task 3.2.2**: Benchmark runtime performance for common protocol operations. (postponed to after release)
  - [ ] **Task 3.2.3**: Explore and implement optimization techniques for type-level logic and runtime. (postponed to after release)
- [x] **Task 3.3**: Explore Macro-Based DSL for Protocol Definition ✅ **COMPLETED**
  - [x] **Task 3.3.1**: Enhanced Declarative Macros Infrastructure ✅ **COMPLETED**
  - [x] **Task 3.3.2**: Implement Simple Derive Macros ✅ **COMPLETED** (2025-05-30)
    - [x] **Task 3.3.2a**: Created separate `besedarium-derive` proc-macro crate with proper workspace configuration
    - [x] **Task 3.3.2b**: Implemented all core derive macros: `#[derive(Message)]`, `#[derive(Role)]`, `#[derive(MsgLbl)]`, `#[derive(GlobalProtocol)]`
    - [x] **Task 3.3.2c**: Added comprehensive utility functions for derive macro operations
    - [x] **Task 3.3.2d**: Created integration tests with all 5 test scenarios passing
    - [x] **Task 3.3.2e**: Resolved circular dependency issues and optimized for proc-macro best practices
  - [x] **Task 3.3.3**: Develop Basic Attribute Macros (`#[protocol]`, `#[role]`, `#[endpoint]`, `#[session_type]`) ✅ **COMPLETED** (2025-05-31)
    - [x] **Task 3.3.3a**: Phase 1 - Basic attribute macro infrastructure with AST structures and parsing foundations
    - [x] **Task 3.3.3b**: Phase 2 - Enhanced protocol parsing with doc comment support, sophisticated session type generation, and comprehensive testing (25/25 tests passing)
    - [x] **Task 3.3.3c**: Phase 3 - Complete `#[session_type]` attribute macro with syn API compatibility fixes and full macro exports
  - [x] **Task 3.3.4**: Create Advanced DSL Features (choice, branching, loops, recursion) ✅ **COMPLETED**
  - [x] **Task 3.3.5**: Implement Dual Protocol Generation Integration ✅ **COMPLETED** (2025-05-31)
    - [x] **Phase 1**: Extended Protocol Attributes with dual generation fields ✅
      - Added `generate_dual: bool`, `dual_name: Option<String>`, `verify_duality: bool`, `dual_documentation: bool` to `ProtocolAttributes`
      - Updated attribute parsing with comprehensive validation and error handling
      - Added 21 comprehensive tests covering all dual generation attribute scenarios
    - [x] **Phase 2**: Created comprehensive dual generation infrastructure ✅
      - Implemented complete `DualGenerator` struct in `dual_generation.rs` module
      - Added role swapping, protocol flow transformation, and code generation functions
      - Comprehensive test suite with 8 tests covering all dual generation functionality  
    - [x] **Phase 3**: Integration with main protocol macro ✅
      - Added dual generation module imports and Clone derives to support workflow
      - Enhanced `generate_protocol_implementation` to conditionally generate dual protocols
      - Fixed compilation issues and completed integration workflow
    - [x] **Phase 4**: Integration testing and validation ✅
      - Created comprehensive integration test suite with 8 tests
      - Verified end-to-end dual generation workflow from attributes to code generation
      - All 48 derive crate tests passing, including dual generation functionality
- [ ] **Task 3.4**: Consider Integration with Existing Actor Frameworks (postponed to after release)
- [x] **Task 3.5**: Develop Visualization Tools for Protocols ✅ **COMPLETED**
  - [x] **Task 3.5.1**: Phase 1 - Manual Mermaid Integration ✅ **COMPLETED** (2025-05-31)
    - [x] **Task 3.5.1a**: Integrated `simple-mermaid` for diagram embedding in documentation
    - [x] **Task 3.5.1b**: Created example protocols with embedded sequence diagrams
    - [x] **Task 3.5.1c**: Verified diagram rendering in `cargo doc` output
  - [x] **Task 3.5.2**: Phase 2 - Automatic Diagram Generation from Protocol Definitions ✅ **COMPLETED**
    - [x] **Task 3.5.2a**: Protocol Introspection Infrastructure (✅ **COMPLETED** - Step 1)
      - [x] **Step 1**: Core introspection module implementation ✅ **COMPLETED**
        - [x] Created `src/protocol/introspection.rs` with `ProtocolFlow` trait, `SequenceStep` enum, `ProtocolAnalyzer` helper trait
        - [x] Implemented `GeneratesDiagram` marker trait and `DiagramConfig`/`DiagramTheme` for customization
        - [x] Added Mermaid generation engine with sequence diagram support
        - [x] Created comprehensive test suite (8 tests) covering all core functionality
        - [x] Added type utilities for name extraction within stable Rust constraints
      - [x] **Step 2**: Extend derive macro with GenerateDiagram support ✅ **COMPLETED**
        - [x] Extended `besedarium-derive/src/protocol.rs` with `derive_generate_diagram_impl` function
        - [x] Generated `ProtocolFlow` trait implementation for automatic diagram capability
        - [x] Added export in `besedarium-derive/src/lib.rs` with comprehensive documentation
        - [x] Created integration test in `tests/derive_macros.rs` verifying functionality
        - [x] All tests passing (6/6 derive macro tests) with `GenerateDiagram` derive working correctly
    - [x] **Task 3.5.2b**: Mermaid Generation Engine ✅ **COMPLETED** (2 edits completed)
      - [x] Create `besedarium-derive/src/diagram_generation.rs` with `ProtocolDiagramGenerator`
      - [x] Integrate automatic `#[doc = mermaid!(...)]` generation into protocol derive macro
    - [x] **Task 3.5.2c**: Integration and Testing ✅ **COMPLETED** (3 edits completed)
      - **Updated examples/verify_protocol_examples.rs**: Added `#[derive(GenerateDiagram)]` to protocol wrappers with enhanced documentation and automatic diagram generation demo
      - **Enhanced tests/derive_macros.rs**: Added comprehensive tests for diagram generation functionality, method signatures, and multi-protocol differentiation
      - **Fixed method call syntax**: Corrected from instance methods to static function calls (`Protocol::generate_diagram()`)
      - **Protocol-specific content**: Updated derive macro to include protocol names in generated diagrams for unique identification
      - **Successful integration**: All tests passing, examples working, automatic documentation generation functional
      - [x] Update `examples/verify_protocol_examples.rs` to demonstrate `#[derive(GenerateDiagram)]` ✅ **COMPLETED**
      - [x] Create comprehensive test suite for diagram generation functionality ✅ **COMPLETED** 
      - [x] Update documentation with automatic generation examples ✅ **COMPLETED**

## Phase 4: Comprehensive Documentation

This phase focuses on creating user-facing documentation and in-code doccomments
once the library features are stable and well-tested.

- [x] **Task 4.1**: Create Core Concept Documentation
  - [x] **Task 4.1.1**: Create `docs/Projections.md` (detailing `Project<P, Role>`). ✅ **COMPLETED**
  - [x] **Task 4.1.2**: Create `docs/recursion.md` (detailing `Rec<P>`, `Var<N>`). (**COMPLETED** - 2025-05-27)
  - [x] **Task 4.1.3**: Update/Refine `docs/duality.md` with implementation insights. ✅ **COMPLETED** (2025-06-01) - **PR #62**
  - [x] Enhanced duality documentation with comprehensive practical implementation section
  - [x] Added detailed derive macro infrastructure documentation with automatic dual generation
  - [x] Documented visual verification through Mermaid diagram generation
  - [x] Provided concrete examples of `#[protocol(generate_dual = true)]` usage
  - [x] Enhanced docs/Projections.md and docs/recursion.md with practical implementation examples
  - [x] Created examples/verify_protocol_examples.rs for practical verification patterns
- [x] **Task 4.2**: Write Comprehensive Usage Examples ✅ **COMPLETED**
  - [x] **Task 4.2.1**: Create `docs/protocol-examples.md` (leveraging integration tests from Task 2.4). ✅ **COMPLETED**
- [x] **Task 4.3**: Update Project README ✅ **COMPLETED**
  - [x] **Task 4.3.1**: Refresh `README.md` with current status, features, and quick start. ✅ **COMPLETED**
- [x] **Task 4.4**: Add Doccomments ✅ **COMPLETED** (2025-06-02)
  - [x] **Task 4.4.1**: Add comprehensive doccomments to all public types, traits, and functions in the library. ✅ **COMPLETED** (2025-06-02)
    - [x] **Phase 1**: Foundation Documentation (20% effort) - Added comprehensive doccomments to foundation types, role system, message system, global protocols, and local endpoints. Achieved 64/64 doctests passing.
    - [x] **Phase 2**: Protocol Types Documentation (60% effort) - Added comprehensive doccomments to all protocol types, constructors, and trait implementations with practical examples.
    - [x] **Phase 3**: Advanced Systems Documentation (20% effort) - Added comprehensive doccomments to Projection system (Project trait, helpers, boolean logic), Duality system (IsDual trait, helpers), macro infrastructure, and projection error handling. Achieved 113/113 doctests passing with all new documentation examples compiling and passing.
    - [x] **Phase 1**: Foundation Documentation (40% effort) - ✅ **COMPLETED** (2025-06-01)
      - [x] Document core foundation traits (`Role`, `Message`, `GlobalProtocol`, `LocalProtocol`) ✅
      - [x] Document channel and message identification (`ChanId`, `MsgLbl`) ✅  
      - [x] Document metadata system (`Metadata`, `CommMetadata`, `CommMetadataTrait`) ✅
      - [x] Document Action I/O system (`ActionIOTMarker`, `SupportsActionIO`, action types) ✅
      - [x] All 50 doctests passing, examples compile correctly ✅
    - [x] **Phase 2**: Protocol Types Documentation (35% effort) - ✅ **COMPLETED** (2025-06-01)
      - [x] Document all 7 global protocol types (`TChanSend`, `TChanRecv`, `TChanChoice`, `TChanOffer`, `TChanPar`, `TChanEnd`, `TChanStart`) ✅
      - [x] Document all 7 local endpoint types (`EpChanSend`, `EpChanRecv`, `EpChanChoice`, `EpChanOffer`, `EpChanPar`, `EpChanEnd`, `EpChanStart`) ✅
      - [x] All 14 protocol type doctests passing, examples compile correctly ✅
      - [x] Total: 64 doctests passing across all documentation phases ✅  
    - [x] **Phase 3**: Advanced Systems Documentation (20% effort) - ✅ **COMPLETED** (2025-06-02)
      - [x] Document Projection system (`Project` trait, helper traits, boolean logic) ✅
      - [x] Document Duality system (`IsDual` trait, validation helpers) ✅
      - [x] Document macro infrastructure and derive macros ✅
      - [x] Document projection error handling and validation ✅
      - [x] All 113 doctests passing, advanced examples compile correctly ✅
    - [x] **Phase 4**: Polish and Cross-References (5% effort) - ✅ **COMPLETED** (2025-06-02)
      - [x] Enhanced module navigation sections across foundation, projection, duality, macro, global, and local modules ✅
      - [x] Added comprehensive cross-references using `[`crate::protocol::*`]` syntax between related modules ✅
      - [x] Added integration test documentation with specific references to `tests/client_server_integration.rs` and `tests/integration_common.rs` ✅
      - [x] Added quick start examples and integration test examples sections ✅
      - [x] Fixed doctest compilation issues by adding proper role definitions and IO capability types ✅
      - [x] All 115 doctests passing (up from 113), 0 failed, demonstrating successful polish improvements ✅
- [x] **Task 4.5**: Review and Update Internal Documentation ✅ **COMPLETED**
  - [x] **Task 4.5.1**: Review and update `work/learnings.md` with insights from all phases. ✅ **COMPLETED** (2025-05-31)
  - [x] **Task 4.5.2**: Check PR 57 review comments and act on them. ✅ **COMPLETED** (2025-05-31)
    - [x] Retrieved and analyzed 2 review comments from GitHub API
    - [x] Refactored duplicated display_name extraction logic in role.rs
    - [x] Created helper function `extract_display_name_from_name_value` to eliminate 30+ lines of duplication
    - [x] Improved error handling consistency across Meta::List and Meta::NameValue branches  
    - [x] All 239 tests continue to pass, code formatting and clippy clean
    - [x] Committed changes with detailed explanation and context

## Ongoing Tasks

- [x] **Task: TODO Analysis and Resolution** ✅ **COMPLETED** (2025-01-02)
  - [x] **Analysis Phase**: Comprehensive TODO comment analysis across codebase using `grep -ri "TODO" src`
    - **Findings**: 5 TODO comments identified in `src/runtime/` components 
    - **Key Discovery**: Session module (`src/runtime/session/mod.rs`) was incorrectly marked as incomplete with TODO comment
    - **Reality**: Session module contains 1,193 lines of production-ready code with comprehensive functionality
  - [x] **Resolution Phase**: Fixed misleading TODO comment and enabled session module exports
    - **Fixed**: Removed incorrect TODO comment from `src/runtime/mod.rs` 
    - **Enabled**: Session module exports (`Session`, `SessionConfig`, `SessionManager`) now available to users
    - **Verified**: All builds and tests pass (239 library tests) with session functionality accessible
  - [x] **Next Steps Planning**: Added Task 2.5 with 3 specific integration test improvement tasks
    - **Task 2.5.1**: Implement error propagation test using available session functionality
    - **Task 2.5.2**: Implement multi-session concurrent execution test using SessionManager
    - **Task 2.5.3**: Implement concurrent state and channel operations test
  - **Impact**: Major functionality (session management) that was hidden behind TODO is now available to users
- [ ] **Refactoring**: Continuously refactor code for clarity, efficiency, and maintainability.
  - [ ] Refactor `Label` and `Labelled` types (as part of Task 1.2 or ongoing).
  - [ ] Standardize error handling (as part of Task 1.3, 3.1 or ongoing).
  - [ ] Improve `Projection` trait (as part of Task 1.1.5 or ongoing).
- [ ] **Learning & Research**: Stay updated with advancements in type-level programming, session types, and formal methods.
- [ ] **Changelog & Status**: Maintain `CHANGELOG.md` and `work/Status.md`.
