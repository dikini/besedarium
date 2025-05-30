//! Robust typed channel communication for session type protocols
//!
//! This module provides enhanced typed channels that enforce session type protocols at runtime,
//! with comprehensive timeout handling, detailed error reporting, and channel health monitoring.

use std::fmt;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::protocol::foundation::{
    ActionIOTMarker, CommMetadataTrait, LocalProtocol, Message, Role, SupportsActionIO,
};
use crate::runtime::error::{
    ChannelOperation, CommunicationError, ErrorContext, ErrorSeverity, RecoverySuggestion,
    RuntimeError,
};

/// Unique identifier for a channel
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelId(String);

impl ChannelId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_string(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ChannelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a session
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(String);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_string(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Hierarchical timeout configuration
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    /// Global default timeout for all operations
    pub global_timeout_ms: Option<u64>,
    /// Session-specific timeout override
    pub session_timeout_ms: Option<u64>,
    /// Operation-specific timeout overrides
    pub send_timeout_ms: Option<u64>,
    pub receive_timeout_ms: Option<u64>,
    pub connect_timeout_ms: Option<u64>,
    pub close_timeout_ms: Option<u64>,
}

impl TimeoutConfig {
    pub fn new() -> Self {
        Self {
            global_timeout_ms: Some(5000), // 5 second default
            session_timeout_ms: None,
            send_timeout_ms: None,
            receive_timeout_ms: None,
            connect_timeout_ms: None,
            close_timeout_ms: None,
        }
    }

    /// Get the effective timeout for a specific operation
    pub fn effective_timeout(&self, operation: ChannelOperation) -> Option<Duration> {
        let timeout_ms = match operation {
            ChannelOperation::Send => self
                .send_timeout_ms
                .or(self.session_timeout_ms)
                .or(self.global_timeout_ms),
            ChannelOperation::Receive => self
                .receive_timeout_ms
                .or(self.session_timeout_ms)
                .or(self.global_timeout_ms),
            ChannelOperation::Connect => self
                .connect_timeout_ms
                .or(self.session_timeout_ms)
                .or(self.global_timeout_ms),
            ChannelOperation::Close => self
                .close_timeout_ms
                .or(self.session_timeout_ms)
                .or(self.global_timeout_ms),
        };

        timeout_ms.map(Duration::from_millis)
    }
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Channel health tracking for failure pattern detection
#[derive(Debug, Clone)]
pub struct ChannelHealth {
    /// Total number of successful operations
    pub successful_operations: Arc<AtomicU64>,
    /// Total number of failed operations
    pub failed_operations: Arc<AtomicU64>,
    /// Number of timeout failures
    pub timeout_failures: Arc<AtomicU64>,
    /// Number of serialization failures
    pub serialization_failures: Arc<AtomicU64>,
    /// Number of deserialization failures
    pub deserialization_failures: Arc<AtomicU64>,
    /// Last failure time
    pub last_failure_time: Arc<Mutex<Option<SystemTime>>>,
    /// Channel creation time
    pub created_at: SystemTime,
    /// Whether the channel is considered healthy
    pub is_healthy: Arc<Mutex<bool>>,
}

impl ChannelHealth {
    pub fn new() -> Self {
        Self {
            successful_operations: Arc::new(AtomicU64::new(0)),
            failed_operations: Arc::new(AtomicU64::new(0)),
            timeout_failures: Arc::new(AtomicU64::new(0)),
            serialization_failures: Arc::new(AtomicU64::new(0)),
            deserialization_failures: Arc::new(AtomicU64::new(0)),
            last_failure_time: Arc::new(Mutex::new(None)),
            created_at: SystemTime::now(),
            is_healthy: Arc::new(Mutex::new(true)),
        }
    }

    /// Record a successful operation
    pub async fn record_success(&self) {
        self.successful_operations.fetch_add(1, Ordering::Relaxed);
        self.update_health_status().await;
    }

    /// Record a failed operation
    pub async fn record_failure(&self, operation: ChannelOperation) {
        self.failed_operations.fetch_add(1, Ordering::Relaxed);

        match operation {
            ChannelOperation::Send | ChannelOperation::Receive => {
                self.timeout_failures.fetch_add(1, Ordering::Relaxed);
            }
            _ => {}
        }

        let mut last_failure = self.last_failure_time.lock().await;
        *last_failure = Some(SystemTime::now());

        self.update_health_status().await;
    }

    /// Record a serialization failure
    pub async fn record_serialization_failure(&self) {
        self.failed_operations.fetch_add(1, Ordering::Relaxed);
        self.serialization_failures.fetch_add(1, Ordering::Relaxed);

        let mut last_failure = self.last_failure_time.lock().await;
        *last_failure = Some(SystemTime::now());

        self.update_health_status().await;
    }

    /// Record a deserialization failure
    pub async fn record_deserialization_failure(&self) {
        self.failed_operations.fetch_add(1, Ordering::Relaxed);
        self.deserialization_failures
            .fetch_add(1, Ordering::Relaxed);

        let mut last_failure = self.last_failure_time.lock().await;
        *last_failure = Some(SystemTime::now());

        self.update_health_status().await;
    }

    /// Update health status based on failure rates
    async fn update_health_status(&self) {
        let successful = self.successful_operations.load(Ordering::Relaxed);
        let failed = self.failed_operations.load(Ordering::Relaxed);
        let total = successful + failed;

        if total == 0 {
            return;
        }

        // Consider channel unhealthy if failure rate > 50% and total operations > 10
        let failure_rate = failed as f64 / total as f64;
        let is_healthy = failure_rate <= 0.5 || total < 10;

        let mut health_guard = self.is_healthy.lock().await;
        *health_guard = is_healthy;
    }

    /// Get current health status
    pub async fn is_healthy(&self) -> bool {
        *self.is_healthy.lock().await
    }

    /// Get failure rate
    pub fn failure_rate(&self) -> f64 {
        let successful = self.successful_operations.load(Ordering::Relaxed);
        let failed = self.failed_operations.load(Ordering::Relaxed);
        let total = successful + failed;

        if total == 0 {
            0.0
        } else {
            failed as f64 / total as f64
        }
    }
}

impl Default for ChannelHealth {
    fn default() -> Self {
        Self::new()
    }
}

/// Enhanced configuration for typed channels
#[derive(Debug, Clone)]
pub struct ChannelConfig {
    /// Buffer size for the underlying mpsc channel
    pub buffer_size: usize,
    /// Hierarchical timeout configuration
    pub timeout_config: TimeoutConfig,
    /// Whether to enable message ordering guarantees
    pub ordered: bool,
    /// Channel identifier
    pub channel_id: ChannelId,
    /// Session identifier
    pub session_id: SessionId,
    /// Optional peer role identification
    pub peer_role: Option<String>,
}

impl ChannelConfig {
    pub fn new(session_id: SessionId) -> Self {
        Self {
            buffer_size: 32,
            timeout_config: TimeoutConfig::default(),
            ordered: true,
            channel_id: ChannelId::new(),
            session_id,
            peer_role: None,
        }
    }

    pub fn with_peer_role(mut self, role: String) -> Self {
        self.peer_role = Some(role);
        self
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    pub fn with_timeout_config(mut self, config: TimeoutConfig) -> Self {
        self.timeout_config = config;
        self
    }
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self::new(SessionId::new())
    }
}

/// Enhanced message wrapper with operation metadata
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
    pub timestamp: SystemTime,
    pub operation_id: String,
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
            timestamp: SystemTime::now(),
            operation_id: Uuid::new_v4().to_string(),
        }
    }
}

