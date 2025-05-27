//! Integration tests for runtime components
//!
//! This module provides comprehensive integration tests that validate the interaction
//! between state machine, channel communication, session management, and error handling.

#![cfg(test)]

use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

use crate::protocol::foundation::{
    ActionIOTMarker, BiDirectionalAction, CommMetadata, DefaultChan, HandshakeMsg, 
    InputAction, LocalProtocol, OutputAction, Role, SupportsActionIO
};
use crate::runtime::{
    channel::{ChannelConfig, TypedChannel},
    error::{RuntimeError, ProtocolViolation},
    session::{Session, SessionId, SessionManager, SessionStatus},
    state::{ExecutionContext, ProtocolState, StateTransition},
};

// Test roles and messages for integration testing
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Alice;
impl Role for Alice {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bob;
impl Role for Bob {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct TestMessage {
    content: String,
    timestamp: u64,
}

impl crate::protocol::foundation::Message for TestMessage {}

// Test protocol implementation
#[derive(Debug, Clone)]
struct TestProtocol;
impl LocalProtocol for TestProtocol {}
impl SupportsActionIO<BiDirectionalAction> for TestProtocol {}

/// Integration test: Channel + State machine integration
#[tokio::test]
async fn test_channel_state_integration() -> Result<(), RuntimeError> {
    // Create metadata and channels
    let metadata = CommMetadata::<DefaultChan, HandshakeMsg>::new();
    let config = ChannelConfig {
        buffer_size: 10,
        timeout_ms: Some(1000),
        ordered: true,
    };
    
    let (sender, receiver) = TypedChannel::<TestProtocol, _, _>::create_pair(metadata.clone(), config).await?;
    
    // Create protocol state
    let alice_role = Box::new(Alice);
    let mut state = ProtocolState::new("integration-test-1", alice_role);
    
    // Test state transition with message passing
    let test_msg = TestMessage {
        content: "Hello from integration test".to_string(),
        timestamp: 123456789,
    };
    
    // Create execution context
    let context = ExecutionContext::new("test-context", vec!["alice", "bob"]);
    
    // Transition state and send message concurrently
    let send_task = tokio::spawn(async move {
        sender.send(test_msg).await
    });
    
    let receive_task = tokio::spawn(async move {
        receiver.receive().await
    });
    
    // Apply state transition
    let transition = StateTransition::Progress("Message sent".to_string());
    state.apply_transition(&transition, &context)?;
    
    // Wait for message exchange
    let send_result = send_task.await.map_err(|e| RuntimeError::Communication(
        crate::runtime::error::CommunicationError::ChannelError(e.to_string())
    ))??;
    
    let (received_msg, _) = receive_task.await.map_err(|e| RuntimeError::Communication(
        crate::runtime::error::CommunicationError::ChannelError(e.to_string())
    ))??;
    
    // Verify message integrity
    assert_eq!(received_msg.payload.content, "Hello from integration test");
    assert_eq!(received_msg.payload.timestamp, 123456789);
    
    // Verify state progression
    assert_eq!(state.get_session_id(), "integration-test-1");
    
    Ok(())
}

/// Integration test: Session + Channel + State management
#[tokio::test]
async fn test_session_channel_state_integration() -> Result<(), RuntimeError> {
    // Create session manager
    let mut session_manager = SessionManager::new();
    
    // Create session with async execution
    let session_id = SessionId::generate();
    let alice_role = Box::new(Alice);
    
    // Create channel configuration
    let config = ChannelConfig {
        buffer_size: 5,
        timeout_ms: Some(2000),
        ordered: true,
    };
    
    // Create session with protocol execution
    let session = Session::new(
        session_id.clone(),
        alice_role,
        Box::new(move || {
            Box::pin(async move {
                // Simulate protocol execution
                tokio::time::sleep(Duration::from_millis(100)).await;
                
                // Create metadata and channels for protocol
                let metadata = CommMetadata::<DefaultChan, HandshakeMsg>::new();
                let (sender, receiver) = TypedChannel::<TestProtocol, _, _>::create_pair(metadata, config).await?;
                
                // Exchange messages
                let msg = TestMessage {
                    content: "Session protocol message".to_string(),
                    timestamp: 987654321,
                };
                
                tokio::spawn(async move {
                    sender.send(msg).await
                });
                
                let (received, _) = receiver.receive().await?;
                
                // Verify message
                assert_eq!(received.payload.content, "Session protocol message");
                
                Ok("Protocol completed successfully".to_string())
            })
        })
    );
    
    // Add session to manager
    session_manager.add_session(session).await?;
    
    // Start session execution
    session_manager.start_session(&session_id).await?;
    
    // Wait for completion
    let mut attempts = 0;
    while attempts < 50 {
        let status = session_manager.get_session_status(&session_id).await?;
        match status {
            SessionStatus::Completed => break,
            SessionStatus::Failed(err) => return Err(RuntimeError::ProtocolViolation(
                ProtocolViolation::InvalidTransition(format!("Session failed: {}", err))
            )),
            _ => {
                tokio::time::sleep(Duration::from_millis(50)).await;
                attempts += 1;
            }
        }
    }
    
    // Verify final status
    let final_status = session_manager.get_session_status(&session_id).await?;
    assert_eq!(final_status, SessionStatus::Completed);
    
    Ok(())
}

/// Integration test: Error propagation across runtime components
#[tokio::test]
async fn test_error_propagation_integration() -> Result<(), RuntimeError> {
    let mut session_manager = SessionManager::new();
    let session_id = SessionId::new("error-test-session");
    let alice_role = Box::new(Alice);
    
    // Create session that will fail
    let session = Session::new(
        session_id.clone(),
        alice_role,
        Box::new(|| {
            Box::pin(async move {
                // Simulate a protocol violation
                Err(RuntimeError::ProtocolViolation(
                    ProtocolViolation::InvalidTransition("Simulated protocol error".to_string())
                ))
            })
        })
    );
    
    session_manager.add_session(session).await?;
    session_manager.start_session(&session_id).await?;
    
    // Wait for failure
    let mut attempts = 0;
    while attempts < 20 {
        let status = session_manager.get_session_status(&session_id).await?;
        if matches!(status, SessionStatus::Failed(_)) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
        attempts += 1;
    }
    
    // Verify error propagation
    let final_status = session_manager.get_session_status(&session_id).await?;
    match final_status {
        SessionStatus::Failed(err_msg) => {
            assert!(err_msg.contains("protocol error"));
        }
        _ => panic!("Expected session to fail with error, got: {:?}", final_status),
    }
    
    Ok(())
}

/// Integration test: Multi-session concurrent execution
#[tokio::test]
async fn test_multi_session_integration() -> Result<(), RuntimeError> {
    let mut session_manager = SessionManager::new();
    
    // Create multiple sessions with different roles
    let sessions = vec![
        (SessionId::new("session-1"), Alice),
        (SessionId::new("session-2"), Bob),
        (SessionId::new("session-3"), Alice),
    ];
    
    for (session_id, role) in sessions {
        let role_box = if std::any::TypeId::of::<Alice>() == std::any::TypeId::of::<Alice>() {
            Box::new(Alice) as Box<dyn Role>
        } else {
            Box::new(Bob) as Box<dyn Role>
        };
        
        let session = Session::new(
            session_id.clone(),
            role_box,
            Box::new(move || {
                let id = session_id.clone();
                Box::pin(async move {
                    // Simulate varying execution times
                    let delay = match id.0.as_str() {
                        "session-1" => 50,
                        "session-2" => 100,
                        "session-3" => 75,
                        _ => 100,
                    };
                    
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                    
                    // Create channels and exchange messages
                    let metadata = CommMetadata::<DefaultChan, HandshakeMsg>::new();
                    let config = ChannelConfig::default();
                    let (sender, receiver) = TypedChannel::<TestProtocol, _, _>::create_pair(metadata, config).await?;
                    
                    let msg = TestMessage {
                        content: format!("Message from {}", id.0),
                        timestamp: delay as u64,
                    };
                    
                    tokio::spawn(async move {
                        sender.send(msg).await
                    });
                    
                    let (received, _) = receiver.receive().await?;
                    
                    Ok(format!("Completed: {}", received.payload.content))
                })
            })
        );
        
        session_manager.add_session(session).await?;
    }
    
    // Start all sessions
    let session_ids = vec![
        SessionId::new("session-1"),
        SessionId::new("session-2"), 
        SessionId::new("session-3"),
    ];
    
    for session_id in &session_ids {
        session_manager.start_session(session_id).await?;
    }
    
    // Wait for all to complete
    let timeout_duration = Duration::from_secs(5);
    let completion_check = async {
        loop {
            let mut all_completed = true;
            for session_id in &session_ids {
                let status = session_manager.get_session_status(session_id).await?;
                if !matches!(status, SessionStatus::Completed) {
                    if matches!(status, SessionStatus::Failed(_)) {
                        return Err(RuntimeError::ProtocolViolation(
                            ProtocolViolation::InvalidTransition(format!("Session {} failed", session_id))
                        ));
                    }
                    all_completed = false;
                    break;
                }
            }
            
            if all_completed {
                break;
            }
            
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
        
        Ok::<(), RuntimeError>(())
    };
    
    timeout(timeout_duration, completion_check).await.map_err(|_| {
        RuntimeError::Communication(crate::runtime::error::CommunicationError::Timeout)
    })??;
    
    // Verify all sessions completed
    for session_id in &session_ids {
        let status = session_manager.get_session_status(session_id).await?;
        assert_eq!(status, SessionStatus::Completed);
    }
    
    Ok(())
}

/// Integration test: Channel timeout and recovery
#[tokio::test]
async fn test_channel_timeout_integration() -> Result<(), RuntimeError> {
    let metadata = CommMetadata::<DefaultChan, HandshakeMsg>::new();
    let config = ChannelConfig {
        buffer_size: 1,
        timeout_ms: Some(100), // Very short timeout
        ordered: true,
    };
    
    let (sender, receiver) = TypedChannel::<TestProtocol, _, _>::create_pair(metadata, config).await?;
    
    // Test timeout on receive
    let receive_result = receiver.receive().await;
    
    match receive_result {
        Err(RuntimeError::Communication(crate::runtime::error::CommunicationError::Timeout)) => {
            // Expected timeout
        }
        Ok(_) => panic!("Expected timeout, but receive succeeded"),
        Err(e) => panic!("Expected timeout, got different error: {:?}", e),
    }
    
    // Test that channel is still functional after timeout
    let msg = TestMessage {
        content: "Recovery test".to_string(),
        timestamp: 999,
    };
    
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(50)).await;
        sender.send(msg).await
    });
    
