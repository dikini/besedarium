# Project Implementation Plan: Besedarium

This document outlines the next phases of development for the Besedarium project,
based on the re-structured priorities in `work/TASKS.md`. It focuses on core
library implementation first, followed by testing, advanced features, and documentation.

## Development Approach Notes

- **Priority on Core Implementation**: Focus on correctly implementing library features, especially changes related to `duality.md` (structs, traits, interfaces).
- **Doccomments**: Defer extensive doccomments until core functionality is stable.
- **Unit Tests**: Implement unit tests in parallel with feature development. Existing unit tests may need significant rework or replacement.
- **Integration Tests**: Plan to redo integration tests. Consider removing existing ones initially to avoid churn.
- **Documentation**: Address full documentation after the implementation and testing phases are mature.

## Phase 1: Core Protocol & Duality Implementation (Guided by `docs/duality.md`)

This phase focuses on implementing the foundational elements of the library,
with a strong emphasis on the concepts outlined in `docs/duality.md`.

### Task 1.1: Implement Core Types and Traits based on `duality.md`

- **Objective**: Define and implement the fundamental types and traits required by the protocol logic, incorporating `CommMetadata` and `ActionIOType` concepts as described in `duality.md`.
- **Sub-tasks & Key Sections**:
  - **1.1.1**: Define/Update `CommMetadata` (including `ChanId`, `MsgLbl`, and `ActionIOType` concepts).
  - **1.1.2**: Implement Global Protocol Types (e.g., `TChanSend`, `TChanRecv`, `TChanOffer`, `TChanChoice`, `TChanPar`, `TChanRec`, `TChanVar`, `TChanEnd`, `TChanContinue`, `TChanStart`) incorporating `CommMetadata` and `ActionIOType`.
  - **1.1.3**: Implement Local Endpoint Types (e.g., `EpSend`, `EpRecv`, `EpOffer`, `EpChoice`, etc.) ensuring consistent `IO` parameter handling and `SupportsActionIO` trait integration.
  - **1.1.4**: Implement the `IsDual` predicate/trait for verifying duality between protocol specifications, considering `CommMetadata`, message types, and `IO` consistency.
  - **1.1.5**: Implement the `Project<P, Role>` trait for projecting Global Protocols to Local Endpoint Types, ensuring `SupportsActionIO` checks.
- **Affected Code**: `src/types.rs`, `src/protocol/*.rs`, potentially new modules.
- **Dependencies**: `docs/duality.md`.
- **Estimated Edits**: 8-12 (multiple files, significant new type and trait definitions).
- **Verification**: `cargo build`, `cargo clippy`, `cargo fmt`. Unit tests developed in Phase 2.

### Task 1.2: Implement Label Preservation and Transformation Logic

- **Objective**: Enhance protocol definitions with sophisticated label management, including mechanisms for preserving and transforming labels during protocol operations.
- **Sub-tasks & Key Sections**:
  - **1.2.1**: Research label behavior in `Choice`, `Parallel`, `Rec` to inform design.
  - **1.2.2**: Design type-level traits for label transformations (e.g., `TMap`, `TCollect`, `TFilter`).
    - `TMap<L, F>`: Applies a type-level function `F` to transform label `L`.
    - `TCollect<Protocols, Role>`: Collects all labels offered/received by a `Role` in a `Choice` or `Parallel`.
    - `TFilter<Labels, Predicate>`: Filters a list of labels based on a `Predicate`.
  - **1.2.3**: Implement the designed label transformation traits and integrate them with protocol types.
- **Affected Code**: `src/types.rs`, `src/protocol/*.rs`, new modules for label transformations.
- **Dependencies**: Stable core primitives from Task 1.1.
- **Estimated Edits**: 5-7 (multiple files, new modules, tests).
- **Verification**: `cargo build`, `cargo clippy`, `cargo fmt`. Unit tests developed in Phase 2.

### Task 1.3: Implement Basic Runtime Components

