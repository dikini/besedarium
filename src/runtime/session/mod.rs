//! Session lifecycle management for protocol execution
//!
//! This module provides session management functionality for executing session-typed
//! protocols, including session creation, state tracking, and cleanup.

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use tokio::task::JoinHandle;

use crate::protocol::foundation::{ActionIOTMarker, LocalProtocol, Role, SupportsActionIO};
use crate::runtime::{
    error::RuntimeError,
    state::{ExecutionContext, ProtocolState, StateTransition},
    channel::{TypedChannel, ChannelConfig},
};

/// Unique identifier for a session
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(pub String);

impl SessionId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
            SessionStatus::Completed => write!(f, "Completed"),
            SessionStatus::Failed(error) => write!(f, "Failed: {}", error),
            SessionStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

/// A session represents an instance of protocol execution
pub struct Session<P, R, AIO>
where
    P: LocalProtocol,
    R: Role + SupportsActionIO<AIO>,
    AIO: ActionIOTMarker,
{
    id: SessionId,
    status: Arc<RwLock<SessionStatus>>,
    state: Arc<RwLock<ProtocolState<P>>>,
    context: Arc<RwLock<ExecutionContext<R>>>,
    channel: Arc<TypedChannel<P, R, AIO>>,
    task_handle: Arc<Mutex<Option<JoinHandle<Result<(), RuntimeError>>>>>,
}

