//! Enhanced Session lifecycle management with graceful shutdown and resource leak detection
//!
//! This module provides robust session management functionality for executing session-typed
//! protocols, including graceful shutdown mechanisms, resource tracking, and leak detection.

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::{RwLock, Mutex, watch};
use tokio::task::JoinHandle;
use tokio::time::timeout;
use uuid::Uuid;

use crate::protocol::foundation::{ActionIOTMarker, LocalProtocol, Role, SupportsActionIO};
use crate::runtime::{
    error::RuntimeError,
    state::ExecutionContext,
    channel::{TypedChannel, ChannelConfig, ChannelId},
};

#[cfg(test)]
mod tests;

/// Unique identifier for a session
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(pub String);

impl SessionId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<SessionId> for String {
    fn from(id: SessionId) -> String {
        id.0
    }
}

/// Status of a session
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionStatus {
    /// Session is initializing
    Initializing,
    /// Session is actively running
    Running,
    /// Session is paused (can be resumed)
    Paused,
    /// Session is shutting down gracefully
    ShuttingDown,
    /// Session completed successfully
    Completed,
    /// Session failed with an error
    Failed(String),
    /// Session was cancelled
    Cancelled,
}

impl fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionStatus::Initializing => write!(f, "Initializing"),
            SessionStatus::Running => write!(f, "Running"),
            SessionStatus::Paused => write!(f, "Paused"),
            SessionStatus::ShuttingDown => write!(f, "ShuttingDown"),
            SessionStatus::Completed => write!(f, "Completed"),
            SessionStatus::Failed(error) => write!(f, "Failed: {}", error),
            SessionStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

/// Shutdown configuration for sessions
#[derive(Debug, Clone)]
pub struct ShutdownConfig {
    /// Maximum time to wait for graceful shutdown
    pub graceful_shutdown_timeout: Duration,
    /// Maximum time to wait for critical operations to complete
    pub critical_operations_timeout: Duration,
    /// Whether to wait for all tasks to complete or abort them
    pub force_task_termination: bool,
    /// Whether to perform strict resource leak detection
    pub strict_leak_detection: bool,
}

impl Default for ShutdownConfig {
    fn default() -> Self {
        Self {
            graceful_shutdown_timeout: Duration::from_secs(30),
            critical_operations_timeout: Duration::from_secs(5),
            force_task_termination: false,
            strict_leak_detection: true,
        }
    }
}

/// Tracked resource information for leak detection
#[derive(Debug, Clone)]
pub struct TrackedResource {
    pub resource_id: String,
    pub resource_type: ResourceType,
    pub created_at: SystemTime,
    pub is_closed: bool,
}

/// Types of resources that can be tracked
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceType {
    Channel,
    Task,
    Connection,
    FileHandle,
    Other(String),
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceType::Channel => write!(f, "Channel"),
            ResourceType::Task => write!(f, "Task"),
            ResourceType::Connection => write!(f, "Connection"),
            ResourceType::FileHandle => write!(f, "FileHandle"),
            ResourceType::Other(name) => write!(f, "Other({})", name),
        }
    }
}

/// Resource leak detection report
#[derive(Debug, Clone)]
pub struct LeakDetectionReport {
    pub session_id: SessionId,
    pub leaked_resources: Vec<TrackedResource>,
    pub total_resources_created: usize,
    pub total_resources_closed: usize,
    pub detection_time: SystemTime,
}

impl LeakDetectionReport {
    pub fn has_leaks(&self) -> bool {
        !self.leaked_resources.is_empty()
    }

    pub fn leak_count(&self) -> usize {
        self.leaked_resources.len()
    }
}

/// Configuration for session creation
#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub shutdown_config: ShutdownConfig,
    pub enable_resource_tracking: bool,
    pub enable_metrics: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            shutdown_config: ShutdownConfig::default(),
            enable_resource_tracking: true,
            enable_metrics: true,
        }
    }
}