- **Objective**: Provide a foundational runtime component for executing session-typed protocols, including state management and basic error handling.
- **Sub-tasks & Key Sections**:
  - **1.3.1**: Design a foundational runtime state machine for protocol execution: Define how protocol states are tracked at runtime.
  - **1.3.2**: Implement basic channel communication logic: Adapt or create channel abstractions for sending/receiving typed messages according to protocol specifications.
  - **1.3.3**: Define core error types for protocol violations and communication errors at runtime.
- **Affected Code**: New `src/runtime/` module, examples.
- **Dependencies**: Stable core protocol definitions from Task 1.1.
- **Estimated Edits**: 6-8 (new modules, significant implementation effort).
- **Verification**: `cargo build`, `cargo clippy`, `cargo fmt`. Unit tests developed in Phase 2.

### Task 1.4: Research and Formalize Duality Concepts

- **Objective**: Deepen the understanding and formalization of session type duality within the project, ensuring the implementation aligns with theoretical underpinnings.
- **Key Activities**:
  - **1.4.1**: Further research formal definitions of duality for all implemented primitives.
  - **1.4.2**: Investigate and document how duality is checked at the type level within the Rust implementation.
  - **1.4.3**: Explore and document potential for generating dual protocols or verifying compatibility automatically.
- **Affected Code**: Primarily documentation (`docs/duality.md` update), potentially influencing trait design in Task 1.1.
- **Dependencies**: Initial implementation of `IsDual` (Task 1.1.4).
- **Estimated Edits**: 2-3 (documentation, potential minor code adjustments).
- **Verification**: Conceptual soundness, clarity of documentation.

## Phase 2: Core Feature Testing

This phase focuses on ensuring the correctness and robustness of the core library
features implemented in Phase 1. Unit tests should be developed in parallel with
Phase 1 tasks where feasible, but this phase ensures comprehensive coverage.

### Task 2.1: Develop Unit Tests for Core Types and Duality Logic

- **Objective**: Create a comprehensive suite of unit tests for all core types, traits, and duality-related logic implemented in Phase 1.
- **Sub-tasks**:
  - **2.1.1**: Write unit tests for `CommMetadata`, Global Types (`TChan*`), and Local Endpoint Types (`Ep*`).
  - **2.1.2**: Write unit tests for the `IsDual` predicate/trait, covering various protocol constructs and edge cases.
  - **2.1.3**: Write unit tests for the `Project<P, Role>` trait, verifying correct projection for all primitives.
  - **2.1.4**: Write unit tests for `SupportsActionIO` and `ActionIOType` integration and correctness.
- **Affected Code**: New files in `tests/` directory.
- **Dependencies**: Completion of Task 1.1.
- **Estimated Edits**: Numerous small test files/modules.
- **Verification**: `cargo test` passes for all new unit tests.

### Task 2.2: Develop Unit Tests for Label Logic

- **Objective**: Ensure the label preservation and transformation logic is correct through targeted unit tests.
- **Sub-tasks**:
  - **2.2.1**: Write unit tests for label transformation traits (`TMap`, `TCollect`, `TFilter`) and their integration.
- **Affected Code**: New files/modules in `tests/` directory.
- **Dependencies**: Completion of Task 1.2.
- **Verification**: `cargo test` passes for all new label logic unit tests.

### Task 2.3: Develop Unit Tests for Basic Runtime Components

- **Objective**: Verify the functionality of the basic runtime components.
- **Sub-tasks**:
  - **2.3.1**: Write unit tests for the runtime state machine logic.
  - **2.3.2**: Write unit tests for channel communication logic (mocking/simple cases).
  - **2.3.3**: Write unit tests for error handling and propagation in the runtime.
- **Affected Code**: New files/modules in `tests/` directory.
- **Dependencies**: Completion of Task 1.3.
- **Verification**: `cargo test` passes for all new runtime unit tests.

### Task 2.4: Develop Integration Tests (Complex Protocol Examples)

