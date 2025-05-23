<!-- filepath: /home/dikini/Projects/besedarium/work/Plan.md -->
# Plan for Completing docs/duality.md

This plan outlines the steps to finalize the `docs/duality.md` document, ensuring all sections are complete, accurate, and well-explained.

## Phase 1: Core Protocol Constructs Completion

This phase focuses on completing the detailed descriptions and Rust definitions for each protocol construct.

### Sub-task 1.1: Verify Table of Contents and Document Structure

- **File(s):** `docs/duality.md`
- **Section(s):** "Table of Contents", Overall document
- **Goal:** Ensure the ToC is perfectly synchronized with all H2 and H3 headings and that the overall document structure is sound.
- **Invariants:**
  - Document remains markdownlint compliant.
  - Existing correct content is preserved.
- **Pre-conditions:**
  - `docs/duality.md` exists and has a basic structure.
  - Previous ToC and heading standardization edits have been applied.
- **Post-conditions:**
  - ToC accurately reflects all H2 and H3 headings.
  - All ToC links are valid and point to the correct sections.
  - Document structure is logical and easy to navigate.
- **Acceptance Criteria:**
  - Manual verification confirms ToC accuracy and link validity.
  - No MD051 (link-fragment) errors related to the ToC.
  - Headings are consistently formatted (no numbering, correct capitalization).

### Sub-task 1.2: Complete "Role I/O Capabilities and Action I/O Types"

- **File(s):** `docs/duality.md`
- **Section(s):** "Core Protocol Constructs: Definitions and Duality" -> "Role I/O Capabilities and Action I/O Types"
- **Goal:** Finalize the explanation and Rust code examples for `ActionIOType`, `IO` parameter, and `SupportsActionIO` trait.
- **Invariants:**
  - Document remains markdownlint compliant.
  - The core concept of linking session capabilities to action I/O requirements is maintained.
- **Pre-conditions:**
  - Sub-task 1.1 completed.
  - The basic structure and some content for this section exist.
- **Post-conditions:**
  - The explanation of `ActionIOType`, `IO` parameter, and `SupportsActionIO` is clear, complete, and accurate.
  - Rust code examples for `ActionIOTMarker`, `SupportsActionIO`, and their usage are complete, correct, and well-formatted (including the currently truncated example).
  - The "Verification during Projection" subsection is fully elaborated.
- **Acceptance Criteria:**
  - All conceptual explanations are unambiguous.
  - Rust code examples are idiomatic and clearly illustrate the concepts.
  - The section fully explains how I/O capabilities are managed in the MPST framework.
  - Markdown is correctly formatted.

### Sub-task 1.3: Complete "Send Action" Section

- **File(s):** `docs/duality.md`
- **Section(s):** "Core Protocol Constructs: Definitions and Duality" -> "Send Action"
- **Goal:** Provide a complete and accurate description, Rust definitions, `CommMetadata` integration, duality rule, invariants, roles, and parameters for the Send action.
- **Invariants:**
  - Document remains markdownlint compliant.
  - The fundamental semantics of the Send action are preserved.
  - Consistency with other action descriptions is maintained.
- **Pre-conditions:**
  - Sub-task 1.2 completed.
  - A placeholder or basic structure for the "Send Action" section exists.
- **Post-conditions:**
  - Semantic explanation is clear and comprehensive.
  - Rust definitions for Global Type (`TChanSend`) and Local Endpoint Type (`EpSend`) are complete, correct, and include necessary generic parameters (like `IO`, `M`, `Msg`, roles, continuations).
  - `CommMetadata` integration is clearly explained, including the roles of `ChanId` and `MsgLbl`.
  - Duality rule is correctly stated and explained.
  - Invariants for both Global and Local types are thoroughly listed.
  - Involved roles are correctly identified.
  - Parameters for both Global and Local type definitions are accurately listed.
- **Acceptance Criteria:**
  - All subsections (Semantics, Rust Definitions, `CommMetadata` Integration, Duality Rule, Invariants, Involved Roles, Parameters) are fully populated and accurate.
  - Rust type definitions are plausible and follow conventions established in the document.
  - Explanations are consistent with MPST theory.
  - Markdown is correctly formatted.

### Sub-task 1.4: Complete "Receive Action" Section