/// A session represents an instance of protocol execution with enhanced lifecycle management
pub struct Session<P, R, AIO>
where
    P: LocalProtocol + Clone,
    R: Role + SupportsActionIO<AIO> + Clone,
    AIO: ActionIOTMarker,
{
    id: SessionId,
    status: Arc<RwLock<SessionStatus>>,
    protocol: P,
    role: R,
    context: Arc<RwLock<ExecutionContext>>,
    channel: Arc<TypedChannel<P, R, AIO>>,
    task_handle: Arc<Mutex<Option<JoinHandle<Result<(), RuntimeError>>>>>,
    
    // Enhanced lifecycle management
    shutdown_config: ShutdownConfig,
    shutdown_signal: Arc<watch::Sender<bool>>,
    shutdown_receiver: watch::Receiver<bool>,
    
    // Resource tracking for leak detection
    tracked_resources: Arc<RwLock<HashMap<String, TrackedResource>>>,
    task_handles: Arc<RwLock<HashMap<String, JoinHandle<()>>>>,
    channel_handles: Arc<RwLock<HashMap<ChannelId, Arc<TypedChannel<P, R, AIO>>>>>,
    
    // Logging and metrics
    created_at: SystemTime,
    last_activity: Arc<RwLock<SystemTime>>,
    is_completed: Arc<RwLock<bool>>,
}

