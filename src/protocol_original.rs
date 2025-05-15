// This file now re-exports from the modular protocol directory structure.
// It serves as a compatibility layer while transitioning to the new structure.

// Re-export everything from the new modules
pub use self::protocol::*;

// Internal module that contains the actual implementation
mod protocol {
    // Re-export everything from the submodules
    pub use super::protocol::base::*;
    pub use super::protocol::global::*;
    pub use super::protocol::local::*;
    pub use super::protocol::transforms::*;
    pub use super::protocol::utils::*;
}