- **File(s):** `docs/duality.md`
- **Section(s):** "Core Protocol Constructs: Definitions and Duality" -> "Receive Action"
- **Goal:** Provide a complete and accurate description, Rust definitions, `CommMetadata` integration, duality rule, invariants, roles, and parameters for the Receive action.
- **Invariants:**
  - Document remains markdownlint compliant.
  - The fundamental semantics of the Receive action are preserved.
- **Pre-conditions:**
  - Sub-task 1.3 completed.
  - Basic structure for the "Receive Action" section exists.
- **Post-conditions:**
  - Semantic explanation is clear.
  - Rust definitions for `TChanRecv` and `EpRecv` are complete.
  - `CommMetadata` integration is detailed.
  - Duality rule, invariants, roles, and parameters are accurate.
- **Acceptance Criteria:**
  - All subsections are fully populated and accurate.
  - Rust definitions are correct.
  - Markdown is well-formatted.

### Sub-task 1.5: Complete "Offer Action (External Choice - Offering Side)" Section

- **File(s):** `docs/duality.md`
- **Section(s):** "Core Protocol Constructs: Definitions and Duality" -> "Offer Action (External Choice - Offering Side)"
- **Goal:** Provide a complete and accurate description, Rust definitions, `CommMetadata` integration, duality rule, invariants, roles, and parameters for the Offer action.
- **Invariants:**
  - Document remains markdownlint compliant.
  - The fundamental semantics of the Offer action are preserved.
- **Pre-conditions:**
  - Sub-task 1.4 completed.
  - Basic structure for the "Offer Action" section exists.
- **Post-conditions:**
  - Semantic explanation is clear, noting the nature of external choice.
  - Rust definitions for conceptual `TChanOffer` and `EpOffer` (with type-level lists for branches) are complete.
  - `CommMetadata` integration for choice signaling is detailed.
  - Duality rule, invariants, roles, and parameters are accurate.
- **Acceptance Criteria:**
  - All subsections are fully populated and accurate.
  - Rust definitions are correct and handle branches appropriately.
  - Markdown is well-formatted.

### Sub-task 1.6: Complete "Choice Action (External Choice - Choosing Side)" Section

- **File(s):** `docs/duality.md`
- **Section(s):** "Core Protocol Constructs: Definitions and Duality" -> "Choice Action (External Choice - Choosing Side)"
- **Goal:** Provide a complete and accurate description, Rust definitions, `CommMetadata` integration, duality rule, invariants, roles, and parameters for the Choice action.
- **Invariants:**
  - Document remains markdownlint compliant.
  - The fundamental semantics of the Choice action are preserved.
- **Pre-conditions:**
  - Sub-task 1.5 completed.
  - Basic structure for the "Choice Action" section exists.
- **Post-conditions:**
  - Semantic explanation is clear.
  - Rust definitions for conceptual `TChanChoice` and `EpChoice` (with type-level lists for branches) are complete.
  - `CommMetadata` integration for choice signaling is detailed.
  - Duality rule, invariants, roles, and parameters are accurate.
- **Acceptance Criteria:**
  - All subsections are fully populated and accurate.
  - Rust definitions are correct.
  - Markdown is well-formatted.

### Sub-task 1.7: Complete "Par Action (Parallel Composition)" Section

- **File(s):** `docs/duality.md`
- **Section(s):** "Core Protocol Constructs: Definitions and Duality" -> "Par Action (Parallel Composition)"
- **Goal:** Provide a complete and accurate description, Rust definitions, `CommMetadata` integration, duality rule, invariants, roles, and parameters for the Par action.
- **Invariants:**
  - Document remains markdownlint compliant.
  - The fundamental semantics of the Par action are preserved.
  - Emphasis on `ChanId` disjointness is maintained.
- **Pre-conditions:**
  - Sub-task 1.6 completed.
  - Basic structure for the "Par Action" section exists.
- **Post-conditions:**
  - Semantic explanation is clear.
  - Rust definitions for `TChanPar` and `EpPar` are complete.
  - `CommMetadata` integration explains how constituent protocols use their own `CommMetadata` and the disjointness requirement for `ChanId`s.
  - Duality rule, invariants (including `ChanId` disjointness), roles, and parameters are accurate.
  - Mermaid diagram illustrating parallel execution with disjoint `ChanId`s is present, correct, and renders properly.
