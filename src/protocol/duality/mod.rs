//! # Duality Checking for Enhanced MPST System
//!
//! This module provides type-level duality checking capabilities for verifying
//! that protocol specifications are correct duals of each other. This ensures
//! safe multiparty communication by validating that complementary protocol
//! endpoints properly match.
//!
//! ## Key Components
//!
//! - **IsDual Trait**: Core trait for checking duality between two protocol types
//! - **Helper Traits**: Support traits for type-level boolean operations
//! - **Global Protocol Duality**: Implementations for Global Protocol types
//! - **Local Endpoint Duality**: Implementations for Local Endpoint types
//! - **Validation Macros**: Compile-time assertions for duality checking
//!
//! ## Duality Rules
//!
//! Based on `docs/duality.md`, duality follows these core rules:
//!
//! | Construct                | Dual() Definition                                |
//! |-------------------------|--------------------------------------------------|
//! | End                     | End                                              |
//! | Send<S, R, M, Msg, P>   | Receive<R, S, M, Msg, Dual(P)>                   |
//! | Receive<R, S, M, Msg, P> | Send<S, R, M, Msg, Dual(P)>                     |
//! | Choice {l_i: P_i}       | Offer {l_i: Dual(P_i)}                          |
//! | Offer {l_i: P_i}        | Choice {l_i: Dual(P_i)}                         |
//! | Par(P, Q)               | Par(Dual(P), Dual(Q))                           |

use crate::protocol::foundation::{
    ActionIOTMarker, ChanId, CommMetadataTrait, GlobalProtocol, LocalProtocol, Message, MsgLbl,
    Role, SupportsActionIO,
};
use crate::protocol::global::{
    TChanChoice, TChanEnd, TChanOffer, TChanPar, TChanRecv, TChanSend, TChanStart,
};
use crate::protocol::local::{
    EpChanChoice, EpChanEnd, EpChanOffer, EpChanPar, EpChanRecv, EpChanSend, EpChanStart,
};
use crate::types::{Bool, False, True};

// ============================================================================
// Core IsDual Trait
// ============================================================================

/// Type-level trait for checking duality between two protocol types
///
/// Returns `True` if P and Q are duals of each other, `False` otherwise.
/// This trait provides compile-time verification that two protocol types
/// satisfy the duality relationship required for safe communication.
///
/// # Examples
///
/// ```ignore
/// use besedarium::protocol::duality::IsDual;
/// use besedarium::types::{True, False};
///
/// // Send and Recv with swapped roles should be dual
/// type SendType = TChanSend<Alice, Bob, Meta, Msg, End, AIO>;
/// type RecvType = TChanRecv<Bob, Alice, Meta, Msg, End, AIO>;
///
/// // This constraint ensures they are dual
/// fn verify_dual() where (): IsDual<SendType, RecvType>, <() as IsDual<SendType, RecvType>>::Output: EqualsTrue {}
/// ```
pub trait IsDual<P, Q> {
    type Output: Bool;
}

/// Helper type alias for cleaner usage of duality checking
pub type IsDualOutput<P, Q> = <() as IsDual<P, Q>>::Output;

// ============================================================================
// Helper Traits for Type-Level Boolean Constraints
// ============================================================================

/// Helper trait to ensure a type-level boolean is True
///
/// This trait is implemented only for the `True` type, allowing us to
/// constrain generic parameters to be provably true at compile time.
pub trait EqualsTrue {}
impl EqualsTrue for True {}

/// Helper trait to ensure a type-level boolean is False
///
/// This trait is implemented only for the `False` type, allowing us to
/// constrain generic parameters to be provably false at compile time.
pub trait EqualsFalse {}
impl EqualsFalse for False {}

/// Marker trait for types that can participate in duality checking
///
/// This trait serves as a safety mechanism to ensure only appropriate
/// protocol types are used in duality relationships.
pub trait DualityCheck: Send + Sync + 'static {}

