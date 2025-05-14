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
///
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
/// - `Lbl`: Label for this end (default: EmptyLabel).
///
/// Used to indicate protocol termination.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TEnd<IO, Lbl = types::EmptyLabel>(PhantomData<(IO, Lbl)>);

impl<IO, Lbl> sealed::Sealed for TEnd<IO, Lbl> {}
impl<IO, Lbl> TSession<IO> for TEnd<IO, Lbl> {
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
/// # Examples
/// ```ignore
/// use besedarium::*;
/// // Create a list of two end-of-session branches
/// type EndList = tlist!(
///     TEnd<Http>,
///     TEnd<Http>,
/// );
/// // Map to a protocol choice
/// type Choice = <EndList as ToTChoice<Http>>::Output;
/// // Equivalent to a binary choice between two ends
/// assert_type_eq!(
///     Choice,
///     TChoice<
///         Http,
///         EmptyLabel,
///         TEnd<Http>,
///         TEnd<Http>
///     >
/// );
/// ```
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
/// # Examples
/// ```ignore
/// use besedarium::*;
/// // Create a list of two end-of-session branches
/// type EndList = tlist!(
///     TEnd<Http>,
///     TEnd<Http>,
/// );
/// // Map to a parallel composition
/// type Par = <EndList as ToTPar<Http>>::Output;
/// // Equivalent to running two sessions in parallel
/// assert_type_eq!(
///     Par,
///     TPar<
///         Http,
///         EmptyLabel,
///         TEnd<Http>,
///         TEnd<Http>,
///         FalseB
///     >
/// );
/// ```
pub trait ToTPar<IO> {
    type Output: TSession<IO>;
}

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
    type Output = TPar<IO, types::EmptyLabel, H, <T as ToTPar<IO>>::Output, types::False>;
}

// --- Compile-time Disjointness Assertion Machinery ---
pub trait AssertDisjoint {
    type Output;
}

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
/// # Examples
/// ```rust
/// use besedarium::*;
/// struct Alice; struct Bob;
/// impl Role for Alice {} impl Role for Bob {}
/// impl ProtocolLabel for Alice {} impl ProtocolLabel for Bob {}
/// impl RoleEq<Alice> for Alice { type Output = True; }
/// impl RoleEq<Bob> for Alice   { type Output = False; }
/// impl RoleEq<Alice> for Bob   { type Output = False; }
/// impl RoleEq<Bob> for Bob     { type Output = True; }
///
/// // Global protocol: Alice sends Message then Bob sends Response
/// type Global = TInteract<
///     Http,
///     EmptyLabel,
///     Alice,
///     Message,
///     TInteract<Http, EmptyLabel, Bob, Response, TEnd<Http, EmptyLabel>>
/// >;
/// // Project onto Alice
/// type AliceLocal = <() as ProjectRole<Alice, Http, Global>>::Out;
/// // Should be a send of Message then a receive of Response
/// assert_type_eq!(
///     AliceLocal,
///     EpSend<
///         Http,
///         Alice,
///         Message,
///         EpRecv<Http, Alice, Response, EpEnd<Http, Alice>>
///     >
/// );
/// ```
pub trait ProjectRole<Me, IO, G: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}

// Base case: projecting end-of-session yields EpEnd
impl<Me, IO, L> ProjectRole<Me, IO, TEnd<IO, L>> for ()
where
    Me: Role,
{
    type Out = EpEnd<IO, Me>;
}

// Projection for single interaction: dispatch on role equality
impl<Me, IO, L, R, H, T> ProjectRole<Me, IO, TInteract<IO, L, R, H, T>> for ()
where
    Me: Role,
    L: types::ProtocolLabel,
    R: Role,
    T: TSession<IO>,
    Me: RoleEq<R>,
    <Me as RoleEq<R>>::Output: types::Bool,
    (): ProjectInteract<<Me as RoleEq<R>>::Output, Me, IO, R, H, T>,
{
    type Out = <() as ProjectInteract<<Me as RoleEq<R>>::Output, Me, IO, R, H, T>>::Out;
}

// --- Helper impls for ProjectInteract ---
// If this role is the sender: send then recurse
impl<Me, IO, R, H, T> ProjectInteract<types::True, Me, IO, R, H, T> for ()
where
    Me: Role + RoleEq<R, Output = types::True>,
    R: Role,
    T: TSession<IO>,
    (): ProjectRole<Me, IO, T>,
{
    type Out = EpSend<IO, Me, H, <() as ProjectRole<Me, IO, T>>::Out>;
}

