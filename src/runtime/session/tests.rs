//! Unit tests for session lifecycle management
//!
//! This module provides comprehensive unit tests for session management functionality,
//! focusing on session creation, state transitions, lifecycle management, and the 
//! SessionManager coordination system.

#![cfg(test)]

use super::*;
use crate::protocol::foundation::{
    BiDirectionalAction, CommMetadata, DefaultChan, HandshakeLabel,
};
use crate::protocol::local::EpChanEnd;

// Test types for session testing
#[derive(Debug, Clone, PartialEq, Eq)]
struct Alice;
impl Role for Alice {}
impl SupportsActionIO<BiDirectionalAction> for Alice {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bob;
impl Role for Bob {}
impl SupportsActionIO<BiDirectionalAction> for Bob {}

type TestMetadata = CommMetadata<DefaultChan, HandshakeLabel>;
type TestProtocol = EpChanEnd<BiDirectionalAction, TestMetadata, BiDirectionalAction>;

// Basic session creation and property tests
#[tokio::test]
async fn test_session_creation() {
    let id = SessionId::new("test-session");
    let protocol = TestProtocol::new();
    let role = Alice;
    let config = ChannelConfig::default();

    let (session, _channel) = Session::new(id.clone(), protocol, role, config);

    assert_eq!(session.id(), &id);
    assert_eq!(session.status().await, SessionStatus::Initializing);
}

#[tokio::test]
async fn test_session_id_operations() {
    // Test manual ID creation
    let id1 = SessionId::new("manual-session-1");
    assert_eq!(id1.to_string(), "manual-session-1");

    // Test generated IDs are unique
    let id2 = SessionId::generate();
    let id3 = SessionId::generate();
    assert_ne!(id2, id3);
    assert!(!id2.0.is_empty());
    assert!(!id3.0.is_empty());

    // Test ID equality and hashing
    let id4 = SessionId::new("same-id");
    let id5 = SessionId::new("same-id");
    assert_eq!(id4, id5);
    
    let mut map = std::collections::HashMap::new();
    map.insert(id4.clone(), "value");
    assert_eq!(map.get(&id5), Some(&"value"));
}

#[tokio::test]
async fn test_session_status_display() {
    assert_eq!(SessionStatus::Initializing.to_string(), "Initializing");
    assert_eq!(SessionStatus::Running.to_string(), "Running");
    assert_eq!(SessionStatus::Paused.to_string(), "Paused");
    assert_eq!(SessionStatus::Completed.to_string(), "Completed");
    assert_eq!(SessionStatus::Cancelled.to_string(), "Cancelled");
    assert_eq!(
        SessionStatus::Failed("test error".to_string()).to_string(),
        "Failed: test error"
    );
}

// Session lifecycle and state transition tests
#[tokio::test]
async fn test_session_lifecycle_happy_path() {
    let id = SessionId::new("lifecycle-test");
    let protocol = TestProtocol::new();
    let role = Alice;
    let config = ChannelConfig::default();

    let (session, _channel) = Session::new(id, protocol, role, config);

    // Initial state
    assert_eq!(session.status().await, SessionStatus::Initializing);

    // Start session
    session.start().await.unwrap();
    assert_eq!(session.status().await, SessionStatus::Running);

    // Pause session
    session.pause().await.unwrap();
    assert_eq!(session.status().await, SessionStatus::Paused);

    // Resume session
    session.resume().await.unwrap();
    assert_eq!(session.status().await, SessionStatus::Running);

    // Cancel session
    session.cancel().await.unwrap();
    assert_eq!(session.status().await, SessionStatus::Cancelled);
}

#[tokio::test]
async fn test_invalid_state_transitions() {
    let id = SessionId::new("invalid-transitions-test");
    let (session, _channel) = Session::new(id, TestProtocol::new(), Alice, ChannelConfig::default());

    // Try to pause without starting
    let result = session.pause().await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RuntimeError::InvalidStateTransition { .. }));

    // Start the session
    session.start().await.unwrap();
    assert_eq!(session.status().await, SessionStatus::Running);

    // Try to resume without being paused
    let result = session.resume().await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RuntimeError::InvalidStateTransition { .. }));

    // Try to start again (already running)
    let result = session.start().await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RuntimeError::InvalidStateTransition { .. }));
}

