//! # Protocol System
//!
//! This module provides a type-level approach to designing, verifying,
//! and implementing communication protocols using session types.
//!
//! ## Module Structure
//!
//! - `base`: Foundational types and traits for type-level programming
//! - `global`: Global protocol types representing multi-party choreography
//! - `local`: Local protocol types representing endpoint behavior
//! - `transforms`: Projection and other transformations between protocol representations
//! - `utils`: Utility traits for protocol manipulation and checking
//!
//! ## Key Concepts
//!
//! - **Global Protocols**: Describe the overall choreography between participants
//! - **Local Protocols**: Describe the behavior of a single participant
//! - **Projection**: The process of deriving local protocols from global ones
//! - **Type-Level Operations**: Compile-time reasoning about protocol properties

// Re-export everything from the submodules
pub mod base;
pub mod global;
pub mod local;
pub mod transforms;
pub mod utils;

// Additional test-specific helpers
#[cfg(test)]
pub mod test_helpers;
#[cfg(test)]
pub mod test_overrides;

// Re-export commonly used items at the protocol module level

// From base.rs
pub use self::base::{Cons, Nil, NotInList, NotSame, NotTypeEq, UniqueList, TypeEq};

// From global.rs
pub use self::global::{
    GlobalProtocol,
    TChanContinue,
    TChanOffer,
    TChanPar,
    TChanRec,
    TChanRecv,
    TChanSend,
    TEnd,
    TStart,
    TSession,
    // Deprecated Aliases
    TChoice, TCont, TEndOld, TPar, TRec, TRecv, TSend, TStartOld,
};

// From local.rs
pub use self::local::{
    EpChoice,
    EpContinue,
    EpEnd,
    EpPar,
    EpRec,
    EpRecv,
    EpSend,
    EpSession,
    EpSkip,
    EpStart,
    EpSilent,
    GetEpSkipTypeMarker,
    IsEpEndVariant,
    IsEpSkipTypeImpl,
    IsEpSkipVariant,
    IsEnd,
    IsSkip,
    IsEpSkipType,
    IsNotEpSkipType,
    Role as LocalRole, // Role trait from local.rs
};

// Re-export fundamental types from crate::types that are core to the protocol system
pub use crate::types::{
    ActionIOTMarker, Bool, CommMetadata, False, ProtocolLabel, RoleMarker, SessionType, SupportsActionIO, True, RoleEq, // Added RoleEq here
};

// From transforms.rs (glob export)
pub use self::transforms::*;

// From utils.rs (glob export)
pub use self::utils::*;

// Items like TBroker, TClient, TServer, TWorker, Void, AssertDisjoint, ToTChoice, ToTPar
// are not defined as re-exportable types in global.rs or local.rs in the provided snippets.
// They are likely specific roles, utility types defined elsewhere, or conceptual.
// GlobalTSession, GlobalTStart, GlobalTEnd were not found as specific types but rather TSession, TStart, TEnd.
