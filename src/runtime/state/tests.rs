//! Unit tests for protocol state machine
//!
//! This module provides comprehensive unit tests for the protocol state machine,
//! focusing on state transitions, execution context management, recursion handling,
//! and the StateManager coordination system.

#![cfg(test)]

use super::*;
use crate::protocol::foundation::{Alice, Bob, TChanEnd};
use crate::runtime::error::{ProtocolViolation, RuntimeError};
use std::time::Duration;
use tokio::time::sleep;

// Test roles for state testing
#[derive(Debug, Clone, PartialEq, Eq)]
struct TestRole;
impl Role for TestRole {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AnotherRole;
impl Role for AnotherRole {}

// Basic ProtocolState creation and property tests
#[test]
fn test_protocol_state_creation() {
    let protocol = TChanEnd::new();
    let state = ProtocolState::new("test-session", Box::new(Alice), protocol);
    
    assert_eq!(state.session_id(), "test-session");
    assert!(!state.is_complete());
    assert_eq!(state.step_count(), 0);
    assert!(state.elapsed().as_nanos() > 0);
}

#[test]
fn test_protocol_state_properties() {
    let protocol = TChanEnd::new();
    let state = ProtocolState::new("properties-test", Box::new(Bob), protocol.clone());
    
    // Verify immutable access to protocol
    assert_eq!(state.current_protocol(), &protocol);
    
    // Verify initial timestamps
    assert!(state.elapsed().as_nanos() >= 0);
    
    // Verify context access
    let context = state.context();
    assert_eq!(context.session_id(), "properties-test");
    assert_eq!(context.recursion_depth(), 0);
}

#[test]
fn test_protocol_state_activity_tracking() {
    let protocol = TChanEnd::new();
    let mut state = ProtocolState::new("activity-test", Box::new(Alice), protocol);
    
    let initial_steps = state.step_count();
    
    // Update activity
    state.update_activity();
    assert_eq!(state.step_count(), initial_steps + 1);
    
    // Multiple updates
    state.update_activity();
    state.update_activity();
    assert_eq!(state.step_count(), initial_steps + 3);
}

// Protocol completion tests
#[test]
fn test_protocol_state_completion() {
    let protocol = TChanEnd::new();
    let mut state = ProtocolState::new("completion-test", Box::new(Alice), protocol);
    
    assert!(!state.is_complete());
    
    // First completion should succeed
    assert!(state.mark_complete().is_ok());
    assert!(state.is_complete());
    assert_eq!(state.step_count(), 1);
    
    // Second completion should fail
    let result = state.mark_complete();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        RuntimeError::Protocol(ProtocolViolation::SessionTerminated { .. })
    ));
}

#[test] 
fn test_protocol_state_completion_step_increment() {
    let protocol = TChanEnd::new();
    let mut state = ProtocolState::new("step-test", Box::new(Bob), protocol);
    
    let initial_steps = state.step_count();
    
    state.mark_complete().unwrap();
    assert_eq!(state.step_count(), initial_steps + 1);
}

// Protocol state transition tests
#[test]
fn test_protocol_state_transition_success() {
    let initial_protocol = TChanEnd::new();
    let state = ProtocolState::new("transition-test", Box::new(Alice), initial_protocol);
    
    let new_protocol = TChanEnd::new();
    let new_state = state.transition(new_protocol.clone()).unwrap();
    
    assert_eq!(new_state.session_id(), "transition-test");
    assert_eq!(new_state.current_protocol(), &new_protocol);
    assert!(!new_state.is_complete());
    assert_eq!(new_state.step_count(), 1); // Incremented during transition
}

#[test]
fn test_protocol_state_transition_from_completed() {
    let protocol = TChanEnd::new();
    let mut state = ProtocolState::new("completed-transition-test", Box::new(Bob), protocol);
    
    // Complete the state first
    state.mark_complete().unwrap();
    
    // Transition should fail
    let new_protocol = TChanEnd::new();
    let result = state.transition(new_protocol);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        RuntimeError::Protocol(ProtocolViolation::SessionTerminated { .. })
    ));
}

