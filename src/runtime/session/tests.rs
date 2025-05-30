//! Tests for enhanced session lifecycle management
//!
//! This module contains comprehensive tests for graceful shutdown, resource leak detection,
//! and enhanced session lifecycle management features.

use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::time::sleep;

use super::*;
use crate::protocol::foundation::{BiDirectionalAction, CommMetadata, DefaultChan, MsgLbl};
use crate::protocol::local::EpChanEnd;

// Test types
#[derive(Debug, Clone, PartialEq, Eq)]
struct Alice;
impl Role for Alice {}
impl SupportsActionIO<BiDirectionalAction> for Alice {}
impl Hash for Alice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Since Alice is a unit struct with a single value, we can use a constant value
        "Alice".hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bob;
impl Role for Bob {}
impl SupportsActionIO<BiDirectionalAction> for Bob {}
impl Hash for Bob {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Since Bob is a unit struct with a single value, we can use a constant value
        "Bob".hash(state);
    }
}

// Dummy IO type for tests
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct TestIO;
impl SupportsActionIO<BiDirectionalAction> for TestIO {}

// Define HandshakeLabel for tests
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct HandshakeLabel;
impl MsgLbl for HandshakeLabel {}

type TestMetadata = CommMetadata<DefaultChan, HandshakeLabel>;
type TestProtocol = EpChanEnd<TestIO, TestMetadata, BiDirectionalAction>;

#[tokio::test]
async fn test_session_creation_with_shutdown_config() {
    let id = SessionId::new("test-session");
    let protocol = TestProtocol::new();
    let role = Alice;
    let config = ChannelConfig::default();
    let shutdown_config = ShutdownConfig {
        graceful_shutdown_timeout: Duration::from_secs(10),
        critical_operations_timeout: Duration::from_secs(2),
        force_task_termination: true,
        strict_leak_detection: false,
    };

    let (session, _channel) =
        Session::new_with_config(id.clone(), protocol, role, config, shutdown_config);

    assert_eq!(session.id(), &id);
    assert_eq!(session.status().await, SessionStatus::Initializing);
    assert_eq!(
        session.shutdown_config.graceful_shutdown_timeout,
        Duration::from_secs(10)
    );
    assert_eq!(session.shutdown_config.force_task_termination, true);
}

#[tokio::test]
async fn test_resource_tracking() {
    let id = SessionId::new("test-session");
    let (session, _channel) =
        Session::new(id, TestProtocol::new(), Alice, ChannelConfig::default());

    // Track some resources
    session
        .track_resource("channel-1".to_string(), ResourceType::Channel)
        .await;
    session
        .track_resource("task-1".to_string(), ResourceType::Task)
        .await;
    session
        .track_resource("connection-1".to_string(), ResourceType::Connection)
        .await;

    // Check that resources are tracked
    let resources = session.tracked_resources.read().await;
    assert_eq!(resources.len(), 3);
    assert!(resources.contains_key("channel-1"));
    assert!(resources.contains_key("task-1"));
    assert!(resources.contains_key("connection-1"));

    // Close a resource
    drop(resources);
    session.close_resource("channel-1").await;

    // Check that resource is marked as closed
    let resources = session.tracked_resources.read().await;
    let channel_resource = resources.get("channel-1").unwrap();
    assert!(channel_resource.is_closed);
}

#[tokio::test]
async fn test_leak_detection_no_leaks() {
    let id = SessionId::new("test-session");
    let (session, _channel) = Session::new(
        id.clone(),
        TestProtocol::new(),
        Alice,
        ChannelConfig::default(),
    );

    // Track and close resources properly
    session
        .track_resource("channel-1".to_string(), ResourceType::Channel)
        .await;
    session
        .track_resource("task-1".to_string(), ResourceType::Task)
        .await;
    session.close_resource("channel-1").await;
    session.close_resource("task-1").await;

    // Perform leak detection
    let report = session.detect_leaks().await.unwrap();

    assert_eq!(report.session_id, id);
    assert!(!report.has_leaks());
    assert_eq!(report.leak_count(), 0);
    assert_eq!(report.total_resources_created, 2);
    assert_eq!(report.total_resources_closed, 2);
}

