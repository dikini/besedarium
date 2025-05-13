# Learnings & Patterns: Type-Level Rust Protocols

This document captures core patterns, idioms, and constraints for type-level
protocol programming in Rust, distilled from recent work on session-type
dispatch and filtering.

## 1. Trait-Based Dispatch with Marker Types

Rust’s stable trait system forbids overlapping impls, negative bounds, and
using associated types as generic parameters. We overcome these limits with:

1. Marker types for each case:

   ```rust
   pub struct IsEpSkipType;
   pub struct IsNotEpSkipType;
   ```

2. A helper trait mapping each endpoint type to a marker:

   ```rust
   pub trait IsEpSkipTypeImpl<IO, Me: Role> { type TypeMarker; }

   impl<IO, Me: Role> IsEpSkipTypeImpl<IO, Me> for EpSkip<IO, Me> {
       type TypeMarker = IsEpSkipType;
   }
   impl<IO, Me: Role, H, T> IsEpSkipTypeImpl<IO, Me> for EpSend<IO, Me, H, T> {
       type TypeMarker = IsNotEpSkipType;
   }
   // ... likewise for EpRecv, EpChoice, EpPar, EpEnd
   ```

3. A single‐impl facade trait that delegates to it:

   ```rust
   pub trait GetEpSkipTypeMarker<IO, Me: Role> { type TypeMarker; }
   impl<IO, Me: Role, T> GetEpSkipTypeMarker<IO, Me> for T
   where T: IsEpSkipTypeImpl<IO, Me>
   {
       type TypeMarker = <T as IsEpSkipTypeImpl<IO,Me>>::TypeMarker;
   }
   ```

This pattern yields exactly two disjoint impls and keeps dispatch stable.

## 2. Compile‐Time Type Equality (

TypeEq
trait)

To enable `assert_type_eq!` assertions, use a “blanket + specific” approach:

```rust
pub struct True;
pub struct False;
pub trait Bool {}
impl Bool for True {} impl Bool for False {}

/// Legacy aliases for tests
pub type TrueB = True;
pub type FalseB = False;

/// Only implemented when A == B
pub trait TypeEq<A> {}
impl<T> TypeEq<T> for T {}
```

Rust’s coherence rules pick the exact match impl and disallow others.

## 3. Type‐Level Filter vs. Map

- **Map**: transforms every element of a type‐list, preserving length.
- **Filter**: keeps only elements satisfying a predicate, shortening the list.

Example: drop all `EpSkip<IO,Me>` from `Cons<H,T>`:

```rust
pub trait FilterSkips<IO, Me: Role, List> { type Out; }
impl<IO, Me: Role> FilterSkips<IO, Me, Nil> for () { type Out = Nil; }

// Delegate to helper based on TypeMarker
impl<IO, Me: Role, H, Tail> FilterSkips<IO,Me,Cons<H,Tail>> for ()
where
    H: GetEpSkipTypeMarker<IO,Me>,
    (): FilterSkipsCase<IO,Me,H,Tail,
        <H as GetEpSkipTypeMarker<IO,Me>>::TypeMarker>,
{
    type Out = <() as FilterSkipsCase<IO,Me,H,Tail,
        <H as GetEpSkipTypeMarker<IO,Me>>::TypeMarker>>::Out;
}

// If head is EpSkip → skip it
impl<IO, Me: Role, Tail>
    FilterSkipsCase<IO,Me, EpSkip<IO,Me>, Tail, IsEpSkipType> for ()
where (): FilterSkips<IO,Me,Tail>
{ type Out = <() as FilterSkips<IO,Me,Tail>>::Out; }

// Otherwise → keep it
impl<IO, Me: Role, H, Tail>
    FilterSkipsCase<IO,Me, H, Tail, IsNotEpSkipType> for ()
where
    H: EpSession<IO,Me>,
    (): FilterSkips<IO,Me,Tail>,
{
    type Out = Cons<H, <() as FilterSkips<IO,Me,Tail>>::Out>;
}
```

## 4. Rust Trait System Constraints & Workarounds

- **No specialization or negative bounds** on stable Rust.
- **No associated types as generic parameters**.
- **No overlapping impls** allowed.

Workarounds:

- Use **sealed helper traits** and **marker types** for stable, mutual exclusion.
- Expose a **facade trait** with a single impl to avoid overlap.
- Enumerate explicit impls rather than blanket covers that break coherence.

## 5. Documentation & Planning

- **Always plan** edits with:
  - A list of affected files, traits, or functions
  - Change order and dependencies
  - Estimated edit count
- **Document intent**: comment non‐obvious type‐level logic.
- **Distinguish** algorithm patterns (map vs filter) in prose first.
- Use examples and ensure doctests compile.

---
*Consult this summary before any future protocol‐projection or
type‐level work to maintain stability, clarity, and correctness.*