impl<P, R, AIO> Session<P, R, AIO>
where
    P: LocalProtocol + Clone + Send + Sync + 'static,
    R: Role + SupportsActionIO<AIO> + Clone + Send + Sync + 'static,
    AIO: ActionIOTMarker + Send + Sync + 'static,
{
    /// Create a new session with enhanced lifecycle management
    pub fn new(
        id: SessionId,
        protocol: P,
        role: R,
        channel_config: ChannelConfig,
    ) -> (Self, TypedChannel<P, R, AIO>) {
        Self::new_with_config(id, protocol, role, channel_config, ShutdownConfig::default())
    }

    /// Create a new session with custom shutdown configuration
    pub fn new_with_config(
        id: SessionId,
        protocol: P,
        role: R,
        channel_config: ChannelConfig,
        shutdown_config: ShutdownConfig,
    ) -> (Self, TypedChannel<P, R, AIO>) {
        // Create context for execution tracking
        let context = ExecutionContext::new(id.0.clone(), format!("{:?}", role));
        let (channel1, channel2) = TypedChannel::new(channel_config);

        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let session = Self {
            id,
            status: Arc::new(RwLock::new(SessionStatus::Initializing)),
            protocol: protocol.clone(),
            role: role.clone(),
            context: Arc::new(RwLock::new(context)),
            channel: Arc::new(channel1),
            task_handle: Arc::new(Mutex::new(None)),
            shutdown_config,
            shutdown_signal: Arc::new(shutdown_tx),
            shutdown_receiver: shutdown_rx,
            tracked_resources: Arc::new(RwLock::new(HashMap::new())),
            task_handles: Arc::new(RwLock::new(HashMap::new())),
            channel_handles: Arc::new(RwLock::new(HashMap::new())),
            created_at: SystemTime::now(),
            last_activity: Arc::new(RwLock::new(SystemTime::now())),
            is_completed: Arc::new(RwLock::new(false)),
        };

        (session, channel2)
    }

    /// Get the session ID
    pub fn id(&self) -> &SessionId {
        &self.id
    }

    /// Get the current session status
    pub async fn status(&self) -> SessionStatus {
        self.status.read().await.clone()
    }

    /// Update last activity timestamp
    async fn update_activity(&self) {
        let mut last_activity = self.last_activity.write().await;
        *last_activity = SystemTime::now();
    }

    /// Track a new resource for leak detection
    pub async fn track_resource(&self, resource_id: String, resource_type: ResourceType) {
        let resource = TrackedResource {
            resource_id: resource_id.clone(),
            resource_type: resource_type.clone(),
            created_at: SystemTime::now(),
            is_closed: false,
        };

        let mut resources = self.tracked_resources.write().await;
        resources.insert(resource_id.clone(), resource);
        
        log::debug!("Session {}: Tracking new resource: {} ({})", 
                   self.id, resource_id, resource_type);
    }

    /// Mark a resource as closed
    pub async fn close_resource(&self, resource_id: &str) {
        let mut resources = self.tracked_resources.write().await;
        if let Some(resource) = resources.get_mut(resource_id) {
            resource.is_closed = true;
            log::debug!("Session {}: Closed resource: {} ({})", 
                       self.id, resource_id, resource.resource_type);
        }
    }

    /// Track a Tokio task
    pub async fn track_task(&self, task_name: String, handle: JoinHandle<()>) {
        self.track_resource(task_name.clone(), ResourceType::Task).await;
        
        let mut tasks = self.task_handles.write().await;
        tasks.insert(task_name, handle);
    }

    /// Track a channel
    pub async fn track_channel(&self, channel_id: ChannelId, channel: Arc<TypedChannel<P, R, AIO>>) {
        self.track_resource(channel_id.to_string(), ResourceType::Channel).await;
        
        let mut channels = self.channel_handles.write().await;
        channels.insert(channel_id, channel);
    }

    /// Start the session execution
    pub async fn start(&self) -> Result<(), RuntimeError> {
        let mut status = self.status.write().await;
        if *status != SessionStatus::Initializing {
            return Err(RuntimeError::Protocol(
                crate::runtime::error::ProtocolViolation::InvalidTransition {
                    current_state: status.to_string(),
                    action_taken: "start".to_string(),
                    expected_actions_or_states: "Initializing".to_string(),
                }
            ));
        }

        log::info!("Session {}: Starting execution", self.id);
        *status = SessionStatus::Running;
        drop(status);

        self.update_activity().await;

        // Start the execution loop
        let session_clone = self.clone_for_task();
        let handle = tokio::spawn(async move {
            session_clone.execution_loop().await
        });

        let mut task_handle = self.task_handle.lock().await;
        *task_handle = Some(handle);

        Ok(())
    }

    /// Clone necessary components for task execution
    fn clone_for_task(&self) -> SessionExecutor<P, R, AIO> {
        SessionExecutor {
            id: self.id.clone(),
            status: Arc::clone(&self.status),
            protocol: self.protocol.clone(),
            role: self.role.clone(),
            context: Arc::clone(&self.context),
            channel: Arc::clone(&self.channel),
            shutdown_receiver: self.shutdown_receiver.clone(),
            shutdown_config: self.shutdown_config.clone(),
            last_activity: Arc::clone(&self.last_activity),
            is_completed: Arc::clone(&self.is_completed),
        }
    }

    /// Gracefully shutdown the session
    pub async fn shutdown(&self) -> Result<LeakDetectionReport, RuntimeError> {
        log::info!("Session {}: Initiating graceful shutdown", self.id);

        // Set status to shutting down
        {
            let mut status = self.status.write().await;
            if matches!(*status, SessionStatus::Completed | SessionStatus::Failed(_) | SessionStatus::Cancelled) {
                log::warn!("Session {}: Already terminated with status: {}", self.id, *status);
                return self.detect_leaks().await;
            }
            *status = SessionStatus::ShuttingDown;
        }

        // Signal shutdown to execution loop
        if let Err(e) = self.shutdown_signal.send(true) {
            log::warn!("Session {}: Failed to send shutdown signal: {}", self.id, e);
        }

        // Wait for graceful shutdown with timeout
        let shutdown_result = if let Some(task_handle) = self.task_handle.lock().await.take() {
            match timeout(self.shutdown_config.graceful_shutdown_timeout, task_handle).await {
                Ok(result) => {
                    log::info!("Session {}: Graceful shutdown completed", self.id);
                    result.map_err(|e| RuntimeError::Execution(format!("Task join error: {}", e)))?
                }
                Err(_) => {
                    log::warn!("Session {}: Graceful shutdown timeout, performing force shutdown", self.id);
                    if self.shutdown_config.force_task_termination {
                        // Force terminate all tracked tasks
                        self.force_terminate_tasks().await;
                    }
                    Err(RuntimeError::Execution("Shutdown timeout".to_string()))
                }
            }
        } else {
            Ok(())
        };

        // Clean up all tracked resources
        self.cleanup_all_resources().await;

        // Update final status
        {
            let mut status = self.status.write().await;
            *status = match shutdown_result {
                Ok(_) => SessionStatus::Completed,
                Err(e) => SessionStatus::Failed(e.to_string()),
            };
        }

        // Perform leak detection
        let leak_report = self.detect_leaks().await?;
        
        if leak_report.has_leaks() {
            log::warn!("Session {}: Detected {} resource leaks", self.id, leak_report.leak_count());
        } else {
            log::info!("Session {}: No resource leaks detected", self.id);
        }

        Ok(leak_report)
    }

    /// Force terminate all tracked tasks
    async fn force_terminate_tasks(&self) {
        let mut tasks = self.task_handles.write().await;
        for (task_name, handle) in tasks.drain() {
            handle.abort();
            log::debug!("Session {}: Force terminated task: {}", self.id, task_name);
        }
    }

    /// Clean up all tracked resources
    async fn cleanup_all_resources(&self) {
        // Close all tracked channels
        let mut channels = self.channel_handles.write().await;
        channels.clear();

        // Abort any remaining tasks
        let mut tasks = self.task_handles.write().await;
        for (task_name, handle) in tasks.drain() {
            if !handle.is_finished() {
                handle.abort();
                log::debug!("Session {}: Aborted task during cleanup: {}", self.id, task_name);
            }
        }
    }

    /// Detect resource leaks
    pub async fn detect_leaks(&self) -> Result<LeakDetectionReport, RuntimeError> {
        let resources = self.tracked_resources.read().await;
        
        let leaked_resources: Vec<TrackedResource> = resources
            .values()
            .filter(|resource| !resource.is_closed)
            .cloned()
            .collect();

        let total_created = resources.len();
        let total_closed = resources.values().filter(|r| r.is_closed).count();

        let report = LeakDetectionReport {
            session_id: self.id.clone(),
            leaked_resources,
            total_resources_created: total_created,
            total_resources_closed: total_closed,
            detection_time: SystemTime::now(),
        };

        if self.shutdown_config.strict_leak_detection && report.has_leaks() {
            log::error!("Session {}: Strict leak detection enabled, found {} leaks", 
                       self.id, report.leak_count());
        }

        Ok(report)
    }

    /// Pause the session
    pub async fn pause(&self) -> Result<(), RuntimeError> {
        let mut status = self.status.write().await;
        match *status {
            SessionStatus::Running => {
                *status = SessionStatus::Paused;
                log::info!("Session {}: Paused", self.id);
                Ok(())
            }
            _ => Err(RuntimeError::Protocol(
                crate::runtime::error::ProtocolViolation::InvalidTransition {
                    current_state: status.to_string(),
                    action_taken: "pause".to_string(),
                    expected_actions_or_states: "Running".to_string(),
                }
            ))
        }
    }

    /// Resume the session
    pub async fn resume(&self) -> Result<(), RuntimeError> {
        let mut status = self.status.write().await;
        match *status {
            SessionStatus::Paused => {
                *status = SessionStatus::Running;
                log::info!("Session {}: Resumed", self.id);
                self.update_activity().await;
                Ok(())
            }
            _ => Err(RuntimeError::Protocol(
                crate::runtime::error::ProtocolViolation::InvalidTransition {
                    current_state: status.to_string(),
                    action_taken: "resume".to_string(),
                    expected_actions_or_states: "Paused".to_string(),
                }
            ))
        }
    }

    /// Cancel the session
    pub async fn cancel(&self) -> Result<LeakDetectionReport, RuntimeError> {
        log::info!("Session {}: Cancelling", self.id);

        {
            let mut status = self.status.write().await;
            *status = SessionStatus::Cancelled;
        }

        // Signal shutdown and force terminate
        let _ = self.shutdown_signal.send(true);
        
        if let Some(task_handle) = self.task_handle.lock().await.take() {
            task_handle.abort();
        }

        self.force_terminate_tasks().await;
        self.cleanup_all_resources().await;

        self.detect_leaks().await
    }

    /// Get session metrics
    pub async fn get_metrics(&self) -> SessionMetrics {
        let status = self.status.read().await.clone();
        let last_activity = *self.last_activity.read().await;
        let resources = self.tracked_resources.read().await;
        let tasks = self.task_handles.read().await;
        let channels = self.channel_handles.read().await;

        SessionMetrics {
            session_id: self.id.clone(),
            status,
            created_at: self.created_at,
            last_activity,
            total_resources: resources.len(),
            active_resources: resources.values().filter(|r| !r.is_closed).count(),
            active_tasks: tasks.len(),
            active_channels: channels.len(),
            uptime: self.created_at.elapsed().unwrap_or(Duration::ZERO),
        }
    }
}

