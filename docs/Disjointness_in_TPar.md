# Disjointness in Parallel Composition (`TPar`) in Session Types: Theory, Design, and Rust Implementation

## Introduction

Parallel composition (`TPar`) is a fundamental combinator in session type theory, enabling the concurrent execution of protocol branches. A critical safety property for `TPar` is **disjointness of roles**: no role may appear in more than one parallel branch. This document consolidates the theory, design considerations, and Rust implementation strategies for enforcing disjointness in `TPar`.

---

## Why Disjoint Role Sets Matter

- **Linearity:** Each role must have a single, unambiguous sequence of actions. If a role appears in multiple branches, it would be required to act in two places at once, violating linearity.
- **Safety:** Disjointness prevents race conditions, deadlocks, and protocol violations.
- **Progress:** Ensures that all branches can proceed independently and synchronize correctly at the end.

---

## Problem Statement

**How can we guarantee, at compile time, that the branches of a parallel composition (`TPar`) are disjoint in their roles?**

This is crucial for protocol safety: if two parallel threads share a role, that role would be required to act in two places at once, violating linearity and potentially causing deadlocks or protocol violations.

---

## Design Considerations for Disjointness Checks

- **Role Extraction:**
  Each session type (`TSession`) must expose the set of roles it uses, typically as a type-level list.
- **Disjointness Checking:**
  There must be a way to check, at the type level, that two (or more) sets of roles are disjoint.
- **Compositionality:**
  Disjointness must be preserved or re-checked after composing sessions, whether by composing two `TPar`s or by composing a branch of a `TPar` with an arbitrary `TSession`.
- **Ergonomics:**
  The API should be ergonomic for users, with compile-time errors when disjointness is violated and minimal boilerplate for common cases.
- **Default Safety:**
  By default, `TPar` branches are **not safe**. Only after an explicit check should a `TPar` be considered safe.

---

## Problems Arising from Composition

### 1. Composing Two `TPar` Sessions

- **Issue:**
  Composing two `TPar` sessions may introduce overlapping roles if not checked.
- **Example:**

  ```rust
  type Par1 = TPar<A, B, False>;
  type Par2 = TPar<C, D, False>;
  // Composing Par1 and Par2: roles in A, B, C, D must all be disjoint.
  ```

### 2. Composing a Branch of `TPar` with a `TSession`

- **Issue:**
  Composing a branch (e.g., the left thread) of a `TPar` with a new session may introduce new roles, potentially violating disjointness.
- **Example:**

  ```rust
  type Par = TPar<A, B, False>;
  type NewPar = TPar<A::ComposeC, B, False>;
  // Need to check that roles in A::ComposeC and B are disjoint.
  ```

---

## Enforcing Disjointness at the Type Level in Rust

Rust's type system can encode type-level sets and traits to enforce disjointness at compile time. Below is a minimal example for four roles (`A`, `B`, `C`, `D`).

### Type-Level List and Role Definitions

```rust
// Define roles
pub struct A;
pub struct B;
pub struct C;
pub struct D;

// Type-level list
pub struct Nil;
pub struct Cons<H, T>(PhantomData<(H, T)>);
```

### Role Extraction Trait

```rust
// Trait to extract roles from a session
type RolesOf = ...; // Implemented for each session combinator
```

### Disjointness Traits

```rust
// Trait to check if a role is in a list
pub trait Contains<X> {}
impl<X, T> Contains<X> for Cons<X, T> {}
impl<X, H, T> Contains<X> for Cons<H, T>
where
    T: Contains<X>,
{}
impl<X> Contains<X> for Nil {} // Not found

// Trait to check disjointness of two lists
pub trait Disjoint<A, B> {}
impl<B> Disjoint<Nil, B> for () {}
impl<H, T, B> Disjoint<Cons<H, T>, B> for ()
where
    B: NotContains<H>,
    (): Disjoint<T, B>,
{}

// Helper trait: NotContains
pub trait NotContains<X> {}
impl<X> NotContains<X> for Nil {}
impl<X, H, T> NotContains<X> for Cons<H, T>
where
    X: NotSame<H>,
    T: NotContains<X>,
{}

// Helper trait: NotSame
pub trait NotSame<X> {}
impl<X, Y> NotSame<X> for Y where X: NotEqual<Y> {}

// Helper trait: NotEqual
pub trait NotEqual<X> {}
impl NotEqual<A> for B {}
impl NotEqual<A> for C {}
impl NotEqual<A> for D {}
impl NotEqual<B> for A {}
impl NotEqual<B> for C {}
impl NotEqual<B> for D {}
impl NotEqual<C> for A {}
impl NotEqual<C> for B {}
impl NotEqual<C> for D {}
impl NotEqual<D> for A {}
impl NotEqual<D> for B {}
impl NotEqual<D> for C {}
```

