//! # Automatic Dual Protocol Generation
//!
//! This module implements automatic dual protocol generation capabilities
//! for the Besedarium session types library. It extends beyond the existing
//! `IsDual` verification trait to provide active generation of dual protocols.
//!
//! ## Core Capabilities
//!
//! - **Global protocols**: Check for well-foundedness (can be safely implemented)
//! - **Local protocols**: Generate duals for endpoint projections  
//! - **Verification**: Ensure projected local protocols are compatible
//!
//! ## Theoretical Foundation
//!
//! Global protocols describe complete communication choreography and do not have
//! "duals" in the traditional sense. Instead, they need well-foundedness checking.
//!
//! Local protocols (endpoint projections) represent individual participant views
//! and can have duals representing complementary participant viewpoints.

use crate::protocol::duality::{helpers::EqualsTrue, IsDual};
use crate::protocol::foundation::{
    ActionIOTMarker, BiDirectionalAction, ChanId, CommMetadata, CommMetadataTrait, DefaultChan,
    GlobalProtocol, LocalProtocol, Message, MsgLbl, RequestLbl, Role, SupportsActionIO,
    TcpOnlySessionIO,
};
use crate::protocol::global::{TChanChoice, TChanEnd, TChanOffer, TChanRecv, TChanSend};
use crate::protocol::local::{EpChanChoice, EpChanEnd, EpChanOffer, EpChanRecv, EpChanSend};

// ============================================================================
// Well-Foundedness for Global Protocols
// ============================================================================

/// Type-level trait for verifying well-foundedness of global protocols
///
/// Global protocols need well-foundedness checking rather than duality.
/// A protocol is well-founded if it can be safely implemented without deadlock.
pub trait WellFounded<P> {
    type Output;
}

/// Verification function for well-foundedness
pub fn verify_well_founded<P>()
where
    (): WellFounded<P>,
{
}

/// TChanEnd is trivially well-founded (base case)
impl<C, L, AIO> WellFounded<TChanEnd<C, L, AIO>> for ()
where
    C: ChanId,
    L: MsgLbl,
    AIO: ActionIOTMarker,
{
    type Output = ();
}

/// TChanSend is well-founded if its continuation is well-founded
impl<S, R, C, L, Msg, P, AIO> WellFounded<TChanSend<S, R, C, L, Msg, P, AIO>> for ()
where
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): WellFounded<P>,
{
    type Output = ();
}

/// TChanRecv is well-founded if its continuation is well-founded
impl<R, S, C, L, Msg, P, AIO> WellFounded<TChanRecv<R, S, C, L, Msg, P, AIO>> for ()
where
    R: Role,
    S: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): WellFounded<P>,
{
    type Output = ();
}

/// TChanChoice is well-founded if both branches are well-founded
impl<R, C, Lbl, Left, Right, AIO> WellFounded<TChanChoice<R, C, Lbl, Left, Right, AIO>> for ()
where
    R: Role,
    C: ChanId,
    Lbl: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): WellFounded<Left>,
    (): WellFounded<Right>,
{
    type Output = ();
}

/// TChanOffer is well-founded if both branches are well-founded
impl<R, C, Lbl, Left, Right, AIO> WellFounded<TChanOffer<R, C, Lbl, Left, Right, AIO>> for ()
where
    R: Role,
    C: ChanId,
    Lbl: MsgLbl,
    Left: GlobalProtocol,
    Right: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): WellFounded<Left>,
    (): WellFounded<Right>,
{
    type Output = ();
}

// ============================================================================
// Local Protocol Dual Generation
// ============================================================================

/// Type-level trait for generating duals of local protocols (endpoints)
///
/// This applies to projected endpoint protocols, not global protocols.
/// Each participant's local protocol should be dual to their communication partner's.
pub trait GenerateLocalDual<P> {
    type Output;
}

/// Type alias for generated dual protocols
pub type LocalDual<P> = <() as GenerateLocalDual<P>>::Output;

/// Verification function for local dual generation
pub fn verify_local_dual_generation<P>()
where
    (): GenerateLocalDual<P>,
{
}

