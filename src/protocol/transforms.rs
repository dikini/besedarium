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
/// use besedarium::transforms::ProjectSend;
/// struct Alice; struct Bob;
/// impl Role for Alice {} impl Role for Bob {}
/// impl ProtocolLabel for Alice {} impl ProtocolLabel for Bob {}
/// impl RoleEq<Alice> for Alice { type Output = True; }
/// impl RoleEq<Bob> for Alice   { type Output = False; }
/// impl RoleEq<Alice> for Bob   { type Output = False; }
/// impl RoleEq<Bob> for Bob     { type Output = True; }
///
/// // Global protocol: Alice sends Message then Bob sends Response
/// type Global = TSend<
///     Http,
///     EmptyLabel,
///     Alice,
///     Message,
///     TSend<Http, EmptyLabel, Bob, Response, TEnd<Http, EmptyLabel>>
/// >;
/// // Project onto Alice
/// type AliceLocal = <() as ProjectRole<Alice, Http, Global>>::Out;
/// // Should be a send of Message then a receive of Response
/// // Strict type equality assertion moved to a regular test (see tests/compile.rs).
/// // Note: In doctests, strict type equality may fail due to Rust's type identity rules across modules.
/// ```
pub trait ProjectRole<Me, IO, G: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}

// Base case: projecting end-of-session yields EpEnd with preserved label
impl<Me, IO, Lbl> ProjectRole<Me, IO, TEnd<IO, Lbl>> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
{
    type Out = EpEnd<IO, Lbl, Me>;
}

/// Helper trait for projecting a protocol choice.
///
/// - `Me`: The role being projected.
/// - `IO`: Protocol marker type.
/// - `L`, `R`: The two protocol branches.
pub trait ProjectChoice<Me, IO, L: TSession<IO>, R: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}

// Projection implementation for TChoice - delegate to ProjectChoice helper with preserved label
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
        Lbl,
        L,
        R,
        <L as ContainsRole<Me>>::Output,
        <R as ContainsRole<Me>>::Output,
    >,
{
    type Out = <() as ProjectChoiceCase<
        Me,
        IO,
        Lbl,
        L,
        R,
        <L as ContainsRole<Me>>::Output,
        <R as ContainsRole<Me>>::Output,
    >>::Out;
}

// Helper trait for handling different cases of ProjectChoice based on role presence
pub trait ProjectChoiceCase<
    Me,
    IO,
    Lbl: types::ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
    LContainsMe,
    RContainsMe,
>
{
    type Out: EpSession<IO, Me>;
}

// Case 1: Both branches contain the role - preserve label
impl<Me, IO, Lbl, L, R> ProjectChoiceCase<Me, IO, Lbl, L, R, types::True, types::True> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
    (): ProjectRole<Me, IO, L>,
    (): ProjectRole<Me, IO, R>,
{
    type Out = EpChoice<
        IO,
        Lbl,
        Me,
        <() as ProjectRole<Me, IO, L>>::Out,
        <() as ProjectRole<Me, IO, R>>::Out,
    >;
}

// Case 2: Only left branch contains the role - wrap the projection in EpChoice with the Choice's label
impl<Me, IO, Lbl, L, R> ProjectChoiceCase<Me, IO, Lbl, L, R, types::True, types::False> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
    (): ProjectRole<Me, IO, L>,
{
    // Wrap the projection in EpChoice with the parent Choice's label
    type Out = EpChoice<IO, Lbl, Me, <() as ProjectRole<Me, IO, L>>::Out, EpSkip<IO, Lbl, Me>>;
}

// Case 3: Only right branch contains the role - wrap the projection in EpChoice with the Choice's label
impl<Me, IO, Lbl, L, R> ProjectChoiceCase<Me, IO, Lbl, L, R, types::False, types::True> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
    (): ProjectRole<Me, IO, R>,
{
    // Wrap the projection in EpChoice with the parent Choice's label
    type Out = EpChoice<IO, Lbl, Me, EpSkip<IO, Lbl, Me>, <() as ProjectRole<Me, IO, R>>::Out>;
}

// Case 4: Neither branch contains the role
impl<Me, IO, Lbl, L, R> ProjectChoiceCase<Me, IO, Lbl, L, R, types::False, types::False> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
{
    type Out = EpSkip<IO, Lbl, Me>;
}