#[tokio::test]
async fn test_session_cancellation_scenarios() {
    // Test cancellation from different states
    
    // Cancel from initializing state
    let (session1, _) = Session::new(SessionId::new("cancel-init"), TestProtocol::new(), Alice, ChannelConfig::default());
    session1.cancel().await.unwrap();
    assert_eq!(session1.status().await, SessionStatus::Cancelled);

    // Cancel from running state
    let (session2, _) = Session::new(SessionId::new("cancel-running"), TestProtocol::new(), Alice, ChannelConfig::default());
    session2.start().await.unwrap();
    session2.cancel().await.unwrap();
    assert_eq!(session2.status().await, SessionStatus::Cancelled);

    // Cancel from paused state
    let (session3, _) = Session::new(SessionId::new("cancel-paused"), TestProtocol::new(), Alice, ChannelConfig::default());
    session3.start().await.unwrap();
    session3.pause().await.unwrap();
    session3.cancel().await.unwrap();
    assert_eq!(session3.status().await, SessionStatus::Cancelled);
}

#[tokio::test]
async fn test_session_multiple_cancel_safe() {
    let (session, _) = Session::new(SessionId::new("multi-cancel"), TestProtocol::new(), Alice, ChannelConfig::default());
    
    // Multiple cancellations should be safe
    session.cancel().await.unwrap();
    session.cancel().await.unwrap();
    session.cancel().await.unwrap();
    
    assert_eq!(session.status().await, SessionStatus::Cancelled);
}

// SessionManager tests
#[tokio::test]
async fn test_session_manager_creation() {
    let manager = SessionManager::default();
    assert_eq!(manager.total_sessions().await, 0);

    let custom_config = ChannelConfig {
        buffer_size: 256,
        timeout_ms: Some(5000),
        ordered: false,
    };
    let custom_manager = SessionManager::new(custom_config);
    assert_eq!(custom_manager.total_sessions().await, 0);
}

#[tokio::test]
async fn test_session_manager_session_lifecycle() {
    let manager = SessionManager::default();

    let id1 = SessionId::new("manager-session-1");
    let id2 = SessionId::new("manager-session-2");

    // Create sessions
    let (session1, _ch1) = manager
        .create_session(id1.clone(), TestProtocol::new(), Alice)
        .await
        .unwrap();

    let (session2, _ch2) = manager
        .create_session(id2.clone(), TestProtocol::new(), Bob)
        .await
        .unwrap();

    // Check total sessions
    assert_eq!(manager.total_sessions().await, 2);

    // Check individual session status
    assert_eq!(manager.get_session_status(&id1).await, Some(SessionStatus::Initializing));
    assert_eq!(manager.get_session_status(&id2).await, Some(SessionStatus::Initializing));

    // Start sessions
    session1.start().await.unwrap();
    session2.start().await.unwrap();

    // Check status counts
    let counts = manager.session_count_by_status().await;
    assert_eq!(counts.get(&SessionStatus::Running), Some(&2));
    assert_eq!(counts.get(&SessionStatus::Initializing), None);
}

#[tokio::test]
async fn test_session_manager_duplicate_session_error() {
    let manager = SessionManager::default();
    let id = SessionId::new("duplicate-session");

    // Create first session
    let _result1 = manager
        .create_session(id.clone(), TestProtocol::new(), Alice)
        .await
        .unwrap();

    // Try to create session with same ID
    let result2 = manager
        .create_session(id, TestProtocol::new(), Bob)
        .await;

    assert!(result2.is_err());
    assert!(matches!(result2.unwrap_err(), RuntimeError::SessionAlreadyExists(_)));
}