- **Objective**: Create integration tests using more complex protocol examples to ensure different components of the library work together correctly.
- **Sub-tasks**:
  - **2.4.1**: Remove or overhaul existing integration tests that are no longer relevant due to API changes.
  - **2.4.2**: Implement multi-party protocol examples as integration tests.
  - **2.4.3**: Implement examples with complex data serialization as integration tests.
  - **2.4.4**: Implement examples integrating with async runtimes as integration tests (if basic async support is part of Task 1.3).
- **Affected Code**: `tests/protocols/`, `tests/integration/` (or similar).
- **Dependencies**: Completion of Phase 1 and Tasks 2.1-2.3.
- **Estimated Edits**: 3-5 new complex test files.
- **Verification**: `cargo test` passes for all integration tests.

## Phase 3: Advanced Features, Optimizations & Tooling

With a tested core library, this phase focuses on enhancements, performance, and
developer experience.

### Task 3.1: Enhance Runtime Checks and Error Handling

- **Objective**: Build upon the basic runtime (Task 1.3) to add more sophisticated runtime checks and robust error handling mechanisms.
- **Details**: Further develop tasks from 1.3, adding more sophisticated mechanisms for state validation, error reporting, and recovery if applicable.
- **Affected Code**: `src/runtime/`, error type definitions.
- **Dependencies**: Completion of Task 1.3 and its unit tests (Task 2.3).
- **Estimated Edits**: 4-6 (refinements and additions to runtime modules).
- **Verification**: `cargo build`, `cargo test`, `cargo clippy`, `cargo fmt`.

### Task 3.2: Investigate Performance and Optimization Opportunities

- **Objective**: Analyze the performance characteristics of the type-level machinery and runtime, and identify areas for optimization.
- **Key Activities**:
  - **3.2.1**: Profile compile times of type-level constructs.
  - **3.2.2**: Benchmark runtime performance for common protocol operations.
  - **3.2.3**: Explore and implement optimization techniques for type-level logic and runtime (e.g., reducing type-level complexity, optimizing runtime state transitions).
- **Affected Code**: Potentially core types, runtime, build configurations.
- **Dependencies**: Mature implementation of core and runtime features (Phase 1 & 2).
- **Estimated Edits**: 2-4 (analysis, potential refactoring, build script changes).
- **Verification**: Performance metrics, maintaining correctness (`cargo test`).

### Task 3.3: Explore Macro-Based DSL for Protocol Definition

- **Objective**: Investigate and potentially develop a macro-based Domain Specific Language (DSL) to simplify protocol definition for users.
- **Affected Code**: New `src/macros/` module or similar.
- **Dependencies**: Stable core API.
- **Estimated Edits**: 3-5 (macro definitions, examples, tests).
- **Verification**: `cargo build`, `cargo test`, usability of the DSL.

### Task 3.4: Consider Integration with Existing Actor Frameworks

- **Objective**: Explore the feasibility and benefits of integrating Besedarium with existing Rust actor frameworks.
- **Affected Code**: Potentially adapter modules, examples.
- **Dependencies**: Stable core API and runtime.
- **Estimated Edits**: 2-4 (research, proof-of-concept integrations).
- **Verification**: Feasibility analysis, example integrations.

### Task 3.5: Develop Visualization Tools for Protocols (Optional/Future)

- **Objective**: Consider tools or techniques for visualizing protocol definitions, which can aid in understanding and debugging.
- **Details**: This is an optional/future task, depending on resources and perceived value.
- **Affected Code**: Potentially external scripts or a new utility crate.
- **Dependencies**: Stable protocol definition format.
- **Estimated Edits**: N/A (exploratory).
- **Verification**: N/A.

## Phase 4: Comprehensive Documentation

This phase focuses on creating user-facing documentation and in-code doccomments
once the library features are stable and well-tested.

### Task 4.1: Create Core Concept Documentation