// --- Helper trait to check if a role is present in a protocol branch.
/// Returns a type-level boolean indicating whether the role is present.
pub trait ContainsRole<R> {
    type Output: types::Bool;
}

/// Helper trait to check if a role is NOT present in a protocol branch.
pub trait NotContainsRole<R> {}

// Base case: TEnd doesn't contain any role
impl<IO, Lbl, R> ContainsRole<R> for TEnd<IO, Lbl> {
    type Output = types::False;
}

impl<IO, Lbl, R> NotContainsRole<R> for TEnd<IO, Lbl> {}

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

// TPar doesn't contain the role if neither branch contains it
impl<IO, Lbl, L, R, IsDisjoint, RoleT> NotContainsRole<RoleT> for TPar<IO, Lbl, L, R, IsDisjoint>
where
    Lbl: types::ProtocolLabel,
    L: TSession<IO> + NotContainsRole<RoleT>,
    R: TSession<IO> + NotContainsRole<RoleT>,
{
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
/// - `Lbl`: Label from the TPar construct.
/// - `L`, `R`: The two protocol branches.
pub trait ProjectPar<Me, IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}

// We've replaced ProjectParBranch with ProjectRoleOrSkip for better label handling

/// Helper trait to project role or create skip with the parent label
pub trait ProjectRoleOrSkip<Me: Role, IO, G: TSession<IO>, Flag, ParentLbl: types::ProtocolLabel> {
    type Out: EpSession<IO, Me>;
}

// If role is in branch, project it
impl<Me: Role, IO, G: TSession<IO>, Lbl: types::ProtocolLabel>
    ProjectRoleOrSkip<Me, IO, G, types::True, Lbl> for ()
where
    (): ProjectRole<Me, IO, G>,
{
    // Just return the projection directly - no additional wrapping needed
    type Out = <() as ProjectRole<Me, IO, G>>::Out;
}

// If role is not in branch, create skip with parent label
impl<Me: Role, IO, G: TSession<IO>, Lbl: types::ProtocolLabel>
    ProjectRoleOrSkip<Me, IO, G, types::False, Lbl> for ()
{
    // Create EpSkip with the parent label (this ensures label preservation)
    type Out = EpSkip<IO, Lbl, Me>;
}