// Manual serde implementation to handle trait bounds properly
impl<M, Msg> serde::Serialize for ChannelMessage<M, Msg>
where
    M: CommMetadataTrait + serde::Serialize,
    Msg: Message + serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("ChannelMessage", 6)?;
        state.serialize_field("metadata", &self.metadata)?;
        state.serialize_field("payload", &self.payload)?;
        state.serialize_field("sender_id", &self.sender_id)?;
        state.serialize_field("sequence_number", &self.sequence_number)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("operation_id", &self.operation_id)?;
        state.end()
    }
}

impl<'de, M, Msg> serde::Deserialize<'de> for ChannelMessage<M, Msg>
where
    M: CommMetadataTrait + serde::Deserialize<'de>,
    Msg: Message + serde::Deserialize<'de>,
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
            Metadata,
            Payload,
            SenderId,
            SequenceNumber,
            Timestamp,
            OperationId,
        }

        struct ChannelMessageVisitor<M, Msg>(PhantomData<(M, Msg)>);

        impl<'de, M, Msg> Visitor<'de> for ChannelMessageVisitor<M, Msg>
        where
            M: CommMetadataTrait + serde::Deserialize<'de>,
            Msg: Message + serde::Deserialize<'de>,
        {
            type Value = ChannelMessage<M, Msg>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ChannelMessage")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ChannelMessage<M, Msg>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut metadata = None;
                let mut payload = None;
                let mut sender_id = None;
                let mut sequence_number = None;
                let mut timestamp = None;
                let mut operation_id = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Metadata => {
                            if metadata.is_some() {
                                return Err(de::Error::duplicate_field("metadata"));
                            }
                            metadata = Some(map.next_value()?);
                        }
                        Field::Payload => {
                            if payload.is_some() {
                                return Err(de::Error::duplicate_field("payload"));
                            }
                            payload = Some(map.next_value()?);
                        }
                        Field::SenderId => {
                            if sender_id.is_some() {
                                return Err(de::Error::duplicate_field("sender_id"));
                            }
                            sender_id = Some(map.next_value()?);
                        }
                        Field::SequenceNumber => {
                            if sequence_number.is_some() {
                                return Err(de::Error::duplicate_field("sequence_number"));
                            }
                            sequence_number = Some(map.next_value()?);
                        }
                        Field::Timestamp => {
                            if timestamp.is_some() {
                                return Err(de::Error::duplicate_field("timestamp"));
                            }
                            timestamp = Some(map.next_value()?);
                        }
                        Field::OperationId => {
                            if operation_id.is_some() {
                                return Err(de::Error::duplicate_field("operation_id"));
                            }
                            operation_id = Some(map.next_value()?);
                        }
                    }
                }

                let metadata = metadata.ok_or_else(|| de::Error::missing_field("metadata"))?;
                let payload = payload.ok_or_else(|| de::Error::missing_field("payload"))?;
                let sender_id = sender_id.ok_or_else(|| de::Error::missing_field("sender_id"))?;
                let sequence_number =
                    sequence_number.ok_or_else(|| de::Error::missing_field("sequence_number"))?;
                let timestamp = timestamp.ok_or_else(|| de::Error::missing_field("timestamp"))?;
                let operation_id =
                    operation_id.ok_or_else(|| de::Error::missing_field("operation_id"))?;

                Ok(ChannelMessage {
                    metadata,
                    payload,
                    sender_id,
                    sequence_number,
                    timestamp,
                    operation_id,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "metadata",
            "payload",
            "sender_id",
            "sequence_number",
            "timestamp",
            "operation_id",
        ];
        deserializer.deserialize_struct(
            "ChannelMessage",
            FIELDS,
            ChannelMessageVisitor(PhantomData),
        )
    }
}

