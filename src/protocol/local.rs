//! # Local Protocol Types
//!
//! This module contains the local protocol session types that model
//! the behavior of individual participants in a communication protocol.
//! These types represent the endpoint-level view of interactions.
//!
//! Key components:
//!
//! - `EpSession`: Core trait for all local session types
//! - `EpSend`: Endpoint sending operation
//! - `EpRecv`: Endpoint receiving operation
//! - `EpChoice`: Endpoint protocol choice
//! - `EpPar`: Endpoint parallel composition
//! - `EpEnd`: Endpoint protocol termination
//! - `EpSkip`: No-op type for roles not involved in a branch
//!
//! Local protocols are derived from global protocols through projection
//! onto specific roles. They describe the sequence of operations that
//! an individual participant must perform.

use crate::sealed;
use crate::types;
use core::marker::PhantomData;
use super::base::*;
use super::global::*;

// Contents will be migrated from protocol.rs
// during the code migration phase