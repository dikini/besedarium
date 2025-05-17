<!-- filepath: /home/dikini/Projects/besedarium/work/Plan.md -->

# Work Plan: Protocol Label Invariant and TInteract → TSend/TRecv Refactor (Issues #15 & #16)

## Overview

This plan outlines the steps to refactor the Besedarium session types library by replacing the global protocol combinator `TInteract` with distinct `TSend` and `TRecv` types. The goal is to improve protocol clarity, type-level expressiveness, and future extensibility. The plan also covers stabilization of the test base, updating documentation, and review/merge criteria.

- All protocol combinators (TEnd, TSend, TRecv, TChoice, TPar, TRec, etc.) must have a label parameter of type `ProtocolLabel`.
- The trait `GetProtocolLabel` must be implemented for all protocol combinators.
- This invariant must be maintained in all future refactors and protocol design changes.

---

## 1. Test Base Stabilization (Precondition for Refactor)

**Goal:** Ensure a stable, passing test base before making protocol-level changes.

- Disable or clear all failing and affected tests (unit, integration, trybuild, protocol examples).
- Fix all failing doctests, especially those affected by protocol combinator changes.
- Confirm that `cargo test` and all doctests pass with no failures.

**Review criteria:** No failing or flaky tests; all doctests pass; CI is green.

---

## 2. Protocol Refactor: TInteract → TSend/TRecv

**Goal:** Replace all uses of `TInteract` with `TSend` and `TRecv` in global protocol definitions and supporting code.

- Refactor protocol combinators in the main library code.
- Update all projection and helper traits to support `TSend`/`TRecv` instead of `TInteract`.
- Update macros, documentation, and code examples to use `TSend`/`TRecv`.

**Precondition:** Test base is stable and all tests are passing.
**Postcondition:** All protocol logic uses the new combinators; code compiles and passes all checks.

**Review criteria:** No remaining uses of `TInteract`; all new combinators are documented and tested; code is idiomatic and clear.

---

## 3. Test Restoration and Update

**Goal:** Restore and update all previously disabled/cleared tests to use the new protocol combinators.

- Update all test files (unit, integration, trybuild, protocol examples) to use `TSend`/`TRecv`.
- Re-enable and verify all tests.

**Precondition:** Protocol refactor is complete and compiles.
**Postcondition:** All tests and doctests pass with the new combinators.

**Review criteria:** Test coverage is restored; all tests are meaningful and up to date.

---

## 4. Documentation, Changelog, and Learnings Update

**Goal:** Ensure all documentation, changelogs, and learnings reflect the new protocol structure and the protocol label invariant.

- Update README, code docs, and protocol examples to show that all combinators are labeled and implement `GetProtocolLabel`.
- Update CHANGELOG.md with a summary of the refactor and the invariant.
- Update work/learnings.md and related files with new patterns and lessons.
- Update work/Status.md

**Review criteria:** Documentation is accurate, clear, and passes markdownlint; changelog is up to date; protocol label invariant is documented and enforced.

---

## 5. PR Review and Merge (Issue #16)

**Goal:** Complete the review and merge process for the refactor.

- Review draft PR for completeness, correctness, and adherence to project guidelines.
- Confirm all CI checks pass.
- Approve and merge PR after review.
- Close issues #15 and #16.

**Review criteria:** All acceptance criteria met; no regressions; project is ready for further development.

---

## Summary

- **Preconditions:** Stable test base, all tests passing.
- **Postconditions:** All protocol logic and tests use `TSend`/`TRecv`; all combinators are labeled and implement `GetProtocolLabel`; documentation and changelog are updated; PR is reviewed and merged.
- **Success criteria:** No regressions, improved clarity and maintainability, all project standards and invariants met.

---

> **Protocol Label Invariant:**
> All protocol combinators must have a label parameter and implement `GetProtocolLabel`.
> This is a core design rule for Besedarium. Update this file and documentation if the invariant changes.
