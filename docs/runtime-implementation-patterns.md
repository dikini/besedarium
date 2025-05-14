# Runtime Implementation Patterns for Session Types

This document provides practical implementation patterns for integrating session types into Rust applications. It focuses on making complex session type combinators explicit and clear in your code while keeping the complexity manageable.

## Introduction

When implementing local per-role runtimes in Rust guided by session types, several combinators present unique challenges:

- **Choice/Offer**: Selection between different protocol branches
- **Parallel Composition**: Concurrent execution of independent protocol branches
- **Recursion**: Repeating protocol patterns

This guide explores different implementation approaches for these combinators, providing practical examples that balance explicitness with manageable complexity.

## Approach 1: Typed Channel Wrappers

Typed channel wrappers use Rust's type system to enforce protocol correctness at compile time. The channel's type encodes the current state of the protocol, and operations advance the type to the next state.

### Choice/Offer Combinator

The Choice/Offer combinator represents a point where one role makes a choice (sender) and another role offers different branches to handle that choice (receiver).

#### Implementing Choice (Sender Side)

```rust
// Session type: EpChoice<Server, "greeting", (EpSend<Server, String, "hello", EpEnd>, EpSend<Server, i32, "count", EpEnd>)>
struct TypedChannel<S: Session> {
    inner: RawChannel,
    _marker: PhantomData<S>,
}

// The select method allows choosing a branch of the protocol
impl<Role, Label, Branches> TypedChannel<EpChoice<Role, Label, Branches>> {
    pub fn select<B, Idx>(self, branch_index: Idx) -> TypedChannel<B>
    where
        Branches: GetBranch<Idx, Output = B>,
    {
        // Send the branch selection over the channel
        self.inner.send_branch_selection(branch_index.into_usize());
        
        // Return a channel typed with the selected branch's session type
        TypedChannel {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

// Usage example:
fn client_protocol(channel: TypedChannel<EpChoice<Server, "greeting", (EpSend<Server, String, "hello", EpEnd>, 
                                                                     EpSend<Server, i32, "count", EpEnd>)>>) {
    // Regular business logic to decide which greeting to use
    let use_hello = get_user_preference();
    
    if use_hello {
        // Select first branch - note how the channel type changes
        let channel: TypedChannel<EpSend<Server, String, "hello", EpEnd>> = 
            channel.select::<_, Branch<0>>(Branch::<0>);
            
        // Now we can only send a String message
        channel.send("Hello, server!".to_string());
    } else {
        // Select second branch
        let channel: TypedChannel<EpSend<Server, i32, "count", EpEnd>> = 
            channel.select::<_, Branch<1>>(Branch::<1>);
            
        // Now we can only send an i32 message
        channel.send(42);
    }
}
```

#### Implementing Offer (Receiver Side)

```rust
// Session type: EpOffer<Client, "greeting", (EpRecv<Client, String, "hello", EpEnd>, EpRecv<Client, i32, "count", EpEnd>)>
impl<Role, Label, Branches> TypedChannel<EpOffer<Role, Label, Branches>> {
    pub fn offer<R>(self, handlers: BranchHandlers<Branches, R>) -> R {
        // Receive the branch selection
        let branch_index = self.inner.receive_branch_selection();
        
        // Dispatch to the appropriate handler based on the branch index
        handlers.handle_branch(branch_index, self)
    }
}

// A type-safe handler collection for branches
struct BranchHandlers<Branches, R> {
    handlers: Vec<Box<dyn FnOnce(TypedChannel<dyn Any>) -> R>>,
    _marker: PhantomData<Branches>,
}

// Usage example:
fn server_protocol(channel: TypedChannel<EpOffer<Client, "greeting", (EpRecv<Client, String, "hello", EpEnd>,
                                                                    EpRecv<Client, i32, "count", EpEnd>)>>) {
    // Use the offer method with branch handlers
    channel.offer(BranchHandlers::new()
        .on_branch::<0, _>(|channel: TypedChannel<EpRecv<Client, String, "hello", EpEnd>>| {
            // First branch: receive String message
            let (message, channel) = channel.receive();
            println!("Received hello message: {}", message);
            
            // Regular business logic to process greeting
            process_greeting(message);
        })
        .on_branch::<1, _>(|channel: TypedChannel<EpRecv<Client, i32, "count", EpEnd>>| {
            // Second branch: receive i32 message
            let (count, channel) = channel.receive();
            println!("Received count: {}", count);
            
            // Regular business logic to process count
            process_count(count);
        })
    );
}
```

#### Key Benefits of This Approach

1. **Type Safety**: Branch selection is verified at compile time
2. **Explicit Branching**: The choice structure is clearly visible in the code
3. **Proper Typing**: Each branch handler receives a correctly-typed channel
4. **Regular Code Integration**: Business logic can be freely interspersed with protocol operations

The Typed Channel Wrappers approach makes the Choice/Offer combinator explicit in the code, with clear branching patterns that mirror the underlying session type structure.

### Parallel Composition Combinator

Parallel composition represents concurrent protocol branches that can execute independently of each other. In session types, this is often represented by the `EpPar` combinator.

#### Basic Implementation

```rust
// Session type: EpPar<(EpSend<Server, String, "log", EpEnd>, EpRecv<Client, Request, "request", EpEnd>)>
struct TypedChannel<S: Session> {
    // ...existing code...
}

// Split method for parallel composition
impl<Branches> TypedChannel<EpPar<Branches>> 
where
    Branches: TupleTypes,
{
    pub fn split(self) -> ParallelChannels<Branches> {
        // Create individual channels for each branch
        let channels = self.inner.split_channels(Branches::count());
        
        // Wrap each raw channel with appropriate session type
        ParallelChannels::new(channels, PhantomData)
    }
}

// A collection of channels for parallel branches
struct ParallelChannels<Branches> {
    channels: Vec<RawChannel>,
    _marker: PhantomData<Branches>,
}

// Access individual branches with type safety
impl<Branches> ParallelChannels<Branches> {
    pub fn branch<B, Idx>(self, index: Idx) -> (TypedChannel<B>, ParallelChannels<Branches::Remove<Idx>>)
    where
        Branches: GetBranch<Idx, Output = B> + RemoveBranch<Idx>,
    {
        let channel = self.channels.remove(index.into_usize());
        
        // Return the selected channel with appropriate type, and remaining channels
        (
            TypedChannel { 
                inner: channel, 
                _marker: PhantomData 
            },
            ParallelChannels {
                channels: self.channels,
                _marker: PhantomData,
            }
        )
    }
}

// Usage example showing how parallel composition is visible in the code
fn server_protocol(channel: TypedChannel<EpPar<(
    EpSend<Client, Status, "status", EpEnd>,
    EpRecv<Client, Request, "request", EpSend<Client, Response, "response", EpEnd>>
)>>) {
    // Split the channel into parallel branches
    let par_channels = channel.split();
    
    // Extract first branch - note explicit typing
    let (status_channel, par_channels): (
        TypedChannel<EpSend<Client, Status, "status", EpEnd>>,
        ParallelChannels<(EpRecv<Client, Request, "request", EpSend<Client, Response, "response", EpEnd>>)>
    ) = par_channels.branch(0);
    
    // Extract second branch (the only one remaining)
    let (request_channel, _empty_channels): (
        TypedChannel<EpRecv<Client, Request, "request", EpSend<Client, Response, "response", EpEnd>>>,
        ParallelChannels<()>
    ) = par_channels.branch(0);
    
    // Regular business logic
    let system_status = get_system_status();
    
    // Create two concurrent tasks for the two branches
    let status_task = tokio::spawn(async move {
        // Send on first branch
        status_channel.send(system_status);
    });
    
    let request_task = tokio::spawn(async move {
        // Handle request-response on second branch
        let (request, channel) = request_channel.receive();
        
        // Regular business logic to process request
        let response = process_request(request);
        
        // Send response
        channel.send(response);
    });
    
    // Wait for both tasks to complete
    tokio::join!(status_task, request_task);
}
```

#### Alternative: Thread-Safe Parallel Channels

For more ergonomic parallel execution, we can add thread-safety to our channels:

```rust
// Thread-safe wrapper for parallel channels
impl<B> TypedChannel<EpPar<B>> 
where 
    B: ParallelBranches,
{
    pub fn parallel<F, G, R1, R2>(self, f: F, g: G) -> (R1, R2)
    where
        F: FnOnce(TypedChannel<B::First>) -> R1 + Send + 'static,
        G: FnOnce(TypedChannel<B::Second>) -> R2 + Send + 'static,
        R1: Send + 'static,
        R2: Send + 'static,
    {
        // Split the channel for parallel usage
        let (ch1, ch2) = self.inner.split_for_par();
        
        // Create properly typed channels
        let ch1 = TypedChannel { inner: ch1, _marker: PhantomData::<B::First> };
        let ch2 = TypedChannel { inner: ch2, _marker: PhantomData::<B::Second> };
        
        // Run the two functions in parallel
        let handle1 = std::thread::spawn(move || f(ch1));
        let handle2 = std::thread::spawn(move || g(ch2));
        
        // Join the threads and return results
        (handle1.join().unwrap(), handle2.join().unwrap())
    }
}

// Usage example:
fn run_parallel_protocol(channel: TypedChannel<EpPar<(
    EpSend<Server, Metrics, "metrics", EpEnd>,
    EpRecv<Client, Command, "command", EpEnd>
)>>) {
    // Run both branches in parallel with explicit handlers
    let ((), command_result) = channel.parallel(
        // First branch handler
        |metrics_channel| {
            // Regular business logic
            let system_metrics = collect_system_metrics();
            
            // Send metrics
            metrics_channel.send(system_metrics);
        },
        // Second branch handler
        |command_channel| {
            // Receive command
            let (command, channel) = command_channel.receive();
            
            // Regular business logic to execute command
            execute_command(command)
        }
    );
}
```

#### Structured Concurrency for N-ary Parallel Composition

For protocols with more than two parallel branches:

```rust
// N-ary parallel composition with structured concurrency
impl<Branches> TypedChannel<EpPar<Branches>> 
where
    Branches: TupleTypes,
{
    pub fn with_branches<F, R>(self, handler: F) -> Vec<R>
    where
        F: FnOnce(Vec<DynamicBranch>) -> Vec<R>,
    {
        // Create dynamic branches that can be dispatched at runtime
        let branches = self.inner.split_channels(Branches::count())
            .into_iter()
            .enumerate()
            .map(|(idx, ch)| DynamicBranch { 
                channel: ch, 
                index: idx, 
                branch_type: TypeId::of::<Branches>() 
            })
            .collect();
            
        // Call the handler with all branches
        handler(branches)
    }
}

// Usage showing business logic interleaved with protocol operations:
fn handle_monitoring(channel: TypedChannel<EpPar<(
    EpRecv<Sensor1, Reading, "temp", EpEnd>,
    EpRecv<Sensor2, Reading, "pressure", EpEnd>,
    EpRecv<Sensor3, Reading, "humidity", EpEnd>,
)>>) {
    // Process all branches with callbacks
    let results = channel.with_branches(|branches| {
        // Regular business logic before protocol operations
        let monitor = SystemMonitor::new();
        
        // Create tasks for each branch
        branches.into_iter().map(|branch| {
            let monitor = monitor.clone(); // Clone for each task
            
            tokio::spawn(async move {
                match branch.index {
                    0 => { // Temperature branch
                        let branch = branch.typed::<EpRecv<Sensor1, Reading, "temp", EpEnd>>();
                        let (reading, _) = branch.receive();
                        
                        // Regular business logic after protocol operation
                        monitor.process_temperature(reading)
                    },
                    1 => { // Pressure branch
                        let branch = branch.typed::<EpRecv<Sensor2, Reading, "pressure", EpEnd>>();
                        let (reading, _) = branch.receive();
                        
                        // Regular business logic after protocol operation
                        monitor.process_pressure(reading)
                    },
                    2 => { // Humidity branch
                        let branch = branch.typed::<EpRecv<Sensor3, Reading, "humidity", EpEnd>>();
                        let (reading, _) = branch.receive();
                        
                        // Regular business logic after protocol operation
                        monitor.process_humidity(reading)
                    },
                    _ => unreachable!()
                }
            })
        }).collect()
    });
    
    // Wait for all branch processing to complete
    for result in results {
        result.await.unwrap();
    }
}
```

#### Key Benefits of This Approach

1. **Explicit Parallelism**: The parallel nature of the protocol branches is clearly visible
2. **Type-Safety**: Each branch maintains its individual session type
3. **Compositional**: Parallel branches can be extracted and handled separately
4. **Concurrency Model Flexibility**: Can be adapted to different concurrency patterns (threads, tasks, etc.)

The Typed Channel Wrappers approach makes parallel composition explicit in the code while maintaining type safety and allowing regular Rust code to be interleaved with protocol operations.

### Recursion Combinator

Recursion in session types allows protocols to repeat or have cyclic behavior. This presents a unique challenge for Rust's type system since recursive types must be represented finitely.

#### Using Type-Level Fixed-Point Recursion

```rust
// Define a recursive protocol using the EpRec combinator
// X = EpSend<Server, Request, "query", EpRecv<Server, Response, "result", X>>
struct TypedChannel<S: Session> {
    // ...existing code...
}

// Rec type to mark the recursion point
struct EpRec<F> {
    _marker: PhantomData<F>,
}

// Continue type to reference back to the recursion point
struct EpContinue<Label> {
    _marker: PhantomData<Label>,
}

// Implementation for entering a recursive protocol
impl<F> TypedChannel<EpRec<F>> 
where 
    F: RecursiveProtocol,
{
    pub fn enter_recursive(self) -> TypedChannel<F::Unwrapped> {
        // No runtime action needed, just change the type
        TypedChannel {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

// Implementation for continuing a recursive protocol
impl<Label> TypedChannel<EpContinue<Label>> {
    pub fn continue_as<F>(self) -> TypedChannel<EpRec<F>> 
    where
        F: RecursiveProtocol,
        Label: RecursiveLabel<Protocol = F>,
    {
        // No runtime action needed, just wrap to restart the protocol
        TypedChannel {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

// Example usage of recursion in a client-server query protocol
fn client_query_loop(channel: TypedChannel<EpRec<QueryProtocol>>) {
    // Enter the recursive protocol
    let mut channel = channel.enter_recursive();
    
    // Regular business logic to determine when to stop
    let mut should_continue = true;
    
    while should_continue {
        // Regular business logic to create a request
        let request = prepare_next_query();
        
        // Send request according to protocol
        let channel_after_send = channel.send(request);
        
        // Receive response according to protocol
        let (response, continue_channel) = channel_after_send.receive();
        
        // Regular business logic to process response
        should_continue = process_response(response);
        
        // Continue the protocol by recursing
        if should_continue {
            channel = continue_channel.continue_as::<QueryProtocol>().enter_recursive();
        } else {
            // Exit the loop, channel goes out of scope
        }
    }
}

// Server-side implementation
fn server_query_handler(mut channel: TypedChannel<EpRec<ServerProtocol>>) {
    // Enter the recursive protocol
    let mut channel = channel.enter_recursive();
    
    // Begin processing loop
    loop {
        // Receive request according to protocol
        let (request, channel_after_recv) = channel.receive();
        
        // Regular business logic to handle request
        let response = process_query(request);
        
        // Check if we should terminate
        if response.is_final() {
            // Send final response and exit
            channel_after_recv.send(response);
            break;
        } else {
            // Send response according to protocol
            let continue_channel = channel_after_recv.send(response);
            
            // Continue the protocol by recursing
            channel = continue_channel.continue_as::<ServerProtocol>().enter_recursive();
        }
    }
}
```

#### Higher-Order Functions for Recursive Protocols

Another approach leverages higher-order functions to handle recursion:

```rust
// Define the recursive protocol structure with explicit unwrapping
type QueryLoopType = EpRec<QueryLoop>;
struct QueryLoop;

impl RecursiveProtocol for QueryLoop {
    type Unwrapped = EpSend<Server, Query, "query", 
        EpRecv<Server, Response, "response", EpContinue<QueryLoop>>>;
}

// Function that handles a single iteration of the protocol
fn handle_query_iteration<F>(
    channel: TypedChannel<EpSend<Server, Query, "query", 
        EpRecv<Server, Response, "response", EpContinue<QueryLoop>>>>,
    continue_handler: F
) where
    F: FnOnce(TypedChannel<EpRec<QueryLoop>>)
{
    // Regular business logic before protocol operations
    let query = create_query();
    
    // Protocol operation: send query
    let channel = channel.send(query);
    
    // Protocol operation: receive response
    let (response, continue_channel) = channel.receive();
    
    // Regular business logic to process response
    process_response(response);
    
    // Check if we should continue
    if should_continue(&response) {
        // Convert back to recursive type and continue
        let rec_channel = continue_channel.continue_as::<QueryLoop>();
        continue_handler(rec_channel);
    }
}

// Recursive function that uses the iteration handler
fn run_query_protocol(channel: TypedChannel<EpRec<QueryLoop>>) {
    // Unwrap the recursive type
    let unwrapped = channel.enter_recursive();
    
    // Handle one iteration, with recursive continuation
    handle_query_iteration(unwrapped, run_query_protocol);
}
```

#### Encoding Recursion with State Machines

A more explicit state machine approach:

```rust
// Define protocol states for the type state pattern
enum ProtocolState {
    Ready,
    WaitingForResponse,
    Complete,
}

// Channel wrapper with explicit state
struct StatefulChannel<S: Session, State> {
    inner: RawChannel,
    _marker: PhantomData<(S, State)>,
}

// Implement the recursive chat protocol as a state machine
impl<R> StatefulChannel<ChatProtocol, ProtocolState::Ready> {
    // Method to start a new message exchange
    pub fn send_message(self, msg: String) -> StatefulChannel<ChatProtocol, ProtocolState::WaitingForResponse> {
        // Send the message
        self.inner.send(msg);
        
        // Transition to next state
        StatefulChannel {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<R> StatefulChannel<ChatProtocol, ProtocolState::WaitingForResponse> {
    // Method to receive response and complete one cycle
    pub fn receive_response(self) -> (String, StatefulChannel<ChatProtocol, ProtocolState::Ready>) {
        // Receive the response
        let response = self.inner.receive();
        
        // Return response and channel in ready state (recursion point)
        (response, StatefulChannel {
            inner: self.inner,
            _marker: PhantomData,
        })
    }
    
    // Method to end the conversation
    pub fn end_conversation(self) -> StatefulChannel<ChatProtocol, ProtocolState::Complete> {
        // Send end marker
        self.inner.send_end();
        
        // Transition to complete state
        StatefulChannel {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

// Usage example showing recursive protocol with interleaved business logic
fn chat_client() {
    // Create channel in ready state
    let mut channel: StatefulChannel<ChatProtocol, ProtocolState::Ready> = 
        create_channel();
    
    // Regular business logic for the chat UI
    let mut ui = ChatUI::new();
    
    while let Some(message) = ui.get_next_message() {
        if message == "/quit" {
            // Switch to end conversation branch
            let end_channel = channel.end_conversation();
            ui.show_conversation_ended();
            break;
        } else {
            // Regular iteration of the recursive protocol
            let waiting_channel = channel.send_message(message);
            
            // Regular business logic during protocol transition
            ui.show_message_sent();
            
            // Continue protocol
            let (response, ready_channel) = waiting_channel.receive_response();
            
            // Regular business logic with received data
            ui.show_received_message(response);
            
            // Update channel for next iteration (recursive step)
            channel = ready_channel;
        }
    }
}
```

