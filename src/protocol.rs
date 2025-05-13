use crate::sealed;
use crate::types;
use core::marker::PhantomData;

/// # Type-Level Map/Fold Pattern
///
/// Many traits in this crate use a type-level map/fold pattern to recursively
/// process type-level lists or protocol structures. This is a common idiom for
/// building, transforming, or checking protocol types at compile time.
///
/// ## How it works
/// - The trait is implemented for the base case (usually `Nil`), which provides
///   the default or terminal value.
/// - The trait is then implemented recursively for `Cons<H, T>` (or similar),
///   where the head is processed and the result is combined with the recursive
///   result for the tail.
///
/// ## Examples
/// - [`ToTPar`]: Maps a type-level list to a nested `TPar` by folding over the list.
///   - Base case: `Nil` maps to `TEnd`.
///   - Recursive case: `Cons<H, T>` maps to `TPar` of `H` and the result of folding `T`.
/// - [`ToTChoice`]: Maps a type-level list to a nested `TChoice`.
/// - Disjointness traits: Recursively check that all roles in two lists are disjoint.
///
/// ## Why use this pattern?
/// - Enables ergonomic n-ary combinators (via macros).
/// - Allows compile-time checks and transformations over protocol structures.
/// - Ensures correctness and compositionality at the type level.
///
/// ## See also
/// - [`ToTPar`], [`ToTChoice`], [`Disjoint`], [`UniqueList`], [`ConcatRoles`]
/// - Macros: `tpar!`, `tchoice!`

/// # Resolving Overlapping Trait Implementations with Helper Traits
///
/// Rust's trait system does not allow overlapping or conflicting trait impls.
/// This is a challenge for type-level programming, especially when projecting
/// global protocols to local types, where we want to dispatch on type-level
/// booleans (e.g., role equality) or other conditions.
///
/// ## Pattern: Helper Traits for Disambiguation
/// - Instead of writing multiple impls for the same trait (which would overlap),
///   we write a single impl that computes a type-level flag (e.g., `TrueB`/`FalseB`)
///   and then delegates to a *helper trait* that is specialized for each case.
/// - The main trait (e.g., `ProjectRole`) computes the flag and calls the helper
///   (e.g., `ProjectInteract<Flag, ...>`), which has non-overlapping impls for each case.
///
/// ## Example: Projecting an Interaction
/// - `ProjectRole<Me, IO, TInteract<IO, L, R, H, T>>` computes whether `Me` is the
///   same as `R` (the role performing the action) using `RoleEq`.
/// - It then dispatches to `ProjectInteract<Flag, Me, IO, R, H, T>`, where `Flag`
///   is `TrueB` if `Me == R`, `FalseB` otherwise.
/// - `ProjectInteract` has separate impls for `TrueB` and `FalseB`, so there is no overlap.
///
/// ## Why use this pattern?
/// - Avoids Rust's coherence/orphan rules and overlapping impl errors.
/// - Makes the logic explicit and easy to extend for new cases.
/// - Keeps the main trait (e.g., `ProjectRole`) simple and compositional.
///
/// ## See also
/// - [`ProjectRole`], [`ProjectInteract`], [`ProjectChoice`], [`ProjectPar`]

/// Type-level empty list for n-ary combinators and role/label sets.
/// Used as the base case for type-level lists.
pub struct Nil;

/// Type-level cons cell for n-ary combinators and role/label sets.
/// `H` is the head element, `T` is the tail (another type-level list).
/// Used to build type-level lists for protocol branches, roles, or labels.
pub struct Cons<H, T>(PhantomData<(H, T)>);

/// Core trait for all global session type combinators.
///
/// - `IO`: Protocol marker type (e.g., Http, Mqtt).
/// - Implemented by all protocol combinators (TEnd, TInteract, TChoice, TPar, TRec).
/// - Used for type-level composition and compile-time protocol checks.
pub trait TSession<IO>: sealed::Sealed {
    /// Compose this session with another session of the same IO type.
    type Compose<Rhs: TSession<IO>>: TSession<IO>;
    /// Is this session type empty (TEnd)?
    const IS_EMPTY: bool;
}

/// End of a protocol session.
///
/// - `IO`: Protocol marker type.
/// - `L`: Label for this end (default: EmptyLabel).
/// Used to indicate protocol termination.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TEnd<IO, L = types::EmptyLabel>(PhantomData<(IO, L)>);

impl<IO, L> sealed::Sealed for TEnd<IO, L> {}
impl<IO, L> TSession<IO> for TEnd<IO, L> {
    type Compose<Rhs: TSession<IO>> = Rhs;
    const IS_EMPTY: bool = true;
}

/// Represents a single interaction in a protocol session.
///
/// - `IO`: Protocol marker type (e.g., Http, Mqtt).
/// - `L`: Label for this interaction (for projection and debugging).
/// - `R`: Role performing the action (sender or receiver).
/// - `H`: Message type being sent or received.
/// - `T`: Continuation protocol after this interaction.
///
/// Used to model a single send/receive step in a protocol.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TInteract<IO, L: types::ProtocolLabel, R, H, T: TSession<IO>>(
    PhantomData<(IO, L, R, H, T)>,
);

