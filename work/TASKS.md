<!-- filepath: /home/dikini/Projects/besedarium/work/TASKS.md -->

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