#### Key Benefits of This Approach

1. **Explicit Recursion Control**: The recursion points and continuations are clearly visible in the code
2. **Type Safety**: Maintains type safety across recursive calls
3. **Regular Code Integration**: Business logic can dictate when to continue or break recursion
4. **State Transitions**: Makes recursive protocol states explicit through type changes

The Typed Channel Wrappers approach handles recursion by making the recursion points explicit in the code, while still allowing regular Rust code to control the flow of the protocol.

## Approach 2: Code Generation with Procedural Macros

This approach uses Rust's procedural macros to generate protocol implementation code from session type definitions. This can significantly reduce boilerplate and create more ergonomic APIs.

### Choice/Offer Combinator

The Choice/Offer combinator is particularly well-suited for code generation since it involves repetitive pattern matching and dispatch logic that can be automated.

#### Sender-Side (Choice) Implementation

```rust
// Define the protocol with a proc macro
#[session_protocol]
enum GreetingProtocol {
    #[role(Client)]
    #[choice("greeting")]
    client: {
        #[branch("hello")]
        hello: Send<Server, String> >> Recv<Server, String> >> End,
        
        #[branch("count")]
        count: Send<Server, i32> >> Recv<Server, i32> >> End,
    }
}

// Generated client code (hidden from user, but shown here for clarity)
impl GreetingProtocol {
    pub fn client_hello(channel: Channel) -> Result<(String, Channel), ProtocolError> {
        // Send branch selection
        let channel = channel.select_branch("hello")?;
        
        // Request message from user code
        println!("Enter greeting message:");
        let message = read_user_input()?;
        
        // Send the message
        let channel = channel.send(message)?;
        
        // Receive response
        let (response, channel) = channel.receive()?;
        
        Ok((response, channel))
    }
    
    pub fn client_count(channel: Channel) -> Result<(i32, Channel), ProtocolError> {
        // Send branch selection
        let channel = channel.select_branch("count")?;
        
        // Request count from user code
        println!("Enter count:");
        let count = read_user_input::<i32>()?;
        
        // Send the count
        let channel = channel.send(count)?;
        
        // Receive response
        let (response, channel) = channel.receive()?;
        
        Ok((response, channel))
    }
}

// Usage example - the actual client code is simple and focuses on business logic
fn run_client() -> Result<(), ProtocolError> {
    // Connect to server
    let channel = connect_to_server()?;
    
    // Regular business logic to determine which greeting to use
    let use_hello = should_use_hello();
    
    if use_hello {
        // Call generated function for "hello" branch
        let (response, _) = GreetingProtocol::client_hello(channel)?;
        println!("Server replied: {}", response);
    } else {
        // Call generated function for "count" branch
        let (response, _) = GreetingProtocol::client_count(channel)?;
        println!("Server returned count: {}", response);
    }
    
    Ok(())
}
```

#### Receiver-Side (Offer) Implementation

```rust
// Server-side code is also generated from the protocol definition
#[session_protocol]
impl GreetingProtocol {
    #[role(Server)]
    #[handler]
    async fn handle_client(channel: ServerChannel) -> Result<(), ProtocolError> {
        // Generated code dispatches to appropriate branch handler
        match channel.receive_choice("greeting")? {
            "hello" => Self::handle_hello(channel).await,
            "count" => Self::handle_count(channel).await,
            _ => Err(ProtocolError::UnknownBranch),
        }
    }
    
    // User-defined handlers for each branch
    #[branch_handler("hello")]
    async fn handle_hello(channel: ServerChannel) -> Result<(), ProtocolError> {
        // Receive greeting
        let (greeting, channel) = channel.receive::<String>()?;
        
        // Regular business logic to process greeting
        println!("Client sent greeting: {}", greeting);
        let response = format!("Hello back to you!");
        
        // Send response
        let _ = channel.send(response)?;
        Ok(())
    }
    
    #[branch_handler("count")]
    async fn handle_count(channel: ServerChannel) -> Result<(), ProtocolError> {
        // Receive count
        let (count, channel) = channel.receive::<i32>()?;
        
        // Regular business logic to process count
        println!("Client sent count: {}", count);
        let doubled = count * 2;
        
        // Send response
        let _ = channel.send(doubled)?;
        Ok(())
    }
}

// Server main function
fn run_server() -> Result<(), ProtocolError> {
    let listener = create_listener()?;
    
    while let Ok(channel) = listener.accept() {
        // Spawn a task to handle the client
        tokio::spawn(async move {
            if let Err(e) = GreetingProtocol::handle_client(channel).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
    
    Ok(())
}
```

#### Alternative: Trait-Based Protocol Generation

An alternative approach uses traits for each role:

```rust
// Protocol definition with proc macro
#[define_protocol]
trait BankProtocol {
    // Define the protocol structure
    type Protocol = Choice<Client, "transaction", (
        Branch<"deposit", Send<Client, Amount> >> Recv<Client, Receipt>>,
        Branch<"withdraw", Send<Client, Amount> >> Choice<Server, "result", (
            Branch<"approved", Send<Server, Cash> >> Recv<Client, Confirmation>>,
            Branch<"denied", Send<Server, Reason> >> End>
        )>
    )>;
}

// Generate role-specific traits
#[derive_roles(BankProtocol)]
trait ClientRole {
    // Generated method signatures for client
    fn deposit(&mut self, amount: Amount) -> Result<Receipt, ProtocolError>;
    fn withdraw(&mut self, amount: Amount) -> Result<WithdrawResult, ProtocolError>;
}

#[derive_roles(BankProtocol)]
trait ServerRole {
    // Generated method signatures for server
    fn handle_transaction(&mut self) -> Result<TransactionRequest, ProtocolError>;
    fn approve_withdrawal(&mut self, cash: Cash) -> Result<Confirmation, ProtocolError>;
    fn deny_withdrawal(&mut self, reason: Reason) -> Result<(), ProtocolError>;
}

// Usage example - client implementation
struct BankClient {
    channel: Channel,
}

impl ClientRole for BankClient {
    fn deposit(&mut self, amount: Amount) -> Result<Receipt, ProtocolError> {
        // Generated code handles branch selection
        self.channel.select_branch("deposit")?;
        
        // Send amount
        self.channel.send(amount)?;
        
        // Receive receipt
        let receipt = self.channel.receive()?;
        
        Ok(receipt)
    }
    
    fn withdraw(&mut self, amount: Amount) -> Result<WithdrawResult, ProtocolError> {
        // Select withdraw branch
        self.channel.select_branch("withdraw")?;
        
        // Send amount
        self.channel.send(amount)?;
        
        // Handle server's choice
        match self.channel.receive_choice("result")? {
            "approved" => {
                let cash = self.channel.receive::<Cash>()?;
                let confirmation = ATMPrompt::ask_for_confirmation();
                self.channel.send(confirmation)?;
                Ok(WithdrawResult::Approved(cash))
            },
            "denied" => {
                let reason = self.channel.receive::<Reason>()?;
                Ok(WithdrawResult::Denied(reason))
            },
            _ => Err(ProtocolError::UnknownBranch),
        }
    }
}

// Client usage example
fn use_bank_client() {
    // Regular business logic
    let mut client = BankClient::connect("localhost:8080").unwrap();
    
    // Make deposit
    if let Ok(receipt) = client.deposit(Amount(100.0)) {
        println!("Deposit successful: {}", receipt);
    }
    
    // Make withdrawal
    match client.withdraw(Amount(50.0)) {
        Ok(WithdrawResult::Approved(cash)) => {
            // Regular business logic with the result
            process_withdrawal_success(cash);
        },
        Ok(WithdrawResult::Denied(reason)) => {
            // Regular business logic with the reason
            show_error_to_user(reason);
        },
        Err(e) => println!("Protocol error: {}", e),
    }
}
```

#### Domain-Specific Language with Custom Syntax

For even more ergonomic protocol definitions, a DSL can be created:

```rust
// DSL for protocol definition
protocol! {
    ShoppingProtocol {
        // Choice is expressed with a more natural syntax
        Customer -> Shop: choice "order" {
            "browse": {
                Customer -> Shop: Category;
                Shop -> Customer: ProductList;
                // Recursively go back to the start of the protocol
                goto ShoppingProtocol;
            },
            "purchase": {
                Customer -> Shop: Product;
                Shop -> Customer: choice "availability" {
                    "available": {
                        Shop -> Customer: Price;
                        Customer -> Shop: choice "decision" {
                            "buy": {
                                Customer -> Shop: PaymentInfo;
                                Shop -> Customer: OrderConfirmation;
                            },
                            "cancel": { }
                        }
                    },
                    "unavailable": {
                        Shop -> Customer: String; // message
                        goto ShoppingProtocol;
                    }
                }
            }
        }
    }
}

// The DSL generates code for each role
impl ShoppingProtocol {
    // Generated client methods handle the protocol communication
    pub fn customer_browse(channel: Channel, category: Category) -> Result<(ProductList, Channel), Error> {
        // Generated code for the browse branch
        let channel = channel.select_branch("browse")?;
        channel.send(category)?;
        let (products, channel) = channel.receive()?;
        Ok((products, channel))
    }
    
    pub fn customer_purchase(channel: Channel, product: Product) -> Result<PurchaseResult, Error> {
        // Generated code for the purchase branch
        let channel = channel.select_branch("purchase")?;
        channel.send(product)?;
        
        // Nested choice handling
        match channel.receive_choice("availability")? {
            "available" => {
                let (price, channel) = channel.receive()?;
                
                // Regular business logic to decide whether to buy
                if should_buy(price) {
                    let channel = channel.select_branch("buy")?;
                    let payment_info = get_payment_info();
                    channel.send(payment_info)?;
                    let (confirmation, channel) = channel.receive()?;
                    Ok(PurchaseResult::Ordered(confirmation))
                } else {
                    let channel = channel.select_branch("cancel")?;
                    Ok(PurchaseResult::Cancelled)
                }
            },
            "unavailable" => {
                let (message, channel) = channel.receive()?;
                Ok(PurchaseResult::Unavailable(message))
            },
            _ => Err(Error::Protocol("Unknown branch".into())),
        }
    }
}

// Usage example
fn customer_app() -> Result<(), Error> {
    let mut channel = connect_to_shop()?;
    
    // Regular business logic
    let browsing = true;
    
    while browsing {
        println!("1. Browse products");
        println!("2. Purchase product");
        println!("3. Exit");
        
        match get_user_choice() {
            1 => {
                let category = prompt_for_category();
                let (products, new_channel) = ShoppingProtocol::customer_browse(channel, category)?;
                channel = new_channel;
                
                // Regular business logic to display products
                display_products(products);
            },
            2 => {
                let product = prompt_for_product();
                let result = ShoppingProtocol::customer_purchase(channel, product)?;
                
                // Regular business logic to handle result
                match result {
                    PurchaseResult::Ordered(conf) => {
                        println!("Order confirmed: {}", conf);
                        break;
                    },
                    PurchaseResult::Cancelled => {
                        println!("Purchase cancelled");
                    },
                    PurchaseResult::Unavailable(msg) => {
                        println!("Product unavailable: {}", msg);
                    }
                }
            },
            3 => break,
            _ => println!("Invalid choice"),
        }
    }
    
    Ok(())
}
```

#### Key Benefits of This Approach

1. **Reduced Boilerplate**: Protocol structure is expressed concisely and implementation details are generated
2. **Multiple Abstraction Levels**: Can expose low-level protocol operations or high-level business methods
3. **Explicit Error Handling**: Generated code typically includes proper error handling
4. **Regular Code Integration**: Business logic can focus on application concerns rather than protocol mechanics

The Code Generation approach makes the Choice/Offer combinator less explicit in the user's code but more declarative in the protocol definition. This approach is particularly useful for complex protocols where the implementation details would otherwise be tedious and error-prone.

### Parallel Composition Combinator

Parallel composition in session types allows concurrent execution of independent protocol branches. With code generation, we can create ergonomic APIs that hide much of the complexity while maintaining type safety.

#### Basic Generated API

```rust
// Define protocol with parallel branches using proc macro
#[session_protocol]
enum MonitoringProtocol {
    #[role(Monitor)]
    #[parallel]
    monitor_system: {
        #[branch]
        metrics: Recv<Sensor, Metrics> >> End,
        
        #[branch]
        logs: Recv<Logger, LogEntry> >> End,
        
        #[branch]
        alerts: Recv<Alerter, Alert> >> Send<Alerter, Acknowledgment> >> End
    }
}

// Generated code (shown for clarity)
impl MonitoringProtocol {
    // Generated function to run all parallel branches
    pub async fn run_monitor_system(channel: Channel) -> Result<MonitoringResult, ProtocolError> {
        // Split the channel for parallel execution
        let (metrics_ch, logs_ch, alerts_ch) = channel.split_for_parallel(3)?;
        
        // Create concurrent tasks for each branch
        let metrics_task = tokio::spawn(Self::handle_metrics_branch(metrics_ch));
        let logs_task = tokio::spawn(Self::handle_logs_branch(logs_ch));
        let alerts_task = tokio::spawn(Self::handle_alerts_branch(alerts_ch));
        
        // Wait for all branches to complete
        let (metrics, logs, alerts) = tokio::try_join!(
            metrics_task,
            logs_task,
            alerts_task
        )?;
        
        // Combine results
        Ok(MonitoringResult {
            metrics: metrics?,
            logs: logs?,
            alerts: alerts?
        })
    }
    
    // Individual branch handlers (also generated)
    async fn handle_metrics_branch(channel: Channel) -> Result<Metrics, ProtocolError> {
        let (metrics, _) = channel.receive::<Metrics>().await?;
        Ok(metrics)
    }
    
    async fn handle_logs_branch(channel: Channel) -> Result<Vec<LogEntry>, ProtocolError> {
        let (logs, _) = channel.receive::<LogEntry>().await?;
        Ok(vec![logs])
    }
    
    async fn handle_alerts_branch(channel: Channel) -> Result<AlertProcessingResult, ProtocolError> {
        // Receive alert
        let (alert, channel) = channel.receive::<Alert>().await?;
        
        // Process alert (this could be user-provided code)
        let acknowledgment = process_alert(&alert);
        
        // Send acknowledgment
        channel.send(acknowledgment).await?;
        
        Ok(AlertProcessingResult { alert, acknowledgment })
    }
}

// Usage example - much simpler with generated code
async fn monitor_system() -> Result<(), ProtocolError> {
    // Connect to the monitored system
    let channel = connect_to_monitored_system().await?;
    
    // Run the parallel protocol
    let result = MonitoringProtocol::run_monitor_system(channel).await?;
    
    // Use the combined results from all parallel branches
    process_monitoring_result(result);
    
    Ok(())
}
```

#### Customizable Branch Handlers

Code generation also allows customization of how parallel branches are handled:

```rust
// User provides custom branch handlers
#[session_protocol]
impl MonitoringProtocol {
    // Main parallel protocol executor is still generated
    pub async fn run_monitor_system(channel: Channel) -> Result<(), ProtocolError> {
        // ...generated code to split channels and run in parallel...
    }
    
    // User-provided branch handler with custom business logic
    #[branch_handler("metrics")]
    async fn process_metrics(metrics: Metrics) -> Result<(), ProtocolError> {
        // Regular business logic to process metrics
        let dashboard = Dashboard::get_instance();
        dashboard.update_metrics(metrics);
        
        // Additional business operations
        if metrics.cpu_usage > 90.0 {
            send_high_cpu_alert();
        }
        
        Ok(())
    }
    
    #[branch_handler("logs")]
    async fn process_logs(log_entry: LogEntry) -> Result<(), ProtocolError> {
        // Regular business logic to process logs
        Logger::get_instance().record(log_entry);
        Ok(())
    }
    
    #[branch_handler("alerts")]
    async fn process_alert(alert: Alert) -> Result<Acknowledgment, ProtocolError> {
        // Regular business logic for alert processing
        let alert_system = AlertSystem::get_instance();
        
        // Business logic determines the response
        let acknowledgment = if alert.severity > 8 {
            alert_system.escalate_alert(alert);
            Acknowledgment::Escalated
        } else {
            alert_system.record_alert(alert);
            Acknowledgment::Recorded
        };
        
        Ok(acknowledgment)
    }
}
```

#### Fork-Join Pattern

Code generation can also support a more explicit fork-join pattern:

```rust
// Fork-join parallel protocol
protocol! {
    DataProcessingProtocol {
        // Parallel section with explicit fork-join
        parallel {
            "image_processing": {
                ImageProcessor -> Coordinator: ImageData;
                Coordinator -> ImageProcessor: ProcessedImage;
            },
            "text_processing": {
                TextProcessor -> Coordinator: TextData;
                Coordinator -> TextProcessor: ProcessedText;
            },
            "metadata_processing": {
                MetadataProcessor -> Coordinator: Metadata;
                Coordinator -> MetadataProcessor: EnrichedMetadata;
            }
        }
        
        // Sequential section after all parallel branches complete
        Coordinator -> Client: CombinedResult;
    }
}

// Generated coordinator implementation
#[coordinator_implementation]
struct DataCoordinator {
    image_processor: ImageProcessingService,
    text_processor: TextProcessingService,
    metadata_processor: MetadataService,
}

// Generated method with custom handlers for parallel branches
impl DataProcessingProtocol for DataCoordinator {
    async fn coordinate(&mut self, client_channel: Channel) -> Result<(), ProtocolError> {
        // Generated code forks into parallel branches
        let results = self.execute_parallel_branches().await?;
        
        // After all branches complete (join point)
        let combined = combine_results(
            results.image_processing,
            results.text_processing, 
            results.metadata_processing
        );
        
        // Send combined result to client
        client_channel.send(combined).await?;
        
        Ok(())
    }
    
    // Custom handler for image processing branch
    async fn handle_image_processing(&mut self, channel: Channel) -> Result<ProcessedImage, ProtocolError> {
        // Receive image data
        let (image_data, channel) = channel.receive::<ImageData>().await?;
        
        // Regular business logic
        let processed = self.image_processor.process(image_data);
        
        // Send processed image
        channel.send(processed).await?;
        
        Ok(processed)
    }
    
    // Other branch handlers...
}
```

#### Structured Concurrency Model

A more structured approach to concurrency can also be generated:

```rust
// Protocol with explicit task management
#[async_protocol]
trait DataAnalysisProtocol {
    // Define the parallel composition structure
    type Protocol = Parallel<(
        Branch<"stream", Recv<Sensor, DataStream> >> Send<Sensor, StreamResult>>,
        Branch<"query", Recv<Client, Query> >> Send<Client, QueryResult>>,
        Branch<"control", Recv<Admin, ControlCommand> >> Send<Admin, ControlStatus>>
    )>;
    
    // Generated method creates a task group
    async fn process_data(&mut self, channel: Channel) -> Result<(), ProtocolError>;
    
    // Custom handlers for each branch with business logic
    async fn handle_stream(&mut self, stream: DataStream) -> Result<StreamResult, ProtocolError>;
    async fn handle_query(&mut self, query: Query) -> Result<QueryResult, ProtocolError>;
    async fn handle_control(&mut self, command: ControlCommand) -> Result<ControlStatus, ProtocolError>;
}

// Implementation with business logic
struct DataAnalyzer {
    database: Database,
    stream_processor: StreamProcessor,
}

#[async_protocol_impl]
impl DataAnalysisProtocol for DataAnalyzer {
    // No need to implement process_data - it's generated
    
    // Implement just the business logic for each branch
    async fn handle_stream(&mut self, stream: DataStream) -> Result<StreamResult, ProtocolError> {
        // Regular business logic
        let result = self.stream_processor.analyze(stream).await?;
        
        // More business operations
        self.database.store_analysis(result.clone()).await?;
        
        Ok(result)
    }
    
    async fn handle_query(&mut self, query: Query) -> Result<QueryResult, ProtocolError> {
        // Regular business logic for queries
        self.database.execute_query(query).await
    }
    
    async fn handle_control(&mut self, command: ControlCommand) -> Result<ControlStatus, ProtocolError> {
        // Regular business logic for control commands
        match command {
            ControlCommand::Start => {
                self.stream_processor.start().await?;
                Ok(ControlStatus::Started)
            },
            ControlCommand::Stop => {
                self.stream_processor.stop().await?;
                Ok(ControlStatus::Stopped)
            },
            // Other commands...
        }
    }
}

// Usage example with generated parallel executor
async fn run_server() -> Result<(), Error> {
    let mut analyzer = DataAnalyzer::new();
    let channel = accept_connection().await?;
    
    // Call the generated method that handles parallel execution
    analyzer.process_data(channel).await?;
    
    Ok(())
}
```

#### Key Benefits of This Approach

1. **Reduced Complexity**: Hides the details of parallel channel management
2. **Separation of Protocol and Business Logic**: Protocol structure is defined separately from business logic
3. **Flexible Concurrency Models**: Can adapt to various async runtimes and patterns
4. **Natural Integration**: Regular Rust business code integrates seamlessly with protocol operations

The Code Generation approach makes parallel composition more declarative, allowing developers to focus on the business logic within each branch rather than the mechanics of channel management and synchronization.

### Recursion Combinator

Recursion in session types poses challenges for code generation since we need to represent potentially infinite protocols with finite code. Procedural macros can generate the necessary recursive structures while keeping the API intuitive.

#### Declarative Recursion with Protocol DSL

```rust
// Define a recursive protocol using a DSL
protocol! {
    ChatProtocol {
        // Recursive session type defined with `rec` and `goto`
        rec X {
            Client -> Server: choice "action" {
                "message": {
                    Client -> Server: String;  // message content
                    Server -> Client: String;  // acknowledgment
                    goto X;  // loop back to recursion point
                },
                "quit": {
                    // End the protocol
                }
            }
        }
    }
}

// Generated client implementation
impl ChatProtocol {
    // Recursive protocol with explicit messaging API
    pub fn run_client(channel: Channel) -> Result<(), ProtocolError> {
        let mut channel = channel;
        
        // Loop until client decides to quit
        loop {
            // Regular business logic to decide what to do
            match get_user_action() {
                UserAction::SendMessage => {
                    // Select "message" branch
                    channel = channel.select_branch("message")?;
                    
                    // Regular business logic to get message content
                    let message = get_user_input_message();
                    
                    // Send message
                    channel = channel.send(message)?;
                    
                    // Regular business logic with result
                    let (ack, new_channel) = channel.receive::<String>()?;
                    display_acknowledgment(ack);
                    
                    // Update channel for next iteration
                    channel = new_channel;
                },
                UserAction::Quit => {
                    // Select "quit" branch
                    channel = channel.select_branch("quit")?;
                    break;
                }
            }
        }
        
        Ok(())
    }
}

// Server-side code also generated from the protocol
impl ChatProtocol {
    pub fn run_server(channel: Channel) -> Result<(), ProtocolError> {
        let mut channel = channel;
        
        // Loop until client quits
        loop {
            // Wait for client action choice
            match channel.receive_choice("action")? {
                "message" => {
                    // Receive message
                    let (message, channel_after_recv) = channel.receive::<String>()?;
                    
                    // Regular business logic to process message
                    process_message(message);
                    
                    // Send acknowledgment
                    let ack = format!("Received message of length {}", message.len());
                    channel = channel_after_recv.send(ack)?;
                },
                "quit" => {
                    // Client quits
                    break;
                },
                _ => return Err(ProtocolError::UnknownBranch),
            }
        }
        
        Ok(())
    }
}
```

#### Component-Based Recursive Protocol

```rust
// Define recursive protocol with components
#[session_protocol]
enum FileTransferProtocol {
    #[role(Client)]
    #[recursive("transfer")]
    client: {
        #[choice("action")]
        choices: {
            #[branch("send_chunk")]
            send_chunk: Send<Server, FileChunk> >> Recv<Server, ChunkAck> >> Continue<"transfer">,
            
            #[branch("finish")]
            finish: Send<Server, EndOfFile> >> Recv<Server, FileReceipt> >> End
        }
    }
}

// Generated code with recursive handlers
impl FileTransferProtocol {
    // Entry point for the protocol
    pub fn transfer_file(file_path: &str) -> Result<FileReceipt, ProtocolError> {
        // Connect to server
        let channel = connect_to_server()?;
        
        // Open file
        let mut file = File::open(file_path)?;
        let mut buffer = [0u8; CHUNK_SIZE];
        
        // Start recursive protocol
        Self::transfer_loop(channel, &mut file, &mut buffer)
    }
    
    // Generated recursive function
    fn transfer_loop(
        mut channel: Channel,
        file: &mut File,
        buffer: &mut [u8]
    ) -> Result<FileReceipt, ProtocolError> {
        // Read chunk from file (regular business logic)
        match file.read(buffer) {
            Ok(0) => {
                // End of file reached, select finish branch
                channel = channel.select_branch("finish")?;
                
                // Send EOF marker
                channel = channel.send(EndOfFile {})?;
                
                // Receive receipt
                let (receipt, _) = channel.receive::<FileReceipt>()?;
                Ok(receipt)
            },
            Ok(bytes_read) => {
                // Create chunk from buffer
                let chunk = FileChunk {
                    data: buffer[0..bytes_read].to_vec(),
                };
                
                // Select send_chunk branch
                channel = channel.select_branch("send_chunk")?;
                
                // Send chunk
                channel = channel.send(chunk)?;
                
                // Receive acknowledgment
                let (ack, channel) = channel.receive::<ChunkAck>()?;
                
                // Continue recursion (tailcall)
                Self::transfer_loop(channel, file, buffer)
            },
            Err(e) => Err(ProtocolError::IoError(e)),
        }
    }
}

// Client usage
fn upload_file() -> Result<(), Error> {
    // Regular business logic
    let file_path = select_file_to_upload();
    
    // Use generated recursive protocol
    let receipt = FileTransferProtocol::transfer_file(&file_path)?;
    
    // Regular business logic with result
    display_upload_confirmation(receipt);
    
    Ok(())
}
```

#### Trait-Based Recursive Protocols

```rust
// Define recursive protocol with traits
#[recursive_protocol]
trait StreamingProtocol {
    // Define the protocol structure with recursion
    type Protocol = Recv<Producer, Item> >>
        Choice<Consumer, "control", (
            Branch<"continue", Send<Consumer, Ack> >> Recurse>,
            Branch<"stop", Send<Consumer, Stop> >> End>
        )>;
    
    // Handler for processing items
    fn process_item(&mut self, item: Item) -> ControlDecision;
}

// Implementation with business logic
struct DataStreamConsumer {
    item_count: usize,
    max_items: usize,
}

#[recursive_protocol_impl]
impl StreamingProtocol for DataStreamConsumer {
    // Only implement the business logic
    fn process_item(&mut self, item: Item) -> ControlDecision {
        // Process the item
        process_data_item(item);
        
        // Update state
        self.item_count += 1;
        
        // Decide whether to continue or stop
        if self.item_count >= self.max_items {
            ControlDecision::Stop
        } else {
            ControlDecision::Continue
        }
    }
}

// The macro generates the recursive runtime logic
impl DataStreamConsumer {
    // Generated method
    pub fn run(&mut self, channel: Channel) -> Result<(), ProtocolError> {
        // Initial setup
        let mut channel = channel;
        
        // Begin recursive protocol
        loop {
            // Receive item
            let (item, channel_after_recv) = channel.receive::<Item>()?;
            
            // Call user-provided handler
            let decision = self.process_item(item);
            
            // Handle recursion based on business logic decision
            match decision {
                ControlDecision::Continue => {
                    // Select continue branch
                    let channel_after_choice = channel_after_recv.select_branch("continue")?;
                    
                    // Send acknowledgment
                    channel = channel_after_choice.send(Ack {})?;
                    
                    // Continue loop (recursive step)
                    continue;
                },
                ControlDecision::Stop => {
                    // Select stop branch
                    let channel_after_choice = channel_after_recv.select_branch("stop")?;
                    
                    // Send stop signal
                    channel_after_choice.send(Stop {})?;
                    
                    // Exit loop
                    break;
                }
            }
        }
        
        Ok(())
    }
}

// Usage
fn consume_data_stream() -> Result<(), Error> {
    // Regular business logic to set up consumer
    let mut consumer = DataStreamConsumer {
        item_count: 0,
        max_items: 1000,
    };
    
    // Connect to producer
    let channel = connect_to_producer()?;
    
    // Run protocol with generated recursive logic
    consumer.run(channel)?;
    
    Ok(())
}
```

#### State Machine Generator

More complex recursive protocols can benefit from explicit state machine generation:

```rust
// Define protocol with states and transitions
#[state_machine_protocol]
enum DatabaseProtocol {
    #[initial_state]
    NotConnected {
        #[transition(to = "Connected")]
        connect: Send<Client, Credentials> >> Recv<Server, ConnectionResult>,
    },
    
    #[state]
    Connected {
        #[transition(to = "WaitingForResult")]
        query: Send<Client, QueryString>,
        
        #[transition(to = "NotConnected")]
        disconnect: Send<Client, Disconnect> >> End,
    },
    
    #[state]
    WaitingForResult {
        #[transition(to = "Connected")]
        result: Recv<Server, QueryResult>,
        
        #[transition(to = "Connected")]
        error: Recv<Server, ErrorMessage>,
    }
}

// Generated code (not shown to user)
impl DatabaseProtocol {
    // State-specific methods
    pub fn connect(&mut self, credentials: Credentials) -> Result<ConnectionResult, ProtocolError> {
        // Assert that we're in NotConnected state
        self.assert_state(State::NotConnected)?;
        
        // Send credentials
        self.channel = self.channel.send(credentials)?;
        
        // Receive result
        let (result, new_channel) = self.channel.receive::<ConnectionResult>()?;
        self.channel = new_channel;
        
        // Change state
        self.state = State::Connected;
        
        Ok(result)
    }
    
    pub fn query(&mut self, query: QueryString) -> Result<(), ProtocolError> {
        // Assert that we're in Connected state
        self.assert_state(State::Connected)?;
        
        // Send query
        self.channel = self.channel.send(query)?;
        
        // Change state
        self.state = State::WaitingForResult;
        
        Ok(())
    }
    
    pub fn receive_result(&mut self) -> Result<QueryResult, ProtocolError> {
        // Assert that we're in WaitingForResult state
        self.assert_state(State::WaitingForResult)?;
        
        // Try to receive result or error
        let choice = self.channel.peek_message_type()?;
        
        match choice {
            "result" => {
                let (result, new_channel) = self.channel.receive::<QueryResult>()?;
                self.channel = new_channel;
                self.state = State::Connected;
                Ok(result)
            },
            "error" => {
                let (error, new_channel) = self.channel.receive::<ErrorMessage>()?;
                self.channel = new_channel;
                self.state = State::Connected;
                Err(ProtocolError::DatabaseError(error))
            },
            _ => Err(ProtocolError::UnexpectedMessageType),
        }
    }
}

// Usage showing how recursive protocol with state transitions can be used
fn database_client() -> Result<(), Error> {
    // Create protocol state machine
    let mut db = DatabaseProtocol::new();
    
    // Connect to database
    let credentials = get_user_credentials();
    let conn_result = db.connect(credentials)?;
    
    if (conn_result.success) {
        // Run queries in a loop
        while let Some(query) = get_next_query() {
            // Send query
            db.query(query)?;
            
            // Get result
            match db.receive_result() {
                Ok(result) => display_result(result),
                Err(e) => handle_error(e),
            }
        }
        
        // Disconnect when done
        db.disconnect()?;
    }
    
    Ok(())
}
```

#### Key Benefits of This Approach

1. **Abstracted Recursion**: Recursion mechanics are hidden behind a clean API
2. **Business Logic Integration**: Separates protocol mechanics from application logic
3. **Error Handling**: Generated code includes proper error handling for recursive protocols
4. **State Management**: Can generate state machines for complex recursive protocols

The Code Generation approach handles recursion by generating appropriate recursive functions or state machines, allowing the developer to focus on application logic rather than protocol mechanics. The recursion is still explicitly visible in the protocol definition but automated in the implementation.

## Approach 3: State Machine Builders

This approach represents session types as explicit state machines with well-defined transitions. It makes the state of the protocol visible in the type system and provides a fluent, builder-style API for protocol operations.

### Choice/Offer Combinator

The Choice/Offer combinator represents a point where one party makes a choice and the other offers different branches to handle that choice. With state machine builders, these branches become explicit transitions to different states.

#### Implementing Choice (Sender Side)

```rust
// Each protocol state is a distinct type
struct Ready;
struct WaitingForGreetingResponse;
struct WaitingForCountResponse;
struct Done;

// Protocol session with type-state pattern
struct ClientSession<S> {
    channel: RawChannel,
    _state: PhantomData<S>,
}

// Ready state can transition to multiple next states via choice
impl ClientSession<Ready> {
    // Method for selecting the "greeting" branch
    pub fn choose_greeting(self, greeting: String) -> ClientSession<WaitingForGreetingResponse> {
        // Send branch selection
        self.channel.send_branch_selection("greeting");
        
        // Send the greeting message
        self.channel.send(greeting);
        
        // Return session in new state
        ClientSession {
            channel: self.channel,
            _state: PhantomData,
        }
    }
    
    // Method for selecting the "count" branch
    pub fn choose_count(self, count: i32) -> ClientSession<WaitingForCountResponse> {
        // Send branch selection
        self.channel.send_branch_selection("count");
        
        // Send the count message
        self.channel.send(count);
        
        // Return session in new state
        ClientSession {
            channel: self.channel,
            _state: PhantomData,
        }
    }
}

// WaitingForGreetingResponse state can only transition to Done
impl ClientSession<WaitingForGreetingResponse> {
    pub fn receive_response(self) -> (String, ClientSession<Done>) {
        // Receive response
        let response = self.channel.receive();
        
        // Return response and session in done state
        (response, ClientSession {
            channel: self.channel,
            _state: PhantomData,
        })
    }
}

// WaitingForCountResponse state can only transition to Done
impl ClientSession<WaitingForCountResponse> {
    pub fn receive_response(self) -> (i32, ClientSession<Done>) {
        // Receive response
        let response = self.channel.receive();
        
        // Return response and session in done state
        (response, ClientSession {
            channel: self.channel,
            _state: PhantomData,
        })
    }
}

// Usage example showing how choice is represented as type transitions
fn client_protocol() {
    // Connect to server, getting session in Ready state
    let session = ClientSession::<Ready>::connect("localhost:8080");
    
    // Regular business logic to decide which branch to take
    let use_greeting = get_user_preference();
    
    if use_greeting {
        // Choose greeting branch - note how the session type changes
        let greeting = "Hello, server!".to_string();
        let waiting_session = session.choose_greeting(greeting);
        
        // Regular business logic between protocol steps
        update_ui_status("Waiting for response...");
        
        // Receive response - session transitions to Done state
        let (response, done_session) = waiting_session.receive_response();
        
        // Regular business logic with the result
        display_response(response);
    } else {
        // Choose count branch - different session type
        let count = 42;
        let waiting_session = session.choose_count(count);
        
        // Regular business logic between protocol steps
        update_ui_status("Waiting for count response...");
        
        // Receive response - session transitions to Done state
        let (response, done_session) = waiting_session.receive_response();
        
        // Regular business logic with the result
        display_count_response(response);
    }
}
```

#### Implementing Offer (Receiver Side)

```rust
// Server-side state machine states
struct ServerReady;
struct ProcessingGreeting;
struct ProcessingCount;
struct ServerDone;

// Server session with state type parameter
struct ServerSession<S> {
    channel: RawChannel,
    _state: PhantomData<S>,
}

// Ready state handles branch offering
impl ServerSession<ServerReady> {
    pub fn offer(self) -> ServerBranchResult {
        // Receive branch selection
        let branch = self.channel.receive_branch_selection();
        
        match branch {
            "greeting" => {
                // Receive greeting message
                let greeting: String = self.channel.receive();
                
                // Transition to ProcessingGreeting state
                ServerBranchResult::Greeting(
                    greeting,
                    ServerSession::<ProcessingGreeting> {
                        channel: self.channel,
                        _state: PhantomData,
                    }
                )
            },
            "count" => {
                // Receive count message
                let count: i32 = self.channel.receive();
                
                // Transition to ProcessingCount state
                ServerBranchResult::Count(
                    count,
                    ServerSession::<ProcessingCount> {
                        channel: self.channel,
                        _state: PhantomData,
                    }
                )
            },
            _ => ServerBranchResult::Invalid,
        }
    }
}

// Result of branch offering with strongly typed sessions for each branch
enum ServerBranchResult {
    Greeting(String, ServerSession<ProcessingGreeting>),
    Count(i32, ServerSession<ProcessingCount>),
    Invalid,
}

// ProcessingGreeting state can only send string response
impl ServerSession<ProcessingGreeting> {
    pub fn send_response(self, response: String) -> ServerSession<ServerDone> {
        // Send response
        self.channel.send(response);
        
        // Transition to Done state
        ServerSession {
            channel: self.channel,
            _state: PhantomData,
        }
    }
}

// ProcessingCount state can only send int response
impl ServerSession<ProcessingCount> {
    pub fn send_response(self, response: i32) -> ServerSession<ServerDone> {
        // Send response
        self.channel.send(response);
        
        // Transition to Done state
        ServerSession {
            channel: self.channel,
            _state: PhantomData,
        }
    }
}

// Usage example:
fn server_protocol() {
    // Accept connection, getting session in Ready state
    let session = ServerSession::<ServerReady>::accept();
    
    // Offer branches to client
    match session.offer() {
        ServerBranchResult::Greeting(greeting, session) => {
            // Regular business logic to process greeting
            println!("Received greeting: {}", greeting);
            let response = format!("Hello from server!");
            
            // Send response - transition to Done state
            let done_session = session.send_response(response);
        },
        ServerBranchResult::Count(count, session) => {
            // Regular business logic to process count
            println!("Received count: {}", count);
            let double_count = count * 2;
            
            // Send response - transition to Done state
            let done_session = session.send_response(double_count);
        },
        ServerBranchResult::Invalid => {
            // Handle error
            eprintln!("Invalid branch selection");
        }
    }
}
```

