# Plan: Introducing User-Definable Labels to Protocols

## Overview

This document outlines the plan for adding user-definable labels to protocol combinators in Besedarium. The goal is to improve protocol clarity, enable better code generation and projection, and enforce label uniqueness at compile time, while keeping the system ergonomic and flexible for library users.

---

## 1. Label Representation

- **Marker Types (Implemented):**
  - Use Rust marker types (structs implementing `ProtocolLabel`) for labels.
  - Users define their own label types, e.g.
    `struct MyLabel; impl ProtocolLabel for MyLabel {}`
  - Const generics are not used, as they are not feasible on stable Rust for this use case.

## 2. Placeholder/Empty Labels

- Provide a documented convention for an empty label, e.g.:

  ```rust
  pub struct EmptyLabel;
  impl ProtocolLabel for EmptyLabel {}
  ```

- Users can use this as a placeholder where a label is not meaningful.

## 3. Adding Labels to Protocol Combinators

- All combinators (TInteract, TChoice, TRec, TEnd, TPar, etc.) take a label type parameter.
- Users supply their own label types when defining protocols.
- For combinators where a label is not meaningful, use the empty label as a default.

## 4. Enforcing Uniqueness

- **Type Guards/Traits:**
  - Implement type-level checks (traits/macros) that ensure all labels in a protocol are unique, regardless of how they are defined.
- **Macro Support (Optional):**
  - Provide a macro for protocol definition that checks for duplicate labels at compile time, but always allow users to supply their own.
- **Runtime Check (fallback):**
  - Not used; all checks are at compile time.

## 5. Migration and Backwards Compatibility

- Not applicable: this is a greenfield library, so no migration is needed.

## 6. Documentation and Examples

- Documentation explains label usage, uniqueness requirements, and the purpose of empty labels.
- Examples are provided for both labeled and unlabeled (placeholder) protocols.
- Any built-in/test labels are clearly for demonstration and not for production.

## 7. Optional: Tooling Support

- Consider writing a linter or codegen tool to help users generate unique labels and check protocols (future work).

---

## Example Usage

```rust
// User-defined labels
struct StartLabel; impl ProtocolLabel for StartLabel {}
struct AcceptLabel; impl ProtocolLabel for AcceptLabel {}
struct RetryLabel; impl ProtocolLabel for RetryLabel {}

// Using in a protocol
type MyProtocol = TRec<Http, StartLabel, ...>;
```

---

## Summary Table

| Step                     | Choice/Option              | User-definable? | Notes                     |
|--------------------------|----------------------------|-----------------|---------------------------|
| Label representation     | Marker types (structs)     | Yes             | Users define their own    |
| Placeholder labels       | EmptyLabel                 | Yes             | Convention, not enforced  |
| Uniqueness enforcement   | Traits/macros              | Yes             | Works with user labels    |
| Test/dev labels          | Temporary, separate module | Yes             | Remove before release     |

---

## Guidance

- **Default to user-definable labels:** All protocol labels should be supplied by users, not hardcoded in the library.
- **Document conventions:** Clearly explain how to define and use labels, and how to use the empty label as a placeholder.
- **Enforce uniqueness:** Use type-level or macro-based checks to ensure all labels in a protocol are unique.
- **No migration needed:** This is a greenfield library.
- **Keep test/dev labels separate:** Make it easy to remove or replace any built-in labels before release.

---

Prepared by GitHub Copilot, 12 May 2025