#[test]
fn test_protocol_state_transition_preserves_context() {
    let initial_protocol = TChanEnd::new();
    let state = ProtocolState::new("context-preserve-test", Box::new(TestRole), initial_protocol);
    
    // Get initial context data
    let initial_session_id = state.context().session_id().to_string();
    let initial_role = state.context().role().to_string();
    
    let new_protocol = TChanEnd::new();
    let new_state = state.transition(new_protocol).unwrap();
    
    // Context should be preserved
    assert_eq!(new_state.context().session_id(), initial_session_id);
    assert_eq!(new_state.context().role(), initial_role);
}

// ExecutionContext tests
#[test]
fn test_execution_context_creation() {
    let context = ExecutionContext::new("context-test".to_string(), Box::new(Bob));
    
    assert_eq!(context.session_id(), "context-test");
    assert_eq!(context.recursion_depth(), 0);
    assert_eq!(context.max_recursion_depth(), 100); // Default
    assert!(context.metadata().is_empty());
    assert!(context.elapsed().as_nanos() >= 0);
}

#[test]
fn test_execution_context_role_storage() {
    let context = ExecutionContext::new("role-test".to_string(), Box::new(AnotherRole));
    
    // Role should be stored as debug string
    assert!(context.role().contains("AnotherRole"));
}

#[test]
fn test_execution_context_metadata_operations() {
    let mut context = ExecutionContext::new("metadata-test".to_string(), Box::new(Alice));
    
    // Initially empty
    assert!(context.metadata().is_empty());
    
    // Add metadata
    {
        let metadata = context.metadata_mut();
        metadata.insert("key1".to_string(), "value1".to_string());
        metadata.insert("key2".to_string(), "value2".to_string());
    }
    
    // Verify metadata
    let metadata = context.metadata();
    assert_eq!(metadata.len(), 2);
    assert_eq!(metadata.get("key1"), Some(&"value1".to_string()));
    assert_eq!(metadata.get("key2"), Some(&"value2".to_string()));
}

// Recursion management tests
#[test]
fn test_execution_context_recursion_basic() {
    let mut context = ExecutionContext::new("recursion-test".to_string(), Box::new(Bob));
    
    assert_eq!(context.recursion_depth(), 0);
    
    // Enter recursion
    assert!(context.enter_recursion().is_ok());
    assert_eq!(context.recursion_depth(), 1);
    
    // Enter deeper
    assert!(context.enter_recursion().is_ok());
    assert_eq!(context.recursion_depth(), 2);
    
    // Exit recursion
    context.exit_recursion();
    assert_eq!(context.recursion_depth(), 1);
    
    context.exit_recursion();
    assert_eq!(context.recursion_depth(), 0);
}

#[test]
fn test_execution_context_recursion_depth_limit() {
    let mut context = ExecutionContext::new("depth-limit-test".to_string(), Box::new(Alice));
    context.set_max_recursion_depth(2);
    
    assert_eq!(context.max_recursion_depth(), 2);
    
    // Should succeed within limit
    assert!(context.enter_recursion().is_ok()); // depth 1
    assert!(context.enter_recursion().is_ok()); // depth 2
    
    // Should fail when exceeding limit
    let result = context.enter_recursion(); // depth 3
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        RuntimeError::Protocol(ProtocolViolation::RecursionDepthExceeded { .. })
    ));
    
    // Depth should remain at limit
    assert_eq!(context.recursion_depth(), 3); // Still incremented even though it failed
}

#[test]
fn test_execution_context_exit_recursion_underflow_safe() {
    let mut context = ExecutionContext::new("underflow-test".to_string(), Box::new(Bob));
    
    // Exit when already at 0 should be safe
    assert_eq!(context.recursion_depth(), 0);
    context.exit_recursion();
    assert_eq!(context.recursion_depth(), 0);
    
    // Multiple exits should be safe
    context.exit_recursion();
    context.exit_recursion();
    assert_eq!(context.recursion_depth(), 0);
}

