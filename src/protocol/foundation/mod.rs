//! # Foundation Types for Enhanced MPST System
//!
//! This module provides the foundational trait definitions and core infrastructure
//! for the Besedarium MPST library as specified in `docs/duality.md`.
//!
//! ## Key Components
//!
//! - **Foundation Traits**: Basic traits for roles, messages, and protocol types
//! - **CommMetadata**: Communication metadata for precise channel and message identification
//! - **Channel and Message Labels**: Type-safe identifiers for communication channels and messages
//! - **Action I/O Types**: Markers for different I/O capabilities (Input, Output, BiDirectional)
//! - **SupportsActionIO**: Trait for verifying I/O capability compatibility

use std::fmt::Debug;
use std::hash::Hash;

// ============================================================================
// Task 1.1.1a: Foundation Trait Definitions
// ============================================================================

/// Fundamental trait for role identification in protocols
pub trait Role: Send + Sync + 'static + Debug + Clone + PartialEq + Eq + Hash {}

/// Trait for messages that can be exchanged in protocols  
pub trait Message: Send + Sync + 'static + Debug + Clone {}

/// Marker trait for Global Protocol types
pub trait GlobalProtocol: Send + Sync + 'static + Debug {}

/// Marker trait for Local Endpoint Protocol types  
pub trait LocalProtocol: Send + Sync + 'static + Debug {}

// ============================================================================
// Task 1.1.1c: Channel and Message Label Traits
// ============================================================================

/// Trait for channel identifiers
pub trait ChanId: Send + Sync + 'static + Debug + Clone + PartialEq + Eq + Hash {}

/// Trait for message labels within channels  
pub trait MsgLbl: Send + Sync + 'static + Debug + Clone + PartialEq + Eq + Hash {}

// Example concrete channel types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct DefaultChan;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct HandshakeChan;

impl ChanId for DefaultChan {}
impl ChanId for HandshakeChan {}

// Example concrete message label types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct RequestLbl;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct ResponseLbl;

impl MsgLbl for RequestLbl {}
impl MsgLbl for ResponseLbl {}

// ============================================================================
// Task 1.1.1b: CommMetadata Implementation
// ============================================================================

/// Trait for communication metadata types that can be used in protocols
/// This enables downstream implementations to extend metadata capabilities
pub trait Metadata: Send + Sync + 'static + Debug + Clone + PartialEq + Eq + Hash {
    /// Channel identifier type for this metadata
    type ChanId: ChanId;
    /// Message label type for this metadata  
    type MsgLbl: MsgLbl;

    /// Get the channel ID from this metadata
    fn chan_id(&self) -> &Self::ChanId;
    /// Get the message label from this metadata
    fn msg_lbl(&self) -> &Self::MsgLbl;
}

/// Communication metadata for precise channel and message identification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommMetadata<C: ChanId, L: MsgLbl> {
    pub chan_id: C,
    pub msg_lbl: L,
}

impl<C: ChanId, L: MsgLbl> CommMetadata<C, L> {
    pub fn new(chan_id: C, msg_lbl: L) -> Self {
        Self { chan_id, msg_lbl }
    }
}

impl<C: ChanId, L: MsgLbl> Metadata for CommMetadata<C, L> {
    type ChanId = C;
    type MsgLbl = L;

    fn chan_id(&self) -> &Self::ChanId {
        &self.chan_id
    }

    fn msg_lbl(&self) -> &Self::MsgLbl {
        &self.msg_lbl
    }
}

// Manual serde implementations for CommMetadata
impl<C, L> serde::Serialize for CommMetadata<C, L>
where
    C: ChanId + serde::Serialize,
    L: MsgLbl + serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("CommMetadata", 2)?;
        state.serialize_field("chan_id", &self.chan_id)?;
        state.serialize_field("msg_lbl", &self.msg_lbl)?;
        state.end()
    }
}