- **Acceptance Criteria:**
  - All subsections are fully populated and accurate.
  - Rust definitions are correct.
  - Explanation of `ChanId` disjointness is clear.
  - Mermaid diagram is correct and illustrative.
  - Markdown is well-formatted.

### Sub-task 1.8: Complete "Seq Action (Sequential Composition)" Section

- **File(s):** `docs/duality.md`
- **Section(s):** "Core Protocol Constructs: Definitions and Duality" -> "Seq Action (Sequential Composition)"
- **Goal:** Provide a complete and accurate description, Rust definitions, `CommMetadata` integration, duality rule, invariants, roles, and parameters for the Seq action.
- **Invariants:**
  - Document remains markdownlint compliant.
  - The fundamental semantics of the Seq action are preserved.
- **Pre-conditions:**
  - Sub-task 1.7 completed.
  - Basic structure for the "Seq Action" section exists.
- **Post-conditions:**
  - Semantic explanation is clear.
  - Rust definitions for `TChanSeq` and `EpSeq` are complete.
  - `CommMetadata` integration explains how constituent protocols use their own `CommMetadata`.
  - Duality rule, invariants, roles, and parameters are accurate.
- **Acceptance Criteria:**
  - All subsections are fully populated and accurate.
  - Rust definitions are correct.
  - Markdown is well-formatted.

### Sub-task 1.9: Complete "Rec Action (Recursion)" Section

- **File(s):** `docs/duality.md`
- **Section(s):** "Core Protocol Constructs: Definitions and Duality" -> "Rec Action (Recursion)"
- **Goal:** Provide a complete and accurate description, Rust definitions, `CommMetadata` integration, duality rule, invariants, roles, and parameters for the Rec action.
- **Invariants:**
  - Document remains markdownlint compliant.
  - The fundamental semantics of the Rec action are preserved.
- **Pre-conditions:**
  - Sub-task 1.8 completed.
  - Basic structure for the "Rec Action" section exists.
- **Post-conditions:**
  - Semantic explanation of recursion and recursion variables is clear.
  - Rust definitions for `TChanRec` and `EpRec` (using recursion variable types) are complete.
  - `CommMetadata` integration (typically none for Rec itself, but for its body) is explained.
  - Duality rule, invariants, roles, and parameters are accurate.
- **Acceptance Criteria:**
  - All subsections are fully populated and accurate.
  - Rust definitions correctly model recursion.
  - Markdown is well-formatted.

### Sub-task 1.10: Complete "Continue Action (Recursion Invocation)" Section

- **File(s):** `docs/duality.md`
- **Section(s):** "Core Protocol Constructs: Definitions and Duality" -> "Continue Action (Recursion Invocation)"
- **Goal:** Provide a complete and accurate description, Rust definitions, `CommMetadata` integration, duality rule, invariants, roles, and parameters for the Continue action.
- **Invariants:**
  - Document remains markdownlint compliant.
  - The fundamental semantics of the Continue action are preserved.
- **Pre-conditions:**
  - Sub-task 1.9 completed.
  - Basic structure for the "Continue Action" section exists.
- **Post-conditions:**
  - Semantic explanation of invoking a recursion variable is clear.
  - Rust definitions for `TChanContinue` and `EpContinue` (using recursion variable types) are complete.
  - `CommMetadata` integration (typically none) is explained.
  - Duality rule, invariants, roles, and parameters are accurate.
- **Acceptance Criteria:**
  - All subsections are fully populated and accurate.
  - Rust definitions correctly model recursion invocation.
  - Markdown is well-formatted.

### Sub-task 1.11: Complete "Init Action (Session Initialization)" Section

- **File(s):** `docs/duality.md`
- **Section(s):** "Core Protocol Constructs: Definitions and Duality" -> "Init Action (Session Initialization)"
- **Goal:** Provide a complete and accurate description, Rust definitions, `CommMetadata` integration, duality rule, invariants, roles, and parameters for the Init action.
- **Invariants:**
  - Document remains markdownlint compliant.
  - The fundamental semantics of the Init action are preserved.
- **Pre-conditions:**
  - Sub-task 1.10 completed.
  - Basic structure for the "Init Action" section exists.