#[tokio::test]
async fn test_leak_detection_with_leaks() {
    let id = SessionId::new("test-session");
    let (session, _channel) = Session::new(
        id.clone(),
        TestProtocol::new(),
        Alice,
        ChannelConfig::default(),
    );

    // Track resources but don't close them all
    session
        .track_resource("channel-1".to_string(), ResourceType::Channel)
        .await;
    session
        .track_resource("task-1".to_string(), ResourceType::Task)
        .await;
    session
        .track_resource("connection-1".to_string(), ResourceType::Connection)
        .await;

    // Only close one resource
    session.close_resource("channel-1").await;

    // Perform leak detection
    let report = session.detect_leaks().await.unwrap();

    assert_eq!(report.session_id, id);
    assert!(report.has_leaks());
    assert_eq!(report.leak_count(), 2); // task-1 and connection-1 are leaked
    assert_eq!(report.total_resources_created, 3);
    assert_eq!(report.total_resources_closed, 1);

    // Check that the correct resources are reported as leaked
    let leaked_ids: Vec<&String> = report
        .leaked_resources
        .iter()
        .map(|r| &r.resource_id)
        .collect();
    assert!(leaked_ids.contains(&&"task-1".to_string()));
    assert!(leaked_ids.contains(&&"connection-1".to_string()));
    assert!(!leaked_ids.contains(&&"channel-1".to_string()));
}

#[tokio::test]
async fn test_graceful_shutdown_completed_session() {
    let id = SessionId::new("test-session");
    let (session, _channel) =
        Session::new(id, TestProtocol::new(), Alice, ChannelConfig::default());

    // Start and immediately simulate completion
    session.start().await.unwrap();

    // Manually set status to completed for testing
    {
        let mut status = session.status.write().await;
        *status = SessionStatus::Completed;
    }

    // Graceful shutdown should succeed immediately
    let result = session.shutdown().await;
    assert!(result.is_ok());
    assert_eq!(session.status().await, SessionStatus::Completed);
}

#[tokio::test]
async fn test_graceful_shutdown_running_session() {
    let id = SessionId::new("test-session");
    let (session, _channel) =
        Session::new(id, TestProtocol::new(), Alice, ChannelConfig::default());

    // Start the session
    session.start().await.unwrap();
    assert_eq!(session.status().await, SessionStatus::Running);

    // Track some resources
    session
        .track_resource("test-channel".to_string(), ResourceType::Channel)
        .await;
    session
        .track_resource("test-task".to_string(), ResourceType::Task)
        .await;

    // Initiate graceful shutdown
    let result = session.shutdown().await;
    assert!(result.is_ok());

    // Session should be in a final state
    let final_status = session.status().await;
    assert!(matches!(
        final_status,
        SessionStatus::Completed | SessionStatus::Cancelled | SessionStatus::Failed(_)
    ));
}

#[tokio::test]
async fn test_shutdown_timeout() {
    let id = SessionId::new("test-session");
    let shutdown_config = ShutdownConfig {
        graceful_shutdown_timeout: Duration::from_millis(100), // Very short timeout
        critical_operations_timeout: Duration::from_millis(50),
        force_task_termination: true,
        strict_leak_detection: false,
    };

    let (session, _channel) = Session::new_with_config(
        id,
        TestProtocol::new(),
        Alice,
        ChannelConfig::default(),
        shutdown_config,
    );

    // Start the session
    session.start().await.unwrap();

    // Give the execution loop time to start running and begin processing
    sleep(Duration::from_millis(150)).await;

    // Initiate shutdown - this should timeout and force shutdown
    let result = session.shutdown().await;
    assert!(result.is_ok()); // Force shutdown should still succeed

    // Session should be cancelled after forced shutdown
    assert_eq!(session.status().await, SessionStatus::Cancelled);
}

