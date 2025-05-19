//! Protocol Transformations Module Root
//!
//! Re-exports all public traits and helpers for protocol transformations.

pub mod choice;
pub mod end;
pub mod parallel;
pub mod projection;
pub mod recursion;
pub mod recv;
pub mod send;
pub mod start;
pub mod util;

pub use choice::*;
pub use end::*;
pub use parallel::*;
pub use projection::ProjectRole;
pub use recv::ProjectRecvCase;
pub use send::ProjectSendCase;
pub use start::ProjectStartCase;
pub use util::GetProtocolLabel;
pub use util::*;
