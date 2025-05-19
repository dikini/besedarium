//! Contains all choice projection traits and helpers.
//!
//! # Example
//!
//! ```rust
//! use besedarium::ProjectChoice;
//! // ...
//! ```
//!
//! - `ProjectChoice` projects a global choice protocol onto a local endpoint.
//!   - Example:
//!     - `type Out: EpSession<IO, Me>;`
//!
//! # Traits
//!
//! - `ProjectChoice<Me, IO, L, R>`
//!   - Projects a global choice protocol onto a local endpoint for role `Me`.
//!   - Associated type:
//!     - `type Out: EpSession<IO, Me>;`
//!
//! - `ProjectChoiceCase<Me, IO, Lbl, L, R, LContainsMe, RContainsMe>`
//!   - Handles different cases of `ProjectChoice` based on role presence.
//!   - Associated type:
//!     - `type Out: EpSession<IO, Me>;`

pub trait ProjectChoice<
    Me,
    IO,
    L: crate::protocol::global::TSession<IO>,
    R: crate::protocol::global::TSession<IO>,
>
{
    type Out: crate::protocol::local::EpSession<IO, Me>;
}

pub trait ProjectChoiceCase<
    Me,
    IO,
    Lbl: crate::types::ProtocolLabel,
    L: crate::protocol::global::TSession<IO>,
    R: crate::protocol::global::TSession<IO>,
    LContainsMe,
    RContainsMe,
>
{
    type Out: crate::protocol::local::EpSession<IO, Me>;
}