    let (recovered_msg, _) = receiver.receive().await?;
    assert_eq!(recovered_msg.payload.content, "Recovery test");
    
    Ok(())
}

/// Integration test: State transitions with concurrent channel operations
#[tokio::test]
async fn test_concurrent_state_channel_integration() -> Result<(), RuntimeError> {
    let alice_role = Box::new(Alice);
    let mut state = ProtocolState::new("concurrent-test", alice_role);
    let context = ExecutionContext::new("concurrent-context", vec!["alice", "bob"]);
    
    // Create multiple channels
    let mut channels = Vec::new();
    for i in 0..3 {
        let metadata = CommMetadata::<DefaultChan, HandshakeMsg>::new();
        let config = ChannelConfig {
            buffer_size: 5,
            timeout_ms: Some(1000),
            ordered: true,
        };
        
        let (sender, receiver) = TypedChannel::<TestProtocol, _, _>::create_pair(metadata, config).await?;
        channels.push((sender, receiver));
    }
    
    // Spawn concurrent operations
    let mut tasks = Vec::new();
    
    for (i, (sender, receiver)) in channels.into_iter().enumerate() {
        let task = tokio::spawn(async move {
            let msg = TestMessage {
                content: format!("Concurrent message {}", i),
                timestamp: i as u64,
            };
            
            // Send and receive concurrently
            let send_handle = tokio::spawn(async move {
                sender.send(msg).await
            });
            
            let receive_handle = tokio::spawn(async move {
                receiver.receive().await
            });
            
            let (send_result, receive_result) = tokio::join!(send_handle, receive_handle);
            
            let _ = send_result??;
            let (received, _) = receive_result??;
            
            Ok::<_, RuntimeError>(received.payload.content)
        });
        
        tasks.push(task);
    }
    
    // Apply state transitions concurrently
    for i in 0..3 {
        let transition = StateTransition::Progress(format!("Concurrent operation {}", i));
        state.apply_transition(&transition, &context)?;
    }
    
    // Wait for all channel operations
    let mut results = Vec::new();
    for task in tasks {
        let result = task.await.map_err(|e| RuntimeError::Communication(
            crate::runtime::error::CommunicationError::ChannelError(e.to_string())
        ))??;
        results.push(result);
    }
    
    // Verify all operations completed
    assert_eq!(results.len(), 3);
    for (i, result) in results.iter().enumerate() {
        assert_eq!(*result, format!("Concurrent message {}", i));
    }
    
    Ok(())
}