#[tokio::test]
async fn test_session_manager_cancel_operations() {
    let manager = SessionManager::default();

    let id1 = SessionId::new("cancel-session-1");
    let id2 = SessionId::new("cancel-session-2");
    let id3 = SessionId::new("nonexistent-session");

    // Create and start sessions
    let (session1, _) = manager.create_session(id1.clone(), TestProtocol::new(), Alice).await.unwrap();
    let (session2, _) = manager.create_session(id2.clone(), TestProtocol::new(), Bob).await.unwrap();
    
    session1.start().await.unwrap();
    session2.start().await.unwrap();

    // Cancel individual session
    manager.cancel_session(&id1).await.unwrap();
    assert_eq!(session1.status().await, SessionStatus::Cancelled);
    assert_eq!(session2.status().await, SessionStatus::Running);

    // Try to cancel nonexistent session
    let result = manager.cancel_session(&id3).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RuntimeError::SessionNotFound(_)));

    // Cancel all remaining sessions
    manager.cancel_all_sessions().await.unwrap();
    assert_eq!(session2.status().await, SessionStatus::Cancelled);
}

#[tokio::test]
async fn test_session_manager_cleanup_operations() {
    let manager = SessionManager::default();

    // Create sessions in different states
    let (session1, _) = manager.create_session(SessionId::new("completed"), TestProtocol::new(), Alice).await.unwrap();
    let (session2, _) = manager.create_session(SessionId::new("cancelled"), TestProtocol::new(), Bob).await.unwrap();
    let (session3, _) = manager.create_session(SessionId::new("running"), TestProtocol::new(), Alice).await.unwrap();

    session1.start().await.unwrap();
    session2.start().await.unwrap();
    session3.start().await.unwrap();

    // Simulate different end states
    session1.cancel().await.unwrap(); // Will be cancelled
    session2.cancel().await.unwrap(); // Will be cancelled
    // session3 remains running

    // Initial state
    assert_eq!(manager.total_sessions().await, 3);

    // Cleanup finished sessions
    let cleaned = manager.cleanup_finished_sessions().await;
    assert_eq!(cleaned, 2); // session1 and session2 removed
    assert_eq!(manager.total_sessions().await, 1); // Only session3 remains

    // Verify status counts
    let counts = manager.session_count_by_status().await;
    assert_eq!(counts.get(&SessionStatus::Running), Some(&1));
    assert_eq!(counts.get(&SessionStatus::Cancelled), None); // Cleaned up
}

#[tokio::test]
async fn test_session_manager_status_counts() {
    let manager = SessionManager::default();

    // Create sessions in different states
    let (session1, _) = manager.create_session(SessionId::new("init"), TestProtocol::new(), Alice).await.unwrap();
    let (session2, _) = manager.create_session(SessionId::new("running"), TestProtocol::new(), Bob).await.unwrap();
    let (session3, _) = manager.create_session(SessionId::new("paused"), TestProtocol::new(), Alice).await.unwrap();

    // Set up different states
    session2.start().await.unwrap();
    session3.start().await.unwrap();
    session3.pause().await.unwrap();

    // Check status counts
    let counts = manager.session_count_by_status().await;
    assert_eq!(counts.get(&SessionStatus::Initializing), Some(&1));
    assert_eq!(counts.get(&SessionStatus::Running), Some(&1));
    assert_eq!(counts.get(&SessionStatus::Paused), Some(&1));
    assert_eq!(counts.len(), 3);
}

// Session debugging and introspection tests
#[tokio::test]
async fn test_session_debug_representation() {
    let (session, _) = Session::new(
        SessionId::new("debug-test"), 
        TestProtocol::new(), 
        Alice, 
        ChannelConfig::default()
    );

    let debug_str = format!("{:?}", session);
    assert!(debug_str.contains("Session"));
    assert!(debug_str.contains("debug-test"));
}

#[tokio::test]
async fn test_session_channel_access() {
    let (session, external_channel) = Session::new(
        SessionId::new("channel-test"), 
        TestProtocol::new(), 
        Alice, 
        ChannelConfig::default()
    );

    // Get channel reference from session
    let session_channel = session.channel();
    
    // Both channels should exist (they're pairs)
    // This test validates the channel creation and access pattern
    assert!(session_channel.is_send_open().await);
    assert!(external_channel.is_receive_open().await);
}

#[tokio::test]
async fn test_session_state_access() {
    let (session, _) = Session::new(
        SessionId::new("state-test"), 
        TestProtocol::new(), 
        Alice, 
        ChannelConfig::default()
    );

    // Get current state for debugging/monitoring
    let state = session.get_state().await;
    assert_eq!(state.session_id(), "state-test");
}