impl<IO, L: types::ProtocolLabel, R, H, T: TSession<IO>> sealed::Sealed
    for TInteract<IO, L, R, H, T>
{
}
impl<IO, L: types::ProtocolLabel, R, H, T: TSession<IO>> TSession<IO>
    for TInteract<IO, L, R, H, T>
{
    type Compose<Rhs: TSession<IO>> = TInteract<IO, L, R, H, T::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

/// Recursive session type for repeating protocol fragments.
///
/// - `IO`: Protocol marker type.
/// - `L`: Label for this recursion (for projection and debugging).
/// - `S`: The protocol fragment to repeat (may refer to itself).
///
/// Used to model loops or streaming protocols.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TRec<IO, L: types::ProtocolLabel, S: TSession<IO>>(PhantomData<(IO, L, S)>);

impl<IO, L: types::ProtocolLabel, S: TSession<IO>> sealed::Sealed for TRec<IO, L, S> {}
impl<IO, L: types::ProtocolLabel, S: TSession<IO>> TSession<IO> for TRec<IO, L, S> {
    type Compose<Rhs: TSession<IO>> = TRec<IO, L, S::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

/// Binary protocol choice between two branches.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this choice (for projection and debugging).
/// - `L`, `R`: The two protocol branches.
///
/// Used to model branching points in a protocol (e.g., offer/choose).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TChoice<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>>(
    PhantomData<(IO, Lbl, L, R)>,
);

impl<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>> sealed::Sealed
    for TChoice<IO, Lbl, L, R>
{
}
impl<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>> TSession<IO>
    for TChoice<IO, Lbl, L, R>
{
    type Compose<Rhs: TSession<IO>> = TChoice<IO, Lbl, L::Compose<Rhs>, R::Compose<Rhs>>;
    const IS_EMPTY: bool = false;
}

/// Trait for mapping a type-level list to a nested `TChoice`.
///
/// Used by the `tchoice!` macro for n-ary protocol branching.
pub trait ToTChoice<IO> {
    type Output: TSession<IO>;
}

/// Branded parallel composition of two protocol branches.
///
/// - `IO`: Protocol marker type.
/// - `Lbl`: Label for this parallel composition.
/// - `L`, `R`: The two protocol branches to run in parallel.
/// - `IsDisjoint`: Type-level boolean indicating if branches are disjoint.
///
/// Used to model concurrency in protocols. Disjointness is enforced at compile time.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TPar<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>, IsDisjoint>(
    PhantomData<(IO, Lbl, L, R, IsDisjoint)>,
);

impl<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>, IsDisjoint> sealed::Sealed
    for TPar<IO, Lbl, L, R, IsDisjoint>
{
}
impl<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>, IsDisjoint> TSession<IO>
    for TPar<IO, Lbl, L, R, IsDisjoint>
{
    type Compose<Rhs: TSession<IO>> = TPar<IO, Lbl, L::Compose<Rhs>, R::Compose<Rhs>, IsDisjoint>;
    const IS_EMPTY: bool = false;
}

/// Trait for mapping a type-level list to a nested `TPar`.
///
/// Used by the `tpar!` macro for n-ary protocol parallel composition.
pub trait ToTPar<IO> {
    type Output: TSession<IO>;
}

pub struct TrueB;
pub struct FalseB;

// --- ToTChoice trait, base case for Nil ---
impl<IO> ToTChoice<IO> for Nil {
    type Output = TEnd<IO>;
}

// --- ToTChoice trait, recursive case ---
impl<IO, H: TSession<IO>, T: ToTChoice<IO>> ToTChoice<IO> for Cons<H, T> {
    type Output = TChoice<IO, types::EmptyLabel, H, <T as ToTChoice<IO>>::Output>;
}

// --- ToTPar trait, base case for Nil ---
impl<IO> ToTPar<IO> for Nil {
    type Output = TEnd<IO>;
}

// --- ToTPar trait, recursive case ---
impl<IO, H: TSession<IO>, T: ToTPar<IO>> ToTPar<IO> for Cons<H, T> {
    type Output = TPar<IO, types::EmptyLabel, H, <T as ToTPar<IO>>::Output, FalseB>;
}

// --- Compile-time Disjointness Assertion Machinery ---
pub trait AssertDisjoint {
    type Output;
}

// --- Type-level equality trait for compile-time assertions. ---
pub trait TypeEq<B> {}
impl<A> TypeEq<A> for A {}

// --- Uniqueness Traits for Type-level Lists ---
pub trait NotInList<X> {}
impl<X> NotInList<X> for Nil {}
impl<X, H, T> NotInList<X> for Cons<H, T>
where
    X: NotSame<H>,
    T: NotInList<X>,
{
}

pub trait NotSame<T> {}
impl<A, B> NotSame<B> for A where A: NotTypeEq<B> {}

pub trait NotTypeEq<B> {}
impl<A, B> NotTypeEq<B> for A {}
// Overlap: no impl for A == A

