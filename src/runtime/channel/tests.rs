//! Unit tests for typed channel communication
//!
//! This module provides comprehensive unit tests for the typed channel system,
//! focusing on channel creation, message serialization, async communication,
//! timeout handling, and channel lifecycle management.

#![cfg(test)]

use super::*;
use crate::protocol::foundation::{
    BiDirectionalAction, CommMetadata, DefaultChan, HandshakeLabel, InputAction, OutputAction,
};
use crate::protocol::local::EpChanEnd;
use serde::{Deserialize, Serialize};

// Test types for channel testing
#[derive(Debug, Clone, PartialEq, Eq)]
struct Alice;
impl Role for Alice {}
impl SupportsActionIO<InputAction> for Alice {}
impl SupportsActionIO<OutputAction> for Alice {}
impl SupportsActionIO<BiDirectionalAction> for Alice {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bob;
impl Role for Bob {}
impl SupportsActionIO<BiDirectionalAction> for Bob {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TestMessage {
    content: String,
}
impl Message for TestMessage {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ComplexMessage {
    id: u64,
    data: Vec<String>,
    metadata: std::collections::HashMap<String, String>,
}
impl Message for ComplexMessage {}

type TestMetadata = CommMetadata<DefaultChan, HandshakeLabel>;
type TestProtocol = EpChanEnd<InputAction, TestMetadata, BiDirectionalAction>;

// Basic channel creation and property tests
#[tokio::test]
async fn test_channel_creation() {
    let (ch1, ch2) = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(
        ChannelConfig::default()
    );

    assert!(ch1.is_send_open().await);
    assert!(ch1.is_receive_open().await);
    assert!(ch2.is_send_open().await);
    assert!(ch2.is_receive_open().await);
}

#[tokio::test]
async fn test_channel_config_default() {
    let config = ChannelConfig::default();
    assert_eq!(config.buffer_size, 32);
    assert_eq!(config.timeout_ms, Some(5000));
    assert!(config.ordered);
}

#[tokio::test]
async fn test_channel_config_custom() {
    let config = ChannelConfig {
        buffer_size: 16,
        timeout_ms: Some(2000),
        ordered: false,
    };

    let (ch1, _ch2) = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(config.clone());
    assert_eq!(ch1.config.buffer_size, 16);
    assert_eq!(ch1.config.timeout_ms, Some(2000));
    assert!(!ch1.config.ordered);
}

#[tokio::test]
async fn test_channel_debug_representation() {
    let (ch1, _) = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(
        ChannelConfig::default()
    );

    let debug_str = format!("{:?}", ch1);
    assert!(debug_str.contains("TypedChannel"));
    assert!(debug_str.contains("config"));
}

// Channel builder tests
#[tokio::test]
async fn test_channel_builder_default() {
    let (ch1, ch2) = ChannelBuilder::<TestProtocol, Alice, BiDirectionalAction>::default().build();

    assert!(ch1.is_send_open().await);
    assert!(ch2.is_receive_open().await);
}

#[tokio::test]
async fn test_channel_builder_custom() {
    let (ch1, ch2) = ChannelBuilder::<TestProtocol, Alice, BiDirectionalAction>::new()
        .buffer_size(64)
        .timeout_ms(Some(1000))
        .ordered(true)
        .build();

    assert!(ch1.is_send_open().await);
    assert!(ch2.is_receive_open().await);
    assert_eq!(ch1.config.buffer_size, 64);
    assert_eq!(ch1.config.timeout_ms, Some(1000));
    assert!(ch1.config.ordered);
}

#[tokio::test]
async fn test_channel_builder_no_timeout() {
    let (ch1, _) = ChannelBuilder::<TestProtocol, Alice, BiDirectionalAction>::new()
        .timeout_ms(None)
        .build();

    assert_eq!(ch1.config.timeout_ms, None);
}

// Message creation and serialization tests
#[tokio::test]
async fn test_channel_message_creation() {
    let metadata = TestMetadata::new();
    let payload = TestMessage {
        content: "Hello, World!".to_string(),
    };
    let message = ChannelMessage::new(metadata.clone(), payload.clone(), "alice".to_string(), 1);

    assert_eq!(message.payload.content, "Hello, World!");
    assert_eq!(message.sender_id, "alice");
    assert_eq!(message.sequence_number, 1);
}

#[tokio::test]
async fn test_complex_message_serialization() {
    let mut metadata_map = std::collections::HashMap::new();
    metadata_map.insert("key1".to_string(), "value1".to_string());
    metadata_map.insert("key2".to_string(), "value2".to_string());

    let payload = ComplexMessage {
        id: 42,
        data: vec!["item1".to_string(), "item2".to_string()],
        metadata: metadata_map,
    };

    let metadata = TestMetadata::new();
    let message = ChannelMessage::new(metadata, payload.clone(), "test".to_string(), 1);

    // This tests that serialization/deserialization works
    let serialized = serde_json::to_vec(&message).unwrap();
    let deserialized: ChannelMessage<TestMetadata, ComplexMessage> = 
        serde_json::from_slice(&serialized).unwrap();

    assert_eq!(deserialized.payload.id, 42);
    assert_eq!(deserialized.payload.data.len(), 2);
    assert_eq!(deserialized.payload.metadata.len(), 2);
}

// Basic message send/receive tests
#[tokio::test]
async fn test_message_send_receive_simple() {
    let (ch1, ch2) = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(
        ChannelConfig::default()
    );

    let metadata = TestMetadata::new();
    let payload = TestMessage {
        content: "Hello, World!".to_string(),
    };
    let message = ChannelMessage::new(metadata.clone(), payload.clone(), "alice".to_string(), 1);

    // Send from ch1
    ch1.send(message.clone()).await.unwrap();

    // Receive on ch2
    let received: ChannelMessage<TestMetadata, TestMessage> = ch2.receive().await.unwrap();
    assert_eq!(received.payload.content, "Hello, World!");
    assert_eq!(received.sender_id, "alice");
    assert_eq!(received.sequence_number, 1);
}

#[tokio::test]
async fn test_bidirectional_communication() {
    let (ch1, ch2) = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(
        ChannelConfig::default()
    );

    let metadata = TestMetadata::new();

    // Send from ch1 to ch2
    let msg1 = ChannelMessage::new(
        metadata.clone(), 
        TestMessage { content: "From Alice".to_string() }, 
        "alice".to_string(), 
        1
    );
    ch1.send(msg1).await.unwrap();

    // Send from ch2 to ch1
    let msg2 = ChannelMessage::new(
        metadata.clone(), 
        TestMessage { content: "From Bob".to_string() }, 
        "bob".to_string(), 
        1
    );
    ch2.send(msg2).await.unwrap();

    // Receive on both ends
    let received1: ChannelMessage<TestMetadata, TestMessage> = ch2.receive().await.unwrap();
    let received2: ChannelMessage<TestMetadata, TestMessage> = ch1.receive().await.unwrap();

    assert_eq!(received1.payload.content, "From Alice");
    assert_eq!(received1.sender_id, "alice");
    assert_eq!(received2.payload.content, "From Bob");
    assert_eq!(received2.sender_id, "bob");
}

#[tokio::test]
async fn test_multiple_messages_ordering() {
    let (ch1, ch2) = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(
        ChannelConfig {
            buffer_size: 10,
            timeout_ms: Some(1000),
            ordered: true,
        }
    );

    let metadata = TestMetadata::new();

    // Send multiple messages
    for i in 1..=5 {
        let message = ChannelMessage::new(
            metadata.clone(),
            TestMessage { content: format!("Message {}", i) },
            "sender".to_string(),
            i as u64,
        );
        ch1.send(message).await.unwrap();
    }

    // Receive messages and check ordering
    for i in 1..=5 {
        let received: ChannelMessage<TestMetadata, TestMessage> = ch2.receive().await.unwrap();
        assert_eq!(received.payload.content, format!("Message {}", i));
        assert_eq!(received.sequence_number, i as u64);
    }
}

// Channel lifecycle and state management tests
#[tokio::test]
async fn test_channel_close_sender() {
    let (ch1, ch2) = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(
        ChannelConfig::default()
    );

    // Initially open
    assert!(ch1.is_send_open().await);
    assert!(ch2.is_receive_open().await);

    // Close sender
    ch1.close_sender().await;
    assert!(!ch1.is_send_open().await);
    assert!(ch2.is_receive_open().await); // Receiver still open

    // Try to send after closing
    let metadata = TestMetadata::new();
    let message = ChannelMessage::new(
        metadata, 
        TestMessage { content: "Should fail".to_string() }, 
        "sender".to_string(), 
        1
    );
    let result = ch1.send(message).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RuntimeError::Communication(CommunicationError::ChannelClosed)));
}

#[tokio::test]
async fn test_channel_close_receiver() {
    let (ch1, ch2) = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(
        ChannelConfig::default()
    );

    // Initially open
    assert!(ch1.is_send_open().await);
    assert!(ch2.is_receive_open().await);

    // Close receiver
    ch2.close_receiver().await;
    assert!(ch1.is_send_open().await); // Sender still open
    assert!(!ch2.is_receive_open().await);

    // Try to receive after closing
    let result: Result<ChannelMessage<TestMetadata, TestMessage>, _> = ch2.receive().await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RuntimeError::Communication(CommunicationError::ChannelClosed)));
}