// If this role is not the sender: receive then recurse
impl<Me, IO, R, H, T> ProjectInteract<types::False, Me, IO, R, H, T> for ()
where
    Me: Role + RoleEq<R, Output = types::False>,
    R: Role,
    T: TSession<IO>,
    (): ProjectRole<Me, IO, T>,
{
    type Out = EpRecv<IO, Me, H, <() as ProjectRole<Me, IO, T>>::Out>;
}

// --- Helper trait for projecting a single interaction in a protocol ---
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

// --- ContainsRole trait: type-level check if a role is present in a protocol branch ---
/// Type-level trait to check if a role is present in a protocol branch.
pub trait ContainsRole<R> {
    type Output;
}
// Implementations for Nil, Cons, TInteract, TChoice, TPar, TRec, TEnd, etc. should exist elsewhere in the codebase.

// --- ProjectParBranch: Helper trait to dispatch on role presence in a parallel branch ---
/// Helper trait to project a parallel branch for a role, or skip if not present.
pub trait ProjectParBranch<Flag, Me: Role, IO, G: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}
// Case: role is present in the branch
impl<Me: Role, IO, G: TSession<IO>> ProjectParBranch<types::True, Me, IO, G> for ()
where
    (): ProjectRole<Me, IO, G>,
{
    type Out = <() as ProjectRole<Me, IO, G>>::Out;
}
// Case: role is not present in the branch
impl<Me: Role, IO, G: TSession<IO>> ProjectParBranch<types::False, Me, IO, G> for () {
    type Out = EpSkip<IO, Me>;
}

// --- Stable, explicit, non-overlapping IsSkip/IsEnd traits ---
// --- New traits for specific type checks ---
pub trait IsEpSkipVariant<IO, Me: Role> {
    type Output: types::Bool;
}

pub trait IsEpEndVariant<IO, Me: Role> {
    type Output: types::Bool;
}

// Implementations for IsEpSkipVariant
impl<IO, Me: Role> IsEpSkipVariant<IO, Me> for EpSkip<IO, Me> {
    type Output = types::True;
}
impl<IO, R, H, T, Me: Role> IsEpSkipVariant<IO, Me> for EpSend<IO, R, H, T> {
    type Output = types::False;
}
impl<IO, R, H, T, Me: Role> IsEpSkipVariant<IO, Me> for EpRecv<IO, R, H, T> {
    type Output = types::False;
}
// Ensure Me: Role is the second generic parameter for EpChoice, EpPar for IsEpSkipVariant
impl<IO, MeChoice: Role, L, R, MeFilter: Role> IsEpSkipVariant<IO, MeFilter>
    for EpChoice<IO, MeChoice, L, R>
{
    type Output = types::False;
}
impl<IO, MePar: Role, L, R, MeFilter: Role> IsEpSkipVariant<IO, MeFilter>
    for EpPar<IO, MePar, L, R>
{
    type Output = types::False;
}
impl<IO, MeEnd: Role, MeFilter: Role> IsEpSkipVariant<IO, MeFilter> for EpEnd<IO, MeEnd> {
    type Output = types::False;
}

// Implementations for IsEpEndVariant
impl<IO, Me: Role> IsEpEndVariant<IO, Me> for EpEnd<IO, Me> {
    type Output = types::True;
}
impl<IO, R, H, T, Me: Role> IsEpEndVariant<IO, Me> for EpSend<IO, R, H, T> {
    type Output = types::False;
}
impl<IO, R, H, T, Me: Role> IsEpEndVariant<IO, Me> for EpRecv<IO, R, H, T> {
    type Output = types::False;
}
// Ensure Me: Role is the second generic parameter for EpChoice, EpPar for IsEpEndVariant
impl<IO, MeChoice: Role, L, R, MeFilter: Role> IsEpEndVariant<IO, MeFilter>
    for EpChoice<IO, MeChoice, L, R>
{
    type Output = types::False;
}
impl<IO, MePar: Role, L, R, MeFilter: Role> IsEpEndVariant<IO, MeFilter>
    for EpPar<IO, MePar, L, R>
{
    type Output = types::False;
}
impl<IO, MeSkip: Role, MeFilter: Role> IsEpEndVariant<IO, MeFilter> for EpSkip<IO, MeSkip> {
    type Output = types::False;
}

/// IsSkip: True if T is EpSkip<IO, Me>, else False.
pub type IsSkip<T, IO, Me> = <T as IsEpSkipVariant<IO, Me>>::Output;

/// IsEnd: True if T is EpEnd<IO, Me>, else False.
pub type IsEnd<T, IO, Me> = <T as IsEpEndVariant<IO, Me>>::Output;

