# Recursion in Multiparty Session Types (MPST)

## Overview

Recursion in multiparty session types (MPST) enables the specification of protocols with
repeating, looping, or cyclic communication patterns. This capability is essential for
modeling real-world protocols that involve ongoing interactions, such as streaming data,
persistent connections, or iterative request-response cycles.

### Key Benefits

- **Protocol Expressiveness**: Enables modeling of infinite or long-running protocols
- **Loop Safety**: Provides compile-time guarantees about recursive communication patterns  
- **Resource Management**: Runtime support for tracking and managing recursive contexts
- **Practical Applications**: Chat sessions, streaming protocols, retry mechanisms, and server loops

## Current Implementation Status

The Besedarium library currently provides **runtime recursion support** through the session
context system, with planned compile-time recursion types following the established `TChan*`
pattern used throughout the codebase.

### Runtime Support (Available Now)

The runtime system in `src/runtime/state.rs` provides immediate recursion capabilities:

```rust
use besedarium::runtime::{SessionContext, ChanRef};

async fn server_loop(chan: ChanRef, mut ctx: SessionContext) -> Result<(), Box<dyn Error>> {
    // Enter recursion context with depth tracking
    ctx.enter_recursion("server_main_loop")?;
    
    loop {
        // Handle client request
        let request: ClientRequest = chan.recv().await?;
        
        // Process and respond
        let response = process_request(request).await?;
        chan.send(response).await?;
        
        // Check termination condition
        if should_shutdown() {
            break;
        }
    }
    
    // Clean exit from recursion context  
    ctx.exit_recursion("server_main_loop")?;
    Ok(())
}
```

### Planned Type System (Under Development)

The compile-time recursion types will follow the established `TChan*` naming convention:

- **`TChanRec<Label, S>`**: Recursion point in global protocols
- **`TChanVar<Label>`**: Variable reference to recursion point
- **`EpRec<Label, Role, S>`**: Local protocol recursion point (after projection)
- **`EpVar<Label, Role>`**: Local protocol variable reference

## Planned Type System Design

### Global Protocol Recursion Types

Following the current `TChan*` pattern used in `src/protocol/global/protocols.rs`:

```rust
use std::marker::PhantomData;
use crate::protocol::global::TSession;

/// Recursion point in global protocol
pub struct TChanRec<Label, S> 
where
    Label: ProtocolLabel,
    S: TSession,
{
    _phantom: PhantomData<(Label, S)>,
}

/// Variable reference to recursion point  
pub struct TChanVar<Label>
where
    Label: ProtocolLabel,
{
    _phantom: PhantomData<Label>,
}

impl<Label, S> TSession for TChanRec<Label, S>
where
    Label: ProtocolLabel,
    S: TSession,
{
    // Implementation details...
}

impl<Label> TSession for TChanVar<Label>
where
    Label: ProtocolLabel,
{
    // Implementation details...
}
```

### Local Protocol Recursion Types

After projection to local protocols:

```rust
use crate::protocol::local::Session;

/// Local recursion point
pub struct EpRec<Label, Role, S>
where
    Label: ProtocolLabel,
    Role: 'static,
    S: Session<Role>,
{
    _phantom: PhantomData<(Label, Role, S)>,
}

/// Local variable reference
pub struct EpVar<Label, Role>
where
    Label: ProtocolLabel,
    Role: 'static,
{
    _phantom: PhantomData<(Label, Role)>,
}
```

### Protocol Labels

Type-safe recursion labels using the current pattern:

```rust
/// Trait for type-level protocol labels
pub trait ProtocolLabel: 'static + Send + Sync {
    const LABEL: &'static str;
}

// Example label definitions
#[derive(Debug, Clone, Copy)]
pub struct MainServerLoop;

impl ProtocolLabel for MainServerLoop {
    const LABEL: &'static str = "main_server_loop";
}

#[derive(Debug, Clone, Copy)]  
pub struct ClientRetryLoop;

impl ProtocolLabel for ClientRetryLoop {
    const LABEL: &'static str = "client_retry_loop";
}
```

## Runtime Recursion Support