- **Objective**: Document the core theoretical concepts and their implementation within Besedarium.
- **Sub-tasks & Key Sections**:
  - **4.1.1**: Create `docs/Projections.md`: Document the theory, implementation, and usage of the `Project<P, Role>` trait.
    - Introduction to session type projections.
    - Role of projections in distributed system safety.
    - Detailed explanation of how `Project` is implemented for each core primitive.
    - Examples of projection for simple and complex protocols.
    - Common pitfalls and best practices.
  - **4.1.2**: Create `docs/recursion.md`: Explain the type-level recursion mechanism (`Rec<P>`, `Var<N>`) in detail.
    - Concept of recursion in session types.
    - Implementation details of `Rec` and `Var` marker types.
    - How recursion interacts with projection.
    - Examples of recursive protocols (e.g., server loop, streaming).
    - Limitations and advanced patterns.
  - **4.1.3**: Update/Refine `docs/duality.md` with implementation insights and final decisions from Task 1.4.
- **Dependencies**: Completion of Phase 1 & 2. `Project` implementation (Task 1.1.5), `Rec`/`Var` implementation (part of Task 1.1.2).
- **Estimated Edits**: 3 new/updated major documentation files.
- **Verification**: Markdown linting, clarity and accuracy of content.

### Task 4.2: Write Comprehensive Usage Examples

- **Objective**: Provide a collection of practical examples demonstrating how to define and use protocols with Besedarium.
- **Sub-tasks & Key Sections**:
  - **4.2.1**: Create `docs/protocol-examples.md` (leveraging integration tests from Task 2.4).
    - Basic client-server interaction.
    - Protocol with choices.
    - Recursive protocol (e.g., persistent connection).
    - Parallel composition of protocols.
    - Advanced example combining multiple features.
- **Content Source**: Adapt integration tests (Task 2.4) and create new illustrative ones.
- **Dependencies**: Completion of Phase 1 & 2, especially Task 2.4.
- **Estimated Edits**: 1 new major documentation file.
- **Verification**: Markdown linting, code examples should be valid and illustrative.

### Task 4.3: Update Project README

- **Objective**: Refresh the main `README.md` to reflect the current project status, features, and provide clear instructions for getting started.
- **Key Sections**:
  - Project overview and goals.
  - Core features implemented.
  - Installation and setup.
  - Quick start guide with a simple example.
  - Links to detailed documentation (newly created docs from Task 4.1, 4.2).
  - Contribution guidelines.
- **Dependencies**: Completion of Tasks 4.1 and 4.2.
- **Estimated Edits**: 1 significant update to `README.md`.
- **Verification**: Markdown linting, clarity and usefulness for new users.

### Task 4.4: Add Doccomments

- **Objective**: Add comprehensive doccomments to all public types, traits, and functions in the library, ensuring `cargo doc` produces useful API documentation.
- **Sub-tasks**:
  - **4.4.1**: Systematically go through `src/` and add detailed doccomments.
- **Dependencies**: Stable API from Phase 1, 2, and 3.
- **Estimated Edits**: Extensive edits across all source files.
- **Verification**: `cargo doc` generates complete and helpful documentation. `cargo test --doc` passes.

### Task 4.5: Review and Update Internal Documentation

- **Objective**: Consolidate insights, patterns, and learnings from all development phases into internal documentation like `work/learnings.md`.
- **Sub-tasks & Key Sections**:
  - **4.5.1**: Review and update `work/learnings.md` with insights from all phases.
    - Challenges encountered during implementation and testing.
    - Clarifications on protocol logic or type-level programming.
    - Useful patterns for explaining complex concepts or solving specific Rust challenges.
- **Dependencies**: Completion of all prior phases.
- **Estimated Edits**: 1 major update to `work/learnings.md`.
- **Verification**: N/A (internal document).

## Ongoing Tasks

- **Refactoring**: Continuously refactor code for clarity, efficiency, and maintainability based on insights from new feature development and documentation.
  - Refactor `Label` and `Labelled` types (as part of Task 1.2 or ongoing).
  - Standardize error handling (as part of Task 1.3, 3.1 or ongoing).
  - Improve `Projection` trait (as part of Task 1.1.5 or ongoing).
- **Learning & Research**: Stay updated with advancements in type-level programming, session types, and formal methods.
- **Changelog & Status**: Maintain `CHANGELOG.md` and `work/Status.md` with progress.

This plan will be reviewed and updated as the project progresses.
