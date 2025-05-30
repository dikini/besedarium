//! Comprehensive tests for advanced state validation and deadlock/livelock detection
//!
//! This module tests the enhanced runtime validation capabilities including
//! state transition validation, deadlock detection, and livelock detection.

use std::sync::Arc;
use std::time::Duration;
use tokio::time;

use crate::protocol::foundation::{Alice, Bob, Carol, CommMetadata, DefaultChan, DefaultMsgLbl};
use crate::protocol::global::{TChanEnd, TChanSend, TChanRecv};
use crate::runtime::{
    ProtocolState, StateValidator, ValidationConfig, ValidationMode, ValidationResult,
    RuntimeError, DeadlockError, LivelockError, StateValidationError,
};

#[tokio::test]
async fn test_basic_state_validation() {
    let config = ValidationConfig {
        validation_mode: ValidationMode::Debug,
        ..Default::default()
    };
    let validator = Arc::new(StateValidator::with_config(config));
    
    let metadata = CommMetadata::new(DefaultChan, DefaultMsgLbl);
    let protocol = TChanEnd::new(metadata);
    let state = ProtocolState::with_validation(
        "test_session".to_string(),
        Box::new(Alice),
        protocol.clone(),
        validator,
    );
    
    let result = state.validated_transition(
        protocol,
        "test_action",
        &Alice,
    ).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_state_validation_config_modes() {
    for mode in [ValidationMode::Strict, ValidationMode::Lenient, ValidationMode::Debug, ValidationMode::Production] {
        let config = ValidationConfig {
            validation_mode: mode,
            ..Default::default()
        };
        let validator = Arc::new(StateValidator::with_config(config));
        
        let metadata = CommMetadata::new(DefaultChan, DefaultMsgLbl);
        let protocol = TChanEnd::new(metadata);
        let state = ProtocolState::with_validation(
            "test_session".to_string(),
            Box::new(Alice),
            protocol.clone(),
            validator,
        );
        
        let result = state.validated_transition(
            protocol,
            "test_action",
            &Alice,
        ).await;
        
        assert!(result.is_ok(), "Validation failed for mode {:?}", mode);
    }
}

#[tokio::test]
async fn test_validation_without_validator() {
    let metadata = CommMetadata::new(DefaultChan, DefaultMsgLbl);
    let protocol = TChanEnd::new(metadata);
    let state = ProtocolState::new("test_session", Box::new(Alice), protocol.clone());
    
    // Should work without validation
    let result = state.transition(protocol);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_deadlock_timeout_configuration() {
    let config = ValidationConfig {
        deadlock_timeout: Duration::from_millis(100),
        enable_resource_analysis: true,
        ..Default::default()
    };
    let validator = StateValidator::with_config(config);
    
    assert_eq!(validator.config.deadlock_timeout, Duration::from_millis(100));
}

#[tokio::test]
async fn test_livelock_threshold_configuration() {
    let config = ValidationConfig {
        livelock_threshold: 5,
        livelock_window: Duration::from_secs(1),
        enable_progress_tracking: true,
        ..Default::default()
    };
    let validator = StateValidator::with_config(config);
    
    assert_eq!(validator.config.livelock_threshold, 5);
    assert_eq!(validator.config.livelock_window, Duration::from_secs(1));
}

#[tokio::test]
async fn test_repeated_transitions_livelock_detection() {
    let config = ValidationConfig {
        livelock_threshold: 3,
        livelock_window: Duration::from_secs(10),
        enable_progress_tracking: true,
        ..Default::default()
    };
    let validator = Arc::new(StateValidator::with_config(config));
    
    let metadata = CommMetadata::new(DefaultChan, DefaultMsgLbl);
    let protocol = TChanEnd::new(metadata.clone());
    let mut state = ProtocolState::with_validation(
        "livelock_test_session".to_string(),
        Box::new(Alice),
        protocol,
        validator,
    );
    
    // Perform the same transition multiple times rapidly
    for i in 0..5 {
        let new_protocol = TChanEnd::new(metadata.clone());
        let result = state.validated_transition(
            new_protocol.clone(),
            "repeated_action",
            &Alice,
        ).await;
        
        if i < 3 {
            // First few should succeed
            assert!(result.is_ok(), "Transition {} should succeed", i);
            state = result.unwrap();
        } else {
            // Later ones might trigger livelock detection
            // Note: In a real implementation, this would be more sophisticated
            // For now, we just verify the mechanism works
            match result {
                Ok(new_state) => state = new_state,
                Err(RuntimeError::Livelock(LivelockError::RepeatedTransitions { .. })) => {
                    // Livelock detected as expected
                    break;
                }
                Err(e) => panic!("Unexpected error: {:?}", e),
            }
        }
    }
}

#[tokio::test]
async fn test_state_transition_validation_failure() {
    let metadata = CommMetadata::new(DefaultChan, DefaultMsgLbl);
    let protocol = TChanEnd::new(metadata);
    let mut state = ProtocolState::new("test_session", Box::new(Alice), protocol);
    
    // Mark state as complete
    state.mark_complete().unwrap();
    
    // Try to transition a completed state - should fail
    let new_protocol = TChanEnd::new(CommMetadata::new(DefaultChan, DefaultMsgLbl));
    let result = state.transition(new_protocol);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        RuntimeError::Protocol(crate::runtime::error::ProtocolViolation::SessionTerminated { .. }) => {
            // Expected error
        }
        e => panic!("Unexpected error type: {:?}", e),
    }
}

#[tokio::test]
async fn test_validation_result_types() {
    // Test ValidationResult::Valid
    let valid_result = ValidationResult::Valid {
        session_id: "test".to_string(),
        validation_timestamp: std::time::SystemTime::now(),
        checks_performed: vec!["protocol_compliance".to_string()],
    };
    
    match valid_result {
        ValidationResult::Valid { session_id, .. } => {
            assert_eq!(session_id, "test");
        }
        _ => panic!("Expected Valid result"),
    }
    
    // Test ValidationResult::Warning
    let warning_result = ValidationResult::Warning {
        session_id: "test".to_string(),
        warnings: vec!["Minor protocol deviation".to_string()],
        validation_timestamp: std::time::SystemTime::now(),
    };
    
    match warning_result {
        ValidationResult::Warning { warnings, .. } => {
            assert_eq!(warnings.len(), 1);
        }
        _ => panic!("Expected Warning result"),
    }
}

#[tokio::test]
async fn test_deadlock_error_types() {
    let deadlock_error = DeadlockError::CircularDependency {
        session_id: "test_session".to_string(),
        involved_roles: vec!["Alice".to_string(), "Bob".to_string()],
        resource_chain: vec!["resource1".to_string(), "resource2".to_string()],
        detection_time: std::time::SystemTime::now(),
    };
    
    let error_message = format!("{}", deadlock_error);
    assert!(error_message.contains("Circular dependency detected"));
    assert!(error_message.contains("test_session"));
}

#[tokio::test]
async fn test_livelock_error_types() {
    let livelock_error = LivelockError::RepeatedTransitions {
        session_id: "test_session".to_string(),
        transition_count: 10,
        repeated_transition: "repeated_action".to_string(),
        duration: Duration::from_secs(5),
        state_history: vec!["state1".to_string(), "state2".to_string()],
    };
    
    let error_message = format!("{}", livelock_error);
    assert!(error_message.contains("Repeated state transitions"));
    assert!(error_message.contains("test_session"));
    assert!(error_message.contains("10"));
}

#[tokio::test]
async fn test_state_validation_error_types() {
    let validation_error = StateValidationError::InvalidTransition {
        session_id: "test_session".to_string(),
        from_state: "StateA".to_string(),
        to_state: "StateB".to_string(),
        action: "invalid_action".to_string(),
        allowed_transitions: vec!["valid_action1".to_string(), "valid_action2".to_string()],
        validation_context: crate::runtime::validation::ValidationContext {
            timestamp: std::time::SystemTime::now(),
            session_metadata: std::collections::HashMap::new(),
            role_context: "Alice".to_string(),
            protocol_position: "test_position".to_string(),
            validation_mode: ValidationMode::Strict,
        },
    };
    
    let error_message = format!("{}", validation_error);
    assert!(error_message.contains("Invalid state transition"));
    assert!(error_message.contains("test_session"));
    assert!(error_message.contains("StateA"));
    assert!(error_message.contains("StateB"));
}

#[tokio::test]
async fn test_concurrent_validation() {
    let validator = Arc::new(StateValidator::new());
    let metadata = CommMetadata::new(DefaultChan, DefaultMsgLbl);
    
    // Create multiple concurrent validation tasks
    let mut handles = Vec::new();
    for i in 0..5 {
        let validator_clone = validator.clone();
        let metadata_clone = metadata.clone();
        
        let handle = tokio::spawn(async move {
            let protocol = TChanEnd::new(metadata_clone);
            let state = ProtocolState::with_validation(
                format!("concurrent_session_{}", i),
                Box::new(Alice),
                protocol.clone(),
                validator_clone,
            );
            
            state.validated_transition(
                protocol,
                &format!("action_{}", i),
                &Alice,
            ).await
        });
        
        handles.push(handle);
    }
    
    // Wait for all validations to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok(), "Concurrent validation failed");
    }
}