/// Enhanced typed channel with robust error handling and health monitoring
pub struct TypedChannel<P, R, AIO>
where
    P: LocalProtocol,
    R: Role + SupportsActionIO<AIO>,
    AIO: ActionIOTMarker,
{
    sender: Mutex<Option<Sender<Vec<u8>>>>,
    receiver: Mutex<Option<Receiver<Vec<u8>>>>,
    config: ChannelConfig,
    sequence_counter: AtomicU64,
    health: ChannelHealth,
    _phantom: PhantomData<(P, R, AIO)>,
}

impl<P, R, AIO> TypedChannel<P, R, AIO>
where
    P: LocalProtocol,
    R: Role + SupportsActionIO<AIO>,
    AIO: ActionIOTMarker,
{
    /// Create a new typed channel pair with enhanced configuration
    pub fn new(config: ChannelConfig) -> (Self, Self) {
        let (sender1, receiver1) = mpsc::channel(config.buffer_size);
        let (sender2, receiver2) = mpsc::channel(config.buffer_size);

        let config1 = ChannelConfig {
            channel_id: ChannelId::new(),
            ..config.clone()
        };

        let config2 = ChannelConfig {
            channel_id: ChannelId::new(),
            ..config
        };

        let channel1 = Self {
            sender: Mutex::new(Some(sender2)),
            receiver: Mutex::new(Some(receiver1)),
            config: config1,
            sequence_counter: AtomicU64::new(0),
            health: ChannelHealth::new(),
            _phantom: PhantomData,
        };

        let channel2 = Self {
            sender: Mutex::new(Some(sender1)),
            receiver: Mutex::new(Some(receiver2)),
            config: config2,
            sequence_counter: AtomicU64::new(0),
            health: ChannelHealth::new(),
            _phantom: PhantomData,
        };

        (channel1, channel2)
    }

    /// Send a typed message through the channel with enhanced error reporting
    pub async fn send<M, Msg>(&self, message: ChannelMessage<M, Msg>) -> Result<(), RuntimeError>
    where
        M: CommMetadataTrait + serde::Serialize,
        Msg: Message + serde::Serialize,
    {
        let operation = ChannelOperation::Send;
        let timeout = self.config.timeout_config.effective_timeout(operation);

        let mut sender_guard = self.sender.lock().await;
        let sender = sender_guard
            .as_mut()
            .ok_or_else(|| RuntimeError::Communication {
                error: CommunicationError::ChannelClosed,
                severity: ErrorSeverity::High,
                context: ErrorContext::new()
                    .with_component("channel")
                    .with_operation("send"),
                recovery_suggestion: RecoverySuggestion::RestartSession,
            })?;

        // Serialize the message with enhanced error handling
        let serialized = serde_json::to_vec(&message).map_err(|e| {
            // Record serialization failure
            let health = self.health.clone();
            tokio::spawn(async move {
                health.record_serialization_failure().await;
            });

            RuntimeError::Communication {
                error: CommunicationError::SerializationFailed {
                    channel_id: self.config.channel_id.to_string(),
                    message_type: std::any::type_name::<Msg>().to_string(),
                    session_id: self.config.session_id.to_string(),
                    underlying_error: e.to_string(),
                },
                severity: ErrorSeverity::Medium,
                context: ErrorContext::new()
                    .with_component("channel")
                    .with_operation("serialize"),
                recovery_suggestion: RecoverySuggestion::CheckConfiguration,
            }
        })?;

        // Send with timeout if configured
        let send_result = if let Some(timeout_duration) = timeout {
            match tokio::time::timeout(timeout_duration, sender.send(serialized)).await {
                Ok(result) => result,
                Err(_) => {
                    // Record timeout failure synchronously
                    self.health.record_failure(operation).await;

                    return Err(RuntimeError::Communication {
                        error: CommunicationError::ChannelTimeout {
                            channel_id: self.config.channel_id.to_string(),
                            operation,
                            peer_role: self.config.peer_role.clone(),
                            session_id: self.config.session_id.to_string(),
                            timeout_ms: timeout_duration.as_millis() as u64,
                        },
                        severity: ErrorSeverity::High,
                        context: ErrorContext::new()
                            .with_component("channel")
                            .with_operation("send"),
                        recovery_suggestion: RecoverySuggestion::RetryWithBackoff,
                    });
                }
            }
            .map_err(|_| RuntimeError::Communication {
                error: CommunicationError::ChannelOperationFailed {
                    channel_id: self.config.channel_id.to_string(),
                    operation,
                    peer_role: self.config.peer_role.clone(),
                    session_id: self.config.session_id.to_string(),
                    details: "Channel closed during send operation".to_string(),
                    underlying_error: None,
                },
                severity: ErrorSeverity::High,
                context: ErrorContext::new()
                    .with_component("channel")
                    .with_operation("send"),
                recovery_suggestion: RecoverySuggestion::RestartSession,
            })
        } else {
            sender
                .send(serialized)
                .await
                .map_err(|_| RuntimeError::Communication {
                    error: CommunicationError::ChannelOperationFailed {
                        channel_id: self.config.channel_id.to_string(),
                        operation,
                        peer_role: self.config.peer_role.clone(),
                        session_id: self.config.session_id.to_string(),
                        details: "Channel closed during send operation".to_string(),
                        underlying_error: None,
                    },
                    severity: ErrorSeverity::High,
                    context: ErrorContext::new()
                        .with_component("channel")
                        .with_operation("send"),
                    recovery_suggestion: RecoverySuggestion::RestartSession,
                })
        };

        match send_result {
            Ok(_) => {
                self.health.record_success().await;
                Ok(())
            }
            Err(e) => {
                self.health.record_failure(operation).await;
                Err(e)
            }
        }
    }

    /// Receive a typed message from the channel with enhanced error reporting
    pub async fn receive<M, Msg>(&self) -> Result<ChannelMessage<M, Msg>, RuntimeError>
    where
        M: CommMetadataTrait + for<'de> serde::Deserialize<'de>,
        Msg: Message + for<'de> serde::Deserialize<'de>,
    {
        let operation = ChannelOperation::Receive;
        let timeout = self.config.timeout_config.effective_timeout(operation);

        let mut receiver_guard = self.receiver.lock().await;
        let receiver = receiver_guard
            .as_mut()
            .ok_or_else(|| RuntimeError::Communication {
                error: CommunicationError::ChannelClosed,
                severity: ErrorSeverity::High,
                context: ErrorContext::new()
                    .with_component("channel")
                    .with_operation("receive"),
                recovery_suggestion: RecoverySuggestion::RestartSession,
            })?;

        // Receive with timeout if configured
        let serialized = if let Some(timeout_duration) = timeout {
            match tokio::time::timeout(timeout_duration, receiver.recv()).await {
                Ok(result) => result,
                Err(_) => {
                    // Record timeout failure synchronously
                    self.health.record_failure(operation).await;

                    return Err(RuntimeError::Communication {
                        error: CommunicationError::ChannelTimeout {
                            channel_id: self.config.channel_id.to_string(),
                            operation,
                            peer_role: self.config.peer_role.clone(),
                            session_id: self.config.session_id.to_string(),
                            timeout_ms: timeout_duration.as_millis() as u64,
                        },
                        severity: ErrorSeverity::High,
                        context: ErrorContext::new()
                            .with_component("channel")
                            .with_operation("receive"),
                        recovery_suggestion: RecoverySuggestion::RetryWithBackoff,
                    });
                }
            }
            .ok_or_else(|| RuntimeError::Communication {
                error: CommunicationError::ChannelOperationFailed {
                    channel_id: self.config.channel_id.to_string(),
                    operation,
                    peer_role: self.config.peer_role.clone(),
                    session_id: self.config.session_id.to_string(),
                    details: "Channel closed during receive operation".to_string(),
                    underlying_error: None,
                },
                severity: ErrorSeverity::High,
                context: ErrorContext::new()
                    .with_component("channel")
                    .with_operation("receive"),
                recovery_suggestion: RecoverySuggestion::RestartSession,
            })?
        } else {
            receiver
                .recv()
                .await
                .ok_or_else(|| RuntimeError::Communication {
                    error: CommunicationError::ChannelOperationFailed {
                        channel_id: self.config.channel_id.to_string(),
                        operation,
                        peer_role: self.config.peer_role.clone(),
                        session_id: self.config.session_id.to_string(),
                        details: "Channel closed during receive operation".to_string(),
                        underlying_error: None,
                    },
                    severity: ErrorSeverity::High,
                    context: ErrorContext::new()
                        .with_component("channel")
                        .with_operation("receive"),
                    recovery_suggestion: RecoverySuggestion::RestartSession,
                })?
        };

        // Deserialize the message with enhanced error handling
        let message_result: Result<ChannelMessage<M, Msg>, _> = serde_json::from_slice(&serialized)
            .map_err(|e| {
                // Record deserialization failure
                let health = self.health.clone();
                tokio::spawn(async move {
                    health.record_deserialization_failure().await;
                });

                // Create preview of raw data for debugging (first 32 bytes as hex)
                let raw_data_preview = if serialized.len() > 32 {
                    Some(hex::encode(&serialized[..32]) + "...")
                } else {
                    Some(hex::encode(&serialized))
                };

                RuntimeError::Communication {
                    error: CommunicationError::DeserializationFailed {
                        channel_id: self.config.channel_id.to_string(),
                        expected_type: std::any::type_name::<ChannelMessage<M, Msg>>().to_string(),
                        actual_data_length: serialized.len(),
                        raw_data_preview,
                        session_id: self.config.session_id.to_string(),
                        underlying_error: e.to_string(),
                    },
                    severity: ErrorSeverity::Medium,
                    context: ErrorContext::new()
                        .with_component("channel")
                        .with_operation("deserialize"),
                    recovery_suggestion: RecoverySuggestion::Retry,
                }
            });

        match message_result {
            Ok(message) => {
                self.health.record_success().await;
                Ok(message)
            }
            Err(e) => {
                self.health.record_failure(operation).await;
                Err(e)
            }
        }
    }

    /// Close the channel for sending with timeout
    pub async fn close_sender(&self) -> Result<(), RuntimeError> {
        let operation = ChannelOperation::Close;
        let timeout = self.config.timeout_config.effective_timeout(operation);

        let close_operation = async {
            let mut sender_guard = self.sender.lock().await;
            *sender_guard = None;
            Ok(())
        };

        if let Some(timeout_duration) = timeout {
            tokio::time::timeout(timeout_duration, close_operation)
                .await
                .map_err(|_| RuntimeError::Communication {
                    error: CommunicationError::ChannelTimeout {
                        channel_id: self.config.channel_id.to_string(),
                        operation,
                        peer_role: self.config.peer_role.clone(),
                        session_id: self.config.session_id.to_string(),
                        timeout_ms: timeout_duration.as_millis() as u64,
                    },
                    severity: ErrorSeverity::Medium,
                    context: ErrorContext::new()
                        .with_component("channel")
                        .with_operation("close_sender"),
                    recovery_suggestion: RecoverySuggestion::Retry,
                })?
        } else {
            close_operation.await
        }
    }

    /// Close the channel for receiving with timeout
    pub async fn close_receiver(&self) -> Result<(), RuntimeError> {
        let operation = ChannelOperation::Close;
        let timeout = self.config.timeout_config.effective_timeout(operation);

        let close_operation = async {
            let mut receiver_guard = self.receiver.lock().await;
            *receiver_guard = None;
            Ok(())
        };

        if let Some(timeout_duration) = timeout {
            tokio::time::timeout(timeout_duration, close_operation)
                .await
                .map_err(|_| RuntimeError::Communication {
                    error: CommunicationError::ChannelTimeout {
                        channel_id: self.config.channel_id.to_string(),
                        operation,
                        peer_role: self.config.peer_role.clone(),
                        session_id: self.config.session_id.to_string(),
                        timeout_ms: timeout_duration.as_millis() as u64,
                    },
                    severity: ErrorSeverity::Medium,
                    context: ErrorContext::new()
                        .with_component("channel")
                        .with_operation("close_receiver"),
                    recovery_suggestion: RecoverySuggestion::Retry,
                })?
        } else {
            close_operation.await
        }
    }

    /// Get the next sequence number for messages
    pub fn next_sequence_number(&self) -> u64 {
        self.sequence_counter.fetch_add(1, Ordering::SeqCst) + 1
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

    /// Get channel health information
    pub async fn health(&self) -> ChannelHealth {
        self.health.clone()
    }

    /// Get channel configuration
    pub fn config(&self) -> &ChannelConfig {
        &self.config
    }

    /// Check if channel is healthy
    pub async fn is_healthy(&self) -> bool {
        self.health.is_healthy().await
    }

    /// Get channel ID
    pub fn id(&self) -> &ChannelId {
        &self.config.channel_id
    }

    /// Get session ID
    pub fn session_id(&self) -> &SessionId {
        &self.config.session_id
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
            .field("channel_id", &self.config.channel_id)
            .field("session_id", &self.config.session_id)
            .field("config", &self.config)
            .field("protocol", &std::any::type_name::<P>())
            .field("role", &std::any::type_name::<R>())
            .field("action_io", &std::any::type_name::<AIO>())
            .field("failure_rate", &self.health.failure_rate())
            .finish()
    }
}

/// Enhanced channel builder for creating typed channels with comprehensive configuration
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
    pub fn new(session_id: SessionId) -> Self {
        Self {
            config: ChannelConfig::new(session_id),
            _phantom: PhantomData,
        }
    }

    pub fn buffer_size(mut self, size: usize) -> Self {
        self.config.buffer_size = size;
        self
    }

    pub fn timeout_config(mut self, config: TimeoutConfig) -> Self {
        self.config.timeout_config = config;
        self
    }

    pub fn peer_role(mut self, role: String) -> Self {
        self.config.peer_role = Some(role);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::foundation::{
        BiDirectionalAction, CommMetadata, DefaultChan, InputAction, OutputAction, RequestLbl,
    };
    use crate::protocol::local::EpChanEnd;
    use serde::{Deserialize, Serialize};
    use std::time::Duration;

    // Test types
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Alice;
    impl Role for Alice {}
    impl SupportsActionIO<InputAction> for Alice {}
    impl SupportsActionIO<OutputAction> for Alice {}
    impl SupportsActionIO<BiDirectionalAction> for Alice {}

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Bob;
    impl Role for Bob {}
    impl SupportsActionIO<InputAction> for Bob {}
    impl SupportsActionIO<OutputAction> for Bob {}
    impl SupportsActionIO<BiDirectionalAction> for Bob {}

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct TestMessage {
        content: String,
        number: u32,
    }
    impl Message for TestMessage {}

    type TestMetadata = CommMetadata<DefaultChan, RequestLbl>;
    type TestProtocol = EpChanEnd<Alice, TestMetadata, BiDirectionalAction>;

    fn test_session_id() -> SessionId {
        SessionId::from_string("test-session-123".to_string())
    }

    #[tokio::test]
    async fn test_channel_creation_with_ids() {
        let session_id = test_session_id();
        let config = ChannelConfig::new(session_id.clone());
        let (ch1, ch2) = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(config);

        assert_eq!(ch1.session_id(), &session_id);
        assert_eq!(ch2.session_id(), &session_id);
        assert_ne!(ch1.id(), ch2.id()); // Different channel IDs
        assert!(ch1.is_send_open().await);
        assert!(ch1.is_receive_open().await);
        assert!(ch2.is_send_open().await);
        assert!(ch2.is_receive_open().await);
    }

    #[tokio::test]
    async fn test_channel_builder_comprehensive() {
        let session_id = test_session_id();
        let timeout_config = TimeoutConfig {
            global_timeout_ms: Some(1000),
            send_timeout_ms: Some(500),
            receive_timeout_ms: Some(750),
            ..TimeoutConfig::default()
        };

        let (ch1, _ch2) =
            ChannelBuilder::<TestProtocol, Alice, BiDirectionalAction>::new(session_id.clone())
                .buffer_size(64)
                .timeout_config(timeout_config.clone())
                .peer_role("Bob".to_string())
                .ordered(true)
                .build();

        assert_eq!(ch1.config().buffer_size, 64);
        assert_eq!(ch1.config().peer_role, Some("Bob".to_string()));
        assert_eq!(ch1.config().session_id, session_id);
        assert!(ch1.config().ordered);

        // Test timeout configuration
        let effective_send_timeout = ch1
            .config()
            .timeout_config
            .effective_timeout(ChannelOperation::Send);
        assert_eq!(effective_send_timeout, Some(Duration::from_millis(500)));

        let effective_receive_timeout = ch1
            .config()
            .timeout_config
            .effective_timeout(ChannelOperation::Receive);
        assert_eq!(effective_receive_timeout, Some(Duration::from_millis(750)));
    }

    #[tokio::test]
    async fn test_message_send_receive_with_metadata() {
        let session_id = test_session_id();
        let (ch1, ch2) =
            ChannelBuilder::<TestProtocol, Alice, BiDirectionalAction>::new(session_id).build();

        let metadata = TestMetadata::new(DefaultChan, RequestLbl);
        let payload = TestMessage {
            content: "Hello, Enhanced World!".to_string(),
            number: 42,
        };
        let seq_num = ch1.next_sequence_number();
        let message = ChannelMessage::new(
            metadata.clone(),
            payload.clone(),
            "alice".to_string(),
            seq_num,
        );

        // Send from ch1
        ch1.send(message.clone()).await.unwrap();

        // Receive on ch2
        let received: ChannelMessage<TestMetadata, TestMessage> = ch2.receive().await.unwrap();
        assert_eq!(received.payload.content, "Hello, Enhanced World!");
        assert_eq!(received.payload.number, 42);
        assert_eq!(received.sender_id, "alice");
        assert_eq!(received.sequence_number, seq_num);
        assert!(!received.operation_id.is_empty());

        // Check health metrics
        assert!(ch1.is_healthy().await);
        assert!(ch2.is_healthy().await);
        let ch1_health = ch1.health().await;
        assert_eq!(ch1_health.successful_operations.load(Ordering::Relaxed), 1);
        assert_eq!(ch1_health.failed_operations.load(Ordering::Relaxed), 0);
    }

    #[tokio::test]
    async fn test_send_timeout() {
        let session_id = test_session_id();
        let timeout_config = TimeoutConfig {
            send_timeout_ms: Some(50), // Very short timeout
            ..TimeoutConfig::default()
        };

        let (ch1, _ch2) =
            ChannelBuilder::<TestProtocol, Alice, BiDirectionalAction>::new(session_id)
                .buffer_size(1) // Small buffer to force blocking
                .timeout_config(timeout_config)
                .build();

        let metadata = TestMetadata::new(DefaultChan, RequestLbl);
        let payload = TestMessage {
            content: "Test".to_string(),
            number: 1,
        };

        // Fill the buffer
        let message1 =
            ChannelMessage::new(metadata.clone(), payload.clone(), "alice".to_string(), 1);
        ch1.send(message1).await.unwrap();

        // This should timeout because buffer is full and no one is receiving
        let message2 = ChannelMessage::new(metadata, payload, "alice".to_string(), 2);
        let result = ch1.send(message2).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            RuntimeError::Communication {
                error:
                    CommunicationError::ChannelTimeout {
                        operation,
                        timeout_ms,
                        ..
                    },
                ..
            } => {
                assert_eq!(operation, ChannelOperation::Send);
                assert_eq!(timeout_ms, 50);
            }
            _ => panic!("Expected ChannelTimeout error"),
        }

        // Check that failure was recorded
        let health = ch1.health().await;
        assert!(health.failed_operations.load(Ordering::Relaxed) > 0);
        assert!(health.timeout_failures.load(Ordering::Relaxed) > 0);
    }

    #[tokio::test]
    async fn test_receive_timeout() {
        let session_id = test_session_id();
        let timeout_config = TimeoutConfig {
            receive_timeout_ms: Some(50), // Very short timeout
            ..TimeoutConfig::default()
        };

        let (_ch1, ch2) =
            ChannelBuilder::<TestProtocol, Alice, BiDirectionalAction>::new(session_id)
                .timeout_config(timeout_config)
                .build();

        // Try to receive when no message is available
        let result: Result<ChannelMessage<TestMetadata, TestMessage>, _> = ch2.receive().await;

        assert!(result.is_err());
        match result.unwrap_err() {
            RuntimeError::Communication {
                error:
                    CommunicationError::ChannelTimeout {
                        operation,
                        timeout_ms,
                        ..
                    },
                ..
            } => {
                assert_eq!(operation, ChannelOperation::Receive);
                assert_eq!(timeout_ms, 50);
            }
            _ => panic!("Expected ChannelTimeout error"),
        }

        // Check that failure was recorded
        let health = ch2.health().await;
        assert!(health.failed_operations.load(Ordering::Relaxed) > 0);
        assert!(health.timeout_failures.load(Ordering::Relaxed) > 0);
    }

    #[tokio::test]
    async fn test_channel_close_operations() {
        let session_id = test_session_id();
        let (ch1, ch2) =
            ChannelBuilder::<TestProtocol, Alice, BiDirectionalAction>::new(session_id).build();

        // Initially open
        assert!(ch1.is_send_open().await);
        assert!(ch2.is_receive_open().await);

        // Close sender
        ch1.close_sender().await.unwrap();
        assert!(!ch1.is_send_open().await);

        // Close receiver
        ch2.close_receiver().await.unwrap();
        assert!(!ch2.is_receive_open().await);
    }

    #[tokio::test]
    async fn test_channel_close_with_timeout() {
        let session_id = test_session_id();
        let timeout_config = TimeoutConfig {
            close_timeout_ms: Some(100),
            ..TimeoutConfig::default()
        };

        let (ch1, _ch2) =
            ChannelBuilder::<TestProtocol, Alice, BiDirectionalAction>::new(session_id)
                .timeout_config(timeout_config)
                .build();

        // Close should succeed within timeout
        let result = ch1.close_sender().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sequence_numbers() {
        let session_id = test_session_id();
        let (ch, _) =
            ChannelBuilder::<TestProtocol, Alice, BiDirectionalAction>::new(session_id).build();

        let seq1 = ch.next_sequence_number();
        let seq2 = ch.next_sequence_number();
        let seq3 = ch.next_sequence_number();

        assert_eq!(seq1, 1);
        assert_eq!(seq2, 2);
        assert_eq!(seq3, 3);
    }

    #[tokio::test]
    async fn test_health_monitoring() {
        let health = ChannelHealth::new();

        // Initially healthy
        assert!(health.is_healthy().await);
        assert_eq!(health.failure_rate(), 0.0);

        // Record some successes
        for _ in 0..5 {
            health.record_success().await;
        }
        assert_eq!(health.failure_rate(), 0.0);
        assert!(health.is_healthy().await);

        // Record some failures
        for _ in 0..3 {
            health.record_failure(ChannelOperation::Send).await;
        }
        assert_eq!(health.failure_rate(), 3.0 / 8.0); // 3 failures out of 8 total
        assert!(health.is_healthy().await); // Still healthy (< 50% failure rate)

        // Record enough failures to become unhealthy
        for _ in 0..6 {
            health.record_serialization_failure().await;
        }
        // Total: 5 success + 9 failures = 14 operations, 9/14 = 64% failure rate
        assert!(health.failure_rate() > 0.5);
        assert!(!health.is_healthy().await); // Now unhealthy
    }

    #[tokio::test]
    async fn test_timeout_hierarchy() {
        let timeout_config = TimeoutConfig {
            global_timeout_ms: Some(1000),
            session_timeout_ms: Some(500),
            send_timeout_ms: Some(100),
            receive_timeout_ms: None,
            connect_timeout_ms: None,
            close_timeout_ms: Some(200),
        };

        // Send has specific timeout
        assert_eq!(
            timeout_config.effective_timeout(ChannelOperation::Send),
            Some(Duration::from_millis(100))
        );

        // Receive falls back to session timeout
        assert_eq!(
            timeout_config.effective_timeout(ChannelOperation::Receive),
            Some(Duration::from_millis(500))
        );

        // Connect falls back to session timeout
        assert_eq!(
            timeout_config.effective_timeout(ChannelOperation::Connect),
            Some(Duration::from_millis(500))
        );

        // Close has specific timeout
        assert_eq!(
            timeout_config.effective_timeout(ChannelOperation::Close),
            Some(Duration::from_millis(200))
        );
    }

    #[tokio::test]
    async fn test_error_context_preservation() {
        let session_id = SessionId::from_string("error-test-session".to_string());
        let (ch1, _ch2) =
            ChannelBuilder::<TestProtocol, Alice, BiDirectionalAction>::new(session_id.clone())
                .peer_role("Bob".to_string())
                .build();

        // Close the channel to trigger an error
        ch1.close_sender().await.unwrap();

        let metadata = TestMetadata::new(DefaultChan, RequestLbl);
        let payload = TestMessage {
            content: "Should fail".to_string(),
            number: 42,
        };
        let message = ChannelMessage::new(metadata, payload, "alice".to_string(), 1);

        // Try to send on closed channel
        let result = ch1.send(message).await;
        assert!(result.is_err());

        // The error should contain detailed context
        match result.unwrap_err() {
            RuntimeError::Communication {
                error: CommunicationError::ChannelClosed,
                ..
            } => {
                // This is the expected error for a closed channel
            }
            _ => panic!("Expected ChannelClosed error"),
        }
    }

    #[tokio::test]
    async fn test_channel_debug_output() {
        let session_id = test_session_id();
        let (ch, _) = ChannelBuilder::<TestProtocol, Alice, BiDirectionalAction>::new(session_id)
            .peer_role("TestPeer".to_string())
            .build();

        let debug_output = format!("{:?}", ch);
        assert!(debug_output.contains("TypedChannel"));
        assert!(debug_output.contains("channel_id"));
        assert!(debug_output.contains("session_id"));
        assert!(debug_output.contains("failure_rate"));
    }

    #[tokio::test]
    async fn test_multiple_operations_health_tracking() {
        let session_id = test_session_id();
        let (ch1, ch2) =
            ChannelBuilder::<TestProtocol, Alice, BiDirectionalAction>::new(session_id).build();

        let metadata = TestMetadata::new(DefaultChan, RequestLbl);

        // Perform several successful operations
        for i in 0..5 {
            let payload = TestMessage {
                content: format!("Message {}", i),
                number: i,
            };
            let message = ChannelMessage::new(
                metadata.clone(),
                payload,
                "alice".to_string(),
                ch1.next_sequence_number(),
            );

            ch1.send(message).await.unwrap();
            let _: ChannelMessage<TestMetadata, TestMessage> = ch2.receive().await.unwrap();
        }

        // Check health metrics
        let ch1_health = ch1.health().await;
        let ch2_health = ch2.health().await;

        assert_eq!(ch1_health.successful_operations.load(Ordering::Relaxed), 5);
        assert_eq!(ch2_health.successful_operations.load(Ordering::Relaxed), 5);
        assert_eq!(ch1_health.failed_operations.load(Ordering::Relaxed), 0);
        assert_eq!(ch2_health.failed_operations.load(Ordering::Relaxed), 0);

        assert!(ch1.is_healthy().await);
        assert!(ch2.is_healthy().await);
    }
}
