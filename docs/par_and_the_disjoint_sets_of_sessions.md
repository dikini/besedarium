# The `Par` Combinator and Disjoint Role Sets in Session Types

## Overview

In session type theory, the `Par` (parallel composition) combinator allows two protocol fragments
to proceed concurrently. A crucial safety property is that the sets of roles participating in each
branch must be **disjoint**—no role may appear in both branches. This ensures linearity, prevents
race conditions, and guarantees protocol progress and completion.

---

## Why Disjoint Role Sets Matter

- **Linearity:** Each role must have a single, unambiguous sequence of actions.
- **Safety:** If a role appears in both branches, it would be required to act in two places at
once, violating linearity and potentially causing deadlocks or protocol violations.
- **Progress:** Disjointness ensures that all branches can proceed independently and synchronize
correctly at the end.

---

## Enforcing Disjointness at the Type Level in Rust

Rust's type system can encode type-level sets and traits to enforce disjointness at compile time.
Below is a minimal example for four roles (`A`, `B`, `C`, `D`).

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

### Example Usage

```rust
// Example: Disjoint<Cons<A, Cons<B, Nil>>, Cons<C, Cons<D, Nil>>>
fn _assert_disjoint()
where
    (): Disjoint<Cons<A, Cons<B, Nil>>, Cons<C, Cons<D, Nil>>>,
{
    // This will compile, as the sets are disjoint
}

// Example: This will NOT compile, as B is in both lists
// fn _assert_not_disjoint()
// where
//     (): Disjoint<Cons<A, Cons<B, Nil>>, Cons<B, Cons<C, Nil>>>,
// {}
```

---

## References

- Honda, K., Yoshida, N., & Carbone, M. (2008). [Multiparty Asynchronous Session
Types](https://www.cs.kent.ac.uk/people/staff/srm25/research/multiparty/).
- Gay, S. J., & Vasconcelos, V. T. (2010). [Linear type theory for asynchronous session
types](https://www.dcs.gla.ac.uk/~simon/publications/linear-session-types.pdf).
- Scalas, A., & Yoshida, N. (2016). [Lightweight Session Programming in
Scala](https://www.doc.ic.ac.uk/~cn06/papers/2016-ecoop.pdf).
- Rust type-level programming: [The Typenum crate](https://docs.rs/typenum/latest/typenum/) and
[The typelist crate](https://docs.rs/typelist/latest/typelist/).

---

## Summary

- The `Par` combinator enables parallel protocol composition.
- Disjoint role sets are essential for safety and progress.
- Rust's type system can enforce this property at compile time using type-level programming.
- This approach prevents protocol errors and ensures robust, deadlock-free concurrent session
protocols.

---
