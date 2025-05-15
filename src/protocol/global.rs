//! # Global Protocol Types
//!
//! This module contains the global protocol session types that model
//! multi-party communication protocols. These types represent the
//! choreography-level view of interactions between participants.
//!
//! Key components:
//!
//! - `TSession`: Core trait for all global session type combinators
//! - `TEnd`: Protocol termination
//! - `TInteract`: Individual interaction between roles
//! - `TChoice`: Binary protocol choice
//! - `TPar`: Parallel protocol composition
//! - `TRec`: Recursive protocol definition
//!
//! Global protocols are designed to be projected onto specific roles to
//! produce local (endpoint) protocols that describe the behavior of
//! individual participants.

use crate::sealed;
use crate::types;
use core::marker::PhantomData;
use super::base::*;

// Contents will be migrated from protocol.rs
// during the code migration phase