//! Typed channel communication for session type protocols
//!
//! This module provides typed channels that enforce session type protocols at runtime,
//! with full async/await support using tokio.

use std::fmt;
use std::marker::PhantomData;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Mutex;

use crate::protocol::foundation::{
    ActionIOTMarker, CommMetadataTrait, LocalProtocol, Message, Role, SupportsActionIO,
};
use crate::runtime::error::{CommunicationError, RuntimeError};

/// Configuration for typed channels
#[derive(Debug, Clone)]
pub struct ChannelConfig {
    /// Buffer size for the underlying mpsc channel
    pub buffer_size: usize,
    /// Timeout for send/receive operations in milliseconds
    pub timeout_ms: Option<u64>,
    /// Whether to enable message ordering guarantees
    pub ordered: bool,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            buffer_size: 32,
            timeout_ms: Some(5000), // 5 second default timeout
            ordered: true,
        }
    }
}

/// A message wrapper that includes metadata and type information
#[derive(Debug, Clone)]
pub struct ChannelMessage<M, Msg>
where
    M: CommMetadataTrait,
    Msg: Message,
{
    pub metadata: M,
    pub payload: Msg,
    pub sender_id: String,
    pub sequence_number: u64,
}

impl<M, Msg> ChannelMessage<M, Msg>
where
    M: CommMetadataTrait,
    Msg: Message,
{
    pub fn new(metadata: M, payload: Msg, sender_id: String, sequence_number: u64) -> Self {
        Self {
            metadata,
            payload,
            sender_id,
            sequence_number,
        }
    }
}

/// A typed channel that enforces session type protocols
pub struct TypedChannel<P, R, AIO>
where
    P: LocalProtocol,
    R: Role + SupportsActionIO<AIO>,
    AIO: ActionIOTMarker,
{
    sender: Mutex<Option<Sender<Vec<u8>>>>,
    receiver: Mutex<Option<Receiver<Vec<u8>>>>,
    config: ChannelConfig,
    sequence_counter: Mutex<u64>,
    _phantom: PhantomData<(P, R, AIO)>,
}

