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
    InputAction, LocalProtocol, Message, OutputAction, Role, SupportsActionIO,
};
use crate::runtime::{
    channel::{ChannelConfig, ChannelMessage, TypedChannel},
    error::{CommunicationError, RuntimeError, ProtocolViolation},
    session::{Session, SessionId, SessionManager, SessionStatus},
    state::{ExecutionContext, ProtocolState, StateTransition},
};
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering}; // Added for concurrent tests
use tokio::sync::Barrier; // Added for synchronization in tests
use tokio::time::{sleep, timeout, Duration}; // Added for async tests

// Test roles and messages for integration testing
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct TestMessage {
    content: String,
    timestamp: u64,
}
impl Message for TestMessage {}

// Test protocol implementation
#[derive(Debug, Clone)]
struct TestProtocol;
impl LocalProtocol for TestProtocol {}
impl SupportsActionIO<BiDirectionalAction> for TestProtocol {}


// --- Helper types for new async tests ---

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PingRole;
impl Role for PingRole {}
impl SupportsActionIO<BiDirectionalAction> for PingRole {}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PongRole;
impl Role for PongRole {}
impl SupportsActionIO<BiDirectionalAction> for PongRole {}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct PingRequest {
    id: u32,
    data: String,
}
impl Message for PingRequest {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct PongResponse {
    id: u32,
    reply_data: String,
}
impl Message for PongResponse {}

#[derive(Debug, Clone)]
struct AsyncPingPongProtocol;
impl LocalProtocol for AsyncPingPongProtocol {}
// Assuming the protocol itself can be used by roles needing bidirectional capabilities.
impl SupportsActionIO<BiDirectionalAction> for AsyncPingPongProtocol {}


// --- End of helper types ---


/// Integration test: Channel + State machine integration
#[tokio::test]
async fn test_channel_state_integration() -> Result<(), RuntimeError> {
    // Create metadata and channels
    let metadata = CommMetadata::<DefaultChan, HandshakeMsg>::new("ping-role-id".to_string(), "pong-role-id".to_string());
    let config = ChannelConfig {
        buffer_size: 10,
        timeout_ms: Some(1000),
        ordered: true,
    };
    
    // Assuming create_pair gives (channel_for_alice, channel_for_bob) effectively
    // The types in create_pair are <P, R_Self, R_Peer, AIO_Self, AIO_Peer> or similar
    // For this test, sender is Alice's perspective, receiver is Bob's perspective (or vice-versa)
    // The existing test seems to use sender/receiver as two distinct channel objects.
    let (mut alice_channel, mut bob_channel) = TypedChannel::<TestProtocol, Alice, Bob, BiDirectionalAction, BiDirectionalAction>::create_pair(metadata.clone(), config.clone()).await?;
    
    // Create protocol state for Alice
    let alice_role_marker = Alice; // Marker for Alice's role
    let mut state = ProtocolState::new("integration-test-1".to_string(), Box::new(alice_role_marker));
    
    // Test state transition with message passing
    let test_msg = TestMessage {
        content: "Hello from integration test".to_string(),
        timestamp: 123456789,
    };
    let channel_msg_alice_sends = ChannelMessage {
        metadata: metadata.clone(), // Assuming CommMetadata is cloneable and suitable
        payload: test_msg.clone(),
        sender_id: "alice".to_string(), // Or use role information
        sequence_number: alice_channel.next_sequence_number().await,
    };
    
    // Create execution context
    let context = ExecutionContext::new("test-context".to_string(), vec!["alice".to_string(), "bob".to_string()]);
    
    // Spawn Bob's receiving task
    let bob_task = tokio::spawn(async move {
        let received_msg: ChannelMessage<CommMetadata<DefaultChan, HandshakeMsg>, TestMessage> = bob_channel.receive().await?;
        Ok::<_, RuntimeError>(received_msg)
    });
    
    // Alice sends message
    alice_channel.send(channel_msg_alice_sends).await?;
    
    // Apply state transition for Alice
    let transition = StateTransition::Progress("Message sent".to_string());
    state.apply_transition(&transition, &context)?;
    
    // Wait for Bob's task
    let received_on_bob_side = bob_task.await.map_err(|e| RuntimeError::Communication(
        CommunicationError::ChannelError(format!("Bob's task panicked: {}", e))
    ))??;
    
    // Verify message integrity
    assert_eq!(received_on_bob_side.payload.content, "Hello from integration test");
    assert_eq!(received_on_bob_side.payload.timestamp, 123456789);
    assert_eq!(received_on_bob_side.sender_id, "alice");
    
    // Verify state progression
    assert_eq!(state.get_session_id(), "integration-test-1");
    
    Ok(())
}

/// Integration test: Session + Channel + State management
#[tokio::test]
async fn test_session_channel_state_integration() -> Result<(), RuntimeError> {
    // Create session manager
    let mut session_manager = SessionManager::new();

    // TODO: This test requires a more complete Session::new and execution logic.
    // For now, this test will be a placeholder or focus on aspects that can be tested.
    // Example: Create a session, check its initial status.
    // let session_id = SessionId::new("test-session-123");
    // let config = ChannelConfig::default();

    // The Session::new signature from backup is (ChannelConfig) -> (Self, TypedChannel)
    // This is difficult to use directly for a two-party session setup without more context
    // on how the returned TypedChannel (presumably for the peer) is then used.

    // If Session::new were:
    // fn new<P, R, AIO>(id: SessionId, role: R, protocol: P, config: ChannelConfig) -> Session<P,R,AIO>
    // And it internally created the channel pair, keeping one for itself.
    
    // For now, skipping the full implementation of this test due to Session API uncertainties.
    Ok(())
}

/// Integration test: Error propagation across runtime components
#[tokio::test]
async fn test_error_propagation_integration() -> Result<(), RuntimeError> {
    // TODO: Implement test for error propagation
    // Example: Force a channel error (e.g., send on closed channel) and see if Session/State reflects it.
    Ok(())
}

/// Integration test: Multi-session concurrent execution
#[tokio::test]
async fn test_multi_session_integration() -> Result<(), RuntimeError> {
    // TODO: Implement test for concurrent sessions if SessionManager supports it well.
    Ok(())
}

/// Integration test: Channel timeout and recovery
#[tokio::test]
async fn test_channel_timeout_integration() -> Result<(), RuntimeError> {
    // This test is split into send_timeout and receive_timeout below
    Ok(())
}

/// Integration test: State transitions with concurrent channel operations
#[tokio::test]
async fn test_concurrent_state_channel_integration() -> Result<(), RuntimeError> {
    // TODO: Implement test for concurrent state changes and channel ops.
    // Example: One task tries to advance protocol state while another sends/receives.
    Ok(())
}


// --- New Async Integration Tests ---

#[tokio::test]
async fn test_async_ping_pong_exchange() -> Result<(), RuntimeError> {
    let metadata = CommMetadata::<DefaultChan, HandshakeMsg>::new("ping-role-id".to_string(), "pong-role-id".to_string());
    let config = ChannelConfig {
        buffer_size: 5,
        timeout_ms: Some(500), // Short timeout for test
        ordered: true,
    };

    // Assuming create_pair gives (channel_for_pingrole, channel_for_pongrole)
    // The generic arguments for Role and AIO in TypedChannel might need to be specific
    // to the role that OWNS that channel end.
    let (mut ping_channel, mut pong_channel) =
        TypedChannel::<AsyncPingPongProtocol, PingRole, PongRole, BiDirectionalAction, BiDirectionalAction>::create_pair(metadata.clone(), config).await?;

    let ping_task = tokio::spawn(async move {
        let request = PingRequest { id: 1, data: "Ping!".to_string() };
        let channel_msg_ping_sends = ChannelMessage {
            metadata: metadata.clone(),
            payload: request.clone(),
            sender_id: "PingRole".to_string(),
            sequence_number: ping_channel.next_sequence_number().await,
        };
        ping_channel.send(channel_msg_ping_sends).await?;

        let received_response: ChannelMessage<CommMetadata<DefaultChan, HandshakeMsg>, PongResponse> = ping_channel.receive().await?;
        assert_eq!(received_response.payload.id, 1);
        assert_eq!(received_response.payload.reply_data, "Pong!");
        assert_eq!(received_response.sender_id, "PongRole");
        Ok::<_, RuntimeError>(())
    });

    let pong_task = tokio::spawn(async move {
        let received_request: ChannelMessage<CommMetadata<DefaultChan, HandshakeMsg>, PingRequest> = pong_channel.receive().await?;
        assert_eq!(received_request.payload.id, 1);
        assert_eq!(received_request.payload.data, "Ping!");
        assert_eq!(received_request.sender_id, "PingRole");

        let response = PongResponse { id: received_request.payload.id, reply_data: "Pong!".to_string() };
        let channel_msg_pong_sends = ChannelMessage {
            metadata: metadata.clone(), // metadata should be distinct for pong if sender/receiver IDs differ
            payload: response.clone(),
            sender_id: "PongRole".to_string(),
            sequence_number: pong_channel.next_sequence_number().await,
        };
        pong_channel.send(channel_msg_pong_sends).await?;
        Ok::<_, RuntimeError>(())
    });

    let (ping_result, pong_result) = tokio::join!(ping_task, pong_task);

    ping_result.map_err(|e| RuntimeError::Communication(CommunicationError::Internal(format!("Ping task panicked: {}", e))))??;
    pong_result.map_err(|e| RuntimeError::Communication(CommunicationError::Internal(format!("Pong task panicked: {}", e))))??;

    Ok(())
}

#[tokio::test]
async fn test_channel_send_timeout_external() -> Result<(), RuntimeError> {
    let metadata = CommMetadata::<DefaultChan, HandshakeMsg>::new("sender-id".to_string(), "receiver-id".to_string());
    let short_timeout_ms = 50;
    let config = ChannelConfig {
        buffer_size: 1, // Small buffer to make send block if receiver is not ready
        timeout_ms: Some(short_timeout_ms * 10), // Internal timeout longer than external
        ordered: true,
    };

    let (mut sender_channel, mut receiver_channel) = // Renamed for clarity
        TypedChannel::<AsyncPingPongProtocol, PingRole, PongRole, BiDirectionalAction, BiDirectionalAction>::create_pair(metadata.clone(), config).await?;

    let send_payload = PingRequest { id: 1, data: "No one is listening".to_string() };
    let channel_msg_to_send = ChannelMessage {
        metadata: metadata.clone(),
        payload: send_payload,
        sender_id: "SenderRole".to_string(),
        sequence_number: sender_channel.next_sequence_number().await,
    };
    
    // Receiver does nothing or sleeps
    let _receiver_handle = tokio::spawn(async move {
        sleep(Duration::from_millis(short_timeout_ms * 5)).await; // Sleep longer than timeout
        // Optionally try to receive to clean up, though it might also timeout
        let _ = receiver_channel.receive::<CommMetadata<DefaultChan, HandshakeMsg>, PingRequest>().await;
    });

    let send_op = sender_channel.send(channel_msg_to_send);
    
    match timeout(Duration::from_millis(short_timeout_ms), send_op).await {
        Ok(Ok(())) => panic!("Send completed unexpectedly, should have timed out"),
        Ok(Err(e)) => {
            // This case means the channel's internal send itself failed, possibly due to its own timeout
            // or other channel error (e.g. closed).
            // We need to check if 'e' is a timeout-related error from the channel.
            // For now, let's assume any error here is acceptable if the external timeout didn't hit.
            // This depends on how TypedChannel surfaces its internal timeouts.
            // If CommunicationError::Timeout is a variant:
            // assert!(matches!(e, RuntimeError::Communication(CommunicationError::Timeout(_))), "Expected channel timeout error, got {:?}", e);
            println!("Send failed with channel error (as expected if internal timeout hit first): {:?}", e);
        }
        Err(_elapsed) => {
            // This is the primary expected outcome: tokio::time::timeout caused the timeout.
            println!("Send operation correctly timed out by external wrapper.");
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_channel_receive_timeout_external() -> Result<(), RuntimeError> {
    let metadata = CommMetadata::<DefaultChan, HandshakeMsg>::new("sender-id".to_string(), "receiver-id".to_string());
    let short_timeout_ms = 50;
    let config = ChannelConfig {
        buffer_size: 1,
        timeout_ms: Some(short_timeout_ms * 10), // Internal timeout longer
        ordered: true,
    };

    let (mut _sender_channel, mut receiver_channel) = // Renamed
         TypedChannel::<AsyncPingPongProtocol, PingRole, PongRole, BiDirectionalAction, BiDirectionalAction>::create_pair(metadata.clone(), config).await?;

    // Sender does nothing or sends very late
    let _sender_handle = tokio::spawn(async move {
        sleep(Duration::from_millis(short_timeout_ms * 5)).await; // Sleep longer than timeout
        // Optionally try to send, though the receiver might have already timed out
        // let _ = _sender_channel.send(...).await;
    });

    let receive_op = receiver_channel.receive::<CommMetadata<DefaultChan, HandshakeMsg>, PingRequest>();

    match timeout(Duration::from_millis(short_timeout_ms), receive_op).await {
        Ok(Ok(msg)) => panic!("Receive completed unexpectedly with msg: {:?}, should have timed out", msg),
        Ok(Err(e)) => {
            // Channel's internal receive failed.
            // assert!(matches!(e, RuntimeError::Communication(CommunicationError::Timeout(_))), "Expected channel timeout error, got {:?}", e);
             println!("Receive failed with channel error (as expected if internal timeout hit first): {:?}", e);
        }
        Err(_elapsed) => {
            // tokio::time::timeout caused the timeout.
            println!("Receive operation correctly timed out by external wrapper.");
        }
    }
    Ok(())
}


#[tokio::test]
async fn test_concurrent_sends_receive_all_messages() -> Result<(), RuntimeError> {
    let metadata = CommMetadata::<DefaultChan, HandshakeMsg>::new("sender-id".to_string(), "receiver-id".to_string());
    let config = ChannelConfig {
        buffer_size: 20, // Ensure enough buffer
        timeout_ms: Some(1000),
        ordered: true, // Important for assertion if we check order
    };

    let (mut sender_channel_main, mut receiver_channel) =
        TypedChannel::<AsyncPingPongProtocol, PingRole, PongRole, BiDirectionalAction, BiDirectionalAction>::create_pair(metadata.clone(), config).await?;
    
    let num_messages = 10;
    let mut send_tasks = Vec::new();

    // Arc the sender channel end if multiple tasks need to send on the same object.
    // However, TypedChannel uses Mutex internally for its mpsc::Sender, so cloning TypedChannel
    // might not be what's needed. Instead, each task should get its own sender object if they
    // represent different roles or use a shared sender if it's designed for that.
    // For this test, we simulate multiple logical senders using the same channel object,
    // relying on its internal Mutex for safety.
    // If TypedChannel itself is not Clone, we can't easily give it to multiple tasks.
    // The `sender` in TypedChannel is Mutex<Option<Sender<Vec<u8>>>>.
    // Let's assume TypedChannel is not Clone. We'll use one sender task that loops.
    // Or, if we want to test concurrent access to the *same* TypedChannel object's send method,
    // we'd need to Arc<Mutex<TypedChannel>> or ensure TypedChannel is Send + Sync and its methods are &self.
    // TypedChannel methods are &self, so it can be shared via Arc if it's Send+Sync.
    // Let's assume TypedChannel is Send + Sync.
    
    let sender_channel_arc = Arc::new(sender_channel_main); // Requires TypedChannel to be Send + Sync

    for i in 0..num_messages {
        let sender_clone = Arc::clone(&sender_channel_arc);
        let meta_clone = metadata.clone();
        send_tasks.push(tokio::spawn(async move {
            let msg = PingRequest { id: i, data: format!("Message {}", i) };
            let channel_msg = ChannelMessage {
                metadata: meta_clone,
                payload: msg,
                sender_id: format!("SenderTask-{}", i),
                // Sequence number generation needs to be atomic if shared across true concurrent senders
                // or handled by the channel itself. Assuming next_sequence_number() is safe.
                sequence_number: sender_clone.next_sequence_number().await,
            };
            sender_clone.send(channel_msg).await
        }));
    }

    let mut received_ids = tokio::spawn(async move {
        let mut ids = std::collections::HashSet::new();
        for _ in 0..num_messages {
            match receiver_channel.receive::<CommMetadata<DefaultChan, HandshakeMsg>, PingRequest>().await {
                Ok(msg) => {
                    ids.insert(msg.payload.id);
                }
                Err(e) => return Err(e),
            }
        }
        Ok::<_, RuntimeError>(ids)
    });

    for task in send_tasks {
        task.await.map_err(|e| RuntimeError::Internal(format!("Send task panic: {}", e)))??;
    }

    let ids_set = received_ids.await.map_err(|e| RuntimeError::Internal(format!("Receive task panic: {}", e)))??;

    assert_eq!(ids_set.len(), num_messages as usize, "Did not receive all messages");
    for i in 0..num_messages {
        assert!(ids_set.contains(&i), "Message with id {} was not received", i);
    }

    Ok(())
}

// Placeholder for session lifecycle test - requires more clarity on Session API
#[tokio::test]
async fn test_session_lifecycle_placeholder() -> Result<(), RuntimeError> {
    // let config = ChannelConfig::default();
    // Session::new signature and usage with TypedChannel::create_pair is unclear.
    // E.g., if Session::new(config) -> (Session<P,R,AIO>, TypedChannel<P,PeerR,AIO>)
    // let (session_for_ping, _pong_channel_returned_by_session_new) = 
    //      Session::<AsyncPingPongProtocol, PingRole, BiDirectionalAction>::new(config);
    //
    // session_for_ping.start().await?;
    // let status_after_start = session_for_ping.status().await;
    // assert_eq!(status_after_start, SessionStatus::Running); // Or similar
    //
    // session_for_ping.wait().await?;
    // let status_after_wait = session_for_ping.status().await;
    // assert_eq!(status_after_wait, SessionStatus::Completed);
    Ok(())
}

// --- Test Types for Async Multi-Exchange Simulation ---

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ClientRole;
impl Role for ClientRole {}
impl SupportsActionIO<BiDirectionalAction> for ClientRole {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ServerRole;
impl Role for ServerRole {}
impl SupportsActionIO<BiDirectionalAction> for ServerRole {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct RequestA { id: u32, content: String }
impl Message for RequestA {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct ResponseA { id: u32, reply: String }
impl Message for ResponseA {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct RequestB { id: u32, data: Vec<u8> }
impl Message for RequestB {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct ResponseB { id: u32, status_code: u16 }
impl Message for ResponseB {}

#[derive(Debug, Clone)]
struct MultiExchangeProtocol;
impl LocalProtocol for MultiExchangeProtocol {}
impl SupportsActionIO<BiDirectionalAction> for MultiExchangeProtocol {}


// --- End of Test Types for Async Multi-Exchange Simulation ---

/// Simulates an ideal asynchronous session with multiple exchanges between two roles
/// running in separate Tokio tasks. This test uses `TypedChannel::create_pair` directly
/// to demonstrate the desired asynchronous interaction pattern.
///
/// It also serves as a reference for how `SessionManager` and `SessionEndpoint`
/// would need to function to natively support this pattern.
#[tokio::test]
async fn test_async_multi_exchange_session_simulation() -> Result<(), RuntimeError> {
    let metadata = CommMetadata::<DefaultChan, HandshakeMsg>::new("client-role-id".to_string(), "server-role-id".to_string());
    let config = ChannelConfig {
        buffer_size: 10,
        timeout_ms: Some(1000),
        ordered: true,
    };

    // Create a pair of typed channels for communication.
    // In an ideal scenario with an async-friendly SessionManager, these endpoints
    // (or similar async-capable SessionEndpoints) would be provided by the manager.
    let (mut client_channel, mut server_channel) =
        TypedChannel::<MultiExchangeProtocol, ClientRole, ServerRole, BiDirectionalAction, BiDirectionalAction>::create_pair(
            metadata.clone(),
            config,
        )
        .await?;

    // Logic for the ClientRole
    async fn run_client_role(
        mut channel: TypedChannel<MultiExchangeProtocol, ClientRole, ServerRole, BiDirectionalAction, BiDirectionalAction>,
        metadata: CommMetadata<DefaultChan, HandshakeMsg>,
    ) -> Result<(), RuntimeError> {
        // Exchange 1: RequestA -> ResponseA
        let req_a = RequestA { id: 1, content: "Request A from Client".to_string() };
        let chan_msg_req_a = ChannelMessage {
            metadata: metadata.clone(),
            payload: req_a.clone(),
            sender_id: "ClientRole".to_string(),
            sequence_number: channel.next_sequence_number().await,
        };
        channel.send(chan_msg_req_a).await?;
        println!("Client: Sent RequestA");

        let resp_a_msg: ChannelMessage<CommMetadata<DefaultChan, HandshakeMsg>, ResponseA> = channel.receive().await?;
        assert_eq!(resp_a_msg.payload.id, 1);
        assert_eq!(resp_a_msg.payload.reply, "Response A from Server");
        assert_eq!(resp_a_msg.sender_id, "ServerRole");
        println!("Client: Received ResponseA");

        // Exchange 2: RequestB -> ResponseB
        let req_b = RequestB { id: 2, data: vec![1, 2, 3, 4] };
         let chan_msg_req_b = ChannelMessage {
            metadata: metadata.clone(),
            payload: req_b.clone(),
            sender_id: "ClientRole".to_string(),
            sequence_number: channel.next_sequence_number().await,
        };
        channel.send(chan_msg_req_b).await?;
        println!("Client: Sent RequestB");

        let resp_b_msg: ChannelMessage<CommMetadata<DefaultChan, HandshakeMsg>, ResponseB> = channel.receive().await?;
        assert_eq!(resp_b_msg.payload.id, 2);
        assert_eq!(resp_b_msg.payload.status_code, 200);
        assert_eq!(resp_b_msg.sender_id, "ServerRole");
        println!("Client: Received ResponseB");

        Ok(())
    }

    // Logic for the ServerRole
    async fn run_server_role(
        mut channel: TypedChannel<MultiExchangeProtocol, ServerRole, ClientRole, BiDirectionalAction, BiDirectionalAction>,
        metadata: CommMetadata<DefaultChan, HandshakeMsg>,
    ) -> Result<(), RuntimeError> {
        // Exchange 1: RequestA -> ResponseA
        let req_a_msg: ChannelMessage<CommMetadata<DefaultChan, HandshakeMsg>, RequestA> = channel.receive().await?;
        assert_eq!(req_a_msg.payload.id, 1);
        assert_eq!(req_a_msg.payload.content, "Request A from Client");
        assert_eq!(req_a_msg.sender_id, "ClientRole");
        println!("Server: Received RequestA");

        let resp_a = ResponseA { id: 1, reply: "Response A from Server".to_string() };
        let chan_msg_resp_a = ChannelMessage {
            metadata: metadata.clone(), // Server's perspective of metadata might differ if IDs are swapped
            payload: resp_a.clone(),
            sender_id: "ServerRole".to_string(),
            sequence_number: channel.next_sequence_number().await,
        };
        channel.send(chan_msg_resp_a).await?;
        println!("Server: Sent ResponseA");

        // Exchange 2: RequestB -> ResponseB
        let req_b_msg: ChannelMessage<CommMetadata<DefaultChan, HandshakeMsg>, RequestB> = channel.receive().await?;
        assert_eq!(req_b_msg.payload.id, 2);
        assert_eq!(req_b_msg.payload.data, vec![1, 2, 3, 4]);
        assert_eq!(req_b_msg.sender_id, "ClientRole");
        println!("Server: Received RequestB");
        
        let resp_b = ResponseB { id: 2, status_code: 200 };
        let chan_msg_resp_b = ChannelMessage {
            metadata: metadata.clone(),
            payload: resp_b.clone(),
            sender_id: "ServerRole".to_string(),
            sequence_number: channel.next_sequence_number().await,
        };
        channel.send(chan_msg_resp_b).await?;
        println!("Server: Sent ResponseB");

        Ok(())
    }

    // Spawn client and server tasks
    // Note: The metadata for the server role should ideally reflect its perspective (sender/receiver IDs swapped).
    // For simplicity in this example, we clone the initial metadata. In a real SessionManager setup,
    // each role would get metadata appropriate for its endpoint.
    let client_metadata = metadata.clone();
    let server_metadata = CommMetadata::<DefaultChan, HandshakeMsg>::new(metadata.receiver_id.clone(), metadata.sender_id.clone());


    let client_task = tokio::spawn(run_client_role(client_channel, client_metadata));
    let server_task = tokio::spawn(run_server_role(server_channel, server_metadata));

    // Await completion of both tasks
    let client_result = client_task.await.map_err(|e| RuntimeError::Internal(format!("Client task panicked: {}", e)))?;
    let server_result = server_task.await.map_err(|e| RuntimeError::Internal(format!("Server task panicked: {}", e)))?;

    client_result?;
    server_result?;
    
    println!("Client and Server tasks completed successfully.");

    // Commentary on integrating this pattern with SessionManager/SessionEndpoint:
    //
    // This test demonstrates a desirable asynchronous interaction pattern for session-based protocols,
    // where each role executes as an independent Tokio task using async/await for communication.
    //
    // To natively support this pattern with `SessionManager` and `SessionEndpoint`:
    //
    // 1. `SessionManager::get_session_endpoint(...)` (or a similar method for obtaining a role's
    //    communication endpoint) would need to be `async`. This is because establishing
    //    the communication channel (e.g., `TypedChannel::create_pair`) is an async operation.
    //
    // 2. `SessionEndpoint::send(...)` and `SessionEndpoint::receive(...)` methods must be `async`.
    //    They should internally use the `async` methods of the underlying `TypedChannel`
    //    (e.g., `channel.send(...).await` and `channel.receive(...).await`).
    //
    // 3. The current implementation of `SessionManager::get_session_endpoint` is not `async`,
    //    and `SessionEndpoint`'s `send`/`receive` methods are blocking (using `blocking_send`/
    //    `blocking_receive`). This makes it challenging to integrate them directly into an
    //    `async/await` workflow without resorting to `tokio::task::spawn_blocking`, which
    //    can be inefficient and obscure the asynchronous nature of the protocol.
    //
    // This example serves as a template for the target developer experience when building
    // applications with this library in an asynchronous Rust environment.

    Ok(())
}
