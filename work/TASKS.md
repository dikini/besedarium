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

- [ ] **Task 1.1**: Implement Core Types and Traits based on `duality.md`
  - [ ] **Task 1.1.1**: Define/Update `CommMetadata` (including `ChanId`, `MsgLbl`, and `ActionIOType` concepts) (wip).
  - [ ] **Task 1.1.2**: Implement Global Protocol Types (e.g., `TChanSend`, `TChanRecv`, `TChanOffer`, `TChanChoice`, `TChanPar`, `TChanRec`, `TChanVar`, `TChanEnd`, `TChanContinue`, `TChanStart`) incorporating `CommMetadata` and `ActionIOType`.
  - [ ] **Task 1.1.3**: Implement Local Endpoint Types (e.g., `EpSend`, `EpRecv`, `EpOffer`, `EpChoice`, etc.) ensuring consistent `IO` parameter handling and `SupportsActionIO` trait integration.
  - [ ] **Task 1.1.4**: Implement the `IsDual` predicate/trait for verifying duality between protocol specifications, considering `CommMetadata`, message types, and `IO` consistency.
  - [ ] **Task 1.1.5**: Implement the `Project<P, Role>` trait for projecting Global Protocols to Local Endpoint Types, ensuring `SupportsActionIO` checks.
  - [ ] **Review comment 1**:  sr/types.rs:48 The trait name ActionIOTMarker and docs refer to the concept as ActionIOType. Align the naming between code and documentation (e.g., rename the trait or update docs) for clarity.

```rust 
pub trait ActionIOTMarker: sealed::Sealed + Send + Sync + 'static + core::fmt::Debug {}
```

- [ ] **Review comment 2**: src/protocol/global.rs:30  The TSession trait no longer defines the associated type Compose or const IS_EMPTY, removing critical type-level composition behavior. Consider reintroducing those members (or providing equivalent functionality) to maintain existing protocol composition logic.

```rust
pub trait TSession<IO>: sealed::Sealed {}
```

- [ ] **Task 1.2**: Implement Label Preservation and Transformation Logic
  - [ ] **Task 1.2.1**: Research label behavior in `Choice`, `Parallel`, `Rec` to inform design.
  - [ ] **Task 1.2.2**: Design type-level traits for label transformations (e.g., `TMap`, `TCollect`, `TFilter`).
  - [ ] **Task 1.2.3**: Implement the designed label transformation traits and integrate them with protocol types.
- [ ] **Task 1.3**: Implement Basic Runtime Components
  - [ ] **Task 1.3.1**: Design a foundational runtime state machine for protocol execution.
  - [ ] **Task 1.3.2**: Implement basic channel communication logic for sending/receiving typed messages according to protocol specifications.
  - [ ] **Task 1.3.3**: Define core error types for protocol violations and communication errors at runtime.
- [ ] **Task 1.4**: Research and Formalize Duality Concepts
  - [ ] **Task 1.4.1**: Further research formal definitions of duality for all implemented primitives.
  - [ ] **Task 1.4.2**: Investigate and document how duality is checked at the type level within the Rust implementation.
  - [ ] **Task 1.4.3**: Explore and document potential for generating dual protocols or verifying compatibility automatically.

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