- **Post-conditions:**
  - Semantic explanation of session initialization is clear.
  - Rust definitions for `TChanInit` and `EpInit` are complete.
  - `CommMetadata` integration (if applicable for signaling init) is explained.
  - Duality rule, invariants, roles, and parameters are accurate.
- **Acceptance Criteria:**
  - All subsections are fully populated and accurate.
  - Rust definitions are correct.
  - Markdown is well-formatted.

### Sub-task 1.12: Complete "End Action (Session Termination)" Section

- **File(s):** `docs/duality.md`
- **Section(s):** "Core Protocol Constructs: Definitions and Duality" -> "End Action (Session Termination)"
- **Goal:** Provide a complete and accurate description, Rust definitions, `CommMetadata` integration, duality rule, invariants, roles, and parameters for the End action.
- **Invariants:**
  - Document remains markdownlint compliant.
  - The fundamental semantics of the End action are preserved.
- **Pre-conditions:**
  - Sub-task 1.11 completed.
  - Basic structure for the "End Action" section exists.
- **Post-conditions:**
  - Semantic explanation of session termination is clear.
  - Rust definitions for `TChanEnd` and `EpEnd` are complete.
  - `CommMetadata` integration (typically none) is explained.
  - Duality rule, invariants, roles, and parameters are accurate.
- **Acceptance Criteria:**
  - All subsections are fully populated and accurate.
  - Rust definitions are correct.
  - Markdown is well-formatted.

## Phase 2: Predicates and Functions Completion

This phase focuses on the detailed rules for `IsDual`, `IsWellFormed`, and the Projection Function.

### Sub-task 2.1: Complete "IsDual Predicate: Detailed Rules" Section

- **File(s):** `docs/duality.md`
- **Section(s):** "IsDual Predicate: Detailed Rules"
- **Goal:** Ensure all correspondence rules (Send/Receive, Offer/Choice, etc.) are clearly explained and accurate.
- **Invariants:**
  - Document remains markdownlint compliant.
  - The definition of duality is consistent with MPST principles.
- **Pre-conditions:**
  - Phase 1 (Sub-tasks 1.1 - 1.12) completed.
  - Basic structure for this section exists.
- **Post-conditions:**
  - Each correspondence rule (Send/Receive, Offer/Choice, Parallel, Sequential, Recursive, Init, End) is fully elaborated and correct.
  - The role of `CommMetadata`, message types, and `IO` consistency in duality checks is clear.
  - The "Commentary" subsection, if present, provides valuable insights.
- **Acceptance Criteria:**
  - All subsections for correspondence rules are complete and accurate.
  - Explanations are clear and unambiguous.
  - Examples are provided if they enhance understanding.
  - Markdown is correctly formatted.

### Sub-task 2.2: Complete "Well-Formedness of Global Protocols (`IsWellFormed`)" Section

- **File(s):** `docs/duality.md`
- **Section(s):** "Well-Formedness of Global Protocols (`IsWellFormed`)"
- **Goal:** Detail the process for checking `IsWellFormed(G)` and explain local endpoint protocol well-formedness.
- **Invariants:**
  - Document remains markdownlint compliant.
  - The definition of well-formedness is consistent with MPST principles and its reliance on duality.
- **Pre-conditions:**
  - Sub-task 2.1 completed.
  - Basic structure for this section exists.
- **Post-conditions:**
  - The subsection "Global Protocol Well-Formedness (`IsWellFormed(G)`)" clearly explains the check.
  - The subsection "Local Endpoint Protocol Well-Formedness (`IsWellFormed(EpP<IO, ...>)`)" is complete and accurate.
  - The subsection "Predicate: `IsWellFormed(G)` (Summary of the Check Process)" provides a clear, step-by-step summary.
  - The role of projection and pairwise duality checks is emphasized.
- **Acceptance Criteria:**
  - All subsections are fully elaborated and accurate.
  - The relationship between global well-formedness, projection, and local duality is clearly explained.
  - Markdown is correctly formatted.

### Sub-task 2.3: Complete "Projection Function: Detailed Rules" Section