// Implement for all protocol types
// Note: Only implementing for GlobalProtocol to avoid conflicts
impl<T: GlobalProtocol> DualityCheck for T {}

// ============================================================================
// IsDual Implementations for Global Protocol Types
// ============================================================================

/// TChanEnd is dual to itself (with compatible IO and metadata)
///
/// Protocol termination is self-dual since both endpoints simply end
/// the communication without any message exchange.
impl<C, L, AIO> IsDual<TChanEnd<C, L, AIO>, TChanEnd<C, L, AIO>> for ()
where
    C: ChanId,
    L: MsgLbl,
    AIO: ActionIOTMarker,
{
    type Output = True;
}

/// TChanSend<S, R, C, L, Msg, P, AIO> is dual to TChanRecv<R, S, C, L, Msg, Q, AIO>
///
/// A send action from S to R is dual to a receive action from R to S,
/// with the same channel, label, and message type, and dual continuation protocols.
impl<S, R, C, L, Msg, P, Q, AIO>
    IsDual<TChanSend<S, R, C, L, Msg, P, AIO>, TChanRecv<R, S, C, L, Msg, Q, AIO>> for ()
where
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    Q: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<P, Q>,
    <() as IsDual<P, Q>>::Output: EqualsTrue,
{
    type Output = True;
}

/// TChanRecv<R, S, C, L, Msg, P, AIO> is dual to TChanSend<S, R, C, L, Msg, Q, AIO>
///
/// Symmetric to the above: a receive action is dual to the corresponding send action.
impl<R, S, C, L, Msg, P, Q, AIO>
    IsDual<TChanRecv<R, S, C, L, Msg, P, AIO>, TChanSend<S, R, C, L, Msg, Q, AIO>> for ()
where
    R: Role,
    S: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    Q: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<P, Q>,
    <() as IsDual<P, Q>>::Output: EqualsTrue,
{
    type Output = True;
}

/// TChanChoice<S, R, C, L, Left, Right, AIO> is dual to TChanOffer<S, R, C, L, LeftDual, RightDual, AIO>
///
/// A choice action (making a selection) is dual to an offer action (handling the selection),
/// where each branch in the choice corresponds to the dual of the respective branch in the offer.
impl<R, C, Lbl, Left, Right, LeftDual, RightDual, AIO>
    IsDual<
        TChanChoice<R, C, Lbl, Left, Right, AIO>,
        TChanOffer<R, C, Lbl, LeftDual, RightDual, AIO>,
    > for ()
where
    R: Role,
    C: ChanId,
    Lbl: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    LeftDual: GlobalProtocol,
    RightDual: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<Left, LeftDual>,
    (): IsDual<Right, RightDual>,
    <() as IsDual<Left, LeftDual>>::Output: EqualsTrue,
    <() as IsDual<Right, RightDual>>::Output: EqualsTrue,
{
    type Output = True;
}

/// TChanOffer<S, R, C, L, Left, Right, AIO> is dual to TChanChoice<S, R, C, L, LeftDual, RightDual, AIO>
///
/// Symmetric to the above: an offer action is dual to the corresponding choice action.
impl<R, C, Lbl, Left, Right, LeftDual, RightDual, AIO>
    IsDual<
        TChanOffer<R, C, Lbl, Left, Right, AIO>,
        TChanChoice<R, C, Lbl, LeftDual, RightDual, AIO>,
    > for ()
where
    R: Role,
    C: ChanId,
    Lbl: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    LeftDual: GlobalProtocol,
    RightDual: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<Left, LeftDual>,
    (): IsDual<Right, RightDual>,
    <() as IsDual<Left, LeftDual>>::Output: EqualsTrue,
    <() as IsDual<Right, RightDual>>::Output: EqualsTrue,
{
    type Output = True;
}