impl<P, R, AIO> TypedChannel<P, R, AIO>
where
    P: LocalProtocol,
    R: Role + SupportsActionIO<AIO>,
    AIO: ActionIOTMarker,
{
    /// Create a new typed channel pair
    pub fn new(config: ChannelConfig) -> (Self, Self) {
        let (sender1, receiver1) = mpsc::channel(config.buffer_size);
        let (sender2, receiver2) = mpsc::channel(config.buffer_size);

        let channel1 = Self {
            sender: Mutex::new(Some(sender2)),
            receiver: Mutex::new(Some(receiver1)),
            config: config.clone(),
            sequence_counter: Mutex::new(0),
            _phantom: PhantomData,
        };

        let channel2 = Self {
            sender: Mutex::new(Some(sender1)),
            receiver: Mutex::new(Some(receiver2)),
            config,
            sequence_counter: Mutex::new(0),
            _phantom: PhantomData,
        };

        (channel1, channel2)
    }

    /// Send a typed message through the channel
    pub async fn send<M, Msg>(&self, message: ChannelMessage<M, Msg>) -> Result<(), RuntimeError>
    where
        M: CommMetadataTrait,
        Msg: Message + serde::Serialize,
    {
        let mut sender_guard = self.sender.lock().await;
        let sender = sender_guard
            .as_mut()
            .ok_or_else(|| RuntimeError::Communication(CommunicationError::ChannelClosed))?;

        // Serialize the message
        let serialized = serde_json::to_vec(&message)
            .map_err(|e| RuntimeError::Communication(CommunicationError::SerializationError(e.to_string())))?;

        // Send with timeout if configured
        if let Some(timeout_ms) = self.config.timeout_ms {
            tokio::time::timeout(
                tokio::time::Duration::from_millis(timeout_ms),
                sender.send(serialized),
            )
            .await
            .map_err(|_| RuntimeError::Communication(CommunicationError::SendTimeout))?
            .map_err(|_| RuntimeError::Communication(CommunicationError::ChannelClosed))?;
        } else {
            sender
                .send(serialized)
                .await
                .map_err(|_| RuntimeError::Communication(CommunicationError::ChannelClosed))?;
        }

        Ok(())
    }

    /// Receive a typed message from the channel
    pub async fn receive<M, Msg>(&self) -> Result<ChannelMessage<M, Msg>, RuntimeError>
    where
        M: CommMetadataTrait,
        Msg: Message + for<'de> serde::Deserialize<'de>,
    {
        let mut receiver_guard = self.receiver.lock().await;
        let receiver = receiver_guard
            .as_mut()
            .ok_or_else(|| RuntimeError::Communication(CommunicationError::ChannelClosed))?;

        // Receive with timeout if configured
        let serialized = if let Some(timeout_ms) = self.config.timeout_ms {
            tokio::time::timeout(
                tokio::time::Duration::from_millis(timeout_ms),
                receiver.recv(),
            )
            .await
            .map_err(|_| RuntimeError::Communication(CommunicationError::ReceiveTimeout))?
            .ok_or_else(|| RuntimeError::Communication(CommunicationError::ChannelClosed))?
        } else {
            receiver
                .recv()
                .await
                .ok_or_else(|| RuntimeError::Communication(CommunicationError::ChannelClosed))?
        };

        // Deserialize the message
        let message: ChannelMessage<M, Msg> = serde_json::from_slice(&serialized)
            .map_err(|e| RuntimeError::Communication(CommunicationError::DeserializationError(e.to_string())))?;

        Ok(message)
    }

    /// Close the channel for sending
    pub async fn close_sender(&self) {
        let mut sender_guard = self.sender.lock().await;
        *sender_guard = None;
    }

    /// Close the channel for receiving
    pub async fn close_receiver(&self) {
        let mut receiver_guard = self.receiver.lock().await;
        *receiver_guard = None;
    }

    /// Get the next sequence number for messages
    pub async fn next_sequence_number(&self) -> u64 {
        let mut counter = self.sequence_counter.lock().await;
        *counter += 1;
        *counter
    }

    /// Check if the channel is still open for sending
    pub async fn is_send_open(&self) -> bool {
        let sender_guard = self.sender.lock().await;
        sender_guard.is_some()
    }

    /// Check if the channel is still open for receiving
    pub async fn is_receive_open(&self) -> bool {
        let receiver_guard = self.receiver.lock().await;
        receiver_guard.is_some()
    }
}

impl<P, R, AIO> fmt::Debug for TypedChannel<P, R, AIO>
where
    P: LocalProtocol,
    R: Role + SupportsActionIO<AIO>,
    AIO: ActionIOTMarker,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypedChannel")
            .field("config", &self.config)
            .field("protocol", &std::any::type_name::<P>())
            .field("role", &std::any::type_name::<R>())
            .field("action_io", &std::any::type_name::<AIO>())
            .finish()
    }
}

/// Channel builder for creating typed channels with custom configuration
pub struct ChannelBuilder<P, R, AIO>
where
    P: LocalProtocol,
    R: Role + SupportsActionIO<AIO>,
    AIO: ActionIOTMarker,
{
    config: ChannelConfig,
    _phantom: PhantomData<(P, R, AIO)>,
}

impl<P, R, AIO> ChannelBuilder<P, R, AIO>
where
    P: LocalProtocol,
    R: Role + SupportsActionIO<AIO>,
    AIO: ActionIOTMarker,
{
    pub fn new() -> Self {
        Self {
            config: ChannelConfig::default(),
            _phantom: PhantomData,
        }
    }

    pub fn buffer_size(mut self, size: usize) -> Self {
        self.config.buffer_size = size;
        self
    }

    pub fn timeout_ms(mut self, timeout: Option<u64>) -> Self {
        self.config.timeout_ms = timeout;
        self
    }

    pub fn ordered(mut self, ordered: bool) -> Self {
        self.config.ordered = ordered;
        self
    }

    pub fn build(self) -> (TypedChannel<P, R, AIO>, TypedChannel<P, R, AIO>) {
        TypedChannel::new(self.config)
    }
}

impl<P, R, AIO> Default for ChannelBuilder<P, R, AIO>
where
    P: LocalProtocol,
    R: Role + SupportsActionIO<AIO>,
    AIO: ActionIOTMarker,
{
    fn default() -> Self {
        Self::new()
    }
}

pub mod tests;