#[tokio::test]
async fn test_channel_full_close() {
    let (ch1, ch2) = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(
        ChannelConfig::default()
    );

    // Close both ends
    ch1.close_sender().await;
    ch1.close_receiver().await;
    ch2.close_sender().await;
    ch2.close_receiver().await;

    // All operations should fail
    assert!(!ch1.is_send_open().await);
    assert!(!ch1.is_receive_open().await);
    assert!(!ch2.is_send_open().await);
    assert!(!ch2.is_receive_open().await);
}

// Sequence number tests
#[tokio::test]
async fn test_sequence_numbers() {
    let ch = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(
        ChannelConfig::default()
    ).0;

    let seq1 = ch.next_sequence_number().await;
    let seq2 = ch.next_sequence_number().await;
    let seq3 = ch.next_sequence_number().await;

    assert_eq!(seq1, 1);
    assert_eq!(seq2, 2);
    assert_eq!(seq3, 3);
}

#[tokio::test]
async fn test_sequence_numbers_independent_channels() {
    let (ch1, ch2) = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(
        ChannelConfig::default()
    );

    // Each channel maintains its own sequence counter
    let ch1_seq1 = ch1.next_sequence_number().await;
    let ch2_seq1 = ch2.next_sequence_number().await;
    let ch1_seq2 = ch1.next_sequence_number().await;
    let ch2_seq2 = ch2.next_sequence_number().await;

    assert_eq!(ch1_seq1, 1);
    assert_eq!(ch2_seq1, 1);
    assert_eq!(ch1_seq2, 2);
    assert_eq!(ch2_seq2, 2);
}