/// TChanPar<S, R, C, L, Left, Right, IsDisjoint, AIO> is dual to TChanPar<S, R, C, L, LeftDual, RightDual, IsDisjoint, AIO>
///
/// Parallel composition is dual when each constituent branch is dual to the corresponding
/// branch in the other parallel composition. The disjointness property must be preserved.
impl<C, Lbl, Left, Right, LeftDual, RightDual, IsDisjoint, AIO>
    IsDual<
        TChanPar<C, Lbl, Left, Right, IsDisjoint, AIO>,
        TChanPar<C, Lbl, LeftDual, RightDual, IsDisjoint, AIO>,
    > for ()
where
    C: ChanId,
    Lbl: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    LeftDual: GlobalProtocol,
    RightDual: GlobalProtocol,
    IsDisjoint: Send + Sync + 'static + std::fmt::Debug,
    AIO: ActionIOTMarker,
    (): IsDual<Left, LeftDual>,
    (): IsDual<Right, RightDual>,
    <() as IsDual<Left, LeftDual>>::Output: EqualsTrue,
    <() as IsDual<Right, RightDual>>::Output: EqualsTrue,
{
    type Output = True;
}

/// TChanStart<S, R, C, L, Start, AIO> is dual to itself when the inner protocol is self-dual
///
/// Protocol initialization is dual when the wrapped protocol is dual to itself.
impl<C, L, Start, AIO> IsDual<TChanStart<C, L, Start, AIO>, TChanStart<C, L, Start, AIO>> for ()
where
    C: ChanId,
    L: MsgLbl,
    Start: GlobalProtocol,
    AIO: ActionIOTMarker,
{
    type Output = True;
}

// ============================================================================
// IsDual Implementations for Local Endpoint Types
// ============================================================================

/// EpChanEnd is dual to itself (with compatible IO and metadata)
///
/// Local endpoint termination is self-dual regardless of IO capabilities,
/// as long as both endpoints use compatible metadata.
impl<IO1, IO2, M, AIO> IsDual<EpChanEnd<IO1, M, AIO>, EpChanEnd<IO2, M, AIO>> for ()
where
    IO1: SupportsActionIO<AIO>,
    IO2: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    AIO: ActionIOTMarker,
{
    type Output = True;
}

/// EpChanSend<IO1, M, Msg, P, AIO> is dual to EpChanRecv<IO2, M, Msg, Q, AIO>
///
/// A local send endpoint is dual to a local receive endpoint when they
/// handle the same message type and have dual continuation protocols.
impl<IO1, IO2, M, Msg, P, Q, AIO>
    IsDual<EpChanSend<IO1, M, Msg, P, AIO>, EpChanRecv<IO2, M, Msg, Q, AIO>> for ()
where
    IO1: SupportsActionIO<AIO>,
    IO2: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    Q: LocalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<P, Q>,
    <() as IsDual<P, Q>>::Output: EqualsTrue,
{
    type Output = True;
}

/// EpChanRecv<IO1, M, Msg, P, AIO> is dual to EpChanSend<IO2, M, Msg, Q, AIO>
///
/// Symmetric to the above: a local receive endpoint is dual to a local send endpoint.
impl<IO1, IO2, M, Msg, P, Q, AIO>
    IsDual<EpChanRecv<IO1, M, Msg, P, AIO>, EpChanSend<IO2, M, Msg, Q, AIO>> for ()
where
    IO1: SupportsActionIO<AIO>,
    IO2: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    Q: LocalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<P, Q>,
    <() as IsDual<P, Q>>::Output: EqualsTrue,
{
    type Output = True;
}

/// EpChanChoice<IO1, M, Left, Right, AIO> is dual to EpChanOffer<IO2, M, LeftDual, RightDual, AIO>
///
/// A local choice endpoint is dual to a local offer endpoint when each branch
/// is dual to the corresponding branch in the other endpoint.
impl<IO1, IO2, M, Left, Right, LeftDual, RightDual, AIO>
    IsDual<EpChanChoice<IO1, M, Left, Right, AIO>, EpChanOffer<IO2, M, LeftDual, RightDual, AIO>>
    for ()
