# Copilot guidance

````instructions
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

- All documentation must pass markdownlint-cli2 checks.
- Use line wrapping at 80-100 characters for readability.
- Ensure code examples in docs are up to date and compile (doctest where possible).
- Use clear section headings and bullet points for structure.
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
- Use footnotes for citations

## Code Style

- Follow Rust’s standard formatting (rustfmt).
- Use idiomatic Rust patterns and avoid unnecessary complexity.
- All code must build `cargo build`, pass `cargo fmt --all -- --check`,`cargo clippy` and `cargo test` before merging.

## Protocol & Type-Level Design

- When designing type-level or macro-heavy code, double-check trait bounds, recursion, and type
safety.
- Add compile-time assertions (e.g., `assert_type_eq!`, `assert_disjoint!`) for protocol invariants.
- Document any non-trivial type-level logic.

### Rust Trait System Constraints

Be aware of these stable Rust limitations when implementing protocol code:

- **No specialization**: Cannot provide specialized implementations for subsets of types
- **No negative bounds**: Cannot constrain generics by what they are not
- **No associated types as generic parameters**: Types must be directly specified
- **No overlapping impls**: Must have disjoint implementation sets

### Core Type-Level Programming Patterns

When implementing type-level functionality, prefer these established patterns:

- **Marker Type Dispatch**: Use marker types to represent cases and delegate implementations
  when different behavior is needed for specific types
- **Helper Trait Case Analysis**: Create helper traits with specialized implementations for each
  case combination when protocol projection requires different behavior based on multiple computed
  properties
- **Recursive Type Structure Traversal**: Use recursive trait implementations with proper base cases
  for analyzing nested, recursive protocol structures
- **Type-Level Boolean Logic**: Implement boolean operators as traits with associated types for
  combining multiple type-level conditions in protocol analysis

### Critical Implementation Principles

Adhere to these principles when implementing protocol-related code:

- **Type-Level Dispatch** is fundamental for handling different protocol cases without specialization
- **Helper Traits** resolve implementation conflicts through indirection
- **Recursive Type Traversal** requires careful handling of base cases and conditionals
- **Edge Case Testing** must be comprehensive to reveal subtle protocol implementation issues
- **Protocol Projection** decisions must account for role presence in multiple communication paths
- **Compositional Design** with small, focused traits improves modularity and evolution
- **Type Safety at Compile Time** is achievable through proper trait bounds and assertions

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

All documentation must pass `markdownlint-cli2 **/*.md` checks.

## Work progress planning, tracking and learning

- maintain CHANGELOG.md in 'keep a changelog' style
- maintain project status in work/Status.md. This should be a high level overview of the
  project status, including:
  - current work in progress
  - current issues - link to github
  - current PRs - link to github
  - current tasks - link to work/TASKS.md
  - current learnings - link to work/learnings.md

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
- at task completion update the task in work/TASKS.md to indicate completion.
- at task completion update the task in work/Status.md to indicate the current status of structure, tests, documentation,etc...

### Learnings

- Maintain a running learnings and patterns document of the concepts, ideas, tricks you've learned
during a session.
- Update regularly.
- Mandatory updates after successful task completion.
- Write all learnings, patterns, concepts to work/learnings.md
- Use descriptive language. They are not just documentation, but running help.
- at the end of the work on a PR - when all work/TASKS.md tasks are complete, update
work/learnings.md. Revise and summarise the content of the learnings file in such a way
that it is suitable for context injection into an LLM context window. Make sure that we don't
lose any important information, like code insights, code patterns, type level programming, etc...

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


### Size and Module Structure Guidelines

**Keep Files Compact and Focused:**

- **Target Size**: Individual module files should stay under 300 lines including documentation
- **Test Separation**: When a module reaches ~200 lines, extract tests to separate `tests.rs` files
- **Implementation/Test Ratio**: Aim for roughly 2:1 implementation to test code ratio
- **Documentation Balance**: Include essential documentation but avoid excessive inline examples
- **Module Structure Pattern for Growth**: When modules grow beyond manageable size, follow this consistent structure:

```text
.../[module_name]/
├── mod.rs              # Core implementation (~150-250 lines)
├── tests.rs            # Unit tests (~50-150 lines)  
├── builders.rs         # Builder functions/helpers (optional)
└── examples.rs         # Extended examples (optional)
```


**Refactoring Process Pattern:**

1. **Identify Split Points**: Look for natural boundaries (tests, type groups, helpers)
2. **Create Module Directory**: Convert `module.rs` to `module/mod.rs`
3. **Extract Tests**: Move all `#[cfg(test)]` blocks to `tests.rs`
4. **Update Imports**: Add `mod tests;` declaration in `mod.rs`
5. **Verify Compilation**: Ensure all tests still pass and no circular dependencies
6. **Update Documentation**: Reflect new structure in module docs

