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

// Re-export commonly used items at the protocol module level
pub use self::base::{Cons, Nil, NotInList, NotSame, NotTypeEq, UniqueList};
pub use self::global::{
    AssertDisjoint, TChoice, TEnd, TInteract, TPar, TRec, TSession, ToTChoice, ToTPar,
};
pub use self::local::{
    EpChoice, EpEnd, EpPar, EpRecv, EpSend, EpSession, EpSkip, GetEpSkipTypeMarker, IsEnd,
    IsEpEndVariant, IsEpSkipTypeImpl, IsEpSkipVariant, IsSkip, Role, RoleEq, TBroker, TClient,
    TServer, TWorker, Void,
};
pub use self::transforms::{
    ComposeProjectedParBranches, ComposeProjectedParBranchesCase, ContainsRole, FilterSkips,
    FilterSkipsCase, NotContainsRole, ProjectChoice, ProjectChoiceCase, ProjectInteract,
    ProjectPar, ProjectParBranch, ProjectRole, TParContainsRoleImpl,
};
pub use self::utils::{
    CheckNil, Concat, ConcatCons, Disjoint, DisjointCons, IsEmpty, IsNil, IsNotNil,
};