#### Builder Pattern for More Complex Protocols

For protocols with many branches, a builder pattern can make the API more ergonomic:

```rust
// Branch builder for complex choice protocols
struct BranchBuilder<S> {
    channel: RawChannel,
    _state: PhantomData<S>,
}

impl BranchBuilder<Ready> {
    pub fn with_branches<F, R>(self, handler: F) -> R
    where
        F: FnOnce(BranchSelector) -> R,
    {
        // Create branch selector
        let selector = BranchSelector {
            channel: self.channel,
        };
        
        // Let caller use the selector to build branches
        handler(selector)
    }
}

// Branch selector for fluent API
struct BranchSelector {
    channel: RawChannel,
}

impl BranchSelector {
    // Define a branch with its handler
    pub fn branch<T, F, R>(self, name: &str, message: T, handler: F) -> BranchResult<R>
    where
        T: Serialize,
        F: FnOnce(RawChannel) -> R,
    {
        // Send branch selection
        self.channel.send_branch_selection(name);
        
        // Send the message
        self.channel.send(message);
        
        // Process response with handler
        let result = handler(self.channel);
        
        BranchResult {
            result,
            channel: self.channel,
        }
    }
}

// Result wrapper for branch execution
struct BranchResult<R> {
    result: R,
    channel: RawChannel,
}

// Usage example with fluent branch building
fn complex_client_protocol() {
    // Create session builder
    let builder = BranchBuilder::<Ready>::new("localhost:8080");
    
    // Regular business logic to prepare data
    let user_data = prepare_user_data();
    let command = determine_command();
    
    // Use branch builder with fluent API
    let result = builder.with_branches(|selector| {
        match command {
            Command::Query => {
                // Select query branch
                selector.branch("query", user_data.query, |channel| {
                    // Handle response for query branch
                    let response: QueryResult = channel.receive();
                    process_query_result(response)
                })
            },
            Command::Update => {
                // Select update branch
                selector.branch("update", user_data.update_request, |channel| {
                    // Handle response for update branch
                    let response: UpdateResult = channel.receive();
                    process_update_result(response)
                })
            },
            Command::Delete => {
                // Select delete branch
                selector.branch("delete", user_data.id, |channel| {
                    // Handle response for delete branch
                    let response: DeleteResult = channel.receive();
                    process_delete_result(response)
                })
            }
        }
    });
    
    // Regular business logic with the result
    display_operation_result(result.result);
}
```

#### Key Benefits of This Approach

1. **Explicit State Transitions**: The protocol state is clearly represented in the type system
2. **IDE Support**: Method discovery through autocomplete shows only valid operations for current state
3. **Type Safety**: Compiler enforces the protocol structure
4. **Builder Patterns**: Fluent APIs make complex protocols more readable
5. **Regular Code Integration**: Business logic can be freely interspersed between state transitions

The State Machine Builders approach makes the Choice/Offer combinator explicit in the type system, with clear state transitions that mirror the underlying session type structure. This approach is particularly good for visualizing the protocol flow in the code.

### Parallel Composition Combinator

The Parallel Composition combinator represents concurrent protocol branches that can execute independently. In the State Machine Builders approach, parallel composition is represented through explicit splitting and joining of state machines.

#### Basic Parallel State Machine

```rust
// State types for different protocol stages
struct InitialState;
struct SplitState;
struct StatusSentState;
struct RequestHandledState;
struct JoinedState;
struct FinalState;

// Session with type-state pattern
struct ServerSession<S> {
    channel: RawChannel,
    _state: PhantomData<S>,
}

// Initial state can transition to split state
impl ServerSession<InitialState> {
    pub fn split(self) -> (
        ServerSession<SplitState, StatusBranch>,
        ServerSession<SplitState, RequestBranch>
    ) {
        // Split the underlying channel
        let (status_channel, request_channel) = self.channel.split();
        
        // Create two separate typed sessions for the parallel branches
        (
            ServerSession {
                channel: status_channel,
                _state: PhantomData,
            },
            ServerSession {
                channel: request_channel,
                _state: PhantomData,
            }
        )
    }
}

// First branch can only send status
impl ServerSession<SplitState, StatusBranch> {
    pub fn send_status(self, status: Status) -> ServerSession<StatusSentState, StatusBranch> {
        // Send status message
        self.channel.send(status);
        
        // Transition to next state
        ServerSession {
            channel: self.channel,
            _state: PhantomData,
        }
    }
}

// Second branch can handle requests
impl ServerSession<SplitState, RequestBranch> {
    pub fn receive_request(self) -> (Request, ServerSession<RequestReceivedState, RequestBranch>) {
        // Receive request
        let request = self.channel.receive();
        
        // Transition to next state
        (request, ServerSession {
            channel: self.channel,
            _state: PhantomData,
        })
    }
}

impl ServerSession<RequestReceivedState, RequestBranch> {
    pub fn send_response(self, response: Response) -> ServerSession<RequestHandledState, RequestBranch> {
        // Send response
        self.channel.send(response);
        
        // Transition to next state
        ServerSession {
            channel: self.channel,
            _state: PhantomData,
        }
    }
}

// Join operation to synchronize parallel branches
fn join<S1, S2>(
    status_session: ServerSession<StatusSentState, StatusBranch>,
    request_session: ServerSession<RequestHandledState, RequestBranch>
) -> ServerSession<JoinedState> {
    // Join the underlying channels
    let joined_channel = RawChannel::join(status_session.channel, request_session.channel);
    
    // Create joint session
    ServerSession {
        channel: joined_channel,
        _state: PhantomData,
    }
}

// Final state after join
impl ServerSession<JoinedState> {
    pub fn finalize(self) -> ServerSession<FinalState> {
        // Any cleanup or synchronization
        
        // Transition to final state
        ServerSession {
            channel: self.channel,
            _state: PhantomData,
        }
    }
}

// Usage example showing split-join pattern for parallel composition
fn server_protocol() {
    // Create initial session
    let initial_session = ServerSession::<InitialState>::new();
    
    // Split into parallel branches
    let (status_session, request_session) = initial_session.split();
    
    // Create two tasks for parallel execution
    let status_task = std::thread::spawn(move || {
        // Regular business logic to prepare status
        let system_status = get_system_status();
        
        // Protocol operation: send status
        status_session.send_status(system_status)
    });
    
    let request_task = std::thread::spawn(move || {
        // Protocol operation: receive request
        let (request, session) = request_session.receive_request();
        
        // Regular business logic to process request
        let response = process_request(request);
        
        // Protocol operation: send response
        session.send_response(response)
    });
    
    // Wait for both branches to complete
    let status_session = status_task.join().unwrap();
    let request_session = request_task.join().unwrap();
    
    // Join the parallel branches
    let joined_session = join(status_session, request_session);
    
    // Finalize the protocol
    let final_session = joined_session.finalize();
    
    println!("Protocol completed successfully");
}
```

#### Builder Pattern for Parallel Composition

A more flexible builder pattern can make complex parallel protocols more manageable:

```rust
// Builder for parallel protocol branches
struct ParallelBuilder {
    channels: Vec<RawChannel>,
}

impl ParallelBuilder {
    // Add a branch to the parallel composition
    pub fn branch<F, R>(mut self, handler: F) -> ParallelResult<R>
    where
        F: FnOnce(RawChannel) -> R + Send + 'static,
        R: Send + 'static,
    {
        // Take one channel for this branch
        let channel = self.channels.remove(0);
        
        // Create a thread for this branch
        let handle = std::thread::spawn(move || handler(channel));
        
        // Return result container
        ParallelResult {
            handle,
            remaining: self,
        }
    }
}

// Result of a parallel branch execution
struct ParallelResult<R> {
    handle: JoinHandle<R>,
    remaining: ParallelBuilder,
}

impl<R1> ParallelResult<R1> {
    // Add another branch to execute in parallel
    pub fn branch<F, R2>(self, handler: F) -> ParallelResults<R1, R2>
    where
        F: FnOnce(RawChannel) -> R2 + Send + 'static,
        R2: Send + 'static,
    {
        // Execute the next branch
        let next_result = self.remaining.branch(handler);
        
        // Combine results
        ParallelResults {
            handle1: self.handle,
            handle2: next_result.handle,
            remaining: next_result.remaining,
        }
    }
}

// Combined results of two parallel branches
struct ParallelResults<R1, R2> {
    handle1: JoinHandle<R1>,
    handle2: JoinHandle<R2>,
    remaining: ParallelBuilder,
}

impl<R1, R2> ParallelResults<R1, R2> {
    // Join parallel branches and get their results
    pub fn join(self) -> (R1, R2) {
        // Wait for both branches to complete
        let result1 = self.handle1.join().unwrap();
        let result2 = self.handle2.join().unwrap();
        
        (result1, result2)
    }
    
    // Add a third branch
    pub fn branch<F, R3>(self, handler: F) -> ParallelResults3<R1, R2, R3>
    where
        F: FnOnce(RawChannel) -> R3 + Send + 'static,
        R3: Send + 'static,
    {
        // Execute the next branch
        let next_result = self.remaining.branch(handler);
        
        // Combine results
        ParallelResults3 {
            handle1: self.handle1,
            handle2: self.handle2,
            handle3: next_result.handle,
            remaining: next_result.remaining,
        }
    }
}

// Usage example with fluent builder API
fn monitoring_protocol() {
    // Create builder with multiple channels
    let builder = ParallelBuilder::from_protocol("monitoring");
    
    // Execute branches in parallel with fluent API
    let results = builder
        .branch(|channel| {
            // First branch: handle metrics
            let metrics_session = MetricsSession::new(channel);
            
            // Regular business logic interleaved with protocol operations
            let (metrics, session) = metrics_session.receive_metrics();
            process_metrics(metrics);
            
            metrics
        })
        .branch(|channel| {
            // Second branch: handle log entries
            let logs_session = LogsSession::new(channel);
            
            // Regular business logic interleaved with protocol operations
            let (log, session) = logs_session.receive_log();
            store_log(log);
            
            log
        })
        .branch(|channel| {
            // Third branch: handle alerts with response
            let alerts_session = AlertsSession::new(channel);
            
            // Regular business logic interleaved with protocol operations
            let (alert, session) = alerts_session.receive_alert();
            let acknowledgment = process_alert(alert);
            
            session.send_acknowledgment(acknowledgment);
            
            alert
        });
    
    // Join all branches and get their results
    let (metrics, logs, alert) = results.join();
    
    // Regular business logic with combined results
    generate_monitoring_report(metrics, logs, alert);
}
```

#### Parallel State Machine with Asynchronous Operations

For integrating with async/await:

```rust
// Async state machine for parallel protocol
struct AsyncProtocolMachine<S> {
    state: S,
    channels: HashMap<BranchId, AsyncChannel>,
}

// Initial state can split into parallel branches
impl AsyncProtocolMachine<InitialState> {
    pub async fn split(self) -> (
        AsyncProtocolMachine<BranchState, DataBranch>,
        AsyncProtocolMachine<BranchState, ControlBranch>
    ) {
        // Split the underlying channels
        let (data_channel, control_channel) = self.channel.split().await;
        
        // Create separate state machines for each branch
        (
            AsyncProtocolMachine {
                state: BranchState::new(),
                channels: HashMap::from([(BranchId::Data, data_channel)]),
            },
            AsyncProtocolMachine {
                state: BranchState::new(),
                channels: HashMap::from([(BranchId::Control, control_channel)]),
            }
        )
    }
}

// Data branch operations
impl AsyncProtocolMachine<BranchState, DataBranch> {
    pub async fn receive_data(mut self) -> (Data, AsyncProtocolMachine<DataReceivedState, DataBranch>) {
        // Get the channel for this branch
        let channel = self.channels.get_mut(&BranchId::Data).unwrap();
        
        // Receive data
        let data = channel.receive::<Data>().await;
        
        // Transition to next state
        (data, AsyncProtocolMachine {
            state: DataReceivedState::new(),
            channels: self.channels,
        })
    }
    
    pub async fn send_result(mut self, result: DataResult) -> AsyncProtocolMachine<DataCompleteState, DataBranch> {
        // Get the channel for this branch
        let channel = self.channels.get_mut(&BranchId::Data).unwrap();
        
        // Send result
        channel.send(result).await;
        
        // Transition to next state
        AsyncProtocolMachine {
            state: DataCompleteState::new(),
            channels: self.channels,
        }
    }
}

// Control branch operations
impl AsyncProtocolMachine<BranchState, ControlBranch> {
    pub async fn receive_command(mut self) -> (Command, AsyncProtocolMachine<CommandReceivedState, ControlBranch>) {
        // Get the channel for this branch
        let channel = self.channels.get_mut(&BranchId::Control).unwrap();
        
        // Receive command
        let command = channel.receive::<Command>().await;
        
        // Transition to next state
        (command, AsyncProtocolMachine {
            state: CommandReceivedState::new(),
            channels: self.channels,
        })
    }
    
    pub async fn send_status(mut self, status: Status) -> AsyncProtocolMachine<ControlCompleteState, ControlBranch> {
        // Get the channel for this branch
        let channel = self.channels.get_mut(&BranchId::Control).unwrap();
        
        // Send status
        channel.send(status).await;
        
        // Transition to next state
        AsyncProtocolMachine {
            state: ControlCompleteState::new(),
            channels: self.channels,
        }
    }
}

// Join operation to combine parallel branches
async fn join<S1, S2>(
    data_machine: AsyncProtocolMachine<DataCompleteState, DataBranch>,
    control_machine: AsyncProtocolMachine<ControlCompleteState, ControlBranch>
) -> AsyncProtocolMachine<JoinedState> {
    // Combine channels
    let mut channels = HashMap::new();
    channels.extend(data_machine.channels);
    channels.extend(control_machine.channels);
    
    // Create joined state machine
    AsyncProtocolMachine {
        state: JoinedState::new(),
        channels,
    }
}

// Usage example with async/await
async fn process_protocol() {
    // Create initial state machine
    let machine = AsyncProtocolMachine::<InitialState>::new().await;
    
    // Split into parallel branches
    let (data_machine, control_machine) = machine.split().await;
    
    // Process data branch
    let data_task = tokio::spawn(async move {
        // Receive data
        let (data, machine) = data_machine.receive_data().await;
        
        // Regular business logic
        let result = process_data(data).await;
        
        // Send result
        machine.send_result(result).await
    });
    
    // Process control branch
    let control_task = tokio::spawn(async move {
        // Receive command
        let (command, machine) = control_machine.receive_command().await;
        
        // Regular business logic
        let status = execute_command(command).await;
        
        // Send status
        machine.send_status(status).await
    });
    
    // Wait for both branches to complete
    let data_machine = data_task.await.unwrap();
    let control_machine = control_task.await.unwrap();
    
    // Join the branches
    let joined_machine = join(data_machine, control_machine).await;
    
    // Continue with the protocol
    joined_machine.finalize().await;
}
```

#### Key Benefits of This Approach

1. **Type-Safe Parallelism**: Each branch has its own state machine with appropriate types
2. **Explicit Split and Join**: The parallel structure is clearly visible in the code
3. **Flexible Execution Models**: Can be adapted for threads, tasks, async/await, etc.
4. **State Tracking**: Each branch maintains its own state transitions independently
5. **Composable**: Parallel branches can be combined with other protocol patterns

The State Machine Builders approach makes parallel composition explicit with clear split and join points, while maintaining type safety and allowing business logic to be interleaved with protocol operations.

### Recursion Combinator

Recursion in session types enables protocol loops and cyclic behavior. The State Machine Builders approach represents recursion through explicit state transitions that loop back to previous states, creating clear recursive structures with type safety.

#### Basic Recursive State Machine

```rust
// States for a recursive chat protocol
struct Ready;
struct WaitingForResponse;

// Session type with type-state pattern
struct ChatSession<S> {
    channel: RawChannel,
    _state: PhantomData<S>,
}

// Ready state with recursive behavior
impl ChatSession<Ready> {
    // Can send a message, transitioning to WaitingForResponse
    pub fn send_message(self, message: String) -> ChatSession<WaitingForResponse> {
        // Send the message
        self.channel.send(message);
        
        // Transition to waiting state
        ChatSession {
            channel: self.channel,
            _state: PhantomData,
        }
    }
    
    // Can end the conversation
    pub fn end_conversation(self) {
        // Send end marker
        self.channel.send_end_marker();
        
        // Close the channel
        self.channel.close();
    }
}

// WaitingForResponse state with transition back to Ready (recursive loop)
impl ChatSession<WaitingForResponse> {
    // Receive response and loop back to Ready state
    pub fn receive_response(self) -> (String, ChatSession<Ready>) {
        // Receive the response
        let response = self.channel.receive();
        
        // Transition back to ready state (recursion point)
        (response, ChatSession {
            channel: self.channel,
            _state: PhantomData,
        })
    }
}

// Usage example showing recursive protocol with explicit state transitions
fn chat_client() {
    // Create session in Ready state
    let mut session = ChatSession::<Ready>::connect("chat.server.com");
    
    // Regular business logic to drive the chat
    let mut keep_chatting = true;
    
    while keep_chatting {
        // Get user input (regular business logic)
        println!("Enter message (/quit to end):");
        let message = read_line();
        
        if message == "/quit" {
            // End the conversation
            session.end_conversation();
            keep_chatting = false;
        } else {
            // Send message - transitions to WaitingForResponse
            let waiting_session = session.send_message(message);
            
            // Regular business logic between protocol steps
            println!("Waiting for response...");
            
            // Receive response - transitions back to Ready
            let (response, ready_session) = waiting_session.receive_response();
            
            // Regular business logic with response
            println!("Received: {}", response);
            
            // Continue the loop with session back in Ready state
            session = ready_session;
        }
    }
}
```

#### Type-Parameterized Recursive Protocols

For more complex recursive protocols, we can use type parameters to track recursive paths:

