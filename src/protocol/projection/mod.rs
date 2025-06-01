//! # Protocol Projection Module
//!
//! This module implements the `Project<P, R>` trait for projecting Global Protocols
//! to Local Endpoint Types. The implementation follows the foundation established
//! in Tasks 1.1.1-1.1.4 and provides compile-time projection with role-based dispatch.
//!
//! ## Core Concepts
//!
//! - **Project trait**: Maps global protocols to local endpoint types for specific roles
//! - **Role-based projection**: Different roles see different views of the same protocol
//! - **Type-level validation**: Compile-time verification of projection correctness
//! - **Helper traits**: Modular dispatch system for complex projection cases
//!
//! ## Module Navigation
//!
//! This projection system integrates with other modules:
//!
//! - **[`crate::protocol::foundation`]**: Provides the `Role` and `GlobalProtocol` traits
//! - **[`crate::protocol::global`]**: Source protocols for projection operations
//! - **[`crate::protocol::local`]**: Target endpoint types produced by projection
//! - **[`crate::protocol::duality`]**: Validates projected protocols are proper duals
//! - **[`errors`]**: Comprehensive error handling and validation framework
//! - **[`helpers`]**: Boolean logic and role-based dispatch utilities
//!
//! ## Integration Test Examples
//!
//! For complete working projection examples, see:
//! - `tests/client_server_integration.rs::test_protocol_projection`
//! - `tests/client_server_integration.rs::test_login_protocol_compilation`
//! - `tests/integration_common.rs` - Standard projection patterns and utilities
//!
//! ## Usage
//!
//! ```rust
//! use besedarium::protocol::projection::{Project, ProjectOutput};
//! use besedarium::protocol::foundation::*;
//! use besedarium::protocol::global::*;
//! use besedarium::protocol::local::*;
//!
//! // Example: Alice sends to Bob, project to Alice's view  
//! # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
//! # struct Alice;
//! # impl Role for Alice {}
//! # impl SupportsActionIO<BiDirectionalAction> for Alice {}
//! # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
//! # struct Bob;
//! # impl Role for Bob {}
//! # #[derive(Debug, Clone)]
//! # struct HelloMsg;
//! # impl Message for HelloMsg {}
//! type SendProto = TChanSend<Alice, Bob, DefaultChan, RequestLbl, HelloMsg, TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>, BiDirectionalAction>;
//! type AliceView = ProjectOutput<SendProto, Alice>;  // Results in EpChanSend
//! ```

use crate::protocol::foundation::{GlobalProtocol, LocalProtocol, Role};

// ============================================================================
// Module Structure
// ============================================================================

pub mod errors;
pub mod helpers;
pub mod implementations;

// Re-export essential types for external use
pub use errors::{
    DefaultProjectionValidator, ProjectionError, ProjectionValidator, ValidateProjection,
};
pub use helpers::{Bool, False, ProjectRecvCase, ProjectSendCase, RoleEq, True};

// ============================================================================
// Core Projection Trait
// ============================================================================

// ============================================================================
// Core Projection Trait
// ============================================================================

