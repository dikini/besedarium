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
//! ## Module Navigation
//!
//! This duality system integrates with other modules:
//!
//! - **[`crate::protocol::foundation`]**: Provides base protocol and role traits
//! - **[`crate::protocol::global`]**: Global protocols that participate in duality
//! - **[`crate::protocol::local`]**: Local endpoints that must be dual to each other
//! - **[`crate::protocol::projection`]**: Projection system that preserves duality
//! - **[`helpers`]**: Boolean logic utilities for type-level duality computation
//! - **[`macros`]**: Compile-time assertion macros for duality verification
//!
//! ## Integration Test Examples
//!
//! For complete working duality examples, see:
//! - `tests/client_server_integration.rs::test_protocol_duality`
//! - `tests/client_server_integration.rs::test_comprehensive_protocol_system`
//! - `tests/integration_common.rs` - Standard dual protocol patterns
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
/// This trait provides compile-time verification that two protocol types
/// satisfy the duality relationship required for safe multiparty session type
/// communication. Duality ensures that complementary endpoints can communicate
/// correctly - when one endpoint sends, the dual endpoint receives, and vice versa.
///
/// # Type Parameters
///
/// - `P`: First protocol type to check
/// - `Q`: Second protocol type to check
///
/// # Associated Types
///
/// - `Output`: A [`Bool`] type indicating the duality result
///   - [`True`] if `P` and `Q` are duals of each other
///   - [`False`] if `P` and `Q` are not duals
///
/// # Duality Rules
///
/// The trait implements the formal duality rules from session type theory:
///
/// ## Basic Operations
/// ```text
/// IsDual(End, End) = True
/// IsDual(TSend<S,R,C,L,Msg,P>, TRecv<R,S,C,L,Msg,Q>) = IsDual(P, Q)
/// IsDual(TRecv<R,S,C,L,Msg,P>, TSend<S,R,C,L,Msg,Q>) = IsDual(P, Q)
/// ```
///
/// ## Choice and Offer
/// ```text
/// IsDual(TChoice<R,Branches>, TOffer<R,DualBranches>) = All branches dual
/// IsDual(TOffer<R,Branches>, TChoice<R,DualBranches>) = All branches dual
/// ```
///
/// ## Parallel Composition
/// ```text
/// IsDual(TPar<P,Q>, TPar<P',Q')) = IsDual(P,P') ∧ IsDual(Q,Q')
/// ```
///
/// # Examples
///
/// ## Basic Send/Receive Duality
///
/// ```rust
/// use besedarium::protocol::duality::{IsDual, IsDualOutput};
/// use besedarium::types::{True, False};
/// use besedarium::protocol::foundation::Role;
///
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Alice;
/// # impl Role for Alice {}
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Bob;
/// # impl Role for Bob {}
///
/// // Send and Receive with swapped roles are dual
/// // type SendRecvDual = IsDualOutput<
/// //     TSend<Alice, Bob, Chan1, Lbl1, String, End>,
/// //     TRecv<Bob, Alice, Chan1, Lbl1, String, End>
/// // >;  // True
/// ```
///
/// ## Compound Protocol Duality
///
/// ```rust
/// use besedarium::protocol::duality::IsDual;
/// use besedarium::protocol::foundation::Role;
///
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Alice;
/// # impl Role for Alice {}
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Bob;
/// # impl Role for Bob {}
///
/// // Choice and Offer are dual when their branches are dual
/// // type ChoiceOfferDual = IsDualOutput<
/// //     TChoice<Alice, BranchesA>,
/// //     TOffer<Alice, BranchesB>
/// // >;  // True if all corresponding branches are dual
/// ```
///
/// ## Using Duality in Constraints
///
/// ```rust
/// use besedarium::protocol::duality::{IsDual, EqualsTrue};
/// use besedarium::protocol::foundation::LocalProtocol;
///
/// // Function that requires two protocols to be dual
/// fn establish_session<P, Q>()
/// where
///     P: LocalProtocol,
///     Q: LocalProtocol,
///     (): IsDual<P, Q>,
///     <() as IsDual<P, Q>>::Output: EqualsTrue,
/// {
///     // Safe to establish session - protocols are verified as dual
/// }
/// ```
///
/// # Implementation Strategy
///
/// The trait uses recursive type-level computation to check duality:
///
/// 1. **Base Cases**: Handle `End`, `Start`, and atomic operations
/// 2. **Recursive Cases**: For compound types, check component duality
/// 3. **Symmetry**: Ensure `IsDual<P,Q>` ⟺ `IsDual<Q,P>`
/// 4. **Reflexivity**: Handle same-type comparisons appropriately
///
/// # Compile-Time Verification
///
/// Duality checking happens entirely at compile time, providing zero-runtime-cost
/// verification of protocol correctness. Failed duality checks result in compilation
/// errors, preventing incorrect protocol compositions.
///
/// # See Also
///
/// - [`EqualsTrue`] / [`EqualsTrue`] - Helper traits for boolean constraints
/// - [`GenerateLocalDual`] - Automatic dual generation
/// - [`assert_dual!`] - Macro for compile-time duality assertions
/// - [`Bool`] - Type-level boolean system
pub trait IsDual<P, Q> {
    type Output: Bool;
}

/// Helper type alias for cleaner usage of duality checking
///
/// This type alias provides a more convenient syntax for accessing the result
/// of duality checking between two protocol types.
///
/// # Type Parameters
///
/// - `P`: First protocol type
/// - `Q`: Second protocol type
///
/// # Returns
///
/// A [`Bool`] type indicating whether `P` and `Q` are dual:
/// - [`True`] if the protocols are dual
/// - [`False`] if the protocols are not dual
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use besedarium::protocol::duality::IsDualOutput;
/// use besedarium::types::{True, False};
///
/// // More concise syntax compared to full trait usage
/// // type Result = IsDualOutput<SendProtocol, RecvProtocol>;
/// //
/// // Equivalent to:
/// // type Result = <() as IsDual<SendProtocol, RecvProtocol>>::Output;
/// ```
///
/// ## In Type Constraints
///
/// ```rust
/// use besedarium::protocol::duality::{IsDualOutput, EqualsTrue};
/// use besedarium::protocol::foundation::GlobalProtocol;
///
/// // Using the type alias in bounds - example shows the concept
/// fn session_with_dual<P, Q>()
/// where
///     P: GlobalProtocol,
///     Q: GlobalProtocol,
///     // In practice, specific implementations of IsDual would be provided
///     // This is a conceptual example of how the constraint would look
/// {
///     // This function demonstrates type-level dual checking
///     // In real usage, concrete protocol types with IsDual impls would be used
/// }
/// ```
///
/// # See Also
///
/// - [`IsDual`] - The underlying trait this alias wraps
/// - [`EqualsTrue`] / [`EqualsFalse`] - Helper traits for boolean constraints
pub type IsDualOutput<P, Q> = <() as IsDual<P, Q>>::Output;

// ============================================================================
// Tests Module
// ============================================================================

#[cfg(test)]
mod tests;
