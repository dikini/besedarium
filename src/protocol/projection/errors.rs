//! # Protocol Projection Error Handling and Validation
//!
//! This module provides comprehensive error handling and validation for the protocol
//! projection system, ensuring type safety and runtime correctness when projecting
//! global protocols to local endpoint types.
//!
//! ## Core Components
//!
//! - **ProjectionError**: Enumeration of all possible projection failures
//! - **ValidateProjection**: Compile-time validation trait for projection constraints
//! - **ProjectionValidator**: Runtime validation trait for dynamic checks
//! - **DefaultProjectionValidator**: Default implementation with basic validation logic
//!
//! ## Error Categories
//!
//! The projection system handles four main categories of errors:
//!
//! 1. **Role Involvement**: When a role is not part of a protocol step
//! 2. **Invalid Projection**: Type-level projection constraint violations
//! 3. **Action I/O Mismatch**: Capability mismatches between required and actual
//! 4. **Metadata Issues**: Invalid channel or message metadata
//!
//! ## Usage Examples
//!
//! ### Basic Error Handling
//!
//! ```rust
//! use besedarium::protocol::projection::errors::{ProjectionError, ProjectionValidator, DefaultProjectionValidator};
//!
//! // Validate role involvement
//! match DefaultProjectionValidator::validate_role_involvement("Alice", "send_message") {
//!     Ok(()) => println!("Role validation passed"),
//!     Err(ProjectionError::RoleNotInvolved { role, protocol_step }) => {
//!         eprintln!("Role {} not involved in {}", role, protocol_step);
//!     }
//!     Err(e) => eprintln!("Other error: {}", e),
//! }
//! ```
//!
//! ### Compile-time Validation
//!
//! ```rust
//! use besedarium::protocol::projection::errors::ValidateProjection;
//! use besedarium::protocol::foundation::*;
//! use besedarium::protocol::global::*;
//! use besedarium::protocol::projection::helpers::{True, False};
//!
//! // Example compile-time validation (conceptual)
//! fn check_projection<P, R, V>()
//! where
//!     P: GlobalProtocol,
//!     R: Role,
//!     V: ValidateProjection<P, R, IsValid = True>
//! {
//!     // This function only compiles if projection is valid
//! }
//! ```
//!
//! ## Design Philosophy
//!
//! The error system follows these principles:
//!
//! - **Fail Fast**: Catch errors as early as possible (preferably at compile-time)
//! - **Descriptive**: Provide clear, actionable error messages
//! - **Extensible**: Allow custom validation logic through trait implementations
//! - **Composable**: Enable layered validation strategies

use crate::protocol::foundation::{GlobalProtocol, Role};
use std::fmt;

// ============================================================================
// Projection Error Types and Validation
// ============================================================================

/// Comprehensive error type for protocol projection failures
///
/// This enum captures all possible error conditions that can occur during
/// protocol projection, from compile-time type mismatches to runtime
/// validation failures.
///
/// # Error Categories
///
/// ## Role Involvement Errors
///
/// Occur when a role is not properly involved in a protocol step:
///
/// ```rust
/// use besedarium::protocol::projection::errors::ProjectionError;
///
/// let error = ProjectionError::RoleNotInvolved {
///     role: "Alice".to_string(),
///     protocol_step: "choice_selection".to_string(),
/// };
///
/// println!("{}", error); // "Role 'Alice' is not involved in protocol step: choice_selection"
/// ```
///
/// ## Type Constraint Violations
///
/// Occur when projection violates type-level constraints:
///
/// ```rust
/// use besedarium::protocol::projection::errors::ProjectionError;
///
/// let error = ProjectionError::InvalidProjection {
///     reason: "Send action projected to non-sender role".to_string(),
///     protocol_type: "Send<Alice, Bob, Message>".to_string(),
///     target_role: "Charlie".to_string(),
/// };
/// ```
///
/// ## Capability Mismatches
///
/// Occur when role capabilities don't match protocol requirements:
///
/// ```rust
/// use besedarium::protocol::projection::errors::ProjectionError;
///
/// let error = ProjectionError::ActionIOCapabilityMismatch {
///     required_capability: "BiDirectionalAction".to_string(),
///     actual_capability: "UniDirectionalAction".to_string(),
/// };
/// ```
///
/// # Error Propagation
///
/// ProjectionError implements `std::error::Error` and can be used in
/// standard Rust error handling patterns:
///
/// ```rust
/// use besedarium::protocol::projection::errors::ProjectionError;
/// use std::error::Error;
///
/// fn handle_projection_error(err: ProjectionError) -> Result<(), Box<dyn Error>> {
///     match err {
///         ProjectionError::RoleNotInvolved { .. } => {
///             // Handle role involvement error
///             Err(err.into())
///         }
///         ProjectionError::InvalidProjection { .. } => {
///             // Handle invalid projection
///             Err(err.into())
///         }
///         _ => Err(err.into()),
///     }
/// }
/// ```
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