#[test]
fn test_execution_context_custom_recursion_limit() {
    let mut context = ExecutionContext::new("custom-limit-test".to_string(), Box::new(TestRole));
    
    // Set very low limit
    context.set_max_recursion_depth(1);
    assert_eq!(context.max_recursion_depth(), 1);
    
    // First should succeed
    assert!(context.enter_recursion().is_ok());
    
    // Second should fail
    let result = context.enter_recursion();
    assert!(result.is_err());
}

// AsyncProtocolMachine tests
#[test]
fn test_async_protocol_machine_creation() {
    let initial_state = "initial";
    let context = ExecutionContext::new("machine-test".to_string(), Box::new(Alice));
    
    let machine = AsyncProtocolMachine::new(initial_state, context);
    
    assert_eq!(machine.state(), &"initial");
    assert_eq!(machine.context().session_id(), "machine-test");
}

#[test]
fn test_async_protocol_machine_state_transition() {
    let initial_state = "initial";
    let context = ExecutionContext::new("transition-test".to_string(), Box::new(Bob));
    
    let machine = AsyncProtocolMachine::new(initial_state, context);
    let new_machine = machine.transition("new_state");
    
    assert_eq!(new_machine.state(), &"new_state");
    assert_eq!(new_machine.context().session_id(), "transition-test");
}

#[test]
fn test_async_protocol_machine_channel_management() {
    let initial_state = 42u32;
    let context = ExecutionContext::new("channel-test".to_string(), Box::new(TestRole));
    
    let mut machine = AsyncProtocolMachine::new(initial_state, context);
    
    // Add channel
    let channel_data = String::from("test_channel");
    machine.add_channel("test".to_string(), Box::new(channel_data));
    
    // Retrieve channel
    let retrieved: Option<&String> = machine.get_channel("test");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), "test_channel");
    
    // Remove channel
    let removed = machine.remove_channel("test");
    assert!(removed.is_some());
    
    // Should be gone now
    let not_found: Option<&String> = machine.get_channel("test");
    assert!(not_found.is_none());
}

#[test]
fn test_async_protocol_machine_context_mutation() {
    let initial_state = 100i32;
    let context = ExecutionContext::new("mutation-test".to_string(), Box::new(Alice));
    
    let mut machine = AsyncProtocolMachine::new(initial_state, context);
    
    // Mutate context
    {
        let context_mut = machine.context_mut();
        context_mut.metadata_mut().insert("test_key".to_string(), "test_value".to_string());
    }
    
    // Verify mutation
    assert_eq!(
        machine.context().metadata().get("test_key"), 
        Some(&"test_value".to_string())
    );
}

// StateManager tests
#[tokio::test]
async fn test_state_manager_creation() {
    let manager = StateManager::new();
    assert_eq!(manager.session_count().await, 0);
    
    let default_manager = StateManager::default();
    assert_eq!(default_manager.session_count().await, 0);
}

#[tokio::test]
async fn test_state_manager_session_lifecycle() {
    let manager = StateManager::new();
    let protocol = TChanEnd::new();
    let state = ProtocolState::new("lifecycle-test", Box::new(Alice), protocol);
    
    // Initially empty
    assert_eq!(manager.session_count().await, 0);
    assert!(manager.list_sessions().await.is_empty());
    
    // Add session
    manager.add_session("lifecycle-test".to_string(), state).await;
    assert_eq!(manager.session_count().await, 1);
    
    let sessions = manager.list_sessions().await;
    assert_eq!(sessions, vec!["lifecycle-test"]);
    
    // Remove session
    assert!(manager.remove_session("lifecycle-test").await);
    assert_eq!(manager.session_count().await, 0);
    
    // Remove non-existent session
    assert!(!manager.remove_session("lifecycle-test").await);
}