/// Trait to check that all elements in a type-level list are unique.
///
/// Used for compile-time uniqueness assertions (e.g., for protocol labels).
pub trait UniqueList {}
impl UniqueList for Nil {}
impl<H, T> UniqueList for Cons<H, T> where T: NotInList<H> + UniqueList {}

// --- Concrete Roles for Protocols and Tests ---
pub struct TClient;
pub struct TServer;
pub struct TBroker;
pub struct TWorker;
pub struct Void;

/// Marker trait for protocol participants (roles).
///
/// Implement this trait for each participant in your protocol.
pub trait Role {}
impl Role for TClient {}
impl Role for TServer {}
impl Role for TBroker {}
impl Role for TWorker {}
impl Role for Void {}

/// Type-level equality for roles.
///
/// Used to determine if two roles are the same at compile time (for projection).
pub trait RoleEq<R> {
    type Output;
}

// --- Endpoint (Ep*) types and projection traits ---

/// Endpoint type for sending a message in a local protocol.
///
/// - `IO`: Protocol marker type.
/// - `R`: Role performing the send.
/// - `H`: Message type being sent.
/// - `T`: Continuation after sending.
pub struct EpSend<IO, R, H, T>(PhantomData<(IO, R, H, T)>);
impl<IO, R, H, T> EpSession<IO, R> for EpSend<IO, R, H, T> {}
impl<IO, R, H, T> sealed::Sealed for EpSend<IO, R, H, T> {}

/// Endpoint type for receiving a message in a local protocol.
///
/// - `IO`: Protocol marker type.
/// - `R`: Role performing the receive.
/// - `H`: Message type being received.
/// - `T`: Continuation after receiving.
pub struct EpRecv<IO, R, H, T>(PhantomData<(IO, R, H, T)>);
impl<IO, R, H, T> EpSession<IO, R> for EpRecv<IO, R, H, T> {}
impl<IO, R, H, T> sealed::Sealed for EpRecv<IO, R, H, T> {}

/// Endpoint type for protocol termination in a local protocol.
///
/// - `IO`: Protocol marker type.
/// - `R`: Role for which the protocol ends.
pub struct EpEnd<IO, R>(PhantomData<(IO, R)>);
impl<IO, R> EpSession<IO, R> for EpEnd<IO, R> {}
impl<IO, R> sealed::Sealed for EpEnd<IO, R> {}

/// Endpoint type for local protocol branching (choice/offer).
///
/// - `IO`: Protocol marker type.
/// - `Me`: The role being projected.
/// - `L`, `R`: The two local protocol branches.
pub struct EpChoice<IO, Me, L, R>(PhantomData<(IO, Me, L, R)>);
impl<IO, Me, L, R> EpSession<IO, Me> for EpChoice<IO, Me, L, R> {}
impl<IO, Me, L, R> sealed::Sealed for EpChoice<IO, Me, L, R> {}

/// Endpoint type for local protocol parallel composition.
///
/// - `IO`: Protocol marker type.
/// - `Me`: The role being projected.
/// - `L`, `R`: The two local protocol branches.
pub struct EpPar<IO, Me, L, R>(PhantomData<(IO, Me, L, R)>);
impl<IO, Me, L, R> EpSession<IO, Me> for EpPar<IO, Me, L, R> {}
impl<IO, Me, L, R> sealed::Sealed for EpPar<IO, Me, L, R> {}

/// No-op endpoint type for roles uninvolved in a protocol branch.
///
/// Used to improve type-level precision for projections.
pub struct EpSkip<IO, R>(PhantomData<(IO, R)>);
impl<IO, R> EpSession<IO, R> for EpSkip<IO, R> {}
impl<IO, R> sealed::Sealed for EpSkip<IO, R> {}

/// Trait for all local (endpoint) session types.
///
/// - `IO`: Protocol marker type.
/// - `R`: Role being projected.
pub trait EpSession<IO, R>: sealed::Sealed {}

/// Projects a global protocol onto a single role, producing the local protocol for that role.
///
/// - `Me`: The role being projected.
/// - `IO`: Protocol marker type.
/// - `G`: The global protocol type.
///
/// Used for compile-time projection from global to local session types.
pub trait ProjectRole<Me, IO, G: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}

/// Helper trait for projecting a single interaction in a protocol.
///
/// - `Flag`: Type-level boolean for role equality.
/// - `Me`: The role being projected.
/// - `IO`: Protocol marker type.
/// - `R`: Role performing the action.
/// - `H`: Message type.
/// - `T`: Continuation protocol.
pub trait ProjectInteract<Flag, Me: Role, IO, R: Role, H, T: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}

/// Helper trait for projecting a protocol choice.
///
/// - `Me`: The role being projected.
/// - `IO`: Protocol marker type.
/// - `L`, `R`: The two protocol branches.
pub trait ProjectChoice<Me, IO, L: TSession<IO>, R: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}

/// Helper trait for projecting a protocol parallel composition.
///
/// - `Me`: The role being projected.
/// - `IO`: Protocol marker type.
/// - `L`, `R`: The two protocol branches.
pub trait ProjectPar<Me, IO, L: TSession<IO>, R: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}
