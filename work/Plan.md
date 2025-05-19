# Work Plan

## Task 1: Modularize `transforms.rs` by Protocol Operation (Detailed)

- **Goal**: Break `transforms.rs` into smaller, more manageable modules, organized by protocol operation.
- **New Module Structure**:
  - `transforms/mod.rs`: Module root, re-exports all public traits and helpers.
  - `transforms/projection.rs`: Contains all projection traits and impls (e.g., `ProjectRole`, `ProjectSendDispatch`, `ProjectRecvDispatch`).
  - `transforms/choice.rs`: Contains all choice-specific traits and impls (e.g., `ProjectChoice`, `ProjectChoiceCase`).
  - `transforms/parallel.rs`: Contains all parallel composition traits and impls (e.g., `ProjectPar`, `ComposeProjectedParBranches`).
  - `transforms/recursion.rs`: (Optional, for future-proofing) Recursion-related traits (e.g., for `TMu`, `TVar`).
  - `transforms/util.rs`: Shared helpers and traits (e.g., `ContainsRole`, `NotContainsRole`, `GetProtocolLabel`).

- **Implementation Steps**:
  1. Create the new module files with module-level doc comments describing their purpose and contents. **(done)**
  2. Migrate relevant traits, impls, and helpers from `transforms.rs` into the appropriate new files. **(done)**
  3. Fix all `use`/`mod` statements and imports throughout the codebase to reference the new structure. **(in progress)**
  4. In `mod.rs`, publicly re-export all traits and helpers needed by other modules or users. **(in progress)**
  5. Ensure all doc comments are preserved or improved for clarity and navigation. **(in progress)**
  6. Run `cargo check`, `cargo build`, `cargo test`, `cargo fmt --all -- --check`, and `cargo clippy` to confirm no breakage. **(pending)**
  7. Update documentation (e.g., `docs/ImplementationOverview.md`) to reflect the new file layout and rationale. **(pending)**

- **Items to Watch Out For**:
  - **Import Loops**: Avoid circular dependencies by keeping utility traits in `util.rs` and only importing them where needed.
  - **Trait Visibility**: Ensure all traits and helpers that need to be used outside their module are `pub` and re-exported in `mod.rs`.
  - **Test Coverage**: After migration, verify that all tests (including doctests) still pass and that no logic is lost in the move.
  - **Documentation Consistency**: Update all references to the old `transforms.rs` location in code comments and documentation.
  - **Recursion Module**: If recursion traits are not yet implemented, create a stub file with a `TODO` comment for future work.
  - **File Size Balance**: If any file becomes too large, consider further splitting (e.g., separate `choice/case.rs` for case helpers).

- **Sub-tasks**:
  1. Create new module files + doc comments. **(done)**
  2. Migrate relevant traits/impls to each file. **(done)**
  3. Fix all imports and references in the codebase. **(in progress)**
  4. Verify all tests pass and code is formatted/linted. **(pending)**
  5. Update documentation and code comments for the new structure. **(pending)**

**Dependencies**:
- Minor import changes across multiple modules.
- Ensure consistent naming (no collisions, clear structure).
- Coordinate with any ongoing work that touches `transforms.rs` to avoid merge conflicts.

---