/// Compile-time validation trait for projection type constraints
///
/// This trait provides compile-time validation of projection operations,
/// ensuring that global protocols can be validly projected to specific roles
/// before any runtime execution occurs.
///
/// # Type-Level Safety
///
/// The trait uses associated types to encode validation results at the type level:
///
/// - `IsValid`: Boolean type (`True` or `False`) indicating validation success
/// - `ErrorType`: Specific error type for validation failures
///
/// # Usage Patterns
///
/// ## Compile-Time Constraint Checking
///
/// ```rust
/// use besedarium::protocol::projection::errors::ValidateProjection;
/// use besedarium::protocol::foundation::*;
/// use besedarium::protocol::global::*;
/// use besedarium::protocol::projection::helpers::{True, False};
///
/// // Function that only accepts valid projections
/// fn safe_projection<P, R, V>()
/// where
///     P: GlobalProtocol,
///     R: Role,
///     V: ValidateProjection<P, R, IsValid = True>
/// {
///     // This function only compiles if the projection is valid
///     // The compiler enforces the constraint at compilation time
/// }
/// ```
///
/// ## Custom Validation Logic
///
/// ```rust
/// use besedarium::protocol::projection::errors::ValidateProjection;
/// use besedarium::protocol::foundation::*;
/// use besedarium::protocol::global::*;
/// use besedarium::protocol::projection::helpers::{True, False};
///
/// // Custom validation for specific protocol types
/// # #[derive(Debug)]
/// # struct MyProtocol;
/// # impl GlobalProtocol for MyProtocol {}
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct MyRole;
/// # impl Role for MyRole {}
/// impl ValidateProjection<MyProtocol, MyRole> for () {
///     type IsValid = True;  // This projection is always valid
///     type ErrorType = ();  // No error type needed for valid projections
/// }
/// ```
///
/// # Design Principles
///
/// - **Zero Runtime Cost**: All validation happens at compile-time
/// - **Type Safety**: Invalid projections cannot be constructed
/// - **Extensible**: Custom validation logic through trait implementations
/// - **Composable**: Can be combined with other type-level constraints
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

/// Runtime validation trait for dynamic projection checks
///
/// This trait provides runtime validation capabilities for projection operations
/// that cannot be fully validated at compile-time, such as dynamic role assignment
/// or metadata consistency checks.
///
/// # Validation Categories
///
/// The trait defines three primary validation operations:
///
/// 1. **Role Involvement**: Verify roles are properly involved in protocol steps
/// 2. **Action I/O Capabilities**: Check capability compatibility
/// 3. **Metadata Consistency**: Validate channel and message metadata
///
/// # Usage Patterns
///
/// ## Basic Runtime Validation
///
/// ```rust
/// use besedarium::protocol::projection::errors::{ProjectionValidator, DefaultProjectionValidator};
///
/// // Validate role involvement
/// let result = DefaultProjectionValidator::validate_role_involvement("Alice", "send_message");
/// match result {
///     Ok(()) => println!("Validation passed"),
///     Err(e) => println!("Validation failed: {}", e),
/// }
/// ```
///
/// ## Custom Validator Implementation
///
/// ```rust
/// use besedarium::protocol::projection::errors::{ProjectionValidator, ProjectionError};
///
/// struct StrictValidator;
///
/// impl ProjectionValidator for StrictValidator {
///     fn validate_role_involvement(role: &str, protocol_step: &str) -> Result<(), ProjectionError> {
///         // Implement strict validation logic
///         if role.len() < 3 {
///             return Err(ProjectionError::RoleNotInvolved {
///                 role: role.to_string(),
///                 protocol_step: protocol_step.to_string(),
///             });
///         }
///         Ok(())
///     }
///
///     fn validate_action_io(required: &str, actual: &str) -> Result<(), ProjectionError> {
///         // Implement strict capability checking
///         if !actual.contains(required) {
///             return Err(ProjectionError::ActionIOCapabilityMismatch {
///                 required_capability: required.to_string(),
///                 actual_capability: actual.to_string(),
///             });
///         }
///         Ok(())
///     }
///
///     fn validate_metadata(description: &str) -> Result<(), ProjectionError> {
///         // Implement metadata validation
///         if description.is_empty() {
///             return Err(ProjectionError::InvalidMetadata {
///                 description: "Empty metadata description".to_string(),
///             });
///         }
///         Ok(())
///     }
/// }
/// ```
///
/// # Design Philosophy
///
/// - **Composable**: Multiple validators can be chained together
/// - **Extensible**: Custom validation logic through trait implementation
/// - **Error Transparent**: Clear error propagation through Result types
/// - **Performance Conscious**: Minimal overhead for validation operations
pub trait ProjectionValidator {
    /// Validate that a role is appropriately involved in a protocol step
    fn validate_role_involvement(role: &str, protocol_step: &str) -> Result<(), ProjectionError>;