**When to Split Further:**

1. **Single Responsibility**: If a module handles multiple distinct concepts
2. **Size Threshold**: When mod.rs approaches 300 lines
3. **Test Complexity**: When tests become more complex than implementation
4. **Team Navigation**: When it takes >30 seconds to find relevant code

**Module Naming Consistency:**

- Core types in `mod.rs`
- Tests in `tests.rs` 
- Builders/helpers in `builders.rs`
- Examples in `examples.rs`
- Sub-modules as directories with same pattern

**File Organization Anti-Patterns to Avoid:**

- **Mega Files**: Single files >500 lines mixing concepts
- **Scattered Tests**: Tests mixed throughout implementation files
- **Inconsistent Structure**: Different organization patterns per module
- **Deep Nesting**: More than 3 levels of module directories
- **Circular Dependencies**: Modules importing from their children

**Progressive Enhancement Strategy:**

1. **Phase 1**: Extract tests when files reach ~200 lines
2. **Phase 2**: Split implementation by concept when >300 lines
3. **Phase 3**: Add specialized sub-modules (builders, examples) as needed
4. **Phase 4**: Consider domain-specific organization for complex areas

This pattern ensures predictable, scalable organization as the codebase grows while maintaining clear separation of concerns and excellent maintainability.


### RATE LIMIT AVOIDANCE

- For very large files, suggest splitting changes across multiple sessions
- Prioritize changes that are logically complete units
- Always provide clear stopping points

## Task-Specific Prompts

The `/work/prompts/` directory contains specialized prompt files for various project tasks. These prompts serve several important purposes:

1. **Technical Documentation**: They capture technical details, requirements, and constraints for tasks
2. **Continuity**: They enable seamless continuation of complex tasks across multiple sessions
3. **Knowledge Transfer**: They preserve insights for team members working on related tasks
4. **Guided Implementation**: They provide structured guidance for implementing features

### Prompt File Organization

Prompt files follow a consistent naming convention that aligns with the task structure in `work/TASKS.md`:

```
{task-number}-{task-description}.md
```

For example:
- `1.1.1-foundation-types.md` - Prompt for Task 1.1.1
- `3.1-enhance-runtime-checks.md` - Prompt for Task 3.1
- `4.5.1-review-learnings-md.md` - Prompt for Task 4.5.1

### Prompt Structure

Each prompt file should follow this general structure:

```markdown
# Task X.Y.Z: Task Title

## Objective
Brief description of what the task aims to achieve.

## Background
Context and prerequisites for understanding the task.

## Requirements
Detailed breakdown of what needs to be implemented.

## Implementation Guide
Specific guidance on how to approach the implementation.

## Verification and Testing
Instructions for verifying correctness of the implementation.

## Success Criteria
Concrete measures for determining task completion.

## Next Steps
Follow-up tasks or extensions.
```

### Using Prompts with GitHub Copilot

When working with GitHub Copilot on a specific task:

1. **Load the Task Context**:
   - Use the corresponding prompt file as context for GitHub Copilot
   - Reference related code files and documentation

2. **Guide the AI with Specificity**:
   - Reference specific sections of the prompt in your questions
   - Ask about implementation patterns mentioned in the prompt
   - Request explanations for concepts outlined in the prompt

3. **Refine the Approach**:
   - Use the prompt as a starting point, not a rigid script
   - Ask Copilot to suggest refinements or alternatives to the approaches
   - Update prompt files with new insights gained during implementation

### Maintaining Prompts

As the project evolves, it's important to keep prompt files updated:

1. **Create New Prompts**:
   - Create new prompt files for major new tasks
   - Follow the established naming convention and structure

2. **Update Existing Prompts**:
   - Update prompts when task requirements change
   - Add new insights or approaches discovered during implementation
   - Note limitations or challenges encountered

3. **Archive Completed Prompts**:
   - Keep completed task prompts as reference material
   - Consider consolidating insights into `work/learnings.md`

### Example Interactions

#### Effective:
```
User: I'm working on Task 1.4.3 for automatic dual protocol generation. The prompt 
file mentions using a trait-based approach for type-level dual computation. How 
could I implement a `GenerateDual<P>` trait that works with our existing `IsDual` trait?

Copilot: [Provides detailed implementation guidance specific to the mentioned approach]
```

#### Less Effective:
```
User: Help me with dual protocols.

Copilot: [Provides generic information that might not align with project approach]
```

## Feedback & Improvements

- Explicitly suggest improvements to these instructions as the project evolves.
- If you have suggested improvements, add them to work/TASKS.md for review

---

Last updated: 2025-05-26