The current runtime system provides comprehensive recursion management through the 
`SessionContext` type in `src/runtime/state.rs`.

### Recursion Depth Tracking

```rust
use besedarium::runtime::SessionContext;

async fn recursive_protocol(mut ctx: SessionContext) -> Result<(), Box<dyn Error>> {
    // Enter recursion - automatically tracks depth
    ctx.enter_recursion("my_protocol_loop")?;
    
    // Recursive logic here...
    for i in 0..1000 {
        // Each iteration is tracked
        process_iteration(i).await?;
        
        // Runtime prevents stack overflow
        if ctx.recursion_depth("my_protocol_loop")? > MAX_SAFE_DEPTH {
            return Err("Recursion depth limit exceeded".into());
        }
    }
    
    // Always exit recursion context
    ctx.exit_recursion("my_protocol_loop")?;
    Ok(())
}
```

### Error Handling in Recursive Contexts

```rust
async fn robust_recursive_server(
    chan: ChanRef, 
    mut ctx: SessionContext
) -> Result<(), Box<dyn Error>> {
    ctx.enter_recursion("server_loop")?;
    
    // Use RAII pattern for guaranteed cleanup
    let _guard = RecursionGuard::new(&mut ctx, "server_loop");
    
    loop {
        match handle_client_interaction(&chan).await {
            Ok(should_continue) if should_continue => continue,
            Ok(_) => break, // Clean termination
            Err(e) => {
                eprintln!("Error in server loop: {}", e);
                // Decide whether to continue or abort based on error type
                if is_recoverable_error(&e) {
                    continue;
                } else {
                    return Err(e);
                }
            }
        }
    }
    
    Ok(())
    // _guard automatically calls exit_recursion on drop
}
```

## Protocol Projection for Recursion

When recursion types are implemented, projection will follow these rules:

### Projection Rules

1. **Recursion Point Projection**:

   ```rust
   // Global: TChanRec<Label, S>
   // Projects to each participating role:
   // Role R: EpRec<Label, R, Project(S, R)>
   ```

2. **Variable Reference Projection**:

   ```rust
   // Global: TChanVar<Label>  
   // Projects to each participating role:
   // Role R: EpVar<Label, R>
   ```

3. **Label Preservation**:
   - Recursion labels maintain their identity across projection
   - Type safety ensures matching labels between recursion points and variables

### Example: Ping-Pong Protocol with Planned Types

```rust
use besedarium::protocol::global::*;
use besedarium::protocol::local::*;

// Define roles
#[derive(Debug, Clone, Copy)]
pub struct Client;

#[derive(Debug, Clone, Copy)]
pub struct Server;

// Define recursion label
#[derive(Debug, Clone, Copy)]
pub struct PingLoop;

impl ProtocolLabel for PingLoop {
    const LABEL: &'static str = "ping_loop";
}

// Global protocol with recursion (planned)
type PingPongProtocol = TChanRec<PingLoop,
    TChanSend<Client, Server, String,
    TChanRecv<Server, Client, String,
    TChanChoice<Client, Server, {
        Continue: TChanVar<PingLoop>,
        Stop: TChanEnd
    }>>>>;

// Client's local protocol (after projection)
type ClientPingPong = EpRec<PingLoop, Client,
    EpSend<Server, String,
    EpRecv<Server, String,  
    EpChoice<Server, {
        Continue: EpVar<PingLoop, Client>,
        Stop: EpEnd
    }>>>>;

// Server's local protocol (after projection)  
type ServerPingPong = EpRec<PingLoop, Server,
    EpRecv<Client, String,
    EpSend<Client, String,
    EpOffer<Client, {
        Continue: EpVar<PingLoop, Server>,
        Stop: EpEnd  
    }>>>>;
```

## Practical Usage Examples

### Example 1: HTTP Server with Current Runtime Support