impl<P, R, AIO> Session<P, R, AIO>
where
    P: LocalProtocol + Send + Sync + 'static,
    R: Role + SupportsActionIO<AIO> + Send + Sync + 'static,
    AIO: ActionIOTMarker + Send + Sync + 'static,
{
    /// Create a new session
    pub fn new(
        id: SessionId,
        protocol: P,
        role: R,
        channel_config: ChannelConfig,
    ) -> (Self, TypedChannel<P, R, AIO>) {
        let state = ProtocolState::new(id.clone(), Box::new(protocol));
        let context = ExecutionContext::new(role);
        let (channel1, channel2) = TypedChannel::new(channel_config);

        let session = Self {
            id,
            status: Arc::new(RwLock::new(SessionStatus::Initializing)),
            state: Arc::new(RwLock::new(state)),
            context: Arc::new(RwLock::new(context)),
            channel: Arc::new(channel1),
            task_handle: Arc::new(Mutex::new(None)),
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

    /// Start the session execution
    pub async fn start(&self) -> Result<(), RuntimeError> {
        let mut status = self.status.write().await;
        if *status != SessionStatus::Initializing {
            return Err(RuntimeError::InvalidStateTransition {
                from: status.to_string(),
                to: SessionStatus::Running.to_string(),
            });
        }

        *status = SessionStatus::Running;
        
        // Spawn the session execution task
        let state_arc = Arc::clone(&self.state);
        let context_arc = Arc::clone(&self.context);
        let channel_arc = Arc::clone(&self.channel);
        let status_arc = Arc::clone(&self.status);

        let handle = tokio::spawn(async move {
            Self::execute_session(state_arc, context_arc, channel_arc, status_arc).await
        });

        let mut task_handle = self.task_handle.lock().await;
        *task_handle = Some(handle);

        Ok(())
    }

    /// Pause the session execution
    pub async fn pause(&self) -> Result<(), RuntimeError> {
        let mut status = self.status.write().await;
        if *status != SessionStatus::Running {
            return Err(RuntimeError::InvalidStateTransition {
                from: status.to_string(),
                to: SessionStatus::Paused.to_string(),
            });
        }

        *status = SessionStatus::Paused;
        Ok(())
    }

    /// Resume the session execution
    pub async fn resume(&self) -> Result<(), RuntimeError> {
        let mut status = self.status.write().await;
        if *status != SessionStatus::Paused {
            return Err(RuntimeError::InvalidStateTransition {
                from: status.to_string(),
                to: SessionStatus::Running.to_string(),
            });
        }

        *status = SessionStatus::Running;
        Ok(())
    }

    /// Cancel the session execution
    pub async fn cancel(&self) -> Result<(), RuntimeError> {
        let mut status = self.status.write().await;
        if matches!(*status, SessionStatus::Completed | SessionStatus::Failed(_) | SessionStatus::Cancelled) {
            return Ok(()); // Already finished
        }

        *status = SessionStatus::Cancelled;

        // Cancel the running task if it exists
        let mut task_handle = self.task_handle.lock().await;
        if let Some(handle) = task_handle.take() {
            handle.abort();
        }

        Ok(())
    }

    /// Wait for the session to complete
    pub async fn wait(&self) -> Result<(), RuntimeError> {
        let task_handle = {
            let handle_guard = self.task_handle.lock().await;
            handle_guard.as_ref().map(|h| h.abort_handle())
        };

        if let Some(_handle) = task_handle {
            // Wait for status to become final
            loop {
                let status = self.status().await;
                match status {
                    SessionStatus::Completed | SessionStatus::Failed(_) | SessionStatus::Cancelled => {
                        break;
                    }
                    _ => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    }
                }
            }
        }

        let final_status = self.status().await;
        match final_status {
            SessionStatus::Completed => Ok(()),
            SessionStatus::Failed(error) => Err(RuntimeError::SessionExecutionFailed(error)),
            SessionStatus::Cancelled => Err(RuntimeError::SessionCancelled),
            _ => Err(RuntimeError::SessionExecutionFailed("Session did not reach final state".to_string())),
        }
    }

    /// Internal session execution logic
    async fn execute_session(
        state: Arc<RwLock<ProtocolState<P>>>,
        context: Arc<RwLock<ExecutionContext<R>>>,
        _channel: Arc<TypedChannel<P, R, AIO>>,
        status: Arc<RwLock<SessionStatus>>,
    ) -> Result<(), RuntimeError> {
        // Check if we should continue execution
        loop {
            let current_status = {
                let status_guard = status.read().await;
                status_guard.clone()
            };

            match current_status {
                SessionStatus::Running => {
                    // Execute one step of the protocol
                    let transition_result = {
                        let mut state_guard = state.write().await;
                        let context_guard = context.read().await;
                        
                        // Simulate protocol step execution
                        // In a real implementation, this would execute the actual protocol logic
                        state_guard.execute_step(&*context_guard).await
                    };

                    match transition_result {
                        Ok(Some(_transition)) => {
                            // Continue execution
                            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                        }
                        Ok(None) => {
                            // Protocol completed successfully
                            let mut status_guard = status.write().await;
                            *status_guard = SessionStatus::Completed;
                            break;
                        }
                        Err(error) => {
                            // Protocol execution failed
                            let mut status_guard = status.write().await;
                            *status_guard = SessionStatus::Failed(error.to_string());
                            return Err(error);
                        }
                    }
                }
                SessionStatus::Paused => {
                    // Wait while paused
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
                SessionStatus::Cancelled => {
                    // Session was cancelled
                    return Err(RuntimeError::SessionCancelled);
                }
                SessionStatus::Failed(error) => {
                    // Session already failed
                    return Err(RuntimeError::SessionExecutionFailed(error));
                }
                SessionStatus::Completed => {
                    // Session already completed
                    break;
                }
                SessionStatus::Initializing => {
                    // Should not happen in execution
                    return Err(RuntimeError::SessionExecutionFailed("Session still initializing".to_string()));
                }
            }
        }

        Ok(())
    }

    /// Get channel reference
    pub fn channel(&self) -> Arc<TypedChannel<P, R, AIO>> {
        Arc::clone(&self.channel)
    }

    /// Get current protocol state (for debugging/monitoring)
    pub async fn get_state(&self) -> ProtocolState<P> {
        self.state.read().await.clone()
    }
}

impl<P, R, AIO> fmt::Debug for Session<P, R, AIO>
where
    P: LocalProtocol,
    R: Role + SupportsActionIO<AIO>,
    AIO: ActionIOTMarker,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Session")
            .field("id", &self.id)
            .field("protocol", &std::any::type_name::<P>())
            .field("role", &std::any::type_name::<R>())
            .field("action_io", &std::any::type_name::<AIO>())
            .finish()
    }
}