```rust
// Protocol with multiple recursive paths
struct BinaryTreeProtocol<Node> {
    channel: RawChannel,
    _node: PhantomData<Node>,
}

// Node types to track position in the protocol tree
struct Root;
struct LeftChild<P> { _parent: PhantomData<P> }
struct RightChild<P> { _parent: PhantomData<P> }
struct Leaf<P> { _parent: PhantomData<P> }

// Root operations
impl BinaryTreeProtocol<Root> {
    // Recursively traverse left
    pub fn go_left(self) -> BinaryTreeProtocol<LeftChild<Root>> {
        // Send left direction
        self.channel.send("left");
        
        // Transition to left child
        BinaryTreeProtocol {
            channel: self.channel,
            _node: PhantomData,
        }
    }
    
    // Recursively traverse right
    pub fn go_right(self) -> BinaryTreeProtocol<RightChild<Root>> {
        // Send right direction
        self.channel.send("right");
        
        // Transition to right child
        BinaryTreeProtocol {
            channel: self.channel,
            _node: PhantomData,
        }
    }
    
    // Terminate traversal at root
    pub fn finish(self) -> Result<(), Error> {
        // Send finish marker
        self.channel.send("finish");
        Ok(())
    }
}

// Left child operations with recursive paths
impl<P> BinaryTreeProtocol<LeftChild<P>> {
    // Continue left
    pub fn go_left(self) -> BinaryTreeProtocol<LeftChild<LeftChild<P>>> {
        // Send left direction
        self.channel.send("left");
        
        // Transition to next left child
        BinaryTreeProtocol {
            channel: self.channel,
            _node: PhantomData,
        }
    }
    
    // Go right from current position
    pub fn go_right(self) -> BinaryTreeProtocol<RightChild<LeftChild<P>>> {
        // Send right direction
        self.channel.send("right");
        
        // Transition to right child
        BinaryTreeProtocol {
            channel: self.channel,
            _node: PhantomData,
        }
    }
    
    // Reached a leaf, return to parent
    pub fn leaf(self) -> BinaryTreeProtocol<Leaf<LeftChild<P>>> {
        // Send leaf marker
        self.channel.send("leaf");
        
        // Transition to leaf
        BinaryTreeProtocol {
            channel: self.channel,
            _node: PhantomData,
        }
    }
}

// Similar implementations for RightChild<P>
// ...

// Return to parent from a leaf
impl<P> BinaryTreeProtocol<Leaf<P>> {
    // Return to parent node
    pub fn return_to_parent(self) -> BinaryTreeProtocol<P> {
        // Send return marker
        self.channel.send("return");
        
        // Get parent value from response
        let _ = self.channel.receive();
        
        // Transition back to parent (recursive step back)
        BinaryTreeProtocol {
            channel: self.channel,
            _node: PhantomData,
        }
    }
}

// Usage example showing complex recursive traversal
fn traverse_binary_tree() {
    // Create protocol session
    let protocol = BinaryTreeProtocol::<Root>::new();
    
    // Use explicit state transitions to traverse the tree
    // Root -> Left -> Left -> Leaf -> Left -> Right -> Leaf -> Root
    let result = protocol
        .go_left()
        .go_left()
        .leaf()
        .return_to_parent()
        .go_right()
        .leaf()
        .return_to_parent()
        .return_to_parent()
        .finish();
        
    // Handle result
    match result {
        Ok(()) => println!("Tree traversal completed"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

#### Recursive Builder Pattern

A builder pattern can make recursive protocols more ergonomic:

```rust
// Recursive protocol builder
struct RecursiveProtocolBuilder {
    channel: RawChannel,
    state: RecursiveState,
}

// Protocol state enumeration
enum RecursiveState {
    Initial,
    Processing,
    Terminal,
}

impl RecursiveProtocolBuilder {
    // Start the recursive protocol
    pub fn start<F>(self, handler: F) -> RecursiveProtocolBuilder 
    where
        F: FnOnce(ProtocolStep) -> Step,
    {
        // Create step for handler
        let step = ProtocolStep {
            channel: self.channel.clone(),
            iteration: 0,
        };
        
        // Process first step
        match handler(step) {
            Step::Continue(channel, next_handler) => {
                // Continue recursion
                RecursiveProtocolBuilder {
                    channel,
                    state: RecursiveState::Processing,
                }.continue_with(1, next_handler)
            },
            Step::Terminate(channel) => {
                // End recursion
                RecursiveProtocolBuilder {
                    channel,
                    state: RecursiveState::Terminal,
                }
            }
        }
    }
    
    // Continue the recursive protocol
    fn continue_with<F>(self, iteration: usize, handler: F) -> RecursiveProtocolBuilder
    where
        F: FnOnce(ProtocolStep) -> Step,
    {
        // Create step for handler
        let step = ProtocolStep {
            channel: self.channel.clone(),
            iteration,
        };
        
        // Process next step
        match handler(step) {
            Step::Continue(channel, next_handler) => {
                // Continue recursion
                RecursiveProtocolBuilder {
                    channel,
                    state: RecursiveState::Processing,
                }.continue_with(iteration + 1, next_handler)
            },
            Step::Terminate(channel) => {
                // End recursion
                RecursiveProtocolBuilder {
                    channel,
                    state: RecursiveState::Terminal,
                }
            }
        }
    }
}

// Protocol step for handler
struct ProtocolStep {
    channel: RawChannel,
    iteration: usize,
}

// Step result for determining recursion flow
enum Step {
    Continue(RawChannel, Box<dyn FnOnce(ProtocolStep) -> Step>),
    Terminate(RawChannel),
}

// Usage example with recursive builder
fn file_transfer_protocol() {
    // Create builder
    let builder = RecursiveProtocolBuilder::new();
    
    // Define and start recursive protocol
    let result = builder.start(|step| {
        // Begin file transfer
        println!("Starting file transfer");
        
        // Send initial handshake
        step.channel.send(Handshake::new());
        
        // Receive acknowledgment
        let ack = step.channel.receive();
        
        // Continue with file chunk transfer
        Step::Continue(
            step.channel, 
            Box::new(transfer_file_chunks)
        )
    });
    
    // The transfer_file_chunks function continues recursion
    fn transfer_file_chunks(step: ProtocolStep) -> Step {
        // Check if we have more chunks to send
        if has_more_chunks() {
            // Read next chunk
            let chunk = read_next_chunk();
            
            // Send chunk
            step.channel.send(chunk);
            
            // Receive acknowledgment
            let ack = step.channel.receive();
            
            // Continue with next chunk
            Step::Continue(
                step.channel,
                Box::new(transfer_file_chunks)
            )
        } else {
            // Send end marker
            step.channel.send(EndOfFile);
            
            // Receive final acknowledgment
            let completion = step.channel.receive();
            
            // End recursion
            Step::Terminate(step.channel)
        }
    }
}
```

#### Explicit Looping with State Restoration

Another pattern uses explicit loop transitions with state restoration:

```rust
// Generic protocol state with type and value tracking
struct ProtocolState<T, S> {
    value: S,
    _type: PhantomData<T>,
}

// Session with explicit state tracking
struct StreamingSession<T, S> {
    channel: RawChannel,
    state: ProtocolState<T, S>,
}

// Protocol stages
struct Initial;
struct Streaming;
struct Terminated;

// Initial state transitions
impl StreamingSession<Initial, ()> {
    // Transition to streaming state
    pub fn start_streaming(self) -> StreamingSession<Streaming, usize> {
        // Send start marker
        self.channel.send_start();
        
        // Transition to streaming with counter
        StreamingSession {
            channel: self.channel,
            state: ProtocolState {
                value: 0, // Initial count
                _type: PhantomData,
            },
        }
    }
}

// Streaming state with recursive looping
impl StreamingSession<Streaming, usize> {
    // Process one item and potentially loop
    pub fn process_next(self) -> Either<StreamingSession<Streaming, usize>, StreamingSession<Terminated, usize>> {
        // Check if we should continue
        if let Some(item) = get_next_item() {
            // Send the item
            self.channel.send(item);
            
            // Receive acknowledgment
            let ack = self.channel.receive();
            
            // Update state and continue loop
            Either::Left(StreamingSession {
                channel: self.channel,
                state: ProtocolState {
                    value: self.state.value + 1, // Increment counter
                    _type: PhantomData,
                },
            })
        } else {
            // Send end marker
            self.channel.send_end();
            
            // Transition to terminated state
            Either::Right(StreamingSession {
                channel: self.channel,
                state: ProtocolState {
                    value: self.state.value, // Preserve counter
                    _type: PhantomData,
                },
            })
        }
    }
}

// Terminated state
impl StreamingSession<Terminated, usize> {
    // Get final count
    pub fn get_processed_count(&self) -> usize {
        self.state.value
    }
}

// Either type for branching
enum Either<L, R> {
    Left(L),
    Right(R),
}

// Usage example with explicit looping
fn streaming_client() {
    // Create session
    let session = StreamingSession::<Initial, ()>::new();
    
    // Start streaming
    let mut current_session = session.start_streaming();
    
    // Explicit loop with state transitions
    loop {
        // Process next item
        match current_session.process_next() {
            Either::Left(next_session) => {
                // Continue the loop with updated session
                current_session = next_session;
            },
            Either::Right(final_session) => {
                println!("Processed {} items", final_session.get_processed_count());
                break;
            }
        }
    }
}

```

#### Key Benefits of This Approach

1. **Explicit State Transitions**: Recursion points are clearly visible in the code
2. **State Preservation**: Can maintain state across recursive calls
3. **Type Safety**: Full type checking of recursive patterns
4. **Flow Control**: Regular Rust code controls when and how to continue recursion
5. **Flexibility**: Can model complex recursive patterns like tree traversals

The State Machine Builders approach makes recursion explicit with clear state transitions that loop back to earlier states. This approach is particularly good for visualizing the recursive structure of protocols and maintaining state across recursive calls.

## Approach 4: Actor-Based Runtime

This approach uses actors to implement session types, where each actor encapsulates a role's behavior in a protocol. Actors maintain their protocol state internally and communicate through message passing, which aligns well with the message-passing nature of session types.

### Choice/Offer Combinator

The Choice/Offer combinator represents a point where one actor makes a choice and another actor offers different branches to handle that choice. With actors, this is naturally modeled through message pattern matching.

#### Implementing Choice (Sender Side)

```rust
// Actor that implements client role in a protocol with choice
struct ClientActor {
    connection: ActorRef<ServerActor>,
    state: ClientState,
}

// Actor state to track protocol progress
enum ClientState {
    Ready,
    WaitingForResponse(ResponseType),
    Done,
}

// Message types for the client actor
enum ClientMessage {
    Start(GreetingType),
    ReceivedResponse(ResponseData),
}

// Response type to track which branch was chosen
enum ResponseType {
    GreetingResponse,
    CountResponse,
}

// Actor implementation
impl Actor for ClientActor {
    type Message = ClientMessage;
    
    fn receive(&mut self, msg: Self::Message) {
        match msg {
            ClientMessage::Start(greeting_type) => {
                // Actor in Ready state selects a branch
                match greeting_type {
                    GreetingType::Text => {
                        // Select greeting branch
                        println!("Sending greeting");
                        
                        // Send branch selection and data in one message
                        self.connection.send(ServerMessage::Greeting(
                            "Hello, server!".to_string()
                        ));
                        
                        // Update actor state
                        self.state = ClientState::WaitingForResponse(ResponseType::GreetingResponse);
                    },
                    GreetingType::Number => {
                        // Select count branch
                        println!("Sending count");
                        
                        // Send branch selection and data in one message
                        self.connection.send(ServerMessage::Count(42));
                        
                        // Update actor state
                        self.state = ClientState::WaitingForResponse(ResponseType::CountResponse);
                    }
                }
            },
            ClientMessage::ReceivedResponse(response) => {
                // Process response based on which branch was chosen
                match self.state {
                    ClientState::WaitingForResponse(ResponseType::GreetingResponse) => {
                        // Process greeting response
                        println!("Received greeting response: {}", response.text.unwrap());
                    },
                    ClientState::WaitingForResponse(ResponseType::CountResponse) => {
                        // Process count response
                        println!("Received count response: {}", response.number.unwrap());
                    },
                    _ => println!("Received response in unexpected state"),
                }
                
                // Update state to done
                self.state = ClientState::Done;
            }
        }
    }
}

// Usage example showing client actor that makes a choice
fn client_system() {
    // Create actor system
    let system = ActorSystem::new();
    
    // Create server actor
    let server = system.create_actor(ServerActor::new());
    
    // Create client actor
    let client = system.create_actor(ClientActor::new(server));
    
    // Regular business logic to determine greeting type
    let greeting_type = if read_user_preference() {
        GreetingType::Text
    } else {
        GreetingType::Number
    };
    
    // Send message to client actor to start protocol
    client.send(ClientMessage::Start(greeting_type));
    
    // Run the actor system
    system.run();
}
```

#### Implementing Offer (Receiver Side)

```rust
// Actor that implements server role in protocol with choice
struct ServerActor {
    state: ServerState,
}

// Actor state to track protocol progress
enum ServerState {
    Ready,
    Done,
}

// Message types for the server actor
enum ServerMessage {
    // Branch messages with their data
    Greeting(String),
    Count(i32),
}

// Actor implementation
impl Actor for ServerActor {
    type Message = ServerMessage;
    
    fn receive(&mut self, msg: Self::Message) {
        // Pattern match on message type to determine which branch was selected
        match msg {
            ServerMessage::Greeting(text) => {
                // Handle greeting branch
                println!("Server received greeting: {}", text);
                
                // Regular business logic to process greeting
                let response = format!("Hello to you too!");
                
                // Send response back to sender
                self.context.sender().send(ClientMessage::ReceivedResponse(
                    ResponseData {
                        text: Some(response),
                        number: None,
                    }
                ));
            },
            ServerMessage::Count(number) => {
                // Handle count branch
                println!("Server received count: {}", number);
                
                // Regular business logic to process count
                let doubled = number * 2;
                
                // Send response back to sender
                self.context.sender().send(ClientMessage::ReceivedResponse(
                    ResponseData {
                        text: None,
                        number: Some(doubled),
                    }
                ));
            }
        }
        
        // Update state to done
        self.state = ServerState::Done;
    }
}
```

#### Using a Protocol Supervisor

For more complex protocols, a supervisor actor can coordinate the exchange:

```rust
// Supervisor actor that manages the protocol
struct ProtocolSupervisor {
    client: ActorRef<ClientActor>,
    server: ActorRef<ServerActor>,
    state: ProtocolState,
}

// Protocol state
enum ProtocolState {
    NotStarted,
    InProgress(ProtocolBranch),
    Completed(ProtocolResult),
    Failed(Error),
}

// Branch tracking
enum ProtocolBranch {
    Greeting,
    Count,
}

// Messages for the supervisor
enum SupervisorMessage {
    Start,
    BranchSelected(ProtocolBranch),
    ResponseReceived(ProtocolResult),
    Error(Error),
}

// Actor implementation
impl Actor for ProtocolSupervisor {
    type Message = SupervisorMessage;
    
    fn receive(&mut self, msg: Self::Message) {
        match msg {
            SupervisorMessage::Start => {
                // Regular business logic to determine which branch to take
                let use_greeting = determine_branch();
                
                // Select branch
                if use_greeting {
                    // Tell client to select greeting branch
                    self.client.send(ClientMessage::SelectGreeting);
                    
                    // Update state
                    self.state = ProtocolState::InProgress(ProtocolBranch::Greeting);
                } else {
                    // Tell client to select count branch
                    self.client.send(ClientMessage::SelectCount);
                    
                    // Update state
                    self.state = ProtocolState::InProgress(ProtocolBranch::Count);
                }
            },
            SupervisorMessage::BranchSelected(branch) => {
                // Log branch selection
                println!("Branch selected: {:?}", branch);
            },
            SupervisorMessage::ResponseReceived(result) => {
                // Protocol completed
                println!("Protocol completed with result: {:?}", result);
                
                // Update state
                self.state = ProtocolState::Completed(result);
                
                // Regular business logic with the result
                process_result(result);
            },
            SupervisorMessage::Error(error) => {
                // Handle protocol error
                println!("Protocol error: {:?}", error);
                
                // Update state
                self.state = ProtocolState::Failed(error);
                
                // Regular business logic for error handling
                handle_error(error);
            }
        }
    }
}

// Usage example with supervisor
fn supervised_protocol() {
    // Create actor system
    let system = ActorSystem::new();
    
    // Create actors
    let server = system.create_actor(ServerActor::new());
    let client = system.create_actor(ClientActor::new(server.clone()));
    
    // Create supervisor
    let supervisor = system.create_actor(ProtocolSupervisor::new(client, server));
    
    // Start the protocol
    supervisor.send(SupervisorMessage::Start);
    
    // Run the actor system
    system.run();
}
```

#### Using Behavior Switching for Protocol Phases

Actors can switch behavior to represent different protocol phases:

```rust
// Actor with behavior switching for protocol phases
struct ProtocolActor {
    connection: ActorRef,
}

// Different behaviors for different protocol phases
impl ProtocolActor {
    // Initial behavior
    fn ready_behavior(&mut self, ctx: &mut Context, msg: Message) {
        match msg {
            Message::Start => {
                // Regular business logic
                let branch = decide_branch();
                
                if branch == Branch::A {
                    // Select branch A
                    self.connection.send(ProtocolMessage::SelectA);
                    
                    // Switch to branch A behavior
                    ctx.become(Self::branch_a_behavior);
                } else {
                    // Select branch B
                    self.connection.send(ProtocolMessage::SelectB);
                    
                    // Switch to branch B behavior
                    ctx.become(Self::branch_b_behavior);
                }
            },
            _ => println!("Unexpected message in ready state"),
        }
    }
    
    // Behavior after selecting branch A
    fn branch_a_behavior(&mut self, ctx: &mut Context, msg: Message) {
        match msg {
            Message::Response(data) => {
                // Process branch A response
                println!("Branch A response: {}", data);
                
                // Regular business logic
                process_a_result(data);
                
                // Switch to completed behavior
                ctx.become(Self::completed_behavior);
            },
            _ => println!("Unexpected message in branch A state"),
        }
    }
    
    // Behavior after selecting branch B
    fn branch_b_behavior(&mut self, ctx: &mut Context, msg: Message) {
        match msg {
            Message::Response(data) => {
                // Process branch B response
                println!("Branch B response: {}", data);
                
                // Regular business logic
                process_b_result(data);
                
                // Switch to completed behavior
                ctx.become(Self::completed_behavior);
            },
            _ => println!("Unexpected message in branch B state"),
        }
    }
    
    // Final behavior
    fn completed_behavior(&mut self, _ctx: &mut Context, msg: Message) {
        match msg {
            Message::Restart => {
                // Reset actor to initial behavior
                _ctx.become(Self::ready_behavior);
            },
            _ => println!("Protocol completed, ignoring message"),
        }
    }
}

// Actor implementation with initial behavior
impl Actor for ProtocolActor {
    fn receive(&mut self, ctx: &mut Context, msg: Message) {
        // Use current behavior
        self.ready_behavior(ctx, msg);
    }
}
```

### Parallel Composition Combinator

The Parallel Composition combinator represents concurrent protocol branches. The Actor-Based Runtime approach is particularly well-suited for parallel composition since actors are inherently concurrent and can execute protocol branches independently.

#### Basic Actor-Based Parallel Composition

```rust
// Parent actor that manages parallel protocol branches
struct ParallelProtocolActor {
    // References to child actors handling different branches
    metrics_actor: ActorRef<MetricsActor>,
    logs_actor: ActorRef<LogActor>,
    alerts_actor: ActorRef<AlertActor>,
    // State to track completion of parallel branches
    metrics_done: bool,
    logs_done: bool,
    alerts_done: bool,
}

// Message types for parent actor
enum ParallelProtocolMessage {
    Start,
    BranchCompleted(BranchType, BranchResult),
}

enum BranchType {
    Metrics,
    Logs,
    Alerts,
}

// Actor implementation
impl Actor for ParallelProtocolActor {
    type Message = ParallelProtocolMessage;
    