```rust
use besedarium::runtime::{ChanRef, SessionContext};
use tokio::net::TcpListener;

async fn http_server_main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let mut ctx = SessionContext::new();
    
    // Enter main server loop
    ctx.enter_recursion("http_server_main")?;
    
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);
        
        // Spawn handler for each connection
        let mut client_ctx = ctx.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_http_client(socket, &mut client_ctx).await {
                eprintln!("Client handler error: {}", e);
            }
        });
        
        // Check for shutdown signal
        if should_shutdown() {
            break;
        }
    }
    
    ctx.exit_recursion("http_server_main")?;
    println!("Server shutting down gracefully");
    Ok(())
}

async fn handle_http_client(
    socket: tokio::net::TcpStream, 
    ctx: &mut SessionContext
) -> Result<(), Box<dyn Error>> {
    let chan = ChanRef::from_tcp_stream(socket)?;
    
    // Enter client handling loop
    ctx.enter_recursion("client_session")?;
    
    loop {
        // Read HTTP request
        let request: HttpRequest = chan.recv().await?;
        
        // Process request
        let response = match request.path.as_str() {
            "/health" => HttpResponse::ok("Server is healthy"),
            "/data" => HttpResponse::ok(&get_data().await?),
            _ => HttpResponse::not_found("Resource not found"),
        };
        
        // Send response
        chan.send(response).await?;
        
        // Check if client wants to keep connection alive
        if !request.keep_alive {
            break;
        }
    }
    
    ctx.exit_recursion("client_session")?;
    Ok(())
}
```

### Example 2: Streaming Data Protocol

```rust
use besedarium::runtime::{ChanRef, SessionContext};
use tokio_stream::{Stream, StreamExt};

async fn data_streaming_server<S>(
    chan: ChanRef,
    mut data_stream: S,
    mut ctx: SessionContext
) -> Result<(), Box<dyn Error>>
where
    S: Stream<Item = Result<DataChunk, std::io::Error>> + Unpin,
{
    ctx.enter_recursion("streaming_loop")?;
    
    // Send initial handshake
    chan.send(StreamStart).await?;
    
    loop {
        match data_stream.next().await {
            Some(Ok(chunk)) => {
                // Send data chunk
                chan.send(DataMessage { chunk }).await?;
                
                // Wait for acknowledgment
                let ack: Acknowledgment = chan.recv().await?;
                
                match ack {
                    Acknowledgment::Continue => continue,
                    Acknowledgment::Stop => break,
                    Acknowledgment::Error(e) => {
                        eprintln!("Client reported error: {}", e);
                        break;
                    }
                }
            }
            Some(Err(e)) => {
                // Send error to client
                chan.send(StreamError { error: e.to_string() }).await?;
                break;
            }
            None => {
                // Stream ended naturally
                chan.send(StreamEnd).await?;
                break;
            }
        }
    }
    
    ctx.exit_recursion("streaming_loop")?;
    Ok(())
}
```

## Implementation Guidelines

### 1. Resource Management

Always use proper resource management for recursion contexts:

```rust
// RAII helper for automatic cleanup
pub struct RecursionGuard<'a> {
    ctx: &'a mut SessionContext,
    label: &'static str,
}

impl<'a> RecursionGuard<'a> {
    pub fn new(ctx: &'a mut SessionContext, label: &'static str) -> Result<Self, RecursionError> {
        ctx.enter_recursion(label)?;
        Ok(RecursionGuard { ctx, label })
    }
}

impl<'a> Drop for RecursionGuard<'a> {
    fn drop(&mut self) {
        if let Err(e) = self.ctx.exit_recursion(self.label) {
            eprintln!("Error exiting recursion context: {}", e);
        }
    }
}
```

### 2. Label Management

Use descriptive, hierarchical labels for complex protocols:

```rust
#[derive(Debug, Clone, Copy)]
pub struct ServerMainLoop;

#[derive(Debug, Clone, Copy)] 
pub struct ClientSessionLoop;

#[derive(Debug, Clone, Copy)]
pub struct AuthRetryLoop;

#[derive(Debug, Clone, Copy)]
pub struct DataTransferLoop;

impl ProtocolLabel for ServerMainLoop {
    const LABEL: &'static str = "server.main_loop";
}

impl ProtocolLabel for ClientSessionLoop {
    const LABEL: &'static str = "client.session_loop";
}

impl ProtocolLabel for AuthRetryLoop {
    const LABEL: &'static str = "auth.retry_loop"; 
}

impl ProtocolLabel for DataTransferLoop {
    const LABEL: &'static str = "data.transfer_loop";
}
```

