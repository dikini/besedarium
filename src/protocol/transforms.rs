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

use crate::sealed;
use crate::types;
use core::marker::PhantomData;
use super::base::*;
use super::global::*;
use super::local::*;

// Contents will be migrated from protocol.rs
// during the code migration phase