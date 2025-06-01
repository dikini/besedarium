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

use crate::types::Bool;

// Module declarations
pub mod generation;
mod global_impl;
mod helpers;
mod local_impl;
pub mod macros;

// Re-export helper traits for external use
pub use helpers::{DualityCheck, EqualsFalse, EqualsTrue};

// Re-export dual generation functionality
pub use generation::{
    verify_local_dual_generation, verify_well_founded, GenerateLocalDual, LocalDual, WellFounded,
};

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
// Tests Module
// ============================================================================

#[cfg(test)]
mod tests;