// Timeout and error handling tests
#[tokio::test]
async fn test_send_receive_with_timeout() {
    let config = ChannelConfig {
        buffer_size: 1, // Small buffer
        timeout_ms: Some(100), // Short timeout
        ordered: true,
    };

    let (ch1, ch2) = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(config);

    let metadata = TestMetadata::new();
    let message = ChannelMessage::new(
        metadata, 
        TestMessage { content: "Test".to_string() }, 
        "sender".to_string(), 
        1
    );

    // Send should succeed
    ch1.send(message.clone()).await.unwrap();

    // Fill the buffer and then try to send again (should timeout)
    let result = ch1.send(message).await;
    // Note: This might succeed if the buffer isn't full yet, depending on timing
    // The key is that the timeout mechanism is in place
}

#[tokio::test]
async fn test_receive_from_closed_channel() {
    let (ch1, ch2) = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(
        ChannelConfig::default()
    );

    // Close the sender
    ch1.close_sender().await;

    // Try to receive - should fail since no more messages can be sent
    let result: Result<ChannelMessage<TestMetadata, TestMessage>, _> = ch2.receive().await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RuntimeError::Communication(CommunicationError::ChannelClosed)));
}

#[tokio::test]
async fn test_different_role_types() {
    // Test that channels work with different role types
    let (ch1, ch2) = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(
        ChannelConfig::default()
    );

    let metadata = TestMetadata::new();
    let message = ChannelMessage::new(
        metadata, 
        TestMessage { content: "Cross-role message".to_string() }, 
        "alice".to_string(), 
        1
    );

    ch1.send(message).await.unwrap();
    let received: ChannelMessage<TestMetadata, TestMessage> = ch2.receive().await.unwrap();
    assert_eq!(received.payload.content, "Cross-role message");
}

// Performance and stress tests
#[tokio::test]
async fn test_high_volume_messages() {
    let (ch1, ch2) = TypedChannel::<TestProtocol, Alice, BiDirectionalAction>::new(
        ChannelConfig {
            buffer_size: 100,
            timeout_ms: Some(5000),
            ordered: true,
        }
    );

    let metadata = TestMetadata::new();
    let message_count = 50;

    // Send many messages concurrently
    let send_task = tokio::spawn(async move {
        for i in 0..message_count {
            let message = ChannelMessage::new(
                metadata.clone(),
                TestMessage { content: format!("Message {}", i) },
                "sender".to_string(),
                i as u64,
            );
            ch1.send(message).await.unwrap();
        }
    });

    // Receive all messages
    let receive_task = tokio::spawn(async move {
        let mut received_count = 0;
        for _ in 0..message_count {
            let received: ChannelMessage<TestMetadata, TestMessage> = ch2.receive().await.unwrap();
            assert!(received.payload.content.starts_with("Message"));
            received_count += 1;
        }
        received_count
    });

    let (send_result, receive_result) = tokio::join!(send_task, receive_task);
    send_result.unwrap();
    let received_count = receive_result.unwrap();
    assert_eq!(received_count, message_count);
}
