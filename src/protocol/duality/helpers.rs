//! Helper traits for type-level boolean constraints in duality checking
//!
//! This module provides supporting traits for the duality checking system,
//! particularly for type-level boolean operations and constraints.

use crate::protocol::foundation::GlobalProtocol;
use crate::types::{False, True};

/// Helper trait to ensure a type-level boolean is True
///
/// This trait is implemented only for the [`True`] type, enabling compile-time
/// constraints that require a boolean condition to be provably true. It's used
/// extensively in duality checking to enforce protocol correctness.
///
/// # Usage Pattern
///
/// The trait is typically used in `where` clauses to constrain type-level
/// boolean results to be `True`:
///
/// ```rust
/// use besedarium::protocol::duality::{IsDual, EqualsTrue};
/// use besedarium::protocol::foundation::{GlobalProtocol, LocalProtocol};
///
/// // Function that requires two protocols to be dual
/// fn establish_dual_session<P, Q>()
/// where
///     P: LocalProtocol,
///     Q: LocalProtocol,
///     (): IsDual<P, Q>,
///     <() as IsDual<P, Q>>::Output: EqualsTrue,  // Constraint: must be True
/// {
///     // Safe to proceed - duality is verified at compile time
/// }
/// ```
///
/// # Compile-Time Safety
///
/// By constraining boolean results with `EqualsTrue`, we move protocol
/// verification from runtime to compile time. If the duality check fails,
/// the code will not compile.
///
/// # Examples
///
/// ## Valid Constraint
///
/// ```rust
/// use besedarium::protocol::duality::EqualsTrue;
/// use besedarium::types::True;
///
/// // This compiles because True implements EqualsTrue
/// fn valid_constraint<B: EqualsTrue>() {}
/// let _valid = valid_constraint::<True>();
/// ```
///
/// ## Invalid Constraint (Won't Compile)
///
/// ```rust,compile_fail
/// use besedarium::protocol::duality::EqualsTrue;
/// use besedarium::types::False;
///
/// // This would fail to compile because False doesn't implement EqualsTrue
/// fn invalid_constraint<B: EqualsTrue>() {}
/// let _invalid = invalid_constraint::<False>();  // Compilation error
/// ```
///
/// # See Also
///
/// - [`EqualsFalse`] - Complementary trait for False constraints
/// - [`IsDual`] - Primary trait that produces boolean results
/// - [`True`] - The only type that implements this trait
pub trait EqualsTrue {}
impl EqualsTrue for True {}

/// Helper trait to ensure a type-level boolean is False
///
/// This trait is implemented only for the [`False`] type, enabling compile-time
/// constraints that require a boolean condition to be provably false. It provides
/// the complement to [`EqualsTrue`] for negative assertions in the type system.
///
/// # Usage Pattern
///
/// The trait is typically used to enforce that certain protocol relationships
/// do NOT hold:
///
/// ```rust
/// use besedarium::protocol::duality::{IsDual, EqualsFalse};
/// use besedarium::protocol::foundation::LocalProtocol;
///
/// // Function that requires two protocols to NOT be dual
/// fn incompatible_protocols<P, Q>()
/// where
///     P: LocalProtocol,
///     Q: LocalProtocol,
///     (): IsDual<P, Q>,
///     <() as IsDual<P, Q>>::Output: EqualsFalse,  // Constraint: must be False
/// {
///     // These protocols are guaranteed to be non-dual
/// }
/// ```
///
/// # Use Cases
///
/// While less common than [`EqualsTrue`], this trait is useful for:
///
/// - **Negative Verification**: Ensuring protocols are intentionally incompatible
/// - **Testing**: Verifying that invalid protocol combinations are rejected
/// - **Safety Checks**: Preventing accidental dual relationships
///
/// # Examples
///
/// ## Valid Constraint
///
/// ```rust
/// use besedarium::protocol::duality::EqualsFalse;
/// use besedarium::types::False;
///
/// // This compiles because False implements EqualsFalse
/// fn valid_constraint<B: EqualsFalse>() {}
/// let _valid = valid_constraint::<False>();
/// ```
///
/// ## Invalid Constraint (Won't Compile)
///
/// ```rust,compile_fail
/// use besedarium::protocol::duality::EqualsFalse;
/// use besedarium::types::True;
///
/// // This would fail to compile because True doesn't implement EqualsFalse
/// fn invalid_constraint<B: EqualsFalse>() {}
/// let _invalid = invalid_constraint::<True>();  // Compilation error
/// ```
///
/// # See Also
///
/// - [`EqualsTrue`] - Complementary trait for True constraints
/// - [`IsDual`] - Primary trait that produces boolean results
/// - [`False`] - The only type that implements this trait
pub trait EqualsFalse {}
impl EqualsFalse for False {}

/// Marker trait for types that can participate in duality checking
///
/// This trait serves as a safety mechanism and organizational tool to ensure
/// only appropriate protocol types participate in duality relationships. It
/// provides a clear boundary around which types are considered valid for
/// duality analysis.
///
/// # Purpose
///
/// 1. **Type Safety**: Prevents inappropriate types from being used in duality checks
/// 2. **Documentation**: Makes explicit which types support duality
/// 3. **Future Extension**: Provides a hook for adding duality-specific methods
/// 4. **Trait Bounds**: Enables clean generic constraints
///
/// # Implementation Strategy
///
/// The trait is implemented as a blanket implementation for all [`GlobalProtocol`]
/// types, ensuring that any valid global protocol can participate in duality
/// checking without manual implementation.
///
/// # Examples
///
/// ## Automatic Implementation
///
/// ```rust
/// use besedarium::protocol::duality::DualityCheck;
/// use besedarium::protocol::foundation::GlobalProtocol;
///
/// # #[derive(Debug)]
/// # struct MyProtocol;
/// # impl GlobalProtocol for MyProtocol {}
///
/// // All GlobalProtocol types automatically implement DualityCheck
/// fn verify_duality_support<P: DualityCheck>() {}
/// verify_duality_support::<MyProtocol>();  // Works automatically
/// ```
///
/// ## In Generic Constraints
///
/// ```rust
/// use besedarium::protocol::duality::{DualityCheck, IsDual};
///
/// // Function that works with any duality-compatible types
/// fn process_protocols<P, Q>()
/// where
///     P: DualityCheck,
///     Q: DualityCheck,
///     (): IsDual<P, Q>,
/// {
///     // Both types are verified as duality-capable
/// }
/// ```
///
/// # Design Rationale
///
/// Rather than implementing this trait individually for each protocol type,
/// the blanket implementation approach ensures:
///
/// - **Consistency**: All global protocols automatically support duality
/// - **Maintainability**: No need to manually implement for new protocol types
/// - **Safety**: The trait bounds of [`GlobalProtocol`] ensure appropriate types
///
/// # See Also
///
/// - [`GlobalProtocol`] - The types that automatically implement this trait
/// - [`IsDual`] - The primary duality checking trait
/// - [`EqualsTrue`] / [`EqualsFalse`] - Boolean constraint helpers
pub trait DualityCheck: Send + Sync + 'static {}

// Implement for all protocol types
// Note: Only implementing for GlobalProtocol to avoid conflicts
impl<T: GlobalProtocol> DualityCheck for T {}