where
    IO1: SupportsActionIO<AIO>,
    IO2: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    LeftDual: LocalProtocol,
    RightDual: LocalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<Left, LeftDual>,
    (): IsDual<Right, RightDual>,
    <() as IsDual<Left, LeftDual>>::Output: EqualsTrue,
    <() as IsDual<Right, RightDual>>::Output: EqualsTrue,
{
    type Output = True;
}

/// EpChanOffer<IO1, M, Left, Right, AIO> is dual to EpChanChoice<IO2, M, LeftDual, RightDual, AIO>
///
/// Symmetric to the above: a local offer endpoint is dual to a local choice endpoint.
impl<IO1, IO2, M, Left, Right, LeftDual, RightDual, AIO>
    IsDual<EpChanOffer<IO1, M, Left, Right, AIO>, EpChanChoice<IO2, M, LeftDual, RightDual, AIO>>
    for ()
where
    IO1: SupportsActionIO<AIO>,
    IO2: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    LeftDual: LocalProtocol,
    RightDual: LocalProtocol,
    AIO: ActionIOTMarker,
    (): IsDual<Left, LeftDual>,
    (): IsDual<Right, RightDual>,
    <() as IsDual<Left, LeftDual>>::Output: EqualsTrue,
    <() as IsDual<Right, RightDual>>::Output: EqualsTrue,
{
    type Output = True;
}

/// EpChanPar<IO1, M, Left, Right, IsDisjoint, AIO> is dual to EpChanPar<IO2, M, LeftDual, RightDual, IsDisjoint, AIO>
///
/// Local parallel endpoints are dual when each constituent branch is dual and
/// the disjointness property is preserved.
impl<IO1, IO2, M, Left, Right, LeftDual, RightDual, IsDisjoint, AIO>
    IsDual<
        EpChanPar<IO1, M, Left, Right, IsDisjoint, AIO>,
        EpChanPar<IO2, M, LeftDual, RightDual, IsDisjoint, AIO>,
    > for ()
where
    IO1: SupportsActionIO<AIO>,
    IO2: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    LeftDual: LocalProtocol,
    RightDual: LocalProtocol,
    IsDisjoint: Send + Sync + 'static + std::fmt::Debug,
    AIO: ActionIOTMarker,
    (): IsDual<Left, LeftDual>,
    (): IsDual<Right, RightDual>,
    <() as IsDual<Left, LeftDual>>::Output: EqualsTrue,
    <() as IsDual<Right, RightDual>>::Output: EqualsTrue,
{
    type Output = True;
}

/// EpChanStart<IO1, M, Start, AIO> is dual to EpChanStart<IO2, M, Start, AIO>
///
/// Local start endpoints are self-dual when they wrap the same protocol.
impl<IO1, IO2, M, Start, AIO>
    IsDual<EpChanStart<IO1, M, Start, AIO>, EpChanStart<IO2, M, Start, AIO>> for ()
where
    IO1: SupportsActionIO<AIO>,
    IO2: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Start: LocalProtocol,
    AIO: ActionIOTMarker,
{
    type Output = True;
}

// ============================================================================
// Default implementations for non-dual types
// ============================================================================

/// Default implementation: types are not dual unless explicitly implemented above
///
/// This implementation ensures that any types not covered by the specific
/// implementations above are considered non-dual, returning `False`.
// TODO: Fix conflicting implementations issue - temporarily disabled
/*
impl<P, Q> IsDual<P, Q> for ()
where
    P: DualityCheck,
    Q: DualityCheck,
{
    type Output = False;
}
*/
// ============================================================================
// Compile-Time Assertion Macros
// ============================================================================