impl<'de, C, L> serde::Deserialize<'de> for CommMetadata<C, L>
where
    C: ChanId + serde::Deserialize<'de>,
    L: MsgLbl + serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            ChanId,
            MsgLbl,
        }

        struct CommMetadataVisitor<C, L>(std::marker::PhantomData<(C, L)>);

        impl<'de, C, L> Visitor<'de> for CommMetadataVisitor<C, L>
        where
            C: ChanId + serde::Deserialize<'de>,
            L: MsgLbl + serde::Deserialize<'de>,
        {
            type Value = CommMetadata<C, L>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct CommMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CommMetadata<C, L>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut chan_id = None;
                let mut msg_lbl = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ChanId => {
                            if chan_id.is_some() {
                                return Err(de::Error::duplicate_field("chan_id"));
                            }
                            chan_id = Some(map.next_value()?);
                        }
                        Field::MsgLbl => {
                            if msg_lbl.is_some() {
                                return Err(de::Error::duplicate_field("msg_lbl"));
                            }
                            msg_lbl = Some(map.next_value()?);
                        }
                    }
                }

                let chan_id = chan_id.ok_or_else(|| de::Error::missing_field("chan_id"))?;
                let msg_lbl = msg_lbl.ok_or_else(|| de::Error::missing_field("msg_lbl"))?;

                Ok(CommMetadata { chan_id, msg_lbl })
            }
        }

        const FIELDS: &[&str] = &["chan_id", "msg_lbl"];
        deserializer.deserialize_struct("CommMetadata", FIELDS, CommMetadataVisitor(std::marker::PhantomData))
    }
}

// ============================================================================
// Task 1.1.1d: ActionIOTMarker System
// ============================================================================

/// Marker trait for Action I/O Types - what I/O capability an action requires
pub trait ActionIOTMarker: Send + Sync + 'static + Debug + Clone + PartialEq + Eq {}

/// Standard Action I/O Types
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputAction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputAction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BiDirectionalAction;

impl ActionIOTMarker for InputAction {}
impl ActionIOTMarker for OutputAction {}
impl ActionIOTMarker for BiDirectionalAction {}

// ============================================================================
// Task 1.1.1e: SupportsActionIO Trait
// ============================================================================

/// Trait to verify IO capability compatibility
pub trait SupportsActionIO<AIO: ActionIOTMarker> {
    /// Returns true if this IO capability can handle the specified action type
    fn supports_action_io() -> bool {
        true // Default implementation assumes support
    }
}

// Example implementation: TCP-based session I/O that supports all actions
#[derive(Debug)]
pub struct TcpOnlySessionIO;

impl SupportsActionIO<InputAction> for TcpOnlySessionIO {}
impl SupportsActionIO<OutputAction> for TcpOnlySessionIO {}
impl SupportsActionIO<BiDirectionalAction> for TcpOnlySessionIO {}

// Example implementation: HTTP-based session I/O that only supports output and bidirectional
#[derive(Debug)]
pub struct HttpOnlySessionIO;

impl SupportsActionIO<OutputAction> for HttpOnlySessionIO {}
impl SupportsActionIO<BiDirectionalAction> for HttpOnlySessionIO {}
// Note: HttpOnlySessionIO doesn't support InputAction

// ============================================================================
// Extensible Metadata Infrastructure
// ============================================================================

/// Trait for extensible communication metadata
///
/// This trait enables downstream implementations to extend metadata
/// while maintaining compatibility with the core CommMetadata type.
///
/// Example extensions:
/// - Timestamped metadata for audit trails
/// - Priority-aware metadata for QoS
/// - Routing metadata for distributed protocols
pub trait CommMetadataTrait: Send + Sync + 'static + Debug + Clone + PartialEq + Eq {
    /// The channel identifier type
    type ChanId: ChanId;

    /// The message label type
    type MsgLbl: MsgLbl;

    /// Get the channel identifier
    fn chan_id(&self) -> &Self::ChanId;

    /// Get the message label
    fn msg_lbl(&self) -> &Self::MsgLbl;

    /// Create new metadata from channel and label
    fn new(chan_id: Self::ChanId, msg_lbl: Self::MsgLbl) -> Self;
}

/// Implementation of CommMetadataTrait for the standard CommMetadata type
impl<C: ChanId, L: MsgLbl> CommMetadataTrait for CommMetadata<C, L> {
    type ChanId = C;
    type MsgLbl = L;

    fn chan_id(&self) -> &Self::ChanId {
        &self.chan_id
    }

    fn msg_lbl(&self) -> &Self::MsgLbl {
        &self.msg_lbl
    }

    fn new(chan_id: Self::ChanId, msg_lbl: Self::MsgLbl) -> Self {
        CommMetadata::new(chan_id, msg_lbl)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests;

#[cfg(test)]
mod action_io_tests;

/// Label transformation and preservation logic for session types
pub mod labels;

// Re-export key label transformation traits for convenience
pub use labels::{
    ExtractLabels, LList, Label, LabelComposition, LabelCons, LabelList, LabelNil, LabelPredicate,
    LabelPreservation, LabelTransform, LabelValidation, LabelValidationError, TCollect, TFilter,
    TMap, UniqueLabels, ValidateChoiceLabels,
};