    fn receive(&mut self, msg: Self::Message) {
        match msg {
            ParallelProtocolMessage::Start => {
                // Start all parallel branches simultaneously
                self.metrics_actor.send(MetricsMessage::Start);
                self.logs_actor.send(LogMessage::Start);
                self.alerts_actor.send(AlertMessage::Start);
                
                // No need for explicit state transitions - actors run concurrently
            },
            ParallelProtocolMessage::BranchCompleted(branch_type, result) => {
                // Mark branch as completed
                match branch_type {
                    BranchType::Metrics => {
                        println!("Metrics branch completed");
                        self.metrics_done = true;
                        
                        // Regular business logic with branch result
                        process_metrics_result(result);
                    },
                    BranchType::Logs => {
                        println!("Logs branch completed");
                        self.logs_done = true;
                        
                        // Regular business logic with branch result
                        process_logs_result(result);
                    },
                    BranchType::Alerts => {
                        println!("Alerts branch completed");
                        self.alerts_done = true;
                        
                        // Regular business logic with branch result
                        process_alerts_result(result);
                    },
                }
                
                // Check if all branches are completed
                if self.metrics_done && self.logs_done && self.alerts_done {
                    // All parallel branches completed
                    println!("All parallel branches completed");
                    
                    // Continue with next protocol phase
                    self.start_next_phase();
                }
            }
        }
    }
}

// Actor implementation for metrics branch
struct MetricsActor {
    parent: ActorRef<ParallelProtocolActor>,
    connection: Connection,
}

// Message types for metrics actor
enum MetricsMessage {
    Start,
}

impl Actor for MetricsActor {
    type Message = MetricsMessage;
    
    fn receive(&mut self, msg: Self::Message) {
        match msg {
            MetricsMessage::Start => {
                // Receive metrics according to protocol
                let metrics = self.connection.receive_metrics();
                
                // Regular business logic to process metrics
                let processed_metrics = process_metrics(metrics);
                
                // Notify parent of completion
                self.parent.send(ParallelProtocolMessage::BranchCompleted(
                    BranchType::Metrics,
                    BranchResult::Metrics(processed_metrics),
                ));
            }
        }
    }
}

// Similar implementations for LogActor and AlertActor
// ...

// Usage example showing how parallel branches are created and managed
fn monitoring_system() {
    // Create actor system
    let system = ActorSystem::new();
    
    // Create child actors for each parallel branch
    let metrics_actor = system.create_actor(MetricsActor::new());
    let logs_actor = system.create_actor(LogActor::new());
    let alerts_actor = system.create_actor(AlertActor::new());
    
    // Create parent actor to manage parallel composition
    let protocol_actor = system.create_actor(ParallelProtocolActor::new(
        metrics_actor,
        logs_actor,
        alerts_actor,
    ));
    
    // Start the parallel protocol
    protocol_actor.send(ParallelProtocolMessage::Start);
    
    // Run the actor system
    system.run();
}
```

#### Dynamic Protocol Branches with Routing

For more dynamic parallel composition, a routing actor can be used:

```rust
// Router actor that creates and manages dynamic parallel branches
struct ProtocolRouterActor {
    branches: HashMap<BranchId, ActorRef<BranchActor>>,
    completed_branches: HashSet<BranchId>,
    total_branches: usize,
}

// Message types for router
enum RouterMessage {
    CreateBranches(usize),
    BranchCompleted(BranchId, BranchResult),
}

// Actor implementation
impl Actor for ProtocolRouterActor {
    type Message = RouterMessage;
    
    fn receive(&mut self, msg: Self::Message) {
        match msg {
            RouterMessage::CreateBranches(count) => {
                // Create dynamic number of branch actors
                for i in 0..count {
                    let branch_id = BranchId(i);
                    
                    // Create actor for this branch
                    let branch_actor = self.context.create_actor(BranchActor::new(
                        branch_id,
                        self.context.self_ref(),  // Reference to router
                    ));
                    
                    // Store reference
                    self.branches.insert(branch_id, branch_actor);
                    
                    // Start branch
                    branch_actor.send(BranchMessage::Start);
                }
                
                // Update total branches
                self.total_branches = count;
            },
            RouterMessage::BranchCompleted(branch_id, result) => {
                // Mark branch as completed
                self.completed_branches.insert(branch_id);
                
                // Regular business logic with branch result
                process_branch_result(branch_id, result);
                
                // Check if all branches are done
                if self.completed_branches.len() == self.total_branches {
                    // All branches completed
                    println!("All {} branches completed", self.total_branches);
                    
                    // Continue with next phase
                    self.start_next_phase();
                }
            }
        }
    }
}

// Branch actor for a single parallel branch
struct BranchActor {
    branch_id: BranchId,
    router: ActorRef<ProtocolRouterActor>,
    connection: Connection,
}

// Message types for branch
enum BranchMessage {
    Start,
}

// Actor implementation
impl Actor for BranchActor {
    type Message = BranchMessage;
    
    fn receive(&mut self, msg: Self::Message) {
        match msg {
            BranchMessage::Start => {
                // Regular business logic to determine what to do in this branch
                let branch_type = determine_branch_type(self.branch_id);
                
                match branch_type {
                    BranchType::Receive => {
                        // Receive data according to protocol
                        let data = self.connection.receive_data();
                        
                        // Process data
                        let result = process_data(data);
                        
                        // Notify router of completion
                        self.router.send(RouterMessage::BranchCompleted(
                            self.branch_id,
                            BranchResult::Data(result),
                        ));
                    },
                    BranchType::Send => {
                        // Prepare data to send
                        let data = prepare_data();
                        
                        // Send according to protocol
                        self.connection.send_data(data);
                        
                        // Notify router of completion
                        self.router.send(RouterMessage::BranchCompleted(
                            self.branch_id,
                            BranchResult::Sent,
                        ));
                    }
                }
            }
        }
    }
}

// Usage example with dynamic branch creation
fn dynamic_parallel_protocol() {
    // Create actor system
    let system = ActorSystem::new();
    
    // Create router actor
    let router = system.create_actor(ProtocolRouterActor::new());
    
    // Regular business logic to determine number of parallel branches
    let branch_count = determine_branch_count();
    
    // Create and start branches
    router.send(RouterMessage::CreateBranches(branch_count));
    
    // Run the actor system
    system.run();
}
```

#### Parallel Composition with Join Patterns

For more complex synchronization patterns in parallel composition:

```rust
// Actor with join patterns for parallel branches
struct JoinPatternActor {
    metrics_result: Option<Metrics>,
    logs_result: Option<LogEntries>,
    alerts_result: Option<AlertStatus>,
}

// Message types with join patterns
enum JoinMessage {
    MetricsCompleted(Metrics),
    LogsCompleted(LogEntries),
    AlertsCompleted(AlertStatus),
}

// Actor implementation with join patterns
impl Actor for JoinPatternActor {
    type Message = JoinMessage;
    
    fn receive(&mut self, msg: Self::Message) {
        match msg {
            JoinMessage::MetricsCompleted(metrics) => {
                // Store metrics result
                self.metrics_result = Some(metrics);
                
                // Check if we can join results
                self.try_join();
            },
            JoinMessage::LogsCompleted(logs) => {
                // Store logs result
                self.logs_result = Some(logs);
                
                // Check if we can join results
                self.try_join();
            },
            JoinMessage::AlertsCompleted(alerts) => {
                // Store alerts result
                self.alerts_result = Some(alerts);
                
                // Check if we can join results
                self.try_join();
            }
        }
    }
}

// Join method to synchronize parallel results
impl JoinPatternActor {
    fn try_join(&mut self) {
        // Check if all results are available
        if let (Some(metrics), Some(logs), Some(alerts)) = 
            (&self.metrics_result, &self.logs_result, &self.alerts_result) {
            
            // All branches completed, we can join results
            println!("All parallel branches completed");
            
            // Regular business logic with combined results
            let combined_report = create_combined_report(
                metrics.clone(),
                logs.clone(),
                alerts.clone(),
            );
            
            // Reset state for next cycle
            self.metrics_result = None;
            self.logs_result = None;
            self.alerts_result = None;
            
            // Continue protocol with combined results
            self.send_report(combined_report);
        }
    }
}

// Usage example with join patterns
fn join_pattern_protocol() {
    // Create actor system
    let system = ActorSystem::new();
    
    // Create join actor
    let join_actor = system.create_actor(JoinPatternActor::new());
    
    // Create branch actors
    let metrics_actor = system.create_actor(MetricsActor::new(join_actor.clone()));
    let logs_actor = system.create_actor(LogsActor::new(join_actor.clone()));
    let alerts_actor = system.create_actor(AlertsActor::new(join_actor.clone()));
    
    // Regular business logic to start protocol branches
    let protocol_config = load_protocol_config();
    
    // Start parallel branches with configuration
    metrics_actor.send(MetricsMessage::Start(protocol_config.metrics_config));
    logs_actor.send(LogsMessage::Start(protocol_config.logs_config));
    alerts_actor.send(AlertsMessage::Start(protocol_config.alerts_config));
    
    // Run the actor system
    system.run();
}
```

#### Key Benefits of This Approach

1. **Natural Concurrency**: Actors are inherently concurrent, making parallel composition straightforward
2. **Independent State**: Each branch maintains its own state in its actor
3. **Message-Based Synchronization**: Actor messaging provides natural coordination between parallel branches
4. **Dynamic Creation**: Can create and manage variable numbers of parallel branches
5. **Supervision**: Parent actors can monitor and recover from failures in parallel branches
6. **Join Patterns**: Complex synchronization patterns can be implemented using message handling

The Actor-Based Runtime approach makes parallel composition implicit in the actor structure, with parent actors coordinating child actors that handle individual branches. This approach is particularly effective for protocols with complex concurrent behaviors and synchronization requirements.

### Recursion Combinator

Recursion in session types allows protocols to loop back and repeat behavior. In the Actor-Based Runtime approach, recursion is naturally represented through message-driven state transitions where actors can send messages to themselves to continue a protocol.

#### Basic Recursive Actor

```rust
// Actor that implements a recursive protocol
struct RecursiveProtocolActor {
    connection: ActorRef,
    state: ProtocolState,
    iteration: usize,
}

// Actor state
enum ProtocolState {
    Ready,
    WaitingForResponse,
    Done,
}

// Message types
enum ProtocolMessage {
    Start,
    Continue,
    ReceiveResponse(Response),
    Terminate,
}

// Actor implementation
impl Actor for RecursiveProtocolActor {
    type Message = ProtocolMessage;
    
    fn receive(&mut self, msg: Self::Message) {
        match msg {
            ProtocolMessage::Start => {
                // Initialize recursive protocol
                println!("Starting recursive protocol");
                
                // Send initial message
                let request = self.prepare_request();
                self.connection.send(ConnectionMessage::Request(request));
                
                // Update state
                self.state = ProtocolState::WaitingForResponse;
            },
            ProtocolMessage::ReceiveResponse(response) => {
                // Process response
                println!("Received response: {:?}", response);
                
                // Regular business logic with response
                let should_continue = self.process_response(&response);
                
                if should_continue {
                    // Continue recursion by sending message to self
                    self.context.self_ref().send(ProtocolMessage::Continue);
                } else {
                    // End recursion
                    self.context.self_ref().send(ProtocolMessage::Terminate);
                }
            },
            ProtocolMessage::Continue => {
                // Increment iteration counter
                self.iteration += 1;
                
                // Regular business logic to prepare next iteration
                let request = self.prepare_request();
                
                // Send next request
                self.connection.send(ConnectionMessage::Request(request));
                
                // Update state
                self.state = ProtocolState::WaitingForResponse;
            },
            ProtocolMessage::Terminate => {
                // Regular business logic for termination
                println!("Terminating after {} iterations", self.iteration);
                
                // Update state
                self.state = ProtocolState::Done;
            }
        }
    }
}

// Regular business logic methods
impl RecursiveProtocolActor {
    fn prepare_request(&self) -> Request {
        Request {
            id: self.iteration,
            data: format!("Request data for iteration {}", self.iteration),
        }
    }
    
    fn process_response(&self, response: &Response) -> bool {
        // Decide whether to continue based on response
        self.iteration < 10 && response.status == Status::Ok
    }
}

// Usage example showing recursive protocol with actors
fn recursive_protocol_system() {
    // Create actor system
    let system = ActorSystem::new();
    
    // Create connection actor
    let connection = system.create_actor(ConnectionActor::new());
    
    // Create protocol actor
    let protocol = system.create_actor(RecursiveProtocolActor::new(connection));
    
    // Start the recursive protocol
    protocol.send(ProtocolMessage::Start);
    
    // Run the actor system
    system.run();
}
```

#### Behavior-Switching Recursive Actor

A more sophisticated approach uses behavior switching for recursive protocols:

```rust
// Actor with behavior switching for recursive protocol
struct StreamingActor {
    connection: ActorRef,
    context: ActorContext,
    item_count: usize,
    max_items: usize,
}

// Message types
enum StreamingMessage {
    Start,
    Item(Data),
    Acknowledgment,
    Complete,
}

// Helper for state transitions
impl StreamingActor {
    // Initial behavior
    fn receiving_behavior(&mut self, msg: StreamingMessage) {
        match msg {
            StreamingMessage::Start => {
                // Send request for first item
                self.connection.send(ConnectionMessage::RequestItem);
            },
            StreamingMessage::Item(data) => {
                // Process received data
                println!("Received item {}: {:?}", self.item_count, data);
                
                // Increment counter
                self.item_count += 1;
                
                // Send acknowledgment
                self.connection.send(ConnectionMessage::Acknowledge);
                
                // Check if we should continue
                if self.item_count >= self.max_items {
                    // Switch to completion behavior
                    self.context.become(Self::completing_behavior);
                    
                    // Start completion
                    self.connection.send(ConnectionMessage::Complete);
                } else {
                    // Continue recursion by requesting next item
                    self.connection.send(ConnectionMessage::RequestItem);
                }
            },
            _ => println!("Unexpected message in receiving state"),
        }
    }
    
    // Completion behavior
    fn completing_behavior(&mut self, msg: StreamingMessage) {
        match msg {
            StreamingMessage::Complete => {
                // Regular business logic for completion
                println!("Protocol completed with {} items", self.item_count);
            },
            _ => println!("Unexpected message in completing state"),
        }
    }
}

// Actor implementation with initial behavior
impl Actor for StreamingActor {
    type Message = StreamingMessage;
    
    fn receive(&mut self, msg: Self::Message) {
        // Use current behavior
        self.receiving_behavior(msg);
    }
    
    fn pre_start(&mut self) {
        // Initialize behavior to receiving
        self.context.become(Self::receiving_behavior);
    }
}

// Usage example
fn streaming_protocol() {
    // Create actor system
    let system = ActorSystem::new();
    
    // Create connection actor
    let connection = system.create_actor(ConnectionActor::new());
    
    // Create protocol actor with configuration
    let protocol = system.create_actor(StreamingActor::new(
        connection,
        0,  // initial item count
        100,  // max items
    ));
    
    // Start the protocol
    protocol.send(StreamingMessage::Start);
    
    // Run the actor system
    system.run();
}
```

#### Supervisor-Managed Recursive Protocol

For more complex recursive protocols, a supervisor can manage the recursion:

```rust
// Supervisor actor for recursive protocol
struct RecursionSupervisorActor {
    worker: ActorRef<WorkerActor>,
    connection: ActorRef<ConnectionActor>,
    state: SupervisorState,
    iterations: usize,
    max_iterations: usize,
}

// Supervisor state
enum SupervisorState {
    NotStarted,
    Running,
    Completed,
}

// Message types
enum SupervisorMessage {
    Start,
    IterationCompleted(IterationResult),
    Error(ProtocolError),
}

// Actor implementation
impl Actor for RecursionSupervisorActor {
    type Message = SupervisorMessage;
    
    fn receive(&mut self, msg: Self::Message) {
        match msg {
            SupervisorMessage::Start => {
                if self.state == SupervisorState::NotStarted {
                    // Start the first iteration
                    self.start_next_iteration();
                    
                    // Update state
                    self.state = SupervisorState::Running;
                }
            },
            SupervisorMessage::IterationCompleted(result) => {
                // Process iteration result
                println!("Completed iteration {}: {:?}", self.iterations, result);
                
                // Increment iteration counter
                self.iterations += 1;
                
                // Regular business logic to determine if we continue
                if self.should_continue(&result) {
                    // Start next iteration
                    self.start_next_iteration();
                } else {
                    // Complete the recursive protocol
                    println!("Protocol completed after {} iterations", self.iterations);
                    
                    // Update state
                    self.state = SupervisorState::Completed;
                }
            },
            SupervisorMessage::Error(error) => {
                // Handle error in protocol
                println!("Protocol error: {:?}", error);
                
                // Attempt recovery
                if self.can_recover(&error) {
                    // Restart current iteration
                    println!("Recovering from error");
                    self.start_next_iteration();
                } else {
                    // Cannot recover
                    println!("Unrecoverable error, terminating protocol");
                    
                    // Update state
                    self.state = SupervisorState::Completed;
                }
            }
        }
    }
}

// Helper methods
impl RecursionSupervisorActor {
    fn start_next_iteration(&self) {
        // Create configuration for this iteration
        let config = IterationConfig {
            iteration: self.iterations,
            timeout: Duration::from_secs(30),
        };
        
        // Tell worker to start iteration
        self.worker.send(WorkerMessage::StartIteration(config));
    }
    
    fn should_continue(&self, result: &IterationResult) -> bool {
        // Check termination conditions
        self.iterations < self.max_iterations && result.should_continue
    }
    
    fn can_recover(&self, error: &ProtocolError) -> bool {
        // Check if error is recoverable
        match error {
            ProtocolError::Timeout => true,
            ProtocolError::ConnectionReset => true,
            _ => false,
        }
    }
}

// Worker actor that performs protocol iterations
struct WorkerActor {
    supervisor: ActorRef<RecursionSupervisorActor>,
    connection: ActorRef<ConnectionActor>,
}

// Message types
enum WorkerMessage {
    StartIteration(IterationConfig),
    ResponseReceived(Response),
}

// Actor implementation
impl Actor for WorkerActor {
    type Message = WorkerMessage;
    
    fn receive(&mut self, msg: Self::Message) {
        match msg {
            WorkerMessage::StartIteration(config) => {
                // Regular business logic to prepare request
                let request = self.prepare_request(config.iteration);
                
                // Send request
                self.connection.send(ConnectionMessage::Request(request));
                
                // Set timeout for response
                self.context.set_timeout(config.timeout, || {
                    self.supervisor.send(SupervisorMessage::Error(ProtocolError::Timeout));
                });
            },
            WorkerMessage::ResponseReceived(response) => {
                // Process response
                match self.process_response(response) {
                    Ok(result) => {
                        // Send result to supervisor
                        self.supervisor.send(SupervisorMessage::IterationCompleted(result));
                    },
                    Err(error) => {
                        // Send error to supervisor
                        self.supervisor.send(SupervisorMessage::Error(error));
                    }
                }
            }
        }
    }
}

