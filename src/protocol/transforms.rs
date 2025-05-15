//! # Protocol Transformations
//!
//! This module provides transformations between different protocol representations,
//! primarily focusing on projection from global to local protocols.
//!
//! Key components:
//!
//! - `ProjectRole`: Main trait for projecting global protocols onto specific roles
//! - `ProjectInteract`: Helper trait for projecting individual interactions
//! - `ProjectChoice`: Helper trait for projecting protocol branches
//! - `ProjectPar`: Helper trait for projecting parallel compositions
//! - `ContainsRole`: Helper trait to check if a role participates in a protocol
//!
//! These transformations ensure that global protocols can be correctly
//! interpreted from the perspective of each participating role.

use super::base::*;
use super::global::*;
use super::local::*;
use crate::types;

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
impl<Me, IO, Lbl> ProjectRole<Me, IO, TEnd<IO, Lbl>> for ()
where
    Me: Role,
{
    type Out = EpEnd<IO, Me>;
}

// Projection for single interaction: dispatch on role equality
impl<Me, IO, Lbl, R, H, T> ProjectRole<Me, IO, TInteract<IO, Lbl, R, H, T>> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    R: Role,
    T: TSession<IO>,
    Me: RoleEq<R>,
    <Me as RoleEq<R>>::Output: types::Bool,
    (): ProjectInteract<<Me as RoleEq<R>>::Output, Me, IO, R, H, T>,
{
    type Out = <() as ProjectInteract<<Me as RoleEq<R>>::Output, Me, IO, R, H, T>>::Out;
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

/// Helper trait for projecting a protocol choice.
///
/// - `Me`: The role being projected.
/// - `IO`: Protocol marker type.
/// - `L`, `R`: The two protocol branches.
pub trait ProjectChoice<Me, IO, L: TSession<IO>, R: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}

// Projection implementation for TChoice - delegate to ProjectChoice helper
impl<Me, IO, Lbl, L, R> ProjectRole<Me, IO, TChoice<IO, Lbl, L, R>> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
    L: ContainsRole<Me>,
    <L as ContainsRole<Me>>::Output: types::Bool,
    R: ContainsRole<Me>,
    <R as ContainsRole<Me>>::Output: types::Bool,
    (): ProjectChoiceCase<
        Me,
        IO,
        L,
        R,
        <L as ContainsRole<Me>>::Output,
        <R as ContainsRole<Me>>::Output,
    >,
{
    type Out = <() as ProjectChoiceCase<
        Me,
        IO,
        L,
        R,
        <L as ContainsRole<Me>>::Output,
        <R as ContainsRole<Me>>::Output,
    >>::Out;
}

// Helper trait for handling different cases of ProjectChoice based on role presence
pub trait ProjectChoiceCase<Me, IO, L: TSession<IO>, R: TSession<IO>, LContainsMe, RContainsMe> {
    type Out: EpSession<IO, Me>;
}

// Case 1: Both branches contain the role
impl<Me, IO, L, R> ProjectChoiceCase<Me, IO, L, R, types::True, types::True> for ()
where
    Me: Role,
    L: TSession<IO>,
    R: TSession<IO>,
    (): ProjectRole<Me, IO, L>,
    (): ProjectRole<Me, IO, R>,
{
    type Out =
        EpChoice<IO, Me, <() as ProjectRole<Me, IO, L>>::Out, <() as ProjectRole<Me, IO, R>>::Out>;
}

// Case 2: Only left branch contains the role
impl<Me, IO, L, R> ProjectChoiceCase<Me, IO, L, R, types::True, types::False> for ()
where
    Me: Role,
    L: TSession<IO>,
    R: TSession<IO>,
    (): ProjectRole<Me, IO, L>,
{
    type Out = <() as ProjectRole<Me, IO, L>>::Out;
}

// Case 3: Only right branch contains the role
impl<Me, IO, L, R> ProjectChoiceCase<Me, IO, L, R, types::False, types::True> for ()
where
    Me: Role,
    L: TSession<IO>,
    R: TSession<IO>,
    (): ProjectRole<Me, IO, R>,
{
    type Out = <() as ProjectRole<Me, IO, R>>::Out;
}

// Case 4: Neither branch contains the role
impl<Me, IO, L, R> ProjectChoiceCase<Me, IO, L, R, types::False, types::False> for ()
where
    Me: Role,
    L: TSession<IO>,
    R: TSession<IO>,
{
    type Out = EpSkip<IO, Me>;
}

