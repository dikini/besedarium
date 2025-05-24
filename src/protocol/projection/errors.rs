//! Error types and validation logic for protocol projection
//!
//! This module contains all error types, validation traits, and validation
//! implementations used by the projection system.

use crate::protocol::foundation::{GlobalProtocol, Role};
use std::fmt;

// ============================================================================
// Projection Error Types and Validation
// ============================================================================

/// Error type for projection validation and runtime failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionError {
    /// Role not involved in the protocol step
    RoleNotInvolved { role: String, protocol_step: String },
    /// Invalid projection due to type constraints
    InvalidProjection {
        reason: String,
        protocol_type: String,
        target_role: String,
    },
    /// Action I/O capability mismatch
    ActionIOCapabilityMismatch {
        required_capability: String,
        actual_capability: String,
    },
    /// Invalid channel or message metadata
    InvalidMetadata { description: String },
}

impl fmt::Display for ProjectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectionError::RoleNotInvolved {
                role,
                protocol_step,
            } => {
                write!(
                    f,
                    "Role '{}' is not involved in protocol step: {}",
                    role, protocol_step
                )
            }
            ProjectionError::InvalidProjection {
                reason,
                protocol_type,
                target_role,
            } => {
                write!(
                    f,
                    "Invalid projection of '{}' to role '{}': {}",
                    protocol_type, target_role, reason
                )
            }
            ProjectionError::ActionIOCapabilityMismatch {
                required_capability,
                actual_capability,
            } => {
                write!(
                    f,
                    "Action I/O capability mismatch: required '{}', found '{}'",
                    required_capability, actual_capability
                )
            }
            ProjectionError::InvalidMetadata { description } => {
                write!(f, "Invalid metadata: {}", description)
            }
        }
    }
}

impl std::error::Error for ProjectionError {}

/// Trait for validating projection constraints at compile-time
pub trait ValidateProjection<P, R>
where
    P: GlobalProtocol,
    R: Role,
{
    /// Type-level validation result (True if valid, False if invalid)
    type IsValid: crate::protocol::projection::helpers::Bool;

    /// Validation error message (if any)
    type ErrorType: Send + Sync + 'static;
}

/// Helper trait for runtime projection validation
pub trait ProjectionValidator {
    /// Validate that a role is appropriately involved in a protocol step
    fn validate_role_involvement(role: &str, protocol_step: &str) -> Result<(), ProjectionError>;

    /// Validate action I/O capabilities
    fn validate_action_io(required: &str, actual: &str) -> Result<(), ProjectionError>;

    /// Validate metadata consistency
    fn validate_metadata(description: &str) -> Result<(), ProjectionError>;
}

/// Default implementation of projection validation
pub struct DefaultProjectionValidator;

impl ProjectionValidator for DefaultProjectionValidator {
    fn validate_role_involvement(role: &str, protocol_step: &str) -> Result<(), ProjectionError> {
        // Basic validation - can be extended with more sophisticated checks
        if role.is_empty() {
            return Err(ProjectionError::RoleNotInvolved {
                role: role.to_string(),
                protocol_step: protocol_step.to_string(),
            });
        }
        Ok(())
    }

    fn validate_action_io(required: &str, actual: &str) -> Result<(), ProjectionError> {
        if required != actual {
            return Err(ProjectionError::ActionIOCapabilityMismatch {
                required_capability: required.to_string(),
                actual_capability: actual.to_string(),
            });
        }
        Ok(())
    }

    fn validate_metadata(description: &str) -> Result<(), ProjectionError> {
        if description.contains("invalid") {
            return Err(ProjectionError::InvalidMetadata {
                description: description.to_string(),
            });
        }
        Ok(())
    }
}