// Usage example
fn supervised_recursive_protocol() {
    // Create actor system
    let system = ActorSystem::new();
    
    // Create connection actor
    let connection = system.create_actor(ConnectionActor::new());
    
    // Create worker actor (reference to supervisor will be set later)
    let worker = system.create_actor_with_ref(|worker_ref| {
        // Create supervisor with reference to worker
        let supervisor = system.create_actor(RecursionSupervisorActor::new(
            worker_ref,
            connection.clone(),
        ));
        
        // Create worker with reference to supervisor
        WorkerActor::new(supervisor, connection)
    });
    
    // Get reference to supervisor and start protocol
    let supervisor = system.get_actor("supervisor").unwrap();
    supervisor.send(SupervisorMessage::Start);
    
    // Run actor system
    system.run();
}
```

#### Key Benefits of This Approach

1. **Natural Recursion**: Message passing provides a natural way to express recursion
2. **State Management**: Actors maintain their state across recursive iterations
3. **Error Handling**: Supervisor patterns can manage errors in recursive protocols
4. **Concurrency Control**: Built-in throttling and back-pressure mechanisms
5. **Resource Management**: Actor lifecycle ensures proper cleanup even with deep recursion
6. **Behavior Switching**: Can adapt behavior based on protocol state

The Actor-Based Runtime approach handles recursion through self-messaging and behavior switching, which aligns well with the message-passing nature of session types. This approach is particularly effective for protocols that involve complex state management or error recovery patterns.

## Approach 5: Continuation-Passing Style (CPS)

This approach uses higher-order functions and continuations to represent protocol steps. Each operation in the protocol takes a continuation function that determines what happens next, allowing for flexible composition of protocol behaviors.

### Choice/Offer Combinator

In the Continuation-Passing Style approach, the Choice/Offer combinator is represented as functions that accept branch-specific continuations. This makes branch selection and handling explicit while maintaining the callback-driven flow.

#### Implementing Choice (Sender Side)

```rust
// Basic channel for CPS operations
struct Channel {
    inner: RawChannel,
}

// Choice operation that takes branch-specific continuations
impl Channel {
    pub fn select_branch<R>(
        self,
        branch: &str,
        on_greeting: impl FnOnce(Channel) -> R,
        on_count: impl FnOnce(Channel) -> R
    ) -> R {
        // Send branch selection
        self.inner.send_branch_selection(branch);
        
        // Dispatch to appropriate continuation
        match branch {
            "greeting" => on_greeting(self),
            "count" => on_count(self),
            _ => panic!("Invalid branch selection"),
        }
    }
    
    // Sending operations for each branch
    pub fn send_greeting<R>(
        self,
        greeting: String,
        continue_with: impl FnOnce(Channel) -> R
    ) -> R {
        // Send greeting
        self.inner.send(greeting);
        
        // Continue with next protocol step
        continue_with(self)
    }
    
    pub fn send_count<R>(
        self,
        count: i32,
        continue_with: impl FnOnce(Channel) -> R
    ) -> R {
        // Send count
        self.inner.send(count);
        
        // Continue with next protocol step
        continue_with(self)
    }
    
    // Receive operation for response
    pub fn receive<T, R>(
        self,
        continue_with: impl FnOnce(T, Channel) -> R
    ) -> R {
        // Receive message
        let message = self.inner.receive();
        
        // Continue with received message and channel
        continue_with(message, self)
    }
}

// Usage example showing choice with continuations
fn client_protocol(channel: Channel) {
    // Regular business logic to decide which branch to take
    let use_greeting = get_user_preference();
    
    if use_greeting {
        // Select greeting branch with its continuation
        channel.select_branch(
            "greeting",
            // Greeting branch continuation
            |channel| {
                // Send greeting message
                channel.send_greeting(
                    "Hello, server!".to_string(),
                    // Continuation after sending greeting
                    |channel| {
                        // Receive response
                        channel.receive(
                            // Continuation after receiving response
                            |response: String, channel| {
                                // Regular business logic with response
                                println!("Received greeting response: {}", response);
                            }
                        )
                    }
                )
            },
            // Count branch continuation (not taken in this case)
            |_| { /* would handle count branch */ }
        );
    } else {
        // Select count branch with its continuation
        channel.select_branch(
            "count",
            // Greeting branch continuation (not taken in this case)
            |_| { /* would handle greeting branch */ },
            // Count branch continuation
            |channel| {
                // Send count
                channel.send_count(
                    42,
                    // Continuation after sending count
                    |channel| {
                        // Receive response
                        channel.receive(
                            // Continuation after receiving response
                            |response: i32, channel| {
                                // Regular business logic with response
                                println!("Received count response: {}", response);
                            }
                        )
                    }
                )
            }
        );
    }
}
```

#### Implementing Offer (Receiver Side)

```rust
// Offer operation that handles branch selection
impl Channel {
    pub fn offer<R>(
        self,
        on_greeting: impl FnOnce(String, Channel) -> R,
        on_count: impl FnOnce(i32, Channel) -> R
    ) -> R {
        // Receive branch selection
        let branch = self.inner.receive_branch_selection();
        
        match branch {
            "greeting" => {
                // Receive greeting message
                let greeting: String = self.inner.receive();
                
                // Continue with greeting handler
                on_greeting(greeting, self)
            },
            "count" => {
                // Receive count message
                let count: i32 = self.inner.receive();
                
                // Continue with count handler
                on_count(count, self)
            },
            _ => panic!("Unknown branch received"),
        }
    }
    
    // Send response operations for each branch
    pub fn send<T, R>(
        self,
        message: T,
        continue_with: impl FnOnce(Channel) -> R
    ) -> R {
        // Send message
        self.inner.send(message);
        
        // Continue with next protocol step
        continue_with(self)
    }
}

// Usage example showing offer with continuations
fn server_protocol(channel: Channel) {
    // Offer branches with their continuations
    channel.offer(
        // Greeting branch handler
        |greeting: String, channel| {
            // Regular business logic to process greeting
            println!("Received greeting: {}", greeting);
            let response = format!("Hello from server!");
            
            // Send response with continuation
            channel.send(
                response,
                // Continuation after sending response
                |channel| {
                    // Protocol complete
                    println!("Greeting branch completed");
                }
            )
        },
        // Count branch handler
        |count: i32, channel| {
            // Regular business logic to process count
            println!("Received count: {}", count);
            let response = count * 2;
            
            // Send response with continuation
            channel.send(
                response,
                // Continuation after sending response
                |channel| {
                    // Protocol complete
                    println!("Count branch completed");
                }
            )
        }
    );
}
```

#### Composable Branch Handlers

To make complex protocols more manageable, we can use higher-order functions for branch handling:

```rust
// Functions for creating branch handlers
fn greeting_handler<R>(
    on_complete: impl FnOnce(Channel) -> R
) -> impl FnOnce(String, Channel) -> R {
    |greeting, channel| {
        // Regular business logic
        println!("Processing greeting: {}", greeting);
        let response = prepare_greeting_response(greeting);
        
        // Send response and continue
        channel.send(response, on_complete)
    }
}

fn count_handler<R>(
    on_complete: impl FnOnce(Channel) -> R
) -> impl FnOnce(i32, Channel) -> R {
    |count, channel| {
        // Regular business logic
        println!("Processing count: {}", count);
        let response = process_count(count);
        
        // Send response and continue
        channel.send(response, on_complete)
    }
}

// Usage example with composable handlers
fn composable_server_protocol(channel: Channel) {
    // Common completion handler
    let on_complete = |channel| {
        // Regular business logic after completing branch
        println!("Branch processing completed");
        
        // Any clean-up or additional protocol steps
        cleanup_protocol_resources();
    };
    
    // Offer branches with composed handlers
    channel.offer(
        greeting_handler(on_complete),
        count_handler(on_complete)
    );
}
```

#### Generic Branch Handling

For protocols with many branches, a more generic approach can be used:

```rust
// Generic branch handler with dynamic dispatch
struct BranchHandler<R> {
    handlers: HashMap<String, Box<dyn FnOnce(Box<dyn Any>, Channel) -> R>>,
}

impl<R> BranchHandler<R> {
    // Create new empty handler set
    pub fn new() -> Self {
        BranchHandler {
            handlers: HashMap::new(),
        }
    }
    
    // Add a handler for a specific branch
    pub fn on<T>(
        mut self,
        branch: &str,
        handler: impl FnOnce(T, Channel) -> R + 'static
    ) -> Self
    where
        T: 'static,
    {
        // Store handler with dynamic dispatch wrapper
        self.handlers.insert(
            branch.to_string(),
            Box::new(move |msg, channel| {
                // Cast message to expected type
                let typed_msg = msg.downcast::<T>().expect("Type mismatch");
                
                // Call handler with proper types
                handler(*typed_msg, channel)
            }),
        );
        
        self
    }
    
    // Execute appropriate handler for received branch
    pub fn handle(self, branch: &str, msg: Box<dyn Any>, channel: Channel) -> R {
        // Look up handler for branch
        let handler = self.handlers.remove(branch)
            .expect("No handler for branch");
        
        // Execute handler
        handler(msg, channel)
    }
}

// Generic offer operation
impl Channel {
    pub fn offer_generic<R>(
        self,
        handlers: BranchHandler<R>
    ) -> R {
        // Receive branch selection
        let branch = self.inner.receive_branch_selection();
        
        // Receive message based on branch
        let msg: Box<dyn Any> = match branch {
            "greeting" => Box::new(self.inner.receive::<String>()),
            "count" => Box::new(self.inner.receive::<i32>()),
            "data" => Box::new(self.inner.receive::<Vec<u8>>()),
            _ => panic!("Unknown branch"),
        };
        
        // Dispatch to handler
        handlers.handle(&branch, msg, self)
    }
}

// Usage example with generic branch handling
fn generic_server_protocol(channel: Channel) {
    // Create branch handlers
    let handlers = BranchHandler::new()
        .on("greeting", |greeting: String, channel| {
            // Handle greeting branch
            println!("Received greeting: {}", greeting);
            channel.send("Hello back!".to_string(), |_| {})
        })
        .on("count", |count: i32, channel| {
            // Handle count branch
            println!("Received count: {}", count);
            channel.send(count * 2, |_| {})
        })
        .on("data", |data: Vec<u8>, channel| {
            // Handle data branch
            println!("Received {} bytes of data", data.len());
            channel.send(data.len(), |_| {})
        });
    
    // Offer branches with generic handler
    channel.offer_generic(handlers);
}
```

#### Key Benefits of This Approach

1. **Explicit Control Flow**: The protocol flow is made explicit through continuation functions
2. **Composable Handlers**: Branch handlers can be composed and reused
3. **No Type-Level Programming**: Uses standard Rust functions and closures rather than complex type machinery
4. **Flexible Integration**: Regular business logic can be freely interleaved with protocol operations
5. **Error Handling**: Can pass errors through the continuation chain

The Continuation-Passing Style approach makes the Choice/Offer combinator explicit through continuation functions that determine what happens after each protocol step. This approach is particularly powerful for protocols where different steps need to be composed in flexible ways.

### Parallel Composition Combinator

The Parallel Composition combinator represents concurrent protocol branches. In the Continuation-Passing Style approach, parallelism is expressed through continuations that handle each branch independently and synchronize results.

#### Basic Parallel Operations

```rust
// Basic parallel channel operations
impl Channel {
    // Split method for parallel composition
    pub fn split<R>(
        self,
        handler: impl FnOnce(Channel, Channel) -> R
    ) -> R {
        // Split the underlying channel
        let (channel1, channel2) = self.inner.split();
        
        // Create wrapped channels
        let channel1 = Channel { inner: channel1 };
        let channel2 = Channel { inner: channel2 };
        
        // Call handler with both channels
        handler(channel1, channel2)
    }
    
    // Join method for synchronizing parallel branches
    pub fn join<R>(
        channel1: Channel,
        channel2: Channel,
        continue_with: impl FnOnce(Channel) -> R
    ) -> R {
        // Join the underlying channels
        let joined = RawChannel::join(channel1.inner, channel2.inner);
        
        // Create joined channel
        let joined_channel = Channel { inner: joined };
        
        // Continue with joined channel
        continue_with(joined_channel)
    }
}

// Usage example with basic continuations
fn parallel_protocol(channel: Channel) {
    // Split channel for parallel operations
    channel.split(|status_channel, request_channel| {
        // Process both branches (could be concurrent)
        let status_handler = |status_channel| {
            // Send status
            status_channel.send(get_system_status(), |_| {
                // Status branch complete
                println!("Status sent");
            })
        };
        
        let request_handler = |request_channel| {
            // Receive request
            request_channel.receive(|request, response_channel| {
                // Process request
                let response = process_request(request);
                
                // Send response
                response_channel.send(response, |_| {
                    // Request branch complete
                    println!("Request handled");
                })
            })
        };
        
        // Execute both branches (could use threads)
        let status_result = status_handler(status_channel);
        let request_result = request_handler(request_channel);
        
        // Results available for further processing
        process_results(status_result, request_result);
    })
}
```

#### Concurrent Execution with Threads

```rust
// Thread-based parallel execution
impl Channel {
    pub fn parallel<R1, R2>(
        self,
        branch1: impl FnOnce(Channel) -> R1 + Send + 'static,
        branch2: impl FnOnce(Channel) -> R2 + Send + 'static,
        continue_with: impl FnOnce(R1, R2) -> R
    ) -> R
    where
        R1: Send + 'static,
        R2: Send + 'static,
    {
        // Split the channel
        self.split(|channel1, channel2| {
            // Create threads for each branch
            let handle1 = std::thread::spawn(move || branch1(channel1));
            let handle2 = std::thread::spawn(move || branch2(channel2));
            
            // Wait for both branches to complete
            let result1 = handle1.join().unwrap();
            let result2 = handle2.join().unwrap();
            
            // Continue with both results
            continue_with(result1, result2)
        })
    }
}

// Usage example with concurrent execution
fn concurrent_protocol(channel: Channel) {
    // Execute both branches concurrently
    channel.parallel(
        // Branch 1: send metrics
        |metrics_channel| {
            // Regular business logic
            let metrics = collect_metrics();
            
            // Send metrics
            metrics_channel.send(metrics, |_| {
                // Return result from this branch
                "Metrics sent"
            })
        },
        // Branch 2: handle command
        |command_channel| {
            // Receive command
            command_channel.receive(|command, response_channel| {
                // Regular business logic
                let result = execute_command(command);
                
                // Send result
                response_channel.send(result, |_| {
                    // Return result from this branch
                    "Command handled"
                })
            })
        },
        // Continuation after both branches complete
        |metrics_result, command_result| {
            // Process combined results
            println!("Completed parallel operations:");
            println!("- {}", metrics_result);
            println!("- {}", command_result);
        }
    );
}
```

#### Asynchronous Parallel Operations

```rust
// Async parallel operations
impl Channel {
    pub async fn parallel_async<R1, R2, R>(
        self,
        branch1: impl FnOnce(Channel) -> futures::future::BoxFuture<'static, R1>,
        branch2: impl FnOnce(Channel) -> futures::future::BoxFuture<'static, R2>,
        continue_with: impl FnOnce(R1, R2) -> R
    ) -> R
    where
        R1: Send + 'static,
        R2: Send + 'static,
        R: Send + 'static,
    {
        // Split the channel
        self.split(|channel1, channel2| async move {
            // Create futures for each branch
            let future1 = branch1(channel1);
            let future2 = branch2(channel2);
            
            // Await both futures
            let (result1, result2) = futures::join!(future1, future2);
            
            // Continue with both results
            continue_with(result1, result2)
        }).await
    }
}

// Usage example with async branches
async fn async_protocol(channel: Channel) {
    // Execute both branches asynchronously
    channel.parallel_async(
        // Branch 1: process sensor data
        |sensor_channel| Box::pin(async move {
            // Receive sensor data
            let sensor_data = sensor_channel.receive_async(|data, channel| {
                (data, channel)
            }).await;
            
            // Return result
            sensor_data
        }),
        // Branch 2: handle user requests
        |request_channel| Box::pin(async move {
            // Receive request
            let (request, channel) = request_channel.receive_async(|req, ch| {
                (req, ch)
            }).await;
            
            // Process request
            let response = process_request(request).await;
            
            // Send response
            channel.send_async(response, |channel| {
                channel
            }).await;
            
            // Return status
            "Request handled"
        }),
        // Continuation after both branches complete
        |sensor_result, request_result| {
            println!("Sensor data: {:?}", sensor_result);
            println!("Request status: {}", request_result);
            
            // Return combined result
            CombinedResult {
                sensor: sensor_result,
                request: request_result,
            }
        }
    ).await;
}
```

#### N-ary Parallel Composition

```rust
// Support for arbitrary number of parallel branches
struct ParallelBuilder {
    channels: Vec<Channel>,
}

impl ParallelBuilder {
    // Create from a single channel
    pub fn from_channel(channel: Channel, count: usize) -> Self {
        // Split into multiple channels
        let raw_channels = channel.inner.split_n(count);
        
        // Wrap each raw channel
        let channels = raw_channels.into_iter()
            .map(|raw| Channel { inner: raw })
            .collect();
        
        ParallelBuilder { channels }
    }
    
    // Execute a function for each branch
    pub fn for_each<R>(
        self,
        mut handler: impl FnMut(usize, Channel) -> R
    ) -> Vec<R> {
        // Process each branch with its handler
        self.channels.into_iter()
            .enumerate()
            .map(|(idx, channel)| handler(idx, channel))
            .collect()
    }
    
    // Map each branch through a function
    pub fn map<T>(
        self,
        mut mapper: impl FnMut(usize, Channel) -> T
    ) -> ParallelResults<T> {
        // Map each channel through the function
        let results = self.channels.into_iter()
            .enumerate()
            .map(|(idx, channel)| mapper(idx, channel))
            .collect();
        
        ParallelResults { results }
    }
}

// Collection of results from parallel branches
struct ParallelResults<T> {
    results: Vec<T>,
}

impl<T> ParallelResults<T> {
    // Process all results with a continuation
    pub fn continue_with<R>(
        self,
        handler: impl FnOnce(Vec<T>) -> R
    ) -> R {
        handler(self.results)
    }
}

