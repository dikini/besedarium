<!-- filepath: /home/dikini/Projects/besedarium/work/TASKS.md -->

# Task List: Protocol Label Invariants and Refactor (Issue #15)

- [ ] (invariant) All protocol combinators (TEnd, TSend, TRecv, TChoice, TPar, TRec, etc.) must have a label parameter of type `ProtocolLabel`.
- [ ] (invariant) The trait `GetProtocolLabel` must be implemented for all protocol combinators, not just TSend/TRecv.
- [ ] (invariant) All combinators must preserve and propagate label information through composition and projection.
- [ ] (invariant) All documentation, code comments, and examples must reflect that every combinator is labeled and supports label extraction.
- [ ] (invariant) When removing or modifying traits/impls, always describe explicitly which traits, structures, or impls are affected.

# Task List: TInteract → TSend/TRecv Refactor (Issue #15)

- [x] Stabilize test base: disable or clear all failing and affected tests (unit, integration, trybuild, protocol examples)
- [x] Fix projection doctest in README.md/lib.rs to use correct generic arguments
- [x] Push branch and create draft PR for issue #15
- [ ] Refactor protocol combinators: replace all uses of TInteract with TSend and TRecv in global protocol definitions
- [ ] Update all projection and helper traits to support TSend/TSend instead of TInteract
- [ ] Update macros, documentation, and code examples to use TSend/TRecv
- [ ] Update and re-enable all previously disabled/cleared tests to use the new combinators
- [ ] Ensure all tests and doctests pass after refactor
- [ ] Update changelog, learnings, and documentation to reflect the refactor
- [ ] Request review and finalize PR for merge (issue #16)

---

# Task List: PR Review and Merge (Issue #16)

- [ ] Review draft PR for completeness and correctness
- [ ] Confirm all CI checks pass
- [ ] Approve and merge PR after review
- [ ] Close issues #15 and #16

---

> **Protocol Label Invariant:**
> All protocol combinators must have a label parameter and implement `GetProtocolLabel`.
> This is a core design rule for Besedarium. Update this file and documentation if the invariant changes.