---

## Branding `TPar` with a Type-Level Boolean

Brand `TPar` with a type-level boolean indicating if it is known to be disjoint.

```rust
pub struct True;
pub struct False;

pub struct TPar<L: TSession, R: TSession, IsDisjoint>(PhantomData<(L, R, IsDisjoint)>);
```

- **After composition:**
  Mark as `False` (unknown/disjointness not guaranteed).
- **After explicit check:**
  Provide a trait or function to check and rebrand as `True`.

**Example:**

```rust
type UnsafePar = TPar<A, B, False>;
// User must explicitly check and rebrand:
type SafePar = AssertDisjoint<UnsafePar>;
```

---

## Ergonomic n-ary `TPar` Construction

Use a type-level list and a trait to map it to nested `TPar`:

```rust
pub trait ToTPar {
    type Output: TSession;
}

impl<H: TSession> ToTPar for Cons<H, Nil> {
    type Output = H;
}

impl<H: TSession, T: ToTPar> ToTPar for Cons<H, T> {
    type Output = TPar<H, <T as ToTPar>::Output, False>;
}
```

---

## Examples

### Example 1: Compile-Time Disjointness Assertion

```rust
type ParThreads = Cons<
    TInteract<TClient, Message, TEnd>,
    Cons<
        TInteract<TServer, Publish, TEnd>,
        Nil
    >
>;

type ParSession = <ParThreads as ToTPar>::Output;

// Compile-time assertion (will fail if not disjoint)
fn _assert_disjoint<T: ToTPar>() where
    (): Disjoint<
        <T as ToTPar>::Output, // extract roles from all branches
        Nil // (or union of roles from other branches)
    >,
{}
```

### Example 2: Branding

```rust
type UnsafePar = TPar<A, B, False>;
// User must explicitly check and rebrand:
type SafePar = AssertDisjoint<UnsafePar>;
```

### Example 3: Four-Way Disjointness

```rust
type FourWayThreads = Cons<
    TInteract<TClient, Message, TEnd>,
    Cons<
        TInteract<TServer, Publish, TEnd>,
        Cons<
            TInteract<TBroker, Notify, TEnd>,
            Cons<
                TInteract<TWorker, Subscribe, TEnd>,
                Nil
            >
        >
    >
>;

type FourWayPar = <FourWayThreads as ToTPar>::Output;

// Compile-time assertion: should succeed if all roles are disjoint
fn _assert_fourway_disjoint()
where
    (): Disjoint<
        <TInteract<TClient, Message, TEnd> as RolesOf>::Roles,
        <TInteract<TServer, Publish, TEnd> as RolesOf>::Roles
    >,
    (): Disjoint<
        <TInteract<TBroker, Notify, TEnd> as RolesOf>::Roles,
        <TInteract<TWorker, Subscribe, TEnd> as RolesOf>::Roles
    >,
{}
```

---

## Summary

- Disjointness of roles in `TPar` is essential for protocol safety.
- Type-level lists and traits can extract and check roles at compile time.
- Branding with a type-level boolean can enforce explicit checks after composition.
- By default, `TPar` is not safe; only explicit checks can mark it as safe.
- Compile-time assertions ensure correctness and provide ergonomic, safe APIs.

---

## References

- Honda, K., Yoshida, N., & Carbone, M. (2008). [Multiparty Asynchronous Session Types](https://www.cs.kent.ac.uk/people/staff/srm25/research/multiparty/)
- Gay, S. J., & Vasconcelos, V. T. (2010). [Linear type theory for asynchronous session types](https://www.dcs.gla.ac.uk/~simon/publications/linear-session-types.pdf)
- Scalas, A., & Yoshida, N. (2016). [Lightweight Session Programming in Scala](https://www.doc.ic.ac.uk/~cn06/papers/2016-ecoop.pdf)
- Rust type-level programming: [The Typenum crate](https://docs.rs/typenum/latest/typenum/) and [The typelist crate](https://docs.rs/typelist/latest/typelist/).
- [Session Types in Rust (blog post)](https://blog.sessiontypes.com/)

---

*Discussion and summary by GitHub Copilot and user dikini, 2025.*