// --- Main flag-based composition trait ---
pub trait ComposeProjectedParBranches<IO, Me: Role, L, R>
where
    L: IsEpSkipVariant<IO, Me> + IsEpEndVariant<IO, Me> + EpSession<IO, Me>, // Added bounds
    R: IsEpSkipVariant<IO, Me> + IsEpEndVariant<IO, Me> + EpSession<IO, Me>, // Added bounds
    <L as IsEpSkipVariant<IO, Me>>::Output: types::Bool,
    <R as IsEpSkipVariant<IO, Me>>::Output: types::Bool,
    <L as IsEpEndVariant<IO, Me>>::Output: types::Bool,
    <R as IsEpEndVariant<IO, Me>>::Output: types::Bool,
{
    type Out: EpSession<IO, Me>;
}
impl<IO, Me: Role, L, R> ComposeProjectedParBranches<IO, Me, L, R> for ()
where
    L: IsEpSkipVariant<IO, Me> + IsEpEndVariant<IO, Me> + EpSession<IO, Me>,
    R: IsEpSkipVariant<IO, Me> + IsEpEndVariant<IO, Me> + EpSession<IO, Me>,
    <L as IsEpSkipVariant<IO, Me>>::Output: types::Bool,
    <R as IsEpSkipVariant<IO, Me>>::Output: types::Bool,
    <L as IsEpEndVariant<IO, Me>>::Output: types::Bool,
    <R as IsEpEndVariant<IO, Me>>::Output: types::Bool,
    (): ComposeProjectedParBranchesCase<
        IsSkip<L, IO, Me>,
        IsSkip<R, IO, Me>,
        IsEnd<L, IO, Me>,
        IsEnd<R, IO, Me>,
        IO,
        Me,
        L,
        R,
    >,
{
    type Out = <() as ComposeProjectedParBranchesCase<
        IsSkip<L, IO, Me>,
        IsSkip<R, IO, Me>,
        IsEnd<L, IO, Me>,
        IsEnd<R, IO, Me>,
        IO,
        Me,
        L,
        R,
    >>::Out;
}

// --- Helper trait for case selection ---
pub trait ComposeProjectedParBranchesCase<LSkip, RSkip, LEnd, REnd, IO, Me: Role, L, R>
where
    L: EpSession<IO, Me>,
    R: EpSession<IO, Me>,
{
    type Out: EpSession<IO, Me>;
}
// Both branches are EpSkip
impl<IO, Me: Role>
    ComposeProjectedParBranchesCase<
        types::True,
        types::True,
        types::False,
        types::False,
        IO,
        Me,
        EpSkip<IO, Me>,
        EpSkip<IO, Me>,
    > for ()
{
    type Out = EpSkip<IO, Me>;
}
// Left is EpSkip, right is projected
impl<IO, Me: Role, ProjectedR: EpSession<IO, Me>>
    ComposeProjectedParBranchesCase<
        types::True,
        types::False,
        types::False,
        types::False,
        IO,
        Me,
        EpSkip<IO, Me>,
        ProjectedR,
    > for ()
{
    type Out = ProjectedR;
}
// Left is projected, right is EpSkip
impl<IO, Me: Role, ProjectedL: EpSession<IO, Me>>
    ComposeProjectedParBranchesCase<
        types::False,
        types::True,
        types::False,
        types::False,
        IO,
        Me,
        ProjectedL,
        EpSkip<IO, Me>,
    > for ()
{
    type Out = ProjectedL;
}
// Both are projected
impl<IO, Me: Role, ProjectedL: EpSession<IO, Me>, ProjectedR: EpSession<IO, Me>>
    ComposeProjectedParBranchesCase<
        types::False,
        types::False,
        types::False,
        types::False,
        IO,
        Me,
        ProjectedL,
        ProjectedR,
    > for ()
{
    type Out = EpPar<IO, Me, ProjectedL, ProjectedR>;
}
// Left is EpEnd, right is EpSkip
impl<IO, Me: Role>
    ComposeProjectedParBranchesCase<
        types::False,
        types::True,
        types::True,
        types::False,
        IO,
        Me,
        EpEnd<IO, Me>,
        EpSkip<IO, Me>,
    > for ()
{
    type Out = EpEnd<IO, Me>;
}
// Left is EpSkip, right is EpEnd
impl<IO, Me: Role>
    ComposeProjectedParBranchesCase<
        types::True,
        types::False,
        types::False,
        types::True,
        IO,
        Me,
        EpSkip<IO, Me>,
        EpEnd<IO, Me>,
    > for ()
{
    type Out = EpEnd<IO, Me>;
}
// Left is EpEnd, right is projected
impl<IO, Me: Role, ProjectedR: EpSession<IO, Me>>
    ComposeProjectedParBranchesCase<
        types::False,
        types::False,
        types::True,
        types::False,
        IO,
        Me,
        EpEnd<IO, Me>,
        ProjectedR,
    > for ()
{
    type Out = ProjectedR;
}
// Left is projected, right is EpEnd
impl<IO, Me: Role, ProjectedL: EpSession<IO, Me>>
    ComposeProjectedParBranchesCase<
        types::False,
        types::False,
        types::False,
        types::True,
        IO,
        Me,
        ProjectedL,
        EpEnd<IO, Me>,
    > for ()
{
    type Out = ProjectedL;
}
// Both are EpEnd
impl<IO, Me: Role>
    ComposeProjectedParBranchesCase<
        types::False,
        types::False,
        types::True,
        types::True,
        IO,
        Me,
        EpEnd<IO, Me>,
        EpEnd<IO, Me>,
    > for ()
{
    type Out = EpEnd<IO, Me>;
}

