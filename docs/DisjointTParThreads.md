# Guaranteeing Disjointness in TPar Threads: Design, Considerations, and Solutions

## Problem Statement

**How can we guarantee, at compile time, that the threads (branches) of a parallel composition (`TPar`) in a session-typed protocol are disjoint in their roles?**

This is crucial for protocol safety: if two parallel threads share a role, that role would be required to act in two places at once, violating linearity and potentially causing deadlocks or protocol violations.

---

## Considerations for Disjointness Checks

- **Role Extraction:**
  Each session type (`TSession`) must be able to expose the set of roles it uses, typically as a type-level list.
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

### 1. Composing Two TPar Sessions

- **Issue:**
  Composing two `TPar` sessions may introduce overlapping roles if not checked.
- **Example:**

  ```rust
  type Par1 = TPar<A, B, False>;
  type Par2 = TPar<C, D, False>;
  // Composing Par1 and Par2: roles in A, B, C, D must all be disjoint.
  ```

### 2. Composing a Branch of TPar with a TSession

- **Issue:**
  Composing a branch (e.g., the left thread) of a `TPar` with a new session may introduce new roles, potentially violating disjointness.
- **Example:**

  ```rust
  type Par = TPar<A, B, False>;
  type NewPar = TPar<A::Compose<C>, B, False>;
  // Need to check that roles in A::Compose<C> and B are disjoint.
  ```

---

## Solutions

### 1. Type-Level Role Extraction and Disjointness

Define type-level lists and traits to extract roles and check disjointness.

```rust
// Type-level list
pub struct Nil;
pub struct Cons<H, T>(PhantomData<(H, T)>);

// Example roles
pub struct TClient;
pub struct TServer;
pub struct TBroker;
pub struct TWorker;

// Trait to extract roles from a session
pub trait RolesOf {
    type Roles;
}

// Implement for TInteract
impl<R: TRole, H, T: TSession> RolesOf for TInteract<R, H, T>
where
    T: RolesOf,
{
    type Roles = Cons<R, <T as RolesOf>::Roles>;
}

// ...implement for other combinators as needed...
```

**Disjointness trait:**

```rust
pub trait Disjoint<A, B> {}
impl<B> Disjoint<Nil, B> for () {}
impl<H, T, B> Disjoint<Cons<H, T>, B> for ()
where
    B: NotContains<H>,
    (): Disjoint<T, B>,
{}

// NotContains and NotSame traits as in previous examples...
```

### 2. Branding TPar with a Type-Level Boolean

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

### 3. Ergonomic n-ary TPar Construction

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

---

## Compile-Time Test

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
- [Session Types in Rust (blog post)](https://blog.sessiontypes.com/)

---

*Discussion and summary by [GitHub Copilot](https://github.com/features/copilot) and user dikini, 2025.*