/// Helper struct for session execution loop
struct SessionExecutor<P, R, AIO>
where
    P: LocalProtocol + Clone,
    R: Role + SupportsActionIO<AIO> + Clone,
    AIO: ActionIOTMarker,
{
    id: SessionId,
    status: Arc<RwLock<SessionStatus>>,
    protocol: P,
    role: R,
    context: Arc<RwLock<ExecutionContext>>,
    channel: Arc<TypedChannel<P, R, AIO>>,
    shutdown_receiver: watch::Receiver<bool>,
    shutdown_config: ShutdownConfig,
    last_activity: Arc<RwLock<SystemTime>>,
    is_completed: Arc<RwLock<bool>>,
}

impl<P, R, AIO> SessionExecutor<P, R, AIO>
where
    P: LocalProtocol + Clone + Send + Sync + 'static,
    R: Role + SupportsActionIO<AIO> + Clone + Send + Sync + 'static,
    AIO: ActionIOTMarker + Send + Sync + 'static,
{
    /// Main execution loop with shutdown awareness
    async fn execution_loop(&self) -> Result<(), RuntimeError> {
        let mut shutdown_rx = self.shutdown_receiver.clone();

        loop {
            // Check for shutdown signal
            if *shutdown_rx.borrow_and_update() {
                log::info!("Session {}: Received shutdown signal", self.id);
                break;
            }

            // Check if session is paused
            {
                let status = self.status.read().await;
                if *status == SessionStatus::Paused {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    continue;
                }
            }

            // Execute protocol step with timeout
            let step_result = timeout(
                self.shutdown_config.critical_operations_timeout,
                self.execute_protocol_step()
            ).await;

            match step_result {
                Ok(Ok(should_continue)) => {
                    if !should_continue {
                        log::info!("Session {}: Protocol execution completed", self.id);
                        break;
                    }
                    self.update_activity().await;
                }
                Ok(Err(e)) => {
                    log::error!("Session {}: Protocol step failed: {}", self.id, e);
                    let mut status = self.status.write().await;
                    *status = SessionStatus::Failed(e.to_string());
                    return Err(e);
                }
                Err(_) => {
                    log::warn!("Session {}: Protocol step timeout", self.id);
                    // Continue with next iteration on timeout
                }
            }

            // Small delay to prevent busy waiting
            tokio::time::sleep(Duration::from_millis(10)).await;

            // Check for shutdown signal again
            if shutdown_rx.has_changed().is_ok() && *shutdown_rx.borrow() {
                log::info!("Session {}: Shutdown signal detected during execution", self.id);
                break;
            }
        }

        log::info!("Session {}: Execution loop finished", self.id);
        Ok(())
    }

    /// Execute a single protocol step
    async fn execute_protocol_step(&self) -> Result<bool, RuntimeError> {
        // This is a simplified implementation
        // In practice, this would execute the actual protocol logic
        
        let is_completed = self.is_completed.read().await;
        if *is_completed {
            return Ok(false); // Protocol completed
        }
        
        // Simulate protocol step execution
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        Ok(true) // Continue execution
    }

    async fn update_activity(&self) {
        let mut last_activity = self.last_activity.write().await;
        *last_activity = SystemTime::now();
    }
}