/// Core projection trait for mapping global protocols to local endpoint types
///
/// Projects a Global Protocol `P` to a Local Endpoint Type for role `R`.
/// This trait provides compile-time protocol projection with validation.
///
/// The projection system implements the formal projection rules from session type theory,
/// transforming global protocol descriptions into local endpoint behaviors that each
/// participant should follow. This enables type-safe distributed communication where
/// each role has a clear, statically-verified view of their responsibilities.
///
/// # Type Parameters
///
/// * `P` - The global protocol to project, must implement `GlobalProtocol`
/// * `R` - The role for which to generate the local view, must implement `Role`
///
/// # Projection Rules
///
/// The implementation follows these core projection rules:
///
/// 1. **Send Operations**: The sender role gets `EpChanSend`, receiver gets `EpChanRecv`, others get continuation
/// 2. **Choice Operations**: The chooser role gets `EpChanChoice`, others get `EpChanOffer`  
/// 3. **Parallel Operations**: All roles get `EpChanPar` with projected sub-protocols
/// 4. **End Operations**: All roles get `EpChanEnd` for protocol termination
/// 5. **Start Operations**: All roles get `EpChanStart` with projected continuation
///
/// # Examples
///
/// ## Basic Send/Receive Projection
///
/// ```rust
/// use besedarium::protocol::projection::{Project, ProjectOutput};
/// use besedarium::protocol::foundation::*;
/// use besedarium::protocol::global::*;
/// use besedarium::protocol::local::*;
///
/// // Define roles and message
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Alice;
/// # impl Role for Alice {}
/// # impl SupportsActionIO<BiDirectionalAction> for Alice {}
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Bob;
/// # impl Role for Bob {}
/// # impl SupportsActionIO<BiDirectionalAction> for Bob {}
/// # #[derive(Debug, Clone)]
/// # struct LoginRequest;
/// # impl Message for LoginRequest {}
///
/// // Global protocol: Alice sends login request to Bob, then protocol ends
/// type LoginProtocol = TChanSend<
///     Alice, Bob, DefaultChan, RequestLbl, LoginRequest,
///     TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,
///     BiDirectionalAction
/// >;
///
/// // Project to Alice's view - she sends the request
/// type AliceView = ProjectOutput<LoginProtocol, Alice>;
/// // Results in: EpChanSend<Alice, CommMetadata<DefaultChan, RequestLbl>, LoginRequest, EpChanEnd<...>, BiDirectionalAction>
///
/// // Project to Bob's view - he receives the request  
/// type BobView = ProjectOutput<LoginProtocol, Bob>;
/// // Results in: EpChanRecv<Bob, CommMetadata<DefaultChan, RequestLbl>, LoginRequest, EpChanEnd<...>, BiDirectionalAction>
/// ```
///
/// ## Choice/Offer Projection
///
/// ```rust
/// # use besedarium::protocol::projection::{Project, ProjectOutput};
/// # use besedarium::protocol::foundation::*;
/// # use besedarium::protocol::global::*;
/// # use besedarium::protocol::local::*;
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Client;
/// # impl Role for Client {}
/// # impl SupportsActionIO<BiDirectionalAction> for Client {}
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Server;
/// # impl Role for Server {}
/// # impl SupportsActionIO<BiDirectionalAction> for Server {}
///
/// // Global protocol: Client makes a choice between two paths
/// type ChoiceProtocol = TChanChoice<
///     Client, DefaultChan, RequestLbl,
///     TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,  // Left choice
///     TChanEnd<DefaultChan, ResponseLbl, BiDirectionalAction>, // Right choice
///     BiDirectionalAction
/// >;
///
/// // Client gets the choice (can select which path to take)
/// type ClientView = ProjectOutput<ChoiceProtocol, Client>;
/// // Results in: EpChanChoice<Client, CommMetadata<DefaultChan, RequestLbl>, ...>
///
/// // Server gets the offer (must handle whichever choice Client makes)
/// type ServerView = ProjectOutput<ChoiceProtocol, Server>;
/// // Results in: EpChanOffer<Server, CommMetadata<DefaultChan, RequestLbl>, ...>
/// ```
///
/// # Role-Based Dispatch
///
/// The projection system uses type-level role comparison via the `RoleEq` trait to determine
/// which projection rule to apply. For each protocol construct, roles are categorized as:
///
/// - **Participating roles**: Directly involved in the communication (sender/receiver/chooser)
/// - **Non-participating roles**: Not directly involved, see continuation or skipped behavior
///
/// # Implementation Strategy
///
/// Projection implementations use helper traits like `ProjectSendCase` and `ProjectRecvCase`
/// to dispatch based on Boolean type-level comparisons. This allows the same trait to handle
/// multiple scenarios while maintaining type safety.
///
/// # See Also
///
/// - [`ProjectOutput`] - Type alias for cleaner projection usage
/// - [`RoleEq`] - Helper trait for role-based dispatch
/// - [`ProjectSendCase`] - Helper trait for send operation projection
/// - [`ProjectRecvCase`] - Helper trait for receive operation projection
pub trait Project<P, R>
where
    P: GlobalProtocol,
    R: Role,
{
    /// The resulting local protocol for role R
    ///
    /// This associated type represents the local endpoint view that role `R` should
    /// follow when participating in the global protocol `P`. The projected protocol
    /// maintains all necessary type information for compile-time validation while
    /// providing a clear, role-specific behavioral specification.
    type Output: LocalProtocol;
}

/// Type alias for cleaner projection usage
///
/// This type alias provides a more convenient way to access the projected protocol type
/// without needing to explicitly invoke the trait. It's equivalent to writing
/// `<() as Project<P, R>>::Output` but much more readable.
///
/// # Examples
///
/// ```rust
/// use besedarium::protocol::projection::ProjectOutput;
/// use besedarium::protocol::foundation::*;
/// use besedarium::protocol::global::*;
///
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Alice;
/// # impl Role for Alice {}
/// # impl SupportsActionIO<BiDirectionalAction> for Alice {}
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct Bob;
/// # impl Role for Bob {}
/// # impl SupportsActionIO<BiDirectionalAction> for Bob {}
/// # #[derive(Debug, Clone)]
/// # struct Message1;
/// # impl Message for Message1 {}
///
/// // Using ProjectOutput for cleaner type definitions
/// type SimpleProtocol = TChanSend<
///     Alice, Bob, DefaultChan, RequestLbl, Message1,
///     TChanEnd<DefaultChan, RequestLbl, BiDirectionalAction>,
///     BiDirectionalAction
/// >;
///
/// // Clean, readable projection
/// type AliceEndpoint = ProjectOutput<SimpleProtocol, Alice>;
/// type BobEndpoint = ProjectOutput<SimpleProtocol, Bob>;
/// ```
pub type ProjectOutput<P, R> = <() as Project<P, R>>::Output;

// ============================================================================
// Non-Reflexive RoleEq Implementations (Required for Boolean Logic)
// ============================================================================

// Note: Non-reflexive RoleEq implementations need to be added for specific
// role pairs as needed. For now, the reflexive case (R == R -> True) is handled
// by the blanket implementation above.

// Example implementations would look like:
// impl RoleEq<Bob> for Alice { type Output = False; }
// impl RoleEq<Alice> for Bob { type Output = False; }

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests;