#[tokio::test]
async fn test_state_manager_multiple_sessions() {
    let manager = StateManager::new();
    
    // Add multiple sessions
    for i in 1..=5 {
        let protocol = TChanEnd::new();
        let state = ProtocolState::new(format!("session-{}", i), Box::new(Bob), protocol);
        manager.add_session(format!("session-{}", i), state).await;
    }
    
    assert_eq!(manager.session_count().await, 5);
    
    let sessions = manager.list_sessions().await;
    assert_eq!(sessions.len(), 5);
    
    // Verify all sessions are present
    for i in 1..=5 {
        assert!(sessions.contains(&format!("session-{}", i)));
    }
}

#[tokio::test]
async fn test_state_manager_get_session() {
    let manager = StateManager::new();
    let protocol = TChanEnd::new();
    let original_state = ProtocolState::new("get-test", Box::new(Alice), protocol.clone());
    
    // Add session
    manager.add_session("get-test".to_string(), original_state).await;
    
    // Retrieve session
    let retrieved_state: Option<ProtocolState<TChanEnd>> = manager.get_session("get-test").await;
    assert!(retrieved_state.is_some());
    
    let state = retrieved_state.unwrap();
    assert_eq!(state.session_id(), "get-test");
    assert_eq!(state.current_protocol(), &protocol);
    
    // Try to get non-existent session
    let not_found: Option<ProtocolState<TChanEnd>> = manager.get_session("not-found").await;
    assert!(not_found.is_none());
}

#[tokio::test]
async fn test_state_manager_concurrent_access() {
    let manager = StateManager::new();
    
    // Spawn multiple tasks that add sessions concurrently
    let mut handles = vec![];
    
    for i in 0..10 {
        let manager_clone = std::sync::Arc::new(&manager);
        let handle = tokio::spawn(async move {
            let protocol = TChanEnd::new();
            let state = ProtocolState::new(format!("concurrent-{}", i), Box::new(TestRole), protocol);
            manager_clone.add_session(format!("concurrent-{}", i), state).await;
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
    
    // All sessions should be added
    assert_eq!(manager.session_count().await, 10);
}

// Error condition tests
#[test]
fn test_protocol_violation_error_types() {
    let protocol = TChanEnd::new();
    let mut state = ProtocolState::new("error-test", Box::new(Alice), protocol);
    
    // Complete state
    state.mark_complete().unwrap();
    
    // Try operations that should fail
    let completion_error = state.mark_complete().unwrap_err();
    assert!(matches!(
        completion_error,
        RuntimeError::Protocol(ProtocolViolation::SessionTerminated { .. })
    ));
    
    // Test recursion depth error
    let mut context = ExecutionContext::new("recursion-error-test".to_string(), Box::new(Bob));
    context.set_max_recursion_depth(0);
    
    let recursion_error = context.enter_recursion().unwrap_err();
    assert!(matches!(
        recursion_error,
        RuntimeError::Protocol(ProtocolViolation::RecursionDepthExceeded { .. })
    ));
}

// Performance and timing tests
#[tokio::test]
async fn test_execution_context_timing() {
    let context = ExecutionContext::new("timing-test".to_string(), Box::new(Alice));
    
    let initial_elapsed = context.elapsed();
    
    // Wait a small amount
    sleep(Duration::from_millis(10)).await;
    
    let later_elapsed = context.elapsed();
    assert!(later_elapsed > initial_elapsed);
}

#[test]
fn test_protocol_state_timing() {
    let protocol = TChanEnd::new();
    let mut state = ProtocolState::new("timing-test", Box::new(Bob), protocol);
    
    let initial_elapsed = state.elapsed();
    
    // Perform some operations
    state.update_activity();
    
    let later_elapsed = state.elapsed();
    assert!(later_elapsed >= initial_elapsed);
}