### 3. Depth Limits and Monitoring

Implement appropriate depth limits and monitoring:

```rust
const DEFAULT_MAX_RECURSION_DEPTH: usize = 1000;
const MONITORING_INTERVAL: usize = 100;

async fn monitored_recursive_protocol(mut ctx: SessionContext) -> Result<(), Box<dyn Error>> {
    ctx.enter_recursion("monitored_loop")?;
    let _guard = RecursionGuard::new(&mut ctx, "monitored_loop")?;
    
    let mut iteration_count = 0;
    
    loop {
        // Protocol logic...
        iteration_count += 1;
        
        // Periodic monitoring
        if iteration_count % MONITORING_INTERVAL == 0 {
            let depth = ctx.recursion_depth("monitored_loop")?;
            println!("Recursion depth: {}, iterations: {}", depth, iteration_count);
            
            if depth > DEFAULT_MAX_RECURSION_DEPTH * 0.8 as usize {
                println!("Warning: Approaching recursion depth limit");
            }
        }
        
        // Termination condition
        if should_terminate() {
            break;
        }
    }
    
    println!("Completed {} iterations", iteration_count);
    Ok(())
}
```

## Error Handling and Recovery

### Recursion-Specific Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum RecursionError {
    #[error("Recursion depth limit exceeded for label '{label}': {depth} > {limit}")]
    DepthLimitExceeded {
        label: String,
        depth: usize,
        limit: usize,
    },
    
    #[error("Attempting to exit recursion '{label}' that was never entered")]
    UnbalancedExit { label: String },
    
    #[error("Recursion label '{label}' not found in current context")]
    LabelNotFound { label: String },
    
    #[error("Nested recursion labels must be unique: '{label}' already active")]
    DuplicateLabel { label: String },
}
```

### Recovery Strategies

```rust
async fn resilient_server_loop(
    chan: ChanRef,
    mut ctx: SessionContext
) -> Result<(), Box<dyn Error>> {
    let mut retry_count = 0;
    const MAX_RETRIES: u32 = 3;
    
    loop {
        match run_server_iteration(&chan, &mut ctx).await {
            Ok(()) => {
                retry_count = 0; // Reset on success
            }
            Err(e) if is_recoverable(&e) => {
                retry_count += 1;
                if retry_count <= MAX_RETRIES {
                    eprintln!("Recoverable error (attempt {}): {}", retry_count, e);
                    
                    // Cleanup recursion state if needed
                    if let Err(_) = ctx.exit_recursion("server_iteration") {
                        // Force cleanup
                        ctx = SessionContext::new();
                    }
                    
                    // Brief delay before retry
                    tokio::time::sleep(Duration::from_millis(100 * retry_count as u64)).await;
                    continue;
                } else {
                    return Err(format!("Max retries exceeded: {}", e).into());
                }
            }
            Err(e) => {
                return Err(format!("Unrecoverable error: {}", e).into());
            }
        }
        
        if should_shutdown() {
            break;
        }
    }
    
    Ok(())
}
```

## Advanced Patterns

### Nested Recursion

Using multiple recursion levels with different labels:

```rust
async fn nested_protocol_handler(
    chan: ChanRef,
    mut ctx: SessionContext
) -> Result<(), Box<dyn Error>> {
    // Outer loop: main protocol execution
    ctx.enter_recursion("main_protocol")?;
    
    loop {
        // Handle protocol phase
        let phase_result = {
            // Inner loop: retry mechanism
            ctx.enter_recursion("retry_phase")?;
            let _inner_guard = RecursionGuard::new(&mut ctx, "retry_phase")?;
            
            let mut attempts = 0;
            loop {
                attempts += 1;
                
                match execute_protocol_phase(&chan).await {
                    Ok(result) => break Ok(result),
                    Err(e) if attempts < MAX_PHASE_RETRIES => {
                        eprintln!("Phase attempt {} failed: {}", attempts, e);
                        continue;
                    }
                    Err(e) => break Err(e),
                }
            }
        };
        
        match phase_result {
            Ok(should_continue) if should_continue => continue,
            Ok(_) => break, // Protocol completed successfully
            Err(e) => return Err(e), // Unrecoverable error
        }
    }
    
    ctx.exit_recursion("main_protocol")?;
    Ok(())
}
```

### Conditional Recursion

Implementing choice-based recursion patterns:

```rust
async fn conditional_server_loop(
    chan: ChanRef,
    mut ctx: SessionContext
) -> Result<(), Box<dyn Error>> {
    ctx.enter_recursion("conditional_loop")?;
    
    loop {
        // Receive client choice
        let client_choice: ClientChoice = chan.recv().await?;
        
        match client_choice {
            ClientChoice::ProcessData { data } => {
                let result = process_data(data).await?;
                chan.send(ProcessResult { result }).await?;
                // Continue loop
            }
            ClientChoice::StartSubprotocol => {
                // Enter nested subprotocol
                execute_subprotocol(&chan, &mut ctx).await?;
                // Continue main loop after subprotocol
            }
            ClientChoice::Terminate => {
                chan.send(Acknowledgment::Goodbye).await?;
                break; // Exit loop
            }
        }
    }
    
    ctx.exit_recursion("conditional_loop")?;
    Ok(())
}
```

## Testing Recursive Protocols

### Unit Testing with Mocked Recursion

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use besedarium::testing::{MockChan, TestSessionContext};
    
    #[tokio::test]
    async fn test_bounded_recursion() {
        let mut ctx = TestSessionContext::new();
        let chan = MockChan::new();
        
        // Set up test scenario
        chan.expect_recv::<ClientRequest>()
            .times(5)
            .returning(|| Ok(ClientRequest::Continue));
        
        chan.expect_recv::<ClientRequest>()
            .once()
            .returning(|| Ok(ClientRequest::Terminate));
        
        // Test the recursive protocol
        let result = bounded_server_loop(chan.into(), ctx).await;
        assert!(result.is_ok());
        
        // Verify recursion was properly managed
        assert_eq!(ctx.max_recursion_depth_reached("server_loop"), Some(1));
    }
    
    #[tokio::test]
    async fn test_recursion_depth_limit() {
        let mut ctx = TestSessionContext::with_depth_limit(5);
        let chan = MockChan::new();
        
        // Set up infinite loop scenario
        chan.expect_recv::<ClientRequest>()
            .returning(|| Ok(ClientRequest::Continue));
        
        // Test should fail due to depth limit
        let result = unbounded_server_loop(chan.into(), ctx).await;
        assert!(result.is_err());
        
        // Verify it was a depth limit error
        assert!(matches!(result.unwrap_err().downcast::<RecursionError>()?, 
                        RecursionError::DepthLimitExceeded { .. }));
    }
}
```