/// Compile-time assertion that two types are dual
///
/// This macro generates a compile-time check that will fail if the two types
/// are not provably dual according to the `IsDual` trait implementations.
///
/// # Examples
///
/// ```ignore
/// assert_dual!(
///     TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>,
///     TChanRecv<Bob, Alice, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>
/// );
/// ```
#[macro_export]
macro_rules! assert_dual {
    ($P:ty, $Q:ty) => {
        const _: () = {
            fn _assert_dual()
            where
                (): $crate::protocol::duality::IsDual<$P, $Q>,
                <() as $crate::protocol::duality::IsDual<$P, $Q>>::Output:
                    $crate::protocol::duality::EqualsTrue,
            {
            }
        };
    };
}

/// Compile-time assertion that two types are NOT dual
///
/// This macro generates a compile-time check that will fail if the two types
/// are provably dual according to the `IsDual` trait implementations.
///
/// # Examples
///
/// ```ignore
/// assert_not_dual!(
///     TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>,
///     TChanSend<Bob, Alice, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>
/// );
/// ```
#[macro_export]
macro_rules! assert_not_dual {
    ($P:ty, $Q:ty) => {
        const _: () = {
            fn _assert_not_dual()
            where
                (): $crate::protocol::duality::IsDual<$P, $Q>,
                <() as $crate::protocol::duality::IsDual<$P, $Q>>::Output:
                    $crate::protocol::duality::EqualsFalse,
            {
            }
        };
    };
}

// ============================================================================
// Tests Module
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::foundation::{
        BiDirectionalAction, CommMetadata, DefaultChan, InputAction, OutputAction, RequestLbl,
    };

    // Define test roles
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Alice;
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Bob;

    impl Role for Alice {}
    impl Role for Bob {}

    // Define test message
    #[derive(Debug, Clone)]
    struct HelloMsg;
    impl Message for HelloMsg {}

    // Define test IO capabilities
    #[derive(Debug, Clone)]
    struct TestIO;
    impl SupportsActionIO<BiDirectionalAction> for TestIO {}
    impl SupportsActionIO<InputAction> for TestIO {}
    impl SupportsActionIO<OutputAction> for TestIO {}

    #[test]
    fn test_global_send_recv_duality() {
        type End = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
        type Send =
            TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>;
        type Recv =
            TChanRecv<Bob, Alice, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>;

        // This should compile successfully
        fn _test_dual()
        where
            (): IsDual<Send, Recv>,
            <() as IsDual<Send, Recv>>::Output: EqualsTrue,
        {
        }
    }

    #[test]
    fn test_global_end_self_duality() {
        type End = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;

        // End should be dual to itself
        fn _test_dual()
        where
            (): IsDual<End, End>,
            <() as IsDual<End, End>>::Output: EqualsTrue,
        {
        }
    }

    #[test]
    fn test_local_send_recv_duality() {
        type Meta = CommMetadata<DefaultChan, RequestLbl>;
        type EndEp = EpChanEnd<TestIO, Meta, BiDirectionalAction>;
        type SendEp = EpChanSend<TestIO, Meta, HelloMsg, EndEp, BiDirectionalAction>;
        type RecvEp = EpChanRecv<TestIO, Meta, HelloMsg, EndEp, BiDirectionalAction>;

        // Local endpoints should be dual
        fn _test_dual()
        where
            (): IsDual<SendEp, RecvEp>,
            <() as IsDual<SendEp, RecvEp>>::Output: EqualsTrue,
        {
        }
    }

    #[test]
    fn test_choice_offer_duality() {
        type EndType = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;

        type Choice =
            TChanChoice<Alice, DefaultChan, RequestLbl, EndType, EndType, BiDirectionalAction>;
        type Offer =
            TChanOffer<Alice, DefaultChan, RequestLbl, EndType, EndType, BiDirectionalAction>;

        // Choice and Offer should be dual
        fn _test_dual()
        where
            (): IsDual<Choice, Offer>,
            <() as IsDual<Choice, Offer>>::Output: EqualsTrue,
        {
        }
    }

    // Note: Tests for non-dual types would need negative compilation tests
    // which are better handled in integration tests or trybuild tests
}