#[tokio::test]
async fn test_session_metrics() {
    let id = SessionId::new("test-session");
    let (session, _channel) = Session::new(
        id.clone(),
        TestProtocol::new(),
        Alice,
        ChannelConfig::default(),
    );

    // Track some resources
    session
        .track_resource("channel-1".to_string(), ResourceType::Channel)
        .await;
    session
        .track_resource("task-1".to_string(), ResourceType::Task)
        .await;

    // Simulate some activity
    session.update_activity().await;
    sleep(Duration::from_millis(10)).await;

    let metrics = session.get_metrics().await;

    assert_eq!(metrics.session_id, id);
    assert_eq!(metrics.status, SessionStatus::Initializing);
    assert_eq!(metrics.total_resources, 2);
    assert!(metrics.uptime.as_millis() >= 10);
    assert!(metrics.last_activity >= metrics.created_at);
}

#[tokio::test]
async fn test_session_manager_creation() {
    let manager = SessionManager::default();

    let id1 = SessionId::new("session-1");
    let id2 = SessionId::new("session-2");

    // Create sessions
    let (session1, _ch1) = manager
        .create_session(
            id1.clone(),
            TestProtocol::new(),
            Alice,
            ChannelConfig::default(),
        )
        .await
        .unwrap();

    let (session2, _ch2) = manager
        .create_session(
            id2.clone(),
            TestProtocol::new(),
            Alice,
            ChannelConfig::default(),
        )
        .await
        .unwrap();

    // Check total sessions
    assert_eq!(manager.total_sessions().await, 2);

    // Check session IDs
    let session_ids = manager.list_session_ids().await;
    assert!(session_ids.contains(&id1));
    assert!(session_ids.contains(&id2));

    // Try to create duplicate session
    let result = manager
        .create_session(
            id1.clone(),
            TestProtocol::new(),
            Alice,
            ChannelConfig::default(),
        )
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_session_manager_shutdown_all() {
    let manager = SessionManager::default();

    let id1 = SessionId::new("session-1");
    let id2 = SessionId::new("session-2");

    // Create and start sessions
    let (session1, _ch1) = manager
        .create_session(
            id1.clone(),
            TestProtocol::new(),
            Alice,
            ChannelConfig::default(),
        )
        .await
        .unwrap();

    let (session2, _ch2) = manager
        .create_session(
            id2.clone(),
            TestProtocol::new(),
            Alice,
            ChannelConfig::default(),
        )
        .await
        .unwrap();

    session1.start().await.unwrap();
    session2.start().await.unwrap();

    // Check status counts
    let counts = manager.session_count_by_status().await;
    assert_eq!(counts.get(&SessionStatus::Running), Some(&2));

    // Shutdown all sessions
    manager.shutdown_all_sessions().await.unwrap();

    // Wait a bit for shutdown to complete
    sleep(Duration::from_millis(100)).await;

    // Check that sessions are in final states
    let status1 = manager.get_session_status(&id1).await.unwrap();
    let status2 = manager.get_session_status(&id2).await.unwrap();

    assert!(matches!(
        status1,
        SessionStatus::Completed | SessionStatus::Cancelled | SessionStatus::Failed(_)
    ));
    assert!(matches!(
        status2,
        SessionStatus::Completed | SessionStatus::Cancelled | SessionStatus::Failed(_)
    ));
}

#[tokio::test]
async fn test_session_manager_leak_detection() {
    let manager = SessionManager::default();

    let id1 = SessionId::new("session-1");
    let id2 = SessionId::new("session-2");

    // Create sessions
    let (session1, _ch1) = manager
        .create_session(
            id1.clone(),
            TestProtocol::new(),
            Alice,
            ChannelConfig::default(),
        )
        .await
        .unwrap();

    let (session2, _ch2) = manager
        .create_session(
            id2.clone(),
            TestProtocol::new(),
            Alice,
            ChannelConfig::default(),
        )
        .await
        .unwrap();

    // Add resources to sessions with different leak patterns
    session1
        .track_resource("channel-1".to_string(), ResourceType::Channel)
        .await;
    session1
        .track_resource("task-1".to_string(), ResourceType::Task)
        .await;
    session1.close_resource("channel-1").await; // Close one, leak one

    session2
        .track_resource("channel-2".to_string(), ResourceType::Channel)
        .await;
    session2
        .track_resource("task-2".to_string(), ResourceType::Task)
        .await;
    session2.close_resource("channel-2").await;
    session2.close_resource("task-2").await; // Close all, no leaks

    // Get leak summary
    let summary = manager.get_leak_summary().await.unwrap();

    assert_eq!(summary.total_sessions, 2);
    assert_eq!(summary.sessions_with_leaks, 1); // Only session1 has leaks
    assert_eq!(summary.total_leaked_resources, 1); // Only task-1 is leaked
    assert_eq!(summary.total_resources_created, 4);

    // Test individual session leak detection
    let session1_report = manager.detect_session_leaks(&id1).await.unwrap();
    assert!(session1_report.has_leaks());
    assert_eq!(session1_report.leak_count(), 1);

    let session2_report = manager.detect_session_leaks(&id2).await.unwrap();
    assert!(!session2_report.has_leaks());
    assert_eq!(session2_report.leak_count(), 0);
}

#[tokio::test]
async fn test_session_manager_cleanup() {
    let manager = SessionManager::default();

    let id1 = SessionId::new("completed-session");
    let id2 = SessionId::new("failed-session");
    let id3 = SessionId::new("running-session");

    // Create sessions
    let (session1, _ch1) = manager
        .create_session(
            id1.clone(),
            TestProtocol::new(),
            Alice,
            ChannelConfig::default(),
        )
        .await
        .unwrap();

    let (session2, _ch2) = manager
        .create_session(
            id2.clone(),
            TestProtocol::new(),
            Alice,
            ChannelConfig::default(),
        )
        .await
        .unwrap();

    let (session3, _ch3) = manager
        .create_session(
            id3.clone(),
            TestProtocol::new(),
            Alice,
            ChannelConfig::default(),
        )
        .await
        .unwrap();

    // Set different final states
    {
        let mut status1 = session1.status.write().await;
        *status1 = SessionStatus::Completed;
    }
    {
        let mut status2 = session2.status.write().await;
        *status2 = SessionStatus::Failed("test error".to_string());
    }
    {
        let mut status3 = session3.status.write().await;
        *status3 = SessionStatus::Running;
    }

    // Initial count
    assert_eq!(manager.total_sessions().await, 3);

    // Cleanup finished sessions
    let report = manager.cleanup_finished_sessions().await;

    assert_eq!(report.total_cleaned, 2);
    assert_eq!(report.completed, 1);
    assert_eq!(report.failed, 1);
    assert_eq!(report.cancelled, 0);

    // Only running session should remain
    assert_eq!(manager.total_sessions().await, 1);
    assert!(manager.get_session_status(&id3).await.is_some());
    assert!(manager.get_session_status(&id1).await.is_none());
    assert!(manager.get_session_status(&id2).await.is_none());
}

#[tokio::test]
async fn test_session_manager_metrics() {
    let manager = SessionManager::default();

    let id1 = SessionId::new("session-1");
    let id2 = SessionId::new("session-2");

    // Create sessions with different states
    let (session1, _ch1) = manager
        .create_session(
            id1.clone(),
            TestProtocol::new(),
            Alice,
            ChannelConfig::default(),
        )
        .await
        .unwrap();

    let (session2, _ch2) = manager
        .create_session(
            id2.clone(),
            TestProtocol::new(),
            Alice,
            ChannelConfig::default(),
        )
        .await
        .unwrap();

    session1.start().await.unwrap();
    // Leave session2 in initializing state

    // Add some resources with leaks
    session1
        .track_resource("leaked-channel".to_string(), ResourceType::Channel)
        .await;
    session2
        .track_resource("clean-channel".to_string(), ResourceType::Channel)
        .await;
    session2.close_resource("clean-channel").await;

    let metrics = manager.get_manager_metrics().await;

    assert_eq!(metrics.total_sessions, 2);
    assert_eq!(metrics.status_counts.get(&SessionStatus::Running), Some(&1));
    assert_eq!(
        metrics.status_counts.get(&SessionStatus::Initializing),
        Some(&1)
    );
    assert_eq!(metrics.leak_summary.total_sessions, 2);
    assert_eq!(metrics.leak_summary.sessions_with_leaks, 1);
    assert_eq!(metrics.leak_summary.total_leaked_resources, 1);
}

#[tokio::test]
async fn test_complex_shutdown_scenario() {
    let shutdown_config = ShutdownConfig {
        graceful_shutdown_timeout: Duration::from_millis(500),
        critical_operations_timeout: Duration::from_millis(100),
        force_task_termination: true,
        strict_leak_detection: true,
    };

    let manager = SessionManager::new_with_config(SessionConfig {
        shutdown_config,
        enable_resource_tracking: true,
        enable_metrics: true,
    });

    // Create multiple sessions
    let mut sessions = Vec::new();
    for i in 0..5 {
        let id = SessionId::new(format!("session-{}", i));
        let (session, _ch) = manager
            .create_session(id, TestProtocol::new(), Alice, ChannelConfig::default())
            .await
            .unwrap();

        session.start().await.unwrap();

        // Add some resources to each session
        session
            .track_resource(format!("channel-{}", i), ResourceType::Channel)
            .await;
        session
            .track_resource(format!("task-{}", i), ResourceType::Task)
            .await;

        sessions.push(session);
    }

    assert_eq!(manager.total_sessions().await, 5);

    // Shutdown all sessions
    let start_time = std::time::Instant::now();
    manager.shutdown_all_sessions().await.unwrap();
    let shutdown_duration = start_time.elapsed();

    // Should complete within reasonable time
    assert!(shutdown_duration < Duration::from_secs(2));

    // Wait a bit for cleanup
    sleep(Duration::from_millis(100)).await;

    // Check that all sessions are in final states
    let counts = manager.session_count_by_status().await;
    let completed = counts.get(&SessionStatus::Completed).unwrap_or(&0);
    let cancelled = counts.get(&SessionStatus::Cancelled).unwrap_or(&0);

    // Count all failed sessions regardless of error message
    let failed = counts
        .iter()
        .filter(|(status, _)| matches!(status, SessionStatus::Failed(_)))
        .map(|(_, count)| *count)
        .sum::<usize>();

    let total_final = completed + cancelled + failed;

    assert_eq!(total_final, 5);

    // Check for resource leaks
    let leak_summary = manager.get_leak_summary().await.unwrap();
    if leak_summary.has_leaks() {
        println!(
            "Warning: {} sessions have resource leaks",
            leak_summary.sessions_with_leaks
        );
        println!(
            "Total leaked resources: {}",
            leak_summary.total_leaked_resources
        );
    }
}

#[tokio::test]
async fn test_session_pause_resume_with_shutdown() {
    let id = SessionId::new("pause-resume-session");
    let (session, _channel) =
        Session::new(id, TestProtocol::new(), Alice, ChannelConfig::default());

    // Start session
    session.start().await.unwrap();
    assert_eq!(session.status().await, SessionStatus::Running);

    // Pause session
    session.pause().await.unwrap();
    assert_eq!(session.status().await, SessionStatus::Paused);

    // Resume session
    session.resume().await.unwrap();
    assert_eq!(session.status().await, SessionStatus::Running);

    // Now shutdown gracefully
    session.shutdown_graceful().await.unwrap();

    // Should be in final state
    let final_status = session.status().await;
    assert!(matches!(
        final_status,
        SessionStatus::Completed | SessionStatus::Cancelled | SessionStatus::Failed(_)
    ));
}

#[tokio::test]
async fn test_resource_tracking_with_tasks() {
    let id = SessionId::new("task-tracking-session");
    let (session, _channel) =
        Session::new(id, TestProtocol::new(), Alice, ChannelConfig::default());

    // Track a mock task
    let task_handle = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
    });

    session
        .track_task("background-task".to_string(), task_handle)
        .await;

    // Check that task is tracked
    let tasks = session.task_handles.read().await;
    assert_eq!(tasks.len(), 1);
    assert!(tasks.contains_key("background-task"));

    let resources = session.tracked_resources.read().await;
    assert!(resources.contains_key("background-task"));
    assert_eq!(
        resources.get("background-task").unwrap().resource_type,
        ResourceType::Task
    );
}