// --- Helper trait to check if a role is present in a protocol branch.
/// Returns a type-level boolean indicating whether the role is present.
pub trait ContainsRole<R> {
    type Output: types::Bool;
}

/// Helper trait to check if a role is NOT present in a protocol branch.
pub trait NotContainsRole<R> {}

// End always contains no roles
impl<IO, Lbl, R> ContainsRole<R> for TEnd<IO, Lbl> {
    type Output = types::False;
}

impl<IO, Lbl, R> NotContainsRole<R> for TEnd<IO, Lbl> {}

// TInteract contains the role if either the current role or continuation contains it
impl<IO, Lbl, H, T, R1, R2> ContainsRole<R2> for TInteract<IO, Lbl, R1, H, T>
where
    Lbl: types::ProtocolLabel,
    R1: RoleEq<R2>,
    <R1 as RoleEq<R2>>::Output: types::Bool,
    T: TSession<IO> + ContainsRole<R2>,
    <T as ContainsRole<R2>>::Output: types::Bool,
    // The following ensures Or can be used with these types
    <R1 as RoleEq<R2>>::Output: types::BoolOr<<T as ContainsRole<R2>>::Output>,
{
    // True if either this role or the continuation contains the role
    type Output = types::Or<<R1 as RoleEq<R2>>::Output, <T as ContainsRole<R2>>::Output>;
}

// TInteract doesn't contain the role if both the current role and continuation don't
impl<IO, Lbl, H, T, R1, R2> NotContainsRole<R2> for TInteract<IO, Lbl, R1, H, T>
where
    Lbl: types::ProtocolLabel,
    R1: RoleEq<R2>,
    <R1 as RoleEq<R2>>::Output: types::Bool + types::Not,
    <<R1 as RoleEq<R2>>::Output as types::Not>::Output: types::Bool,
    T: TSession<IO> + NotContainsRole<R2>,
{
}

// TChoice contains the role if either branch contains it
impl<IO, Lbl, L, R, RoleT> ContainsRole<RoleT> for TChoice<IO, Lbl, L, R>
where
    Lbl: types::ProtocolLabel,
    L: TSession<IO> + ContainsRole<RoleT>,
    <L as ContainsRole<RoleT>>::Output: types::Bool,
    R: TSession<IO> + ContainsRole<RoleT>,
    <R as ContainsRole<RoleT>>::Output: types::Bool,
    // The following ensures Or can be used with these types
    <L as ContainsRole<RoleT>>::Output: types::BoolOr<<R as ContainsRole<RoleT>>::Output>,
{
    // True if either branch contains the role
    type Output = types::Or<<L as ContainsRole<RoleT>>::Output, <R as ContainsRole<RoleT>>::Output>;
}

// TChoice doesn't contain the role if neither branch contains it
impl<IO, Lbl, L, R, RoleT> NotContainsRole<RoleT> for TChoice<IO, Lbl, L, R>
where
    Lbl: types::ProtocolLabel,
    L: TSession<IO> + NotContainsRole<RoleT>,
    R: TSession<IO> + NotContainsRole<RoleT>,
{
}

// Use a single implementation with dispatch on L branch containment
impl<IO, Lbl, L, R, IsDisjoint, RoleT> ContainsRole<RoleT> for TPar<IO, Lbl, L, R, IsDisjoint>
where
    Lbl: types::ProtocolLabel,
    L: TSession<IO> + ContainsRole<RoleT>,
    <L as ContainsRole<RoleT>>::Output: types::Bool,
    R: TSession<IO> + ContainsRole<RoleT>,
    <R as ContainsRole<RoleT>>::Output: types::Bool,
    // Use a helper trait to dispatch based on L branch containment
    (): TParContainsRoleImpl<
        <L as ContainsRole<RoleT>>::Output,
        <R as ContainsRole<RoleT>>::Output,
    >,
{
    type Output = <() as TParContainsRoleImpl<
        <L as ContainsRole<RoleT>>::Output,
        <R as ContainsRole<RoleT>>::Output,
    >>::Output;
}

// Helper trait for TPar role containment logic
pub trait TParContainsRoleImpl<LContains, RContains> {
    type Output: types::Bool;
}