### Integration Testing

```rust
#[tokio::test]
async fn test_recursive_client_server_integration() {
    let (server_chan, client_chan) = besedarium::testing::create_test_channel_pair().await;
    
    // Start server in background
    let server_handle = tokio::spawn(async move {
        let mut ctx = SessionContext::new();
        ping_pong_server(server_chan, ctx).await
    });
    
    // Run client
    let client_result = async {
        let mut ctx = SessionContext::new();
        ping_pong_client(client_chan, ctx, 10).await // 10 ping-pong cycles
    }.await;
    
    // Wait for server completion
    let server_result = server_handle.await?;
    
    // Both should complete successfully
    assert!(client_result.is_ok());
    assert!(server_result.is_ok());
}
```

## Performance Considerations

### Recursion Overhead

- **Runtime Tracking**: Each recursion entry/exit involves minimal bookkeeping overhead
- **Memory Usage**: Recursion contexts use stack-allocated tracking structures
- **Optimization**: Consider tail-call optimization patterns where applicable

### Monitoring and Profiling

```rust
use std::time::{Duration, Instant};

async fn profiled_recursive_protocol(mut ctx: SessionContext) -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let mut iteration_count = 0;
    
    ctx.enter_recursion("profiled_loop")?;
    
    loop {
        let iteration_start = Instant::now();
        
        // Protocol iteration logic...
        execute_iteration().await?;
        
        iteration_count += 1;
        let iteration_time = iteration_start.elapsed();
        
        // Log performance metrics periodically
        if iteration_count % 1000 == 0 {
            let avg_time = start_time.elapsed() / iteration_count;
            println!("Iteration {}: last={:?}, avg={:?}", 
                     iteration_count, iteration_time, avg_time);
        }
        
        if should_terminate() {
            break;
        }
    }
    
    ctx.exit_recursion("profiled_loop")?;
    
    let total_time = start_time.elapsed();
    println!("Completed {} iterations in {:?} (avg: {:?})", 
             iteration_count, total_time, total_time / iteration_count);
    
    Ok(())
}
```