- **File(s):** `docs/duality.md`
- **Section(s):** "Projection Function: Detailed Rules"
- **Goal:** Ensure each projection rule (Projecting Send Action, Projecting Receive Action, etc.) is clearly explained and accurate.
- **Invariants:**
  - Document remains markdownlint compliant.
  - Projection rules are consistent with MPST theory.
  - Headings follow the "Projecting X Action" format.
- **Pre-conditions:**
  - Sub-task 2.2 completed.
  - Headings in this section are already standardized.
  - Basic structure for projection rules exists.
- **Post-conditions:**
  - Each projection rule (Send, Receive, Offer, Choice, Par, Seq, Rec, Continue, Init, End) is fully explained.
  - The explanation details how a global action translates to a local action for a specific role.
  - The role of `IO` capabilities and `SupportsActionIO` checks during projection is reiterated where relevant.
- **Acceptance Criteria:**
  - All "Projecting X Action" subsections are complete and accurate.
  - Explanations are clear, detailing the transformation from global to local types.
  - Examples are provided if they enhance understanding.
  - Markdown is correctly formatted.

## Phase 3: Rust Implementation and Finalization

### Sub-task 3.1: Complete "Type-Level Implementation in Rust" Section

- **File(s):** `docs/duality.md`
- **Section(s):** "Type-Level Implementation in Rust"
- **Goal:** Detail how MPST constructs, `IsDual`, and `IsWellFormed` are implemented at the type-level in Rust.
- **Invariants:**
  - Document remains markdownlint compliant.
  - Explanations align with idiomatic Rust type-level programming patterns.
- **Pre-conditions:**
  - Phase 2 (Sub-tasks 2.1 - 2.3) completed.
  - Basic structure for this section exists.
- **Post-conditions:**
  - "Representing Protocol Constructs": Clearly explains how Rust types and traits model global and local protocol actions.
  - "Implementing `IsDual`": Explains the trait-based approach to checking duality at compile time.
  - "Implementing `IsWellFormed`": Explains how projection and `IsDual` are combined for well-formedness checks.
  - Conceptual Rust code snippets are provided to illustrate key implementation patterns.
- **Acceptance Criteria:**
  - All subsections are complete, accurate, and provide a good overview of the type-level implementation strategy.
  - Explanations are accessible to readers familiar with Rust and type systems.
  - Conceptual code snippets are illustrative and correct.
  - Markdown is correctly formatted.

### Sub-task 3.2: Review and Refine Introduction and Conclusion

- **File(s):** `docs/duality.md`
- **Section(s):** Introduction (before ToC), "Conclusion"
- **Goal:** Ensure the introduction accurately sets the stage for the document's content and the conclusion effectively summarizes key takeaways.
- **Invariants:**
  - Document remains markdownlint compliant.
  - The core message of the document is preserved.
- **Pre-conditions:**
  - Sub-task 3.1 completed.
  - Existing introduction and conclusion sections are present.
- **Post-conditions:**
  - Introduction clearly outlines the document's scope, including `CommMetadata`, `IO` parameters, `IsDual`, and well-formedness.
  - Conclusion summarizes the importance of duality, `CommMetadata`, and the verification predicates.
  - Both sections are polished, coherent, and engaging.
- **Acceptance Criteria:**
  - Introduction accurately reflects the final content of the document.
  - Conclusion provides a strong summary and reinforces key concepts.
  - Language is clear, concise, and professional.
  - Markdown is correctly formatted.

### Sub-task 3.3: Final Document Review and Linting

- **File(s):** `docs/duality.md`
- **Section(s):** Entire document
- **Goal:** Perform a comprehensive review for consistency, clarity, technical accuracy, and grammatical correctness. Ensure the document passes all markdown linting checks.
- **Invariants:**
  - All previously completed content remains correct.
- **Pre-conditions:**
  - Sub-task 3.2 completed.
  - The document is feature-complete.
- **Post-conditions:**
  - The document is free of typos, grammatical errors, and awkward phrasing.
  - Terminology is used consistently throughout.
  - All explanations are clear and unambiguous.
  - Technical details are accurate.
  - The document flows logically.
  - The document passes `markdownlint-cli2` checks without errors.
- **Acceptance Criteria:**
  - A full read-through confirms overall quality and coherence.
  - All identified issues (content, grammar, style) are addressed.
  - `scripts/md-lint.sh` (or equivalent `markdownlint-cli2` command) passes for `docs/duality.md`.