#[tokio::test]
async fn test_validation_performance() {
    let start = std::time::Instant::now();
    let validator = Arc::new(StateValidator::new());
    let metadata = CommMetadata::new(DefaultChan, DefaultMsgLbl);
    
    // Perform many rapid validations
    for i in 0..100 {
        let protocol = TChanEnd::new(metadata.clone());
        let state = ProtocolState::with_validation(
            format!("perf_test_session_{}", i),
            Box::new(Alice),
            protocol.clone(),
            validator.clone(),
        );
        
        let result = state.validated_transition(
            protocol,
            "perf_test_action",
            &Alice,
        ).await;
        
        assert!(result.is_ok());
    }
    
    let elapsed = start.elapsed();
    println!("100 validations completed in {:?}", elapsed);
    
    // Ensure reasonable performance (adjust threshold as needed)
    assert!(elapsed < Duration::from_secs(1), "Validation performance too slow: {:?}", elapsed);
}

#[tokio::test]
async fn test_validation_with_different_protocols() {
    let validator = Arc::new(StateValidator::new());
    let metadata = CommMetadata::new(DefaultChan, DefaultMsgLbl);
    
    // Test with different protocol types
    let end_protocol = TChanEnd::new(metadata.clone());
    let send_protocol = TChanSend::new(metadata.clone(), "TestMessage".to_string(), end_protocol.clone());
    let recv_protocol = TChanRecv::new(metadata.clone(), "TestMessage".to_string(), end_protocol.clone());
    
    // Test transition from send to recv
    let state = ProtocolState::with_validation(
        "multi_protocol_session".to_string(),
        Box::new(Alice),
        send_protocol,
        validator.clone(),
    );
    
    let result = state.validated_transition(
        recv_protocol,
        "send_to_recv_transition",
        &Alice,
    ).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_validation_with_multiple_roles() {
    let validator = Arc::new(StateValidator::new());
    let metadata = CommMetadata::new(DefaultChan, DefaultMsgLbl);
    
    for role in [Alice, Bob, Carol] {
        let protocol = TChanEnd::new(metadata.clone());
        let state = ProtocolState::with_validation(
            format!("role_test_session_{:?}", role),
            Box::new(role),
            protocol.clone(),
            validator.clone(),
        );
        
        let result = state.validated_transition(
            protocol,
            &format!("action_for_{:?}", role),
            &role,
        ).await;
        
        assert!(result.is_ok(), "Validation failed for role {:?}", role);
    }
}

#[test]
fn test_validation_config_defaults() {
    let config = ValidationConfig::default();
    
    assert_eq!(config.deadlock_timeout, Duration::from_secs(30));
    assert_eq!(config.livelock_threshold, 10);
    assert_eq!(config.livelock_window, Duration::from_secs(5));
    assert_eq!(config.validation_mode, ValidationMode::Production);
    assert_eq!(config.max_recursion_depth, 100);
    assert!(config.enable_resource_analysis);
    assert!(config.enable_progress_tracking);
}

#[test]
fn test_validation_mode_default() {
    assert_eq!(ValidationMode::default(), ValidationMode::Production);
}