/// Manager for multiple sessions
pub struct SessionManager {
    sessions: RwLock<HashMap<SessionId, Box<dyn SessionHandle>>>,
    default_config: ChannelConfig,
}

/// Trait for type-erased session management
trait SessionHandle: Send + Sync {
    fn id(&self) -> &SessionId;
    fn status(&self) -> impl std::future::Future<Output = SessionStatus> + Send;
    fn cancel(&self) -> impl std::future::Future<Output = Result<(), RuntimeError>> + Send;
}

impl<P, R, AIO> SessionHandle for Session<P, R, AIO>
where
    P: LocalProtocol + Send + Sync,
    R: Role + SupportsActionIO<AIO> + Send + Sync,
    AIO: ActionIOTMarker + Send + Sync,
{
    fn id(&self) -> &SessionId {
        &self.id
    }

    async fn status(&self) -> SessionStatus {
        self.status().await
    }

    async fn cancel(&self) -> Result<(), RuntimeError> {
        self.cancel().await
    }
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(default_config: ChannelConfig) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            default_config,
        }
    }

    /// Create and register a new session
    pub async fn create_session<P, R, AIO>(
        &self,
        id: SessionId,
        protocol: P,
        role: R,
    ) -> Result<(Arc<Session<P, R, AIO>>, TypedChannel<P, R, AIO>), RuntimeError>
    where
        P: LocalProtocol + Send + Sync + 'static,
        R: Role + SupportsActionIO<AIO> + Send + Sync + 'static,
        AIO: ActionIOTMarker + Send + Sync + 'static,
    {
        let (session, channel) = Session::new(id.clone(), protocol, role, self.default_config.clone());
        let session_arc = Arc::new(session);

        // Register the session
        let mut sessions = self.sessions.write().await;
        if sessions.contains_key(&id) {
            return Err(RuntimeError::SessionAlreadyExists(id.to_string()));
        }

        sessions.insert(id, Box::new(Arc::clone(&session_arc)));

        Ok((session_arc, channel))
    }

    /// Get a session by ID
    pub async fn get_session_status(&self, id: &SessionId) -> Option<SessionStatus> {
        let sessions = self.sessions.read().await;
        if let Some(session) = sessions.get(id) {
            Some(session.status().await)
        } else {
            None
        }
    }

    /// Cancel a session by ID
    pub async fn cancel_session(&self, id: &SessionId) -> Result<(), RuntimeError> {
        let sessions = self.sessions.read().await;
        if let Some(session) = sessions.get(id) {
            session.cancel().await
        } else {
            Err(RuntimeError::SessionNotFound(id.to_string()))
        }
    }

    /// Cancel all sessions
    pub async fn cancel_all_sessions(&self) -> Result<(), RuntimeError> {
        let sessions = self.sessions.read().await;
        let mut errors = Vec::new();

        for session in sessions.values() {
            if let Err(error) = session.cancel().await {
                errors.push(error);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(RuntimeError::SessionExecutionFailed(format!(
                "Failed to cancel {} sessions",
                errors.len()
            )))
        }
    }

    /// Remove completed or failed sessions
    pub async fn cleanup_finished_sessions(&self) -> usize {
        let mut sessions = self.sessions.write().await;
        let mut to_remove = Vec::new();

        for (id, session) in sessions.iter() {
            let status = session.status().await;
            match status {
                SessionStatus::Completed | SessionStatus::Failed(_) | SessionStatus::Cancelled => {
                    to_remove.push(id.clone());
                }
                _ => {}
            }
        }

        for id in &to_remove {
            sessions.remove(id);
        }

        to_remove.len()
    }

    /// Get count of sessions by status
    pub async fn session_count_by_status(&self) -> HashMap<SessionStatus, usize> {
        let sessions = self.sessions.read().await;
        let mut counts = HashMap::new();

        for session in sessions.values() {
            let status = session.status().await;
            *counts.entry(status).or_insert(0) += 1;
        }

        counts
    }

    /// Get total number of sessions
    pub async fn total_sessions(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new(ChannelConfig::default())
    }
}

pub mod tests;