impl<P, R, AIO> Drop for Session<P, R, AIO>
where
    P: LocalProtocol + Clone,
    R: Role + SupportsActionIO<AIO> + Clone,
    AIO: ActionIOTMarker,
{
    fn drop(&mut self) {
        log::debug!("Session {}: Dropping session instance", self.id);
    }
}

/// Session metrics for monitoring and debugging
#[derive(Debug, Clone)]
pub struct SessionMetrics {
    pub session_id: SessionId,
    pub status: SessionStatus,
    pub created_at: SystemTime,
    pub last_activity: SystemTime,
    pub total_resources: usize,
    pub active_resources: usize,
    pub active_tasks: usize,
    pub active_channels: usize,
    pub uptime: Duration,
}

/// Session manager for coordinating multiple sessions
pub struct SessionManager<P, R, AIO>
where
    P: LocalProtocol + Clone,
    R: Role + SupportsActionIO<AIO> + Clone,
    AIO: ActionIOTMarker,
{
    sessions: Arc<RwLock<HashMap<SessionId, Arc<Session<P, R, AIO>>>>>,
    default_config: SessionConfig,
    shutdown_signal: Arc<watch::Sender<bool>>,
    shutdown_receiver: watch::Receiver<bool>,
}

impl<P, R, AIO> SessionManager<P, R, AIO>
where
    P: LocalProtocol + Clone + Send + Sync + 'static,
    R: Role + SupportsActionIO<AIO> + Clone + Send + Sync + 'static,
    AIO: ActionIOTMarker + Send + Sync + 'static,
{
    /// Create a new session manager
    pub fn new() -> Self {
        Self::new_with_config(SessionConfig::default())
    }

    /// Create a new session manager with custom configuration
    pub fn new_with_config(config: SessionConfig) -> Self {
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            default_config: config,
            shutdown_signal: Arc::new(shutdown_tx),
            shutdown_receiver: shutdown_rx,
        }
    }

    /// Create and register a new session
    pub async fn create_session(
        &self,
        id: SessionId,
        protocol: P,
        role: R,
        channel_config: ChannelConfig,
    ) -> Result<(Arc<Session<P, R, AIO>>, TypedChannel<P, R, AIO>), RuntimeError> {
        let (session, channel) = Session::new_with_config(
            id.clone(),
            protocol,
            role,
            channel_config,
            self.default_config.shutdown_config.clone(),
        );

        let session_arc = Arc::new(session);
        
        {
            let mut sessions = self.sessions.write().await;
            if sessions.contains_key(&id) {
                return Err(RuntimeError::SessionAlreadyExists(id.to_string()));
            }
            sessions.insert(id.clone(), Arc::clone(&session_arc));
        }

        log::info!("SessionManager: Created session {}", id);
        Ok((session_arc, channel))
    }

    /// Get a session by ID
    pub async fn get_session(&self, id: &SessionId) -> Option<Arc<Session<P, R, AIO>>> {
        let sessions = self.sessions.read().await;
        sessions.get(id).cloned()
    }

    /// Remove a session
    pub async fn remove_session(&self, id: &SessionId) -> Option<Arc<Session<P, R, AIO>>> {
        let mut sessions = self.sessions.write().await;
        let session = sessions.remove(id);
        if session.is_some() {
            log::info!("SessionManager: Removed session {}", id);
        }
        session
    }

    /// List all session IDs
    pub async fn list_sessions(&self) -> Vec<SessionId> {
        let sessions = self.sessions.read().await;
        sessions.keys().cloned().collect()
    }

    /// Get the number of active sessions
    pub async fn session_count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }

    /// Shutdown all sessions gracefully
    pub async fn shutdown_all_sessions(&self) -> Result<Vec<LeakDetectionReport>, RuntimeError> {
        log::info!("SessionManager: Shutting down all sessions");

        let sessions = {
            let sessions_guard = self.sessions.read().await;
            sessions_guard.values().cloned().collect::<Vec<_>>()
        };

        let mut reports = Vec::new();
        let mut errors = Vec::new();

        for session in sessions {
            match session.shutdown().await {
                Ok(report) => reports.push(report),
                Err(e) => {
                    log::error!("SessionManager: Failed to shutdown session {}: {}", session.id(), e);
                    errors.push(e);
                }
            }
        }

        // Clear the sessions map
        {
            let mut sessions_guard = self.sessions.write().await;
            sessions_guard.clear();
        }

        if !errors.is_empty() {
            return Err(RuntimeError::Execution(format!(
                "Failed to shutdown {} sessions", errors.len()
            )));
        }

        log::info!("SessionManager: All sessions shut down successfully");
        Ok(reports)
    }

    /// Detect leaks across all sessions
    pub async fn detect_all_leaks(&self) -> Result<Vec<LeakDetectionReport>, RuntimeError> {
        let sessions = {
            let sessions_guard = self.sessions.read().await;
            sessions_guard.values().cloned().collect::<Vec<_>>()
        };

        let mut reports = Vec::new();
        for session in sessions {
            let report = session.detect_leaks().await?;
            reports.push(report);
        }

        Ok(reports)
    }

    /// Get a summary of resource leaks across all sessions
    pub async fn get_leak_summary(&self) -> Result<(usize, usize), RuntimeError> {
        let reports = self.detect_all_leaks().await?;
        
        let total_sessions = reports.len();
        let sessions_with_leaks = reports.iter().filter(|r| r.has_leaks()).count();
        
        Ok((total_sessions, sessions_with_leaks))
    }

    /// Get metrics for all sessions
    pub async fn get_all_metrics(&self) -> Vec<SessionMetrics> {
        let sessions = {
            let sessions_guard = self.sessions.read().await;
            sessions_guard.values().cloned().collect::<Vec<_>>()
        };

        let mut metrics = Vec::new();
        for session in sessions {
            metrics.push(session.get_metrics().await);
        }

        metrics
    }

    /// Get manager-level metrics
    pub async fn get_manager_metrics(&self) -> ManagerMetrics {
        let sessions = self.sessions.read().await;
        let total_sessions = sessions.len();
        
        let mut active_sessions = 0;
        let mut failed_sessions = 0;
        let mut completed_sessions = 0;

        for session in sessions.values() {
            match session.status().await {
                SessionStatus::Running | SessionStatus::Paused => active_sessions += 1,
                SessionStatus::Failed(_) => failed_sessions += 1,
                SessionStatus::Completed => completed_sessions += 1,
                _ => {}
            }
        }

        ManagerMetrics {
            total_sessions,
            active_sessions,
            failed_sessions,
            completed_sessions,
        }
    }
}

impl<P, R, AIO> Default for SessionManager<P, R, AIO>
where
    P: LocalProtocol + Clone + Send + Sync + 'static,
    R: Role + SupportsActionIO<AIO> + Clone + Send + Sync + 'static,
    AIO: ActionIOTMarker + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Manager-level metrics
#[derive(Debug, Clone)]
pub struct ManagerMetrics {
    pub total_sessions: usize,
    pub active_sessions: usize,
    pub failed_sessions: usize,
    pub completed_sessions: usize,
}