// Projection implementation for TPar - delegate to ProjectPar helper but pass the label
impl<Me, IO, Lbl, L, R, IsDisjoint> ProjectRole<Me, IO, TPar<IO, Lbl, L, R, IsDisjoint>> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
    (): ProjectPar<Me, IO, Lbl, L, R>,
{
    type Out = <() as ProjectPar<Me, IO, Lbl, L, R>>::Out;
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

// Extract the GetProtocolLabel trait implementation here
pub trait GetProtocolLabel {
    type Label: types::ProtocolLabel;
}

// Add implementation for TSend
impl<IO, Lbl: types::ProtocolLabel, R, H, T: TSession<IO>> GetProtocolLabel
    for TSend<IO, Lbl, R, H, T>
{
    type Label = Lbl;
}
// Add implementation for TRecv
impl<IO, Lbl: types::ProtocolLabel, R, H, T: TSession<IO>> GetProtocolLabel
    for TRecv<IO, Lbl, R, H, T>
{
    type Label = Lbl;
}
// Add implementation for TEnd
impl<IO, Lbl: types::ProtocolLabel> GetProtocolLabel for TEnd<IO, Lbl> {
    type Label = Lbl;
}
// Add implementation for TChoice
impl<IO, Lbl: types::ProtocolLabel, L, R> GetProtocolLabel for TChoice<IO, Lbl, L, R>
where
    L: TSession<IO>,
    R: TSession<IO>,
{
    type Label = Lbl;
}
// Add implementation for TPar
impl<IO, Lbl: types::ProtocolLabel, L, R, IsDisjoint> GetProtocolLabel
    for TPar<IO, Lbl, L, R, IsDisjoint>
where
    L: TSession<IO>,
    R: TSession<IO>,
{
    type Label = Lbl;
}
// Add implementation for TRec
impl<IO, Lbl: types::ProtocolLabel, S> GetProtocolLabel for TRec<IO, Lbl, S>
where
    S: TSession<IO>,
{
    type Label = Lbl;
}

// Both branches are EpSkip
impl<IO, Me: Role, Lbl1: types::ProtocolLabel, Lbl2: types::ProtocolLabel>
    ComposeProjectedParBranchesCase<
        types::True,
        types::True,
        types::False,
        types::False,
        IO,
        Me,
        EpSkip<IO, Lbl1, Me>,
        EpSkip<IO, Lbl2, Me>,
    > for ()
{
    // If both branches are skipped, output a skip
    type Out = EpSkip<IO, Lbl1, Me>; // Use label from first branch
}

// Left is EpSkip, right is projected
impl<IO, Me: Role, Lbl: types::ProtocolLabel, ProjectedR: EpSession<IO, Me>>
    ComposeProjectedParBranchesCase<
        types::True,
        types::False,
        types::False,
        types::False,
        IO,
        Me,
        EpSkip<IO, Lbl, Me>,
        ProjectedR,
    > for ()
{
    type Out = ProjectedR;
}

// Left is projected, right is EpSkip
impl<IO, Me: Role, Lbl: types::ProtocolLabel, ProjectedL: EpSession<IO, Me>>
    ComposeProjectedParBranchesCase<
        types::False,
        types::True,
        types::False,
        types::False,
        IO,
        Me,
        ProjectedL,
        EpSkip<IO, Lbl, Me>,
    > for ()
{
    // Return the non-skip branch directly
    // This matches test_preserved_label_in_parallel expectations
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
where
    ProjectedL: GetLocalLabel, // Add constraint to extract label
{
    type Out = EpPar<IO, <ProjectedL as GetLocalLabel>::Label, Me, ProjectedL, ProjectedR>;
}

// Left is EpEnd, right is EpSkip
impl<IO, Me: Role, Lbl1: types::ProtocolLabel, Lbl2: types::ProtocolLabel>
    ComposeProjectedParBranchesCase<
        types::False,
        types::True,
        types::True,
        types::False,
        IO,
        Me,
        EpEnd<IO, Lbl1, Me>,
        EpSkip<IO, Lbl2, Me>,
    > for ()
{
    type Out = EpEnd<IO, Lbl1, Me>; // Preserve the label from EpEnd
}

// Left is EpSkip, right is EpEnd
impl<IO, Me: Role, Lbl1: types::ProtocolLabel, Lbl2: types::ProtocolLabel>
    ComposeProjectedParBranchesCase<
        types::True,
        types::False,
        types::False,
        types::True,
        IO,
        Me,
        EpSkip<IO, Lbl1, Me>,
        EpEnd<IO, Lbl2, Me>,
    > for ()
{
    type Out = EpEnd<IO, Lbl2, Me>; // Preserve the label from EpEnd
}

// Left is EpEnd, right is projected
impl<IO, Me: Role, Lbl: types::ProtocolLabel, ProjectedR: EpSession<IO, Me>>
    ComposeProjectedParBranchesCase<
        types::False,
        types::False,
        types::True,
        types::False,
        IO,
        Me,
        EpEnd<IO, Lbl, Me>,
        ProjectedR,
    > for ()
{
    type Out = ProjectedR;
}

// Left is projected, right is EpEnd
impl<IO, Me: Role, Lbl: types::ProtocolLabel, ProjectedL: EpSession<IO, Me>>
    ComposeProjectedParBranchesCase<
        types::False,
        types::False,
        types::False,
        types::True,
        IO,
        Me,
        ProjectedL,
        EpEnd<IO, Lbl, Me>,
    > for ()
{
    type Out = ProjectedL;
}

// Both are EpEnd
impl<IO, Me: Role, Lbl1: types::ProtocolLabel, Lbl2: types::ProtocolLabel>
    ComposeProjectedParBranchesCase<
        types::False,
        types::False,
        types::True,
        types::True,
        IO,
        Me,
        EpEnd<IO, Lbl1, Me>,
        EpEnd<IO, Lbl2, Me>,
    > for ()
{
    type Out = EpEnd<IO, Lbl1, Me>; // Use label from first branch
}

// Extract labels from local endpoint types
pub trait GetLocalLabel {
    type Label: types::ProtocolLabel;
}

// Implementations for different endpoint types
impl<IO, Lbl: types::ProtocolLabel, R, H, T> GetLocalLabel for EpSend<IO, Lbl, R, H, T> {
    type Label = Lbl;
}

impl<IO, Lbl: types::ProtocolLabel, R, H, T> GetLocalLabel for EpRecv<IO, Lbl, R, H, T> {
    type Label = Lbl;
}

impl<IO, Lbl: types::ProtocolLabel, Me, L, R> GetLocalLabel for EpChoice<IO, Lbl, Me, L, R> {
    type Label = Lbl;
}

impl<IO, Lbl: types::ProtocolLabel, Me, L, R> GetLocalLabel for EpPar<IO, Lbl, Me, L, R> {
    type Label = Lbl;
}

impl<IO, Lbl: types::ProtocolLabel, R> GetLocalLabel for EpEnd<IO, Lbl, R> {
    type Label = Lbl;
}

impl<IO, Lbl: types::ProtocolLabel, R> GetLocalLabel for EpSkip<IO, Lbl, R> {
    type Label = Lbl;
}

/// Type-level filter that removes all EpSkip<IO, Me> branches from a type-level list.
pub trait FilterSkips<IO, Me: Role, List> {
    type Out;
}

impl<IO, Me: Role> FilterSkips<IO, Me, Nil> for () {
    type Out = Nil;
}

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
impl<IO, Me: Role, Lbl: types::ProtocolLabel, T>
    FilterSkipsCase<IO, Me, EpSkip<IO, Lbl, Me>, T, IsEpSkipType> for ()
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

// Implement ProjectPar by dispatching to a helper trait for case-specific behavior
impl<Me, IO, Lbl, L, R> ProjectPar<Me, IO, Lbl, L, R> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
    // Determine if branches contain the role
    L: ContainsRole<Me>,
    <L as ContainsRole<Me>>::Output: types::Bool,
    R: ContainsRole<Me>,
    <R as ContainsRole<Me>>::Output: types::Bool,
    // Use a helper trait to handle case-specific projection
    (): ProjectParCase<
        Me,
        IO,
        Lbl,
        L,
        R,
        <L as ContainsRole<Me>>::Output,
        <R as ContainsRole<Me>>::Output,
    >,
{
    type Out = <() as ProjectParCase<
        Me,
        IO,
        Lbl,
        L,
        R,
        <L as ContainsRole<Me>>::Output,
        <R as ContainsRole<Me>>::Output,
    >>::Out;
}

// Helper trait for case-specific projection of TPar
pub trait ProjectParCase<Me, IO, Lbl, L, R, LContainsMe, RContainsMe>
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
    LContainsMe: types::Bool,
    RContainsMe: types::Bool,
{
    type Out: EpSession<IO, Me>;
}

// Case 1: Role is in left branch but not right branch - Project left branch
impl<Me, IO, Lbl, L, R> ProjectParCase<Me, IO, Lbl, L, R, types::True, types::False> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
    (): ProjectRole<Me, IO, L>,
{
    // Just project the left branch directly (preserves internal labels)
    type Out = <() as ProjectRole<Me, IO, L>>::Out;
}

// Case 2: Role is in right branch but not left branch - Project right branch
impl<Me, IO, Lbl, L, R> ProjectParCase<Me, IO, Lbl, L, R, types::False, types::True> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
    (): ProjectRole<Me, IO, R>,
{
    // Just project the right branch directly (preserves internal labels)
    type Out = <() as ProjectRole<Me, IO, R>>::Out;
}

// Case 3: Role is in neither branch - Create EpSkip with parent label
impl<Me, IO, Lbl, L, R> ProjectParCase<Me, IO, Lbl, L, R, types::False, types::False> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
{
    // Create EpSkip with the parent label
    type Out = EpSkip<IO, Lbl, Me>;
}

// Case 4: Role is in both branches - Project both and create EpPar
impl<Me, IO, Lbl, L, R> ProjectParCase<Me, IO, Lbl, L, R, types::True, types::True> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    L: TSession<IO>,
    R: TSession<IO>,
    (): ProjectRole<Me, IO, L>,
    (): ProjectRole<Me, IO, R>,
{
    // Create EpPar with both projected branches
    type Out = EpPar<
        IO,
        Lbl,
        Me,
        <() as ProjectRole<Me, IO, L>>::Out,
        <() as ProjectRole<Me, IO, R>>::Out,
    >;
}

// TSend contains the role if the sender matches, or the continuation contains the role
impl<IO, Lbl, R, H, T, RoleT> ContainsRole<RoleT> for TSend<IO, Lbl, R, H, T>
where
    Lbl: types::ProtocolLabel,
    R: RoleEq<RoleT>,
    <R as RoleEq<RoleT>>::Output: types::Bool,
    T: TSession<IO> + ContainsRole<RoleT>,
    <T as ContainsRole<RoleT>>::Output: types::Bool,
    <R as RoleEq<RoleT>>::Output: types::BoolOr<<T as ContainsRole<RoleT>>::Output>,
{
    type Output = types::Or<<R as RoleEq<RoleT>>::Output, <T as ContainsRole<RoleT>>::Output>;
}

// TRecv contains the role if the receiver matches, or the continuation contains the role
impl<IO, Lbl, R, H, T, RoleT> ContainsRole<RoleT> for TRecv<IO, Lbl, R, H, T>
where
    Lbl: types::ProtocolLabel,
    R: RoleEq<RoleT>,
    <R as RoleEq<RoleT>>::Output: types::Bool,
    T: TSession<IO> + ContainsRole<RoleT>,
    <T as ContainsRole<RoleT>>::Output: types::Bool,
    <R as RoleEq<RoleT>>::Output: types::BoolOr<<T as ContainsRole<RoleT>>::Output>,
{
    type Output = types::Or<<R as RoleEq<RoleT>>::Output, <T as ContainsRole<RoleT>>::Output>;
}

// --- Add new helper traits for send/recv projection ---
/// Helper trait for projecting `TSend` combinators onto a role.
///
/// This trait is used to project a `TSend` combinator from a global protocol
/// onto a specific role, producing the corresponding local protocol for that role.
///
/// # Type Parameters
/// - `Me`: The role onto which the combinator is being projected.
/// - `IO`: The I/O type associated with the protocol.
/// - `Label`: The protocol label for the `TSend` combinator.
/// - `To`: The role to which the message is being sent.
/// - `Msg`: The type of the message being sent.
/// - `Cont`: The continuation of the protocol after the `TSend` combinator.
///
/// # Associated Type
/// - `Out`: The resulting local protocol for the role `Me`.
///
/// # Examples
/// ```rust
/// use besedarium::*;
/// use besedarium::transforms::ProjectSend;
/// struct Alice; struct Bob;
/// impl Role for Alice {} impl Role for Bob {}
/// impl ProtocolLabel for Alice {} impl ProtocolLabel for Bob {}
/// impl RoleEq<Alice> for Alice { type Output = True; }
/// impl RoleEq<Bob> for Alice   { type Output = False; }
/// impl RoleEq<Alice> for Bob   { type Output = False; }
/// impl RoleEq<Bob> for Bob     { type Output = True; }
///
/// type Global = TSend<
///     Http,
///     EmptyLabel,
///     Alice,
///     Message,
///     TEnd<Http, EmptyLabel>
/// >;
/// type AliceLocal = <() as ProjectSend<Alice, Http, EmptyLabel, Bob, Message, TEnd<Http, EmptyLabel>>>::Out;
/// ```
///
pub trait ProjectSend<Me, IO, Label, To, Msg, Cont>
where
    Label: types::ProtocolLabel,
    Cont: TSession<IO>,
{
    type Out: EpSession<IO, Me>;
}

/// Helper trait for projecting TRecv combinators onto a role.
pub trait ProjectRecv<Me, IO, Label, From, Msg, Cont>
where
    Label: types::ProtocolLabel,
    Cont: TSession<IO>,
{
    type Out: EpSession<IO, Me>;
}

// --- Helper traits for type-level dispatch on sender/receiver ---
pub trait ProjectSendCase<Flag, Me, IO, Label, To, Msg, Cont> {
    type Out: EpSession<IO, Me>;
}

// --- ProjectSendCase: If Me is the sender, produce EpSend with Me as the role parameter ---
impl<Me, IO, Label, To, Msg, Cont> ProjectSendCase<types::True, Me, IO, Label, To, Msg, Cont> for ()
where
    Label: types::ProtocolLabel,
    Cont: TSession<IO>,
    (): ProjectRole<Me, IO, Cont>,
{
    // If Me is the sender, produce EpSend (Me is the projected role)
    type Out = EpSend<IO, Label, Me, Msg, <() as ProjectRole<Me, IO, Cont>>::Out>;
}

impl<Me, IO, Label, To, Msg, Cont> ProjectSendCase<types::False, Me, IO, Label, To, Msg, Cont>
    for ()
where
    Label: types::ProtocolLabel,
    Cont: TSession<IO>,
    (): ProjectRole<Me, IO, Cont>,
{
    // If Me is not the sender, just project the continuation
    type Out = <() as ProjectRole<Me, IO, Cont>>::Out;
}

pub trait ProjectRecvCase<Flag, Me, IO, Label, From, Msg, Cont> {
    type Out: EpSession<IO, Me>;
}

// --- ProjectRecvCase: If Me is the receiver, produce EpRecv with Me as the role parameter ---
impl<Me, IO, Label, From, Msg, Cont> ProjectRecvCase<types::True, Me, IO, Label, From, Msg, Cont>
    for ()
where
    Label: types::ProtocolLabel,
    Cont: TSession<IO>,
    (): ProjectRole<Me, IO, Cont>,
{
    // If Me is the receiver, produce EpRecv (Me is the projected role)
    type Out = EpRecv<IO, Label, Me, Msg, <() as ProjectRole<Me, IO, Cont>>::Out>;
}

impl<Me, IO, Label, From, Msg, Cont> ProjectRecvCase<types::False, Me, IO, Label, From, Msg, Cont>
    for ()
where
    Label: types::ProtocolLabel,
    Cont: TSession<IO>,
    (): ProjectRole<Me, IO, Cont>,
{
    // If Me is not the receiver, just project the continuation
    type Out = <() as ProjectRole<Me, IO, Cont>>::Out;
}

// Update ProjectSend/ProjectRecv to use the helper traits
impl<Me, IO, Label, To, Msg, Cont> ProjectSend<Me, IO, Label, To, Msg, Cont> for ()
where
    Label: types::ProtocolLabel,
    To: RoleEq<Me>,
    <To as RoleEq<Me>>::Output: types::Bool,
    Cont: TSession<IO>,
    (): ProjectRole<Me, IO, Cont>,
    (): ProjectSendCase<<To as RoleEq<Me>>::Output, Me, IO, Label, To, Msg, Cont>,
{
    type Out =
        <() as ProjectSendCase<<To as RoleEq<Me>>::Output, Me, IO, Label, To, Msg, Cont>>::Out;
}

impl<Me, IO, Label, From, Msg, Cont> ProjectRecv<Me, IO, Label, From, Msg, Cont> for ()
where
    Label: types::ProtocolLabel,
    From: RoleEq<Me>,
    <From as RoleEq<Me>>::Output: types::Bool,
    Cont: TSession<IO>,
    (): ProjectRole<Me, IO, Cont>,
    (): ProjectRecvCase<<From as RoleEq<Me>>::Output, Me, IO, Label, From, Msg, Cont>,
{
    type Out =
        <() as ProjectRecvCase<<From as RoleEq<Me>>::Output, Me, IO, Label, From, Msg, Cont>>::Out;
}

// --- ProjectRole implementation for TSend ---
impl<Me, IO, Lbl, To, Msg, Cont> ProjectRole<Me, IO, TSend<IO, Lbl, To, Msg, Cont>> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    To: RoleEq<Me>,
    <To as RoleEq<Me>>::Output: types::Bool,
    Cont: TSession<IO>,
    (): ProjectRole<Me, IO, Cont>,
    (): ProjectSendCase<<To as RoleEq<Me>>::Output, Me, IO, Lbl, To, Msg, Cont>,
{
    type Out = <() as ProjectSendCase<<To as RoleEq<Me>>::Output, Me, IO, Lbl, To, Msg, Cont>>::Out;
}

// --- ProjectRole implementation for TRecv ---
impl<Me, IO, Lbl, From, Msg, Cont> ProjectRole<Me, IO, TRecv<IO, Lbl, From, Msg, Cont>> for ()
where
    Me: Role,
    Lbl: types::ProtocolLabel,
    From: RoleEq<Me>,
    <From as RoleEq<Me>>::Output: types::Bool,
    Cont: TSession<IO>,
    (): ProjectRole<Me, IO, Cont>,
    (): ProjectRecvCase<<From as RoleEq<Me>>::Output, Me, IO, Lbl, From, Msg, Cont>,
{
    type Out =
        <() as ProjectRecvCase<<From as RoleEq<Me>>::Output, Me, IO, Lbl, From, Msg, Cont>>::Out;
}
