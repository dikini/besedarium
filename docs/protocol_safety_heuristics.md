# Protocol Safety Rules and Heuristics in Session Types

Below is a list of key restrictions, heuristics, and rules (with discussion) that are
commonly used to guarantee protocol safety in session-typed systems. These rules may be
layered, providing increasing tiers of safety.

---

## 1. Disjoint Roles in `Par` Branches

- **Rule:**
  Each parallel branch in a `Par` composition must have a disjoint set of roles.

- **Prevents/Guarantees:**
  Prevents a single role from being required to act in two places at once, ensuring
  linearity and avoiding race conditions or deadlocks.

- **Discussion:**
  This is a foundational rule for parallel composition. If violated, a role could be
  forced to make conflicting choices or actions, breaking protocol safety and progress
  guarantees.

---

## 2. Matching Continuations in `Choice` and `Par`

- **Rule:**
  All branches of a `Choice` or `Par` must converge to the same continuation (i.e.,
  after branching, the protocol rejoins at a common point).

- **Prevents/Guarantees:**
  Prevents protocol divergence and ensures that all participants can synchronize and
  proceed together.

- **Discussion:**
  This rule enforces that, regardless of the branch taken, the protocol structure remains
  compatible and predictable for all roles.

---

## 3. Linearity of Channels/Roles

- **Rule:**
  Each channel or role must be used exactly once (no duplication or discarding).

- **Prevents/Guarantees:**
  Prevents resource leaks, double use, or loss of communication endpoints, ensuring that
  all resources are accounted for and protocols complete as intended.

- **Discussion:**
  Linearity is a core property of session types, enforced by the type system (often via
  ownership or affine types in Rust).

---

## 4. No Cyclic Dependencies Without Recursion

- **Rule:**
  Protocols must not have cyclic dependencies unless explicitly marked as recursive
  (e.g., using `Rec` or fixpoint combinators).

- **Prevents/Guarantees:**
  Prevents infinite loops or deadlocks that are not intentional or well-formed.

- **Discussion:**
  Recursion must be explicit and well-scoped to ensure that cycles in the protocol are
  intentional and manageable.

---

## 5. Well-Formedness of Choices

- **Rule:**
  All branches of a `Choice` must be valid and available to the participant making the choice.

- **Prevents/Guarantees:**
  Prevents deadlocks or stuck states where a participant cannot proceed because a branch
  is unavailable.

- **Discussion:**
  This ensures that the protocol remains live and that all advertised choices are actually
  possible.

---

## 6. Progress (No Stuck States)

- **Rule:**
  At every point in the protocol, at least one participant can make progress.

- **Prevents/Guarantees:**
  Prevents deadlocks and ensures liveness.

- **Discussion:**
  This is a global property, often implied by the above rules, but may require additional
  checks (e.g., global progress analysis in multiparty session types).

---

## 7. Duality of Endpoints

- **Rule:**
  For every session type, there must exist a dual session type for the communicating party.

- **Prevents/Guarantees:**
  Ensures that messages sent by one party are expected and received by the other,
  preventing protocol mismatches.

- **Discussion:**
  Duality is a key property for binary session types, ensuring that communication actions
  are compatible.

---

## 8. Explicit Termination

- **Rule:**
  Protocols must terminate explicitly with an `End` or equivalent marker.

- **Prevents/Guarantees:**
  Prevents dangling or incomplete sessions, ensuring that all resources are released and
  all parties know when the protocol is finished.

- **Discussion:**
  Explicit termination is important for resource management and protocol clarity.

---

## 9. No Unreachable Branches

- **Rule:**
  All branches in a protocol must be reachable by some sequence of actions.

- **Prevents/Guarantees:**
  Prevents dead code and ensures protocol completeness.

- **Discussion:**
  This is analogous to exhaustiveness checking in pattern matching.

---

## 10. Tiered Safety Nets

- **Tier 1:** Basic linearity and duality (prevents resource misuse and protocol mismatches).
- **Tier 2:** Disjoint roles in parallel, matching continuations, explicit recursion
  (prevents deadlocks and divergence).
- **Tier 3:** Global progress, well-formedness, and reachability (ensures liveness and
  completeness).

---

## References

- Honda, K., Yoshida, N., & Carbone, M. (2008). [Multiparty Asynchronous Session
Types](https://www.cs.kent.ac.uk/people/staff/srm25/research/multiparty/)
- Gay, S. J., & Vasconcelos, V. T. (2010). [Linear type theory for asynchronous session
types](https://www.dcs.gla.ac.uk/~simon/publications/linear-session-types.pdf)
- Wadler, P. (2012). [Propositions as
sessions](https://homepages.inf.ed.ac.uk/wadler/papers/propositions-as-sessions/propositions-as-sessions.pdf)

---