// Usage example with multiple branches
fn n_ary_parallel_protocol(channel: Channel) {
    // Create builder for 3 parallel branches
    let builder = ParallelBuilder::from_channel(channel, 3);
    
    // Process all branches
    let results = builder.for_each(|branch_idx, branch_channel| {
        match branch_idx {
            0 => {
                // First branch: handle metrics
                branch_channel.receive(|metrics, channel| {
                    // Process metrics
                    let result = process_metrics(metrics);
                    
                    // Return branch result
                    BranchResult::Metrics(result)
                })
            },
            1 => {
                // Second branch: handle logs
                branch_channel.receive(|log, channel| {
                    // Process log
                    store_log(log);
                    
                    // Return branch result
                    BranchResult::Log(log)
                })
            },
            2 => {
                // Third branch: handle alerts
                branch_channel.receive(|alert, channel| {
                    // Process alert
                    let acknowledgment = process_alert(alert);
                    
                    // Send acknowledgment
                    channel.send(acknowledgment, |_| {
                        // Return branch result
                        BranchResult::Alert(alert, acknowledgment)
                    })
                })
            },
            _ => unreachable!(),
        }
    });
    
    // Process combined results
    create_monitoring_report(results);
}
```

#### Composing Parallel and Sequential Operations

```rust
// Combining parallel with sequential operations
fn complex_protocol(channel: Channel) {
    // Initial sequential operation
    channel.receive(|initial_data, channel| {
        // Process initial data
        let config = parse_configuration(initial_data);
        
        // Split into parallel branches
        channel.split(|metrics_channel, control_channel| {
            // Create thread for metrics branch
            let metrics_thread = std::thread::spawn(move || {
                // Continuously receive metrics
                let mut current = metrics_channel;
                let mut metrics = Vec::new();
                
                for _ in 0..config.metric_count {
                    current = current.receive(|metric, channel| {
                        // Process metric
                        metrics.push(metric);
                        
                        // Return updated channel
                        channel
                    });
                }
                
                // Return collected metrics
                metrics
            });
            
            // Process control branch directly
            let control_result = control_channel.receive(|command, channel| {
                // Execute command
                let result = execute_command(command);
                
                // Send result
                channel.send(result, |channel| {
                    // Receive final status
                    channel.receive(|status, _| {
                        // Return control result
                        (command, result, status)
                    })
                })
            });
            
            // Wait for metrics thread to complete
            let metrics = metrics_thread.join().unwrap();
            
            // Combine results
            let combined = CombinedResult {
                metrics,
                control: control_result,
            };
            
            // Regular business logic with combined results
            process_combined_results(combined);
        })
    });
}
```

#### Key Benefits of This Approach

1. **Composable Parallelism**: Parallel operations can be composed with other operations
2. **Natural Synchronization**: Continuations provide natural points for synchronizing parallel branches
3. **Explicit Control Flow**: The flow of control is explicitly visible in the continuation chain
4. **Flexible Concurrency Models**: Can be adapted to different concurrency primitives (threads, futures, etc.)
5. **Regular Code Integration**: Business logic can be interleaved with protocol operations

The Continuation-Passing Style approach makes parallel composition explicit through continuations that handle each branch and synchronize results. This approach is particularly effective for protocols that need to compose parallel and sequential operations in flexible ways.

### Recursion Combinator

Recursion in the Continuation-Passing Style approach is implemented through higher-order functions that can reference themselves, creating looping behavior through repeated continuations.

#### Basic Recursive Operations

```rust
// Basic recursion with explicit loop function
type RecursiveProtocol = Box<dyn FnOnce(Channel) -> ()>;

// Create a recursive protocol
fn create_recursive_protocol() -> RecursiveProtocol {
    // Define the recursive function
    fn loop_protocol(channel: Channel) {
        // Receive a message
        channel.receive(|message, channel| {
            match message {
                Message::Continue(data) => {
                    // Process data
                    println!("Processing: {:?}", data);
                    
                    // Send acknowledgment
                    channel.send(Ack(data.id), |channel| {
                        // Recurse - continue the protocol loop
                        loop_protocol(channel);
                    });
                },
                Message::Terminate => {
                    // End the protocol loop
                    println!("Protocol terminated");
                }
            }
        });
    }
    
    // Return the boxed protocol
    Box::new(loop_protocol)
}

// Usage example
fn run_recursive_protocol(channel: Channel) {
    // Create the recursive protocol
    let protocol = create_recursive_protocol();
    
    // Start the protocol
    protocol(channel);
}
```

#### Parameterized Recursive Protocols

```rust
// Recursive protocol with state
type StatefulRecursiveProtocol<S> = Box<dyn FnOnce(Channel, S) -> ()>;

// Create a stateful recursive protocol
fn create_stateful_protocol<S>(
    initial_state: S,
    continuation: impl Fn(Channel, S, Message) -> Option<S> + 'static
) -> StatefulRecursiveProtocol<S> {
    // Define the recursive function
    fn loop_protocol<S>(
        channel: Channel, 
        state: S,
        continuation: impl Fn(Channel, S, Message) -> Option<S> + 'static
    ) {
        // Receive a message
        channel.receive(|message, channel| {
            // Apply continuation to get new state
            if let Some(new_state) = continuation(channel.clone(), state, message) {
                // Continue the protocol with new state
                loop_protocol(channel, new_state, continuation);
            } else {
                // End the protocol
                println!("Protocol terminated");
            }
        });
    }
    
    // Return the boxed protocol with initial state
    Box::new(move |channel, state| {
        loop_protocol(channel, state, continuation)
    })
}

// Usage example
fn run_stateful_protocol(channel: Channel) {
    // Define the protocol state
    struct ProtocolState {
        count: usize,
        accumulated: Vec<String>,
    }
    
    // Create the recursive protocol with state
    let protocol = create_stateful_protocol(
        // Initial state
        ProtocolState {
            count: 0,
            accumulated: Vec::new(),
        },
        // Continuation function
        |channel, mut state, message| {
            match message {
                Message::Data(data) => {
                    // Update state
                    state.count += 1;
                    state.accumulated.push(data);
                    
                    // Send acknowledgment
                    channel.send(Ack(state.count), |_| {
                        // Continue with updated state if not done
                        if state.count < 10 {
                            Some(state)
                        } else {
                            // End protocol after 10 iterations
                            None
                        }
                    })
                },
                Message::Terminate => {
                    // End protocol early
                    None
                }
            }
        }
    );
    
    // Start the protocol with initial state
    protocol(channel, ProtocolState { count: 0, accumulated: Vec::new() });
}
```

#### Recursion Through Closure Captures

```rust
// Recursion through closure captures
fn recursive_protocol(channel: Channel) {
    // Create a box for self-referential closure
    let protocol_box: Box<Cell<Option<Box<dyn FnMut(Channel)>>>> = 
        Box::new(Cell::new(None));
    
    // Create the recursive closure
    let mut protocol_fn = {
        let protocol_box = protocol_box.clone();
        
        move |channel: Channel| {
            // Receive command
            channel.receive(|command, channel| {
                match command {
                    Command::Process(data) => {
                        // Process the data
                        let result = process_data(data);
                        
                        // Send the result
                        channel.send(result, |channel| {
                            // Continue the protocol recursively
                            if let Some(ref mut protocol) = protocol_box.get() {
                                protocol(channel);
                            }
                        });
                    },
                    Command::Terminate => {
                        // End the protocol
                        println!("Protocol terminated");
                    }
                }
            });
        }
    };
    
    // Store the closure in the box
    protocol_box.set(Some(Box::new(protocol_fn.clone())));
    
    // Start the protocol
    protocol_fn(channel);
}
```

#### Explicit State Machine with Recursion

```rust
// Explicit state machine with recursion
enum ProtocolState {
    AwaitingCommand,
    ProcessingData { data: Data },
    Terminated,
}

// State transitions as continuations
fn run_state_machine(channel: Channel, initial_state: ProtocolState) {
    // Create state transition function
    let transition = move |state: ProtocolState, channel: Channel| {
        match state {
            ProtocolState::AwaitingCommand => {
                // Await command
                channel.receive(|command, channel| {
                    match command {
                        Command::Process(data) => {
                            // Transition to processing state
                            transition(ProtocolState::ProcessingData { data }, channel)
                        },
                        Command::Terminate => {
                            // Transition to terminated state
                            transition(ProtocolState::Terminated, channel)
                        }
                    }
                })
            },
            ProtocolState::ProcessingData { data } => {
                // Process data
                let result = process_data(data);
                
                // Send result
                channel.send(result, |channel| {
                    // Transition back to awaiting command
                    transition(ProtocolState::AwaitingCommand, channel)
                })
            },
            ProtocolState::Terminated => {
                // Protocol completed
                println!("Protocol terminated");
            }
        }
    };
    
    // Start the state machine
    transition(initial_state, channel);
}

// Usage example
fn run_protocol(channel: Channel) {
    run_state_machine(channel, ProtocolState::AwaitingCommand);
}
```

#### Recursion with External Control

```rust
// Recursion with external control flags
fn controlled_recursive_protocol<C>(
    channel: Channel,
    mut controller: C
) where
    C: FnMut() -> ControlSignal,
{
    // Recursive helper function
    fn protocol_step<C>(
        channel: Channel,
        controller: &mut C
    ) where
        C: FnMut() -> ControlSignal,
    {
        // Check if should continue
        match controller() {
            ControlSignal::Continue => {
                // Receive data
                channel.receive(|data, channel| {
                    // Process data
                    let result = process_data(data);
                    
                    // Send result
                    channel.send(result, |channel| {
                        // Continue recursively
                        protocol_step(channel, controller);
                    });
                });
            },
            ControlSignal::Terminate => {
                // End recursion
                println!("Protocol terminated externally");
            }
        }
    }
    
    // Start recursive protocol
    protocol_step(channel, &mut controller);
}

// Usage example with timeout
fn run_timed_protocol(channel: Channel) {
    // Create timeout
    let start_time = Instant::now();
    let timeout = Duration::from_secs(60);
    
    // Controller function
    let controller = || {
        if start_time.elapsed() < timeout {
            ControlSignal::Continue
        } else {
            ControlSignal::Terminate
        }
    };
    
    // Run protocol with timeout
    controlled_recursive_protocol(channel, controller);
}
```

#### Recursion with Asynchronous Continuations

```rust
// Recursive protocol with async continuations
async fn async_recursive_protocol(channel: Channel) {
    // Recursive helper function
    async fn protocol_loop(channel: Channel, count: usize) {
        if count >= 10 {
            // Base case: end recursion
            return;
        }
        
        // Receive data
        let (data, channel) = channel.receive_async(|data, channel| {
            (data, channel)
        }).await;
        
        // Process data (could be async)
        let result = process_data_async(data).await;
        
        // Send result
        let channel = channel.send_async(result, |channel| {
            channel
        }).await;
        
        // Recursive call with updated count
        protocol_loop(channel, count + 1).await;
    }
    
    // Start recursive protocol with count 0
    protocol_loop(channel, 0).await;
}
```

#### Key Benefits of This Approach

1. **Natural Expression of Recursion**: Higher-order functions make recursion patterns explicit and clear
2. **State Management**: Can maintain state across recursive calls through closure captures or explicit state parameters
3. **Termination Control**: Provides multiple mechanisms for controlling recursion termination
4. **Compositional**: Recursive protocols can be composed with other combinators like choice and parallel
5. **Flexible Control Flow**: Can implement both bounded and unbounded recursion patterns

The Continuation-Passing Style approach handles recursion through higher-order functions that can reference themselves, creating looping behavior through repeated continuations. This provides a natural expression of recursive protocols while maintaining explicit control over the recursion structure.

## Implementation Pattern Comparison

This section provides a comparative analysis of the three implementation patterns presented in this document to help you choose the most appropriate approach for your specific use case.

### Type Safety and Protocol Enforcement Analysis

| Approach | Type Safety | Compile-Time Checks | Runtime Checks |
|----------|-------------|---------------------|----------------|
| **Typed Channel Wrappers** | High | Extensive | Minimal |
| **Code Generation** | High | Moderate to High | Low to Moderate |
| **State Machine Builders** | Very High | Extensive | Minimal |
| **Actor-Based Runtime** | Moderate | Limited | Extensive |
| **Continuation-Passing Style** | Good | Moderate | Moderate |

- **Typed Channel Wrappers** provide strong type safety by encoding the protocol state in generic type parameters. This approach relies heavily on Rust's type system to prevent protocol violations at compile time.
- **Code Generation** can provide strong type safety, but it depends on the quality of the generated code. Some runtime checks may be required when the protocol structure cannot be fully expressed in the generated types.
- **State Machine Builders** often provide the strongest type safety by making each protocol state a distinct type, allowing the compiler to verify all transitions.
- **Actor-Based Runtime** provides moderate type safety through message typing but relies more on runtime protocol enforcement. Protocol violations are detected when actors receive unexpected message types.
- **Continuation-Passing Style** offers good type safety through function signatures and callback types, balancing compile-time and runtime protocol checking. Violations are caught when continuation functions receive incorrect types.

### Code Complexity and Verbosity Comparison

| Approach | Initial Setup | Protocol Definition | Protocol Usage |
|----------|---------------|---------------------|----------------|
| **Typed Channel Wrappers** | Moderate | Verbose | Moderate |
| **Code Generation** | High | Concise | Simple |
| **State Machine Builders** | Low | Moderate | Explicit |
| **Actor-Based Runtime** | Low to Moderate | Explicit | Simple |
| **Continuation-Passing Style** | Low | Explicit | Flexible |

- **Typed Channel Wrappers** require moderate initial setup to define the type-level representations but result in verbose protocol definitions due to explicit type annotations.
- **Code Generation** has high initial setup cost to create the macros but leads to very concise protocol definitions and simple usage patterns.
- **State Machine Builders** have low initial setup but create more visible protocol machinery in the code, making the protocol structure explicitly visible but potentially verbose.
- **Actor-Based Runtime** requires moderate initial setup to define actor types but provides straightforward protocol representation through message passing. Protocol steps are defined explicitly, and usage is simple through actor messaging.
- **Continuation-Passing Style** has low initial setup complexity and offers explicit protocol definition through callback chains. Usage is flexible as it adapts to different programming styles and control flow patterns.

### Business Logic Integration Analysis

| Approach | Integration Pattern | Flexibility | Testability |
|----------|---------------------|------------|------------|
| **Typed Channel Wrappers** | Interleaved | High | Good |
| **Code Generation** | Callbacks/Handlers | Moderate | Very Good |
| **State Machine Builders** | State-Driven | High | Excellent |
| **Actor-Based Runtime** | Message Handlers | High | Very Good |
| **Continuation-Passing Style** | Callback-Oriented | High | Good |

- **Typed Channel Wrappers** allow business logic to be freely interleaved with protocol operations, giving developers flexibility in structuring their code.
- **Code Generation** often uses callback or handler patterns that separate business logic from protocol mechanics, which can make testing easier but may constrain code organization.
- **State Machine Builders** make protocol states explicit, allowing business logic to be organized around state transitions, which can improve testability and make the code more maintainable.
- **Actor-Based Runtime** encapsulates business logic in message handlers, separating protocol structure from application code. This promotes high flexibility in implementation while maintaining a clear boundary between protocol and business concerns, enabling good testability through actor isolation.
- **Continuation-Passing Style** structures business logic as callbacks, allowing for flexible composition of protocol steps. This approach enables a natural flow of business logic within the continuation chain while maintaining separation of concerns, though deep callback chains can sometimes make testing more complex.

### Developer Experience Evaluation

| Approach | Learning Curve | IDE Support | Debuggability |
|----------|---------------|------------|--------------|
| **Typed Channel Wrappers** | Steep | Moderate | Moderate |
| **Code Generation** | Moderate | Limited to Good | Challenging |
| **State Machine Builders** | Moderate | Excellent | Good |
| **Actor-Based Runtime** | Moderate | Good | Excellent |
| **Continuation-Passing Style** | Moderate | Good | Challenging |

- **Typed Channel Wrappers** have a steep learning curve due to complex type-level programming but provide good IDE support once the pattern is understood.
- **Code Generation** has a moderate learning curve but may provide limited IDE support due to the indirection introduced by macros, making debugging more challenging.
- **State Machine Builders** have a moderate learning curve and typically provide excellent IDE support through autocomplete for state-specific methods, improving discoverability and making the code easier to understand.
- **Actor-Based Runtime** has a moderate learning curve as it requires understanding actor model concepts, but provides good IDE support for message types and handlers. The actor model excels in debuggability by making message flows explicit and providing supervision hierarchies for error tracking.
- **Continuation-Passing Style** has a moderate learning curve requiring familiarity with higher-order functions and callbacks. It provides good IDE support for function signatures but can be challenging to debug due to the indirect control flow through continuation chains, especially with deeply nested callbacks.

### Project Scale Considerations Matrix

| Approach | Small Projects | Medium Projects | Large Projects | Team Collaboration |
|----------|---------------|----------------|---------------|-------------------|
| **Typed Channel Wrappers** | Good | Good | Moderate | Challenging |
| **Code Generation** | Overkill | Good | Excellent | Good |
| **State Machine Builders** | Excellent | Good | Good | Good |
| **Actor-Based Runtime** | Good | Excellent | Excellent | Good |
| **Continuation-Passing Style** | Excellent | Good | Moderate | Moderate |

- **Typed Channel Wrappers** work well for small to medium projects but may become unwieldy in larger systems with complex protocols due to the verbosity of type annotations.
- **Code Generation** is often overkill for small projects but scales very well to large systems by automating repetitive protocol code, making it maintainable for large teams.
- **State Machine Builders** are excellent for small projects due to their simplicity and work well for medium to large systems when protocol visualization is important.
- **Actor-Based Runtime** works well for small projects but truly shines in medium to large systems where its built-in concurrency model and supervision hierarchies can effectively manage complex distributed protocols. The actor model's familiar patterns make it suitable for team collaboration.
- **Continuation-Passing Style** excels in small projects due to its flexibility and minimal setup, but may become challenging to maintain in larger systems due to deeply nested callback chains. The higher-order function approach can present a steeper learning curve for team collaboration.

### Selection Guidance for Implementation Patterns

#### When to Select Typed Channel Wrappers

1. You need maximum control over protocol implementation details
2. Your team is comfortable with advanced Rust type-level programming
3. You want to freely interleave business logic with protocol operations
4. You want to avoid build-time dependencies on procedural macros
5. Protocol structure changes are infrequent

#### When to Select Code Generation

1. You have complex protocols with many repetitive patterns
2. You want to minimize boilerplate code
3. Your protocol definitions are stable and well-defined
4. You need to support multiple different protocol syntaxes or DSLs
5. You have a large codebase or team where consistency is crucial

#### When to Select State Machine Builders

1. Protocol state visualization is important
2. You want maximum IDE support and discoverability
3. You prefer explicit state transitions over implicit protocol flow
4. You're building a system where protocol states have significant business meaning
5. You need to integrate with existing state machine frameworks

#### When to Select Actor-Based Runtime

1. You need a naturally concurrent implementation of distributed protocols
2. Your system has complex supervision and error recovery requirements
3. You prefer message-passing semantics that align closely with session types
4. You want strong isolation between different protocol roles or participants
5. You need a scalable approach for systems with many parallel protocol instances

#### When to Select Continuation-Passing Style

1. You want flexible composition of protocol steps without complex type machinery
2. Your team is comfortable with functional programming patterns
3. You need to integrate with callback-heavy APIs or asynchronous code
4. You prefer explicit control flow through continuations over implicit state transitions
5. You want to minimize upfront infrastructure code while maintaining protocol clarity

### Combined Implementation Strategies

In practice, many projects benefit from combining elements of multiple approaches. For example:

1. **Generated State Machines**: Using code generation to create state machine builders, combining the conciseness of macros with the explicitness of state machines.
2. **Wrapped Codegen**: Using typed wrappers around generated code to provide additional type safety while reducing boilerplate.
3. **Domain-Specific Abstractions**: Creating higher-level abstractions tailored to your specific domain that use one of these patterns internally.

The key is to select the approach that best aligns with your project's requirements, your team's expertise, and the level of type safety and expressiveness you need.

## Implementation Patterns Summary

Implementing session types in Rust requires balancing type safety, code complexity, developer experience, and business logic integration. The three approaches presented in this document—Typed Channel Wrappers, Code Generation with Procedural Macros, and State Machine Builders—each offer different trade-offs that make them suitable for different scenarios.

By understanding the strengths and weaknesses of each approach, you can select the implementation pattern that best suits your specific needs and constraints, or combine elements from multiple approaches to create a hybrid solution tailored to your project.