## Migration Guide

### From Manual Loops to Runtime Recursion

**Before** (manual loop management):

```rust
async fn old_server_loop(chan: ChanRef) -> Result<(), Box<dyn Error>> {
    loop {
        // Protocol logic without recursion tracking
        handle_request(&chan).await?;
        
        if should_stop() {
            break;
        }
    }
    Ok(())
}
```

**After** (with runtime recursion support):

```rust
async fn new_server_loop(chan: ChanRef, mut ctx: SessionContext) -> Result<(), Box<dyn Error>> {
    ctx.enter_recursion("server_loop")?;
    let _guard = RecursionGuard::new(&mut ctx, "server_loop")?;
    
    loop {
        // Same protocol logic but with recursion tracking
        handle_request(&chan).await?;
        
        if should_stop() {
            break;
        }
    }
    
    Ok(())
    // _guard automatically exits recursion context
}
```

### Preparing for Type-Level Recursion

Structure your code to be ready for compile-time recursion types:

```rust
// Define your protocol labels now
#[derive(Debug, Clone, Copy)]
pub struct MyProtocolLoop;

impl ProtocolLabel for MyProtocolLoop {
    const LABEL: &'static str = "my_protocol_loop";
}

// Use label constants in runtime code
async fn forward_compatible_loop(mut ctx: SessionContext) -> Result<(), Box<dyn Error>> {
    ctx.enter_recursion(MyProtocolLoop::LABEL)?;
    
    // Protocol logic that will work with future type-level recursion
    
    ctx.exit_recursion(MyProtocolLoop::LABEL)?;
    Ok(())
}
```

## Future Enhancements

### Planned Compile-Time Features

1. **Static Recursion Analysis**: Compile-time verification that all recursion points have corresponding variables and termination paths.

2. **Optimization Passes**: Compiler optimizations for common recursion patterns, including tail-call optimization.

3. **Advanced Type Safety**: Enhanced type-level guarantees about recursion depth and termination.

### Research Areas

1. **Coinductive Session Types**: Support for protocols with infinite or unbounded execution.

2. **Distributed Recursion**: Handling recursion across network boundaries with failure recovery.

3. **Temporal Properties**: Integration with temporal logic to specify and verify time-based recursion properties.

4. **Resource Bounds**: Compile-time analysis of resource usage in recursive protocols.

## Related Documentation

- **[Protocol Basics](protocol-basics.md)**: Foundation concepts for session types
- **[Global Protocols](global-protocols.md)**: Understanding global protocol specification
- **[Local Protocols](local-protocols.md)**: Local protocol projections and implementation
- **[Runtime System](runtime.md)**: Runtime support and session management
- **[Error Handling](error-handling.md)**: Error management strategies
- **[Testing](testing.md)**: Testing approaches for session type protocols
- **[Performance](performance.md)**: Performance optimization techniques

## References

1. Honda, K., Vasconcelos, V. T., & Kubo, M. (1998). Language primitives and type discipline for structured communication-based programming.
2. Gay, S., & Hole, M. (2005). Subtyping for session types in the pi calculus.
3. Deniélou, P. M., & Yoshida, N. (2012). Multiparty session types meet communicating automata.
4. Scalas, A., & Yoshida, N. (2016). Lightweight session programming in Scala.
5. Fowler, S. (2019). An Erlang implementation of multiparty session actors.
