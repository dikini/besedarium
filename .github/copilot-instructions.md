# Copilot guidance

## General Guidelines

- **Mandatory planning.**
  1. ALWAYS start by creating a detailed plan BEFORE making any edits
  2. Your plan MUST include:
     - All functions/sections/types that need modification
     - The order in which changes should be applied
     - Dependencies between changes
     - Estimated number of separate edits required
- **rust stable only** - use only features available in rust stable. Always verify your code can be
implemented in rust stable.
- **Prioritize clarity and correctness.**
  Use clear, succinct, but descriptive language. Make sure that concepts are explained in a
  professional, but not high-brow academic style.
- **Document your intent.**
  Add comments or docstrings to clarify non-obvious code, especially for protocol logic and
  type-level programming.
- **Prefer explicitness over cleverness.**
  Readability and maintainability are more important than brevity or “tricks.”
- **Consult your learnings.**
  - Always consult your learnings memory work/learnings.md
  - After a plan is finalized, update work/learnings.md with your current insights, learnings,
  patterns
  - Format your plan as:

```markdown
## PROPOSED EDIT PLAN
 Working with: [filename]
 Total planned edits: [number]
 ---
```

## Markdown & Documentation

- All documentation must pass markdownlint checks.
- Use line wrapping at 80-100 characters for readability.
- Ensure code examples in docs are up to date and compile (doctest where possible).
- Use clear section headings and bullet points for structure.
- Wrap lines at 100 characters for readability
- be careful with list identation
  - Use 2 spaces for top-level lists
  - Use 4 spaces for nested lists (2 additional spaces per level)
  - Consistent indentation is critical for proper rendering
- Ordered List Numbering
  - Use consistent numbering style (1, 2, 3 or 1, 1, 1)
  - Fix with search and replace or the fix_markdown.sh script
  - Consider converting to bullet points when sequential numbering isn't important
- Blank Lines Around Lists
  - Always add blank lines before and after lists
  - This prevents markdown parsers from merging adjacent content
- Wrap URLs in angle brackets `<http://example.com>` or use reference-style links

## Code Style

- Follow Rust’s standard formatting (rustfmt).
- Use idiomatic Rust patterns and avoid unnecessary complexity.
- All code must pass `cargo clippy` and `cargo test` before merging.

## Protocol & Type-Level Design

- When designing type-level or macro-heavy code, double-check trait bounds, recursion, and type
safety.
- Add compile-time assertions (e.g., `assert_type_eq!`, `assert_disjoint!`) for protocol invariants.
- Document any non-trivial type-level logic.

## Commit & PR Workflow

- Summarize Copilot’s involvement in your PR description if it generated significant code or
documentation.
- All PRs must be reviewed by a human before merging.
- Use draft PRs for work-in-progress.

## Security & Safety

- Never suggest code, documentation, or other artifacts that include secrets, credentials, or
unsafe code.
- Review all dependencies and generated code for potential vulnerabilities.

## Required Code Test & Verification Steps

Before submitting code or documentation, you must run and pass all of the following commands
locally:

- `cargo check` — Ensure the code compiles without errors.
- `cargo build` — Build the project to catch any build-time issues.
- `cargo test` — Run all tests to verify correctness.
- `cargo fmt --all -- --check` — Check that all code is properly formatted.
- `cargo clippy` — Run the linter to catch common mistakes and improve code quality.

All code must pass these checks before a pull request is submitted or merged. These steps are also
enforced in CI.

## Work progress planning, tracking and learning

- maintain CHANGELOG.md in 'keep a changelog' style

### TASKS

- maintain a running tasks tasklist in work/TASKS.md in markdown format, using github style
checkboxes to indicate completion.
- add (wip) to the task you are currently working on to indicate task completion.
- consult the tasks/TASKS.md when suggesting next work. Prioritise:
  - continuation of the current work
  - subtasks
  - similar or related tasks
- for large tasks, which may contain subtasks maintain an own task specific tasklist in
work/tasks/[taskname]

### Learnings

- Maintain a running learnings and patterns document of the concepts, ideas, tricks you've learned
during a session.
- Update regularly.
- Mandatory updates after successful task completion.
- Write all learnings, patterns, concepts to work/learnings.md
- Use descriptive language. They are not just documentation, but running help.

### MAKING EDITS

- Focus on one conceptual change at a time
- Show clear "before" and "after" snippets when proposing changes
- Include concise explanations of what changed and why
- Always check if the edit maintains the project's coding style

### Edit sequence

1. [First specific change] - Purpose: [why]
2. [Second specific change] - Purpose: [why]

### EXECUTION PHASE

- After each individual edit, clearly indicate progress:
  "✅ Completed edit [#] of [total]."
- If you discover additional needed changes during editing:
  - STOP and update the plan
  - Get approval before continuing

### REFACTORING GUIDANCE

When refactoring large files:

- Break work into logical, independently functional chunks
- Ensure each intermediate state maintains functionality
- Consider temporary duplication as a valid interim step
- Always indicate the refactoring pattern being applied

### RATE LIMIT AVOIDANCE

- For very large files, suggest splitting changes across multiple sessions
- Prioritize changes that are logically complete units
- Always provide clear stopping points

## Feedback & Improvements

- Suggest improvements to these instructions as the project evolves.

---

Last updated: 2025-05-13
