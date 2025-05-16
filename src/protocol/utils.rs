//! # Protocol Utilities
//!
//! This module provides helper traits and type-level operations that support
//! the protocol system. These utilities enable compile-time verification
//! and transformation of protocol types.
//!
//! Key components:
//!
//! - Type-level boolean operations and checks
//! - Disjointness assertions for parallel composition
//! - Uniqueness checks for type-level lists
//! - Other helper traits for type-level programming
//!
//! These utilities ensure protocol safety and correctness at compile time.

use super::base::*;
use crate::types;

/// Trait for disjointness checking of protocol branches.
/// Two branches are disjoint if they have no roles in common.
pub trait Disjoint<L, R> {}

// Base case: empty list is disjoint with anything
impl<R> Disjoint<Nil, R> for () {}

// Use marker types for type-level dispatch
pub struct IsNil;
pub struct IsNotNil;

// Type family for checking if a type is Nil
pub trait CheckNil {
    type Result;
}

impl CheckNil for Nil {
    type Result = IsNil;
}

impl<H, T> CheckNil for Cons<H, T> {
    type Result = IsNotNil;
}

// Helper traits for disjointness checking with dispatch
pub trait DisjointCons<H, L, R, IsRNil> {}

// Implementation for when R is Nil
impl<H, L> DisjointCons<H, L, Nil, IsNil> for () {}

// Implementation for when R is not Nil
impl<H, L, R> DisjointCons<H, L, R, IsNotNil> for ()
where
    R: NotInList<H>,
    (): Disjoint<L, R>,
{
}

// Recursive case using type-level dispatch
impl<H, L, R> Disjoint<Cons<H, L>, R> for ()
where
    R: CheckNil,
    (): DisjointCons<H, L, R, <R as CheckNil>::Result>,
{
}

/// Type-level marker types for lists
pub struct EmptyList;
pub struct NonEmptyList;

/// Trait to check if a type-level list is empty
pub trait IsEmpty {
    type Output;
}

// Empty list is empty
impl IsEmpty for Nil {
    type Output = types::True;
}

// Cons list is not empty
impl<H, T> IsEmpty for Cons<H, T> {
    type Output = types::False;
}

/// Concatenate two type-level lists
pub trait Concat<R> {
    type Output;
}

// Base case: Nil concatenated with anything is that thing
impl<R> Concat<R> for Nil {
    type Output = R;
}

// Helper traits for Cons concatenation with dispatch
pub trait ConcatCons<H, T, R, IsRNil> {
    type Output;
}

// Case: Cons + Nil = Cons (unchanged)
impl<H, T> ConcatCons<H, T, Nil, IsNil> for () {
    type Output = Cons<H, T>;
}

// Case: Cons + non-Nil = recursive concat
impl<H, T, RH, RT> ConcatCons<H, T, Cons<RH, RT>, IsNotNil> for ()
where
    T: Concat<Cons<RH, RT>>,
{
    type Output = Cons<H, <T as Concat<Cons<RH, RT>>>::Output>;
}

// Main concat implementation for Cons using dispatch
impl<H, T, R> Concat<R> for Cons<H, T>
where
    R: CheckNil,
    (): ConcatCons<H, T, R, <R as CheckNil>::Result>,
{
    type Output = <() as ConcatCons<H, T, R, <R as CheckNil>::Result>>::Output;
}