// If either branch contains the role, TPar contains it
impl TParContainsRoleImpl<types::True, types::True> for () {
    type Output = types::True;
}

impl TParContainsRoleImpl<types::True, types::False> for () {
    type Output = types::True;
}

impl TParContainsRoleImpl<types::False, types::True> for () {
    type Output = types::True;
}

// If neither branch contains the role, TPar doesn't contain it
impl TParContainsRoleImpl<types::False, types::False> for () {
    type Output = types::False;
}

/// Helper trait for projecting a protocol parallel composition.
///
/// - `Me`: The role being projected.
/// - `IO`: Protocol marker type.
/// - `L`, `R`: The two protocol branches.
pub trait ProjectPar<Me, IO, L: TSession<IO>, R: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}

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

// Projection implementation for TPar - delegate to ProjectPar helper
impl<Me, IO, Lbl, L, R, IsDisjoint> ProjectRole<Me, IO, TPar<IO, Lbl, L, R, IsDisjoint>> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
    (): ProjectPar<Me, IO, L, R>,
{
    type Out = <() as ProjectPar<Me, IO, L, R>>::Out;
}

/// Main flag-based composition trait for projected parallel branches
pub trait ComposeProjectedParBranches<IO, Me: Role, L, R>
where
    L: IsEpSkipVariant<IO, Me> + IsEpEndVariant<IO, Me> + EpSession<IO, Me>,
    R: IsEpSkipVariant<IO, Me> + IsEpEndVariant<IO, Me> + EpSession<IO, Me>,
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

/// Helper trait for case selection in composition of parallel branches
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

// Both are projected (non-skip, non-end)
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

// Implement ProjectPar by projecting both branches and composing the result
impl<Me, IO, L, R> ProjectPar<Me, IO, L, R> for ()
where
    Me: Role,
    L: TSession<IO>,
    R: TSession<IO>,
    // Determine if branches contain the role
    L: ContainsRole<Me>,
    <L as ContainsRole<Me>>::Output: types::Bool,
    R: ContainsRole<Me>,
    <R as ContainsRole<Me>>::Output: types::Bool,
    // Project each branch conditionally based on role presence
    (): ProjectParBranch<<L as ContainsRole<Me>>::Output, Me, IO, L>,
    (): ProjectParBranch<<R as ContainsRole<Me>>::Output, Me, IO, R>,
    // Get the resulting endpoint types
    <() as ProjectParBranch<<L as ContainsRole<Me>>::Output, Me, IO, L>>::Out:
        EpSession<IO, Me> +
        IsEpSkipVariant<IO, Me> +
        IsEpEndVariant<IO, Me>,
    <() as ProjectParBranch<<R as ContainsRole<Me>>::Output, Me, IO, R>>::Out:
        EpSession<IO, Me> +
        IsEpSkipVariant<IO, Me> +
        IsEpEndVariant<IO, Me>,
    // Ensure the output types have the right marker types
    <<() as ProjectParBranch<<L as ContainsRole<Me>>::Output, Me, IO, L>>::Out as IsEpSkipVariant<IO, Me>>::Output: types::Bool,
    <<() as ProjectParBranch<<R as ContainsRole<Me>>::Output, Me, IO, R>>::Out as IsEpSkipVariant<IO, Me>>::Output: types::Bool,
    <<() as ProjectParBranch<<L as ContainsRole<Me>>::Output, Me, IO, L>>::Out as IsEpEndVariant<IO, Me>>::Output: types::Bool,
    <<() as ProjectParBranch<<R as ContainsRole<Me>>::Output, Me, IO, R>>::Out as IsEpEndVariant<IO, Me>>::Output: types::Bool,
    // Compose the projected branches
    (): ComposeProjectedParBranches<
        IO,
        Me,
        <() as ProjectParBranch<<L as ContainsRole<Me>>::Output, Me, IO, L>>::Out,
        <() as ProjectParBranch<<R as ContainsRole<Me>>::Output, Me, IO, R>>::Out
    >,
{
    type Out = <() as ComposeProjectedParBranches<
        IO,
        Me,
        <() as ProjectParBranch<<L as ContainsRole<Me>>::Output, Me, IO, L>>::Out,
        <() as ProjectParBranch<<R as ContainsRole<Me>>::Output, Me, IO, R>>::Out
    >>::Out;
}