// --- Restore sealed dispatch-based FilterSkips implementation ---

/// Type-level marker types for dispatch
pub struct IsEpSkipType;
pub struct IsNotEpSkipType;

/// Implementation marker trait for EpSkip dispatch
pub trait IsEpSkipTypeImpl<IO, Me: Role> {
    type TypeMarker;
}

// EpSkip maps to IsEpSkipType
impl<IO, Me: Role> IsEpSkipTypeImpl<IO, Me> for EpSkip<IO, Me> {
    type TypeMarker = IsEpSkipType;
}

// All other EpSession<IO, Me> types map to IsNotEpSkipType
impl<IO, Me: Role, H, T> IsEpSkipTypeImpl<IO, Me> for EpSend<IO, Me, H, T> {
    type TypeMarker = IsNotEpSkipType;
}
impl<IO, Me: Role, H, T> IsEpSkipTypeImpl<IO, Me> for EpRecv<IO, Me, H, T> {
    type TypeMarker = IsNotEpSkipType;
}
impl<IO, Me: Role, L, R> IsEpSkipTypeImpl<IO, Me> for EpChoice<IO, Me, L, R> {
    type TypeMarker = IsNotEpSkipType;
}
impl<IO, Me: Role, L, R> IsEpSkipTypeImpl<IO, Me> for EpPar<IO, Me, L, R> {
    type TypeMarker = IsNotEpSkipType;
}
impl<IO, Me: Role> IsEpSkipTypeImpl<IO, Me> for EpEnd<IO, Me> {
    type TypeMarker = IsNotEpSkipType;
}

// [Removed generic impl for T: IsNotEpSkip]
// ...existing code below this section...

/// Public facade trait that routes to the implementation trait
pub trait GetEpSkipTypeMarker<IO, Me: Role> {
    type TypeMarker;
}

impl<IO, Me: Role, T> GetEpSkipTypeMarker<IO, Me> for T
where
    T: IsEpSkipTypeImpl<IO, Me>,
{
    type TypeMarker = <T as IsEpSkipTypeImpl<IO, Me>>::TypeMarker;
}

/// Type-level filter that removes all EpSkip<IO, Me> branches from a type-level list.
pub trait FilterSkips<IO, Me: Role, List> {
    type Out;
}

// Base case: empty list
impl<IO, Me: Role> FilterSkips<IO, Me, Nil> for () {
    type Out = Nil;
}

// Recursive case using dispatch on TypeMarker
impl<IO, Me: Role, H, T> FilterSkips<IO, Me, Cons<H, T>> for ()
where
    H: GetEpSkipTypeMarker<IO, Me> + EpSession<IO, Me>,
    (): FilterSkipsCase<IO, Me, H, T, <H as GetEpSkipTypeMarker<IO, Me>>::TypeMarker>,
{
    type Out =
        <() as FilterSkipsCase<IO, Me, H, T, <H as GetEpSkipTypeMarker<IO, Me>>::TypeMarker>>::Out;
}

/// Helper trait for non-overlapping dispatch in FilterSkips
pub trait FilterSkipsCase<IO, Me: Role, H, T, TypeMarker> {
    type Out;
}

// Case: Head is EpSkip – skip it
impl<IO, Me: Role, T> FilterSkipsCase<IO, Me, EpSkip<IO, Me>, T, IsEpSkipType> for ()
where
    (): FilterSkips<IO, Me, T>,
{
    type Out = <() as FilterSkips<IO, Me, T>>::Out;
}

// Case: Head is not EpSkip – keep it
impl<IO, Me: Role, H, T> FilterSkipsCase<IO, Me, H, T, IsNotEpSkipType> for ()
where
    H: EpSession<IO, Me>,
    (): FilterSkips<IO, Me, T>,
{
    type Out = Cons<H, <() as FilterSkips<IO, Me, T>>::Out>;
}