    /// Validate action I/O capabilities
    fn validate_action_io(required: &str, actual: &str) -> Result<(), ProjectionError>;

    /// Validate metadata consistency
    fn validate_metadata(description: &str) -> Result<(), ProjectionError>;
}

/// Default implementation of the ProjectionValidator trait
///
/// Provides basic validation logic for common projection scenarios. This
/// implementation serves as a foundation for more sophisticated validation
/// strategies and can be extended or replaced with custom logic.
///
/// # Validation Strategy
///
/// The default validator implements conservative validation rules:
///
/// - **Role Involvement**: Rejects empty role names
/// - **Action I/O**: Requires exact capability string matching
/// - **Metadata**: Rejects descriptions containing "invalid"
///
/// # Usage Examples
///
/// ## Basic Validation
///
/// ```rust
/// use besedarium::protocol::projection::errors::{DefaultProjectionValidator, ProjectionValidator};
///
/// // Validate a role
/// let result = DefaultProjectionValidator::validate_role_involvement("Alice", "send_step");
/// assert!(result.is_ok());
///
/// // This would fail
/// let result = DefaultProjectionValidator::validate_role_involvement("", "send_step");
/// assert!(result.is_err());
/// ```
///
/// ## Capability Validation
///
/// ```rust
/// use besedarium::protocol::projection::errors::{DefaultProjectionValidator, ProjectionValidator};
///
/// // Exact match required
/// let result = DefaultProjectionValidator::validate_action_io("BiDirectional", "BiDirectional");
/// assert!(result.is_ok());
///
/// // Mismatch fails
/// let result = DefaultProjectionValidator::validate_action_io("BiDirectional", "UniDirectional");
/// assert!(result.is_err());
/// ```
///
/// ## Metadata Validation
///
/// ```rust
/// use besedarium::protocol::projection::errors::{DefaultProjectionValidator, ProjectionValidator};
///
/// // Valid metadata
/// let result = DefaultProjectionValidator::validate_metadata("channel_metadata");
/// assert!(result.is_ok());
///
/// // Invalid metadata
/// let result = DefaultProjectionValidator::validate_metadata("invalid metadata");
/// assert!(result.is_err());
/// ```
///
/// # Extension Points
///
/// Custom validators can build upon or replace this implementation:
///
/// ```rust
/// use besedarium::protocol::projection::errors::{DefaultProjectionValidator, ProjectionValidator, ProjectionError};
///
/// struct EnhancedValidator;
///
/// impl ProjectionValidator for EnhancedValidator {
///     fn validate_role_involvement(role: &str, protocol_step: &str) -> Result<(), ProjectionError> {
///         // First apply default validation
///         DefaultProjectionValidator::validate_role_involvement(role, protocol_step)?;
///         
///         // Then add custom logic
///         if role.starts_with("_") {
///             return Err(ProjectionError::RoleNotInvolved {
///                 role: role.to_string(),
///                 protocol_step: protocol_step.to_string(),
///             });
///         }
///         
///         Ok(())
///     }
///
///     fn validate_action_io(required: &str, actual: &str) -> Result<(), ProjectionError> {
///         DefaultProjectionValidator::validate_action_io(required, actual)
///     }
///
///     fn validate_metadata(description: &str) -> Result<(), ProjectionError> {
///         DefaultProjectionValidator::validate_metadata(description)
///     }
/// }
/// ```
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