/// EpChanSend dualizes to EpChanRecv with swapped roles
impl<IO, M, Msg, P, AIO> GenerateLocalDual<EpChanSend<IO, M, Msg, P, AIO>> for ()
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    AIO: ActionIOTMarker,
    (): GenerateLocalDual<P>,
    LocalDual<P>: LocalProtocol,
{
    type Output = EpChanRecv<IO, M, Msg, LocalDual<P>, AIO>;
}

/// EpChanRecv dualizes to EpChanSend with swapped roles
impl<IO, M, Msg, P, AIO> GenerateLocalDual<EpChanRecv<IO, M, Msg, P, AIO>> for ()
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Msg: Message,
    P: LocalProtocol,
    AIO: ActionIOTMarker,
    (): GenerateLocalDual<P>,
    LocalDual<P>: LocalProtocol,
{
    type Output = EpChanSend<IO, M, Msg, LocalDual<P>, AIO>;
}

/// EpChanChoice dualizes to EpChanOffer with dualized branches
impl<IO, M, Left, Right, AIO> GenerateLocalDual<EpChanChoice<IO, M, Left, Right, AIO>> for ()
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    AIO: ActionIOTMarker,
    (): GenerateLocalDual<Left>,
    (): GenerateLocalDual<Right>,
    LocalDual<Left>: LocalProtocol,
    LocalDual<Right>: LocalProtocol,
{
    type Output = EpChanOffer<IO, M, LocalDual<Left>, LocalDual<Right>, AIO>;
}

/// EpChanOffer dualizes to EpChanChoice with dualized branches
impl<IO, M, Left, Right, AIO> GenerateLocalDual<EpChanOffer<IO, M, Left, Right, AIO>> for ()
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    Left: LocalProtocol,
    Right: LocalProtocol,
    AIO: ActionIOTMarker,
    (): GenerateLocalDual<Left>,
    (): GenerateLocalDual<Right>,
    LocalDual<Left>: LocalProtocol,
    LocalDual<Right>: LocalProtocol,
{
    type Output = EpChanChoice<IO, M, LocalDual<Left>, LocalDual<Right>, AIO>;
}

/// EpChanEnd is self-dual (termination is symmetric)
impl<IO, M, AIO> GenerateLocalDual<EpChanEnd<IO, M, AIO>> for ()
where
    IO: SupportsActionIO<AIO>,
    M: CommMetadataTrait,
    AIO: ActionIOTMarker,
{
    type Output = EpChanEnd<IO, M, AIO>;
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::foundation::{Alice, Bob, HelloMsg};

    type TestIO = TcpOnlySessionIO;
    type TestMetadata = CommMetadata<DefaultChan, RequestLbl>;

    #[test]
    fn test_well_founded_verification() {
        // Test well-foundedness for simple protocol ending
        type End = TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>;
        verify_well_founded::<End>();

        // Test well-foundedness for send followed by end
        type Send = TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, End, BiDirectionalAction>;
        verify_well_founded::<Send>();
    }

    #[test]
    fn test_local_dual_generation() {
        // Test dual generation for termination
        type End = EpChanEnd<TestIO, TestMetadata, BiDirectionalAction>;
        verify_local_dual_generation::<End>();

        // Verify that generated dual satisfies IsDual relationship
        fn _verify_local_dual()
        where
            (): IsDual<End, LocalDual<End>>,
            <() as IsDual<End, LocalDual<End>>>::Output: EqualsTrue,
        {
        }
    }

    #[test]
    fn test_local_send_dual_generation() {
        type End = EpChanEnd<TestIO, TestMetadata, BiDirectionalAction>;
        type Send = EpChanSend<TestIO, TestMetadata, HelloMsg, End, BiDirectionalAction>;

        // Test the verification function
        verify_local_dual_generation::<Send>();
    }

    #[test]
    fn test_local_choice_dual_generation() {
        type End = EpChanEnd<TestIO, TestMetadata, BiDirectionalAction>;
        type Choice = EpChanChoice<TestIO, TestMetadata, End, End, BiDirectionalAction>;

        // Test the verification function
        verify_local_dual_generation::<Choice>();
    }

    // Test function to verify module exports
    pub fn test_module_export() {
        // Simple test function
    }
}
