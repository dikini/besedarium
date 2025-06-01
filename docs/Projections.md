# Protocol Projections in Besedarium

Protocol projections are a fundamental concept in session type theory that enable the extraction of local behavior for individual participants from global protocol descriptions. In Besedarium, projections are implemented through the `Project<P, Role>` trait, which provides type-level computation to derive what each role should do in a protocol.

## Table of Contents

- [Overview](#overview)
- [The Project Trait](#the-project-trait)
- [Projection Rules](#projection-rules)
- [Implementation Details](#implementation-details)
- [Examples](#examples)
- [Debugging Projections](#debugging-projections)
- [Advanced Patterns](#advanced-patterns)
- [Integration with Other Components](#integration-with-other-components)

## Overview

When designing distributed systems with session types, we typically start with a **global protocol** that describes the overall communication pattern between all participants. However, each participant needs to know only their own local behavior - what messages they should send, receive, and when.

**Projection** is the process of extracting this local view from the global protocol for a specific role.

```rust
// Global protocol: Alice sends data to Bob, Bob responds with confirmation
type ExampleProtocol = Send<Alice, Bob, String, Recv<Bob, Alice, bool, End>>;

// Local projection for Alice: Send string, then receive bool
type AliceLocal = Send<Bob, String, Recv<Bob, bool, End>>;

// Local projection for Bob: Receive string, then send bool  
type BobLocal = Recv<Alice, String, Send<Alice, bool, End>>;
```

## The Project Trait

The core of Besedarium's projection system is the `Project` trait:

```rust
pub trait Project<P, Role> {
    type Output;
}
```

This trait computes the projection of protocol `P` for a given `Role` at compile time, producing the local protocol as `Output`.

### Usage

```rust
use besedarium::protocol::projection::Project;

// Project a protocol for a specific role
type AliceView = <ExampleProtocol as Project<ExampleProtocol, Alice>>::Output;
type BobView = <ExampleProtocol as Project<ExampleProtocol, Bob>>::Output;
```

### Design Principles

1. **Type-Level Computation**: All projection logic happens at compile time
2. **Role-Specific Views**: Each role gets exactly what they need to know
3. **Safety Guarantees**: Invalid projections are caught at compile time
4. **Compositionality**: Projections work recursively through nested protocols

## Projection Rules

The projection system implements the formal rules from session type theory:

### 1. Send/Receive Messages

**Rule**: If a message is sent from role `R1` to role `R2`:

- For `R1`: Project to a `Send` operation
- For `R2`: Project to a `Recv` operation  
- For other roles: Project the continuation (skip this communication)

```rust
// Global: Send<Alice, Bob, String, Continuation>
// Alice sees: Send<Bob, String, ProjectedContinuation>
// Bob sees: Recv<Alice, String, ProjectedContinuation>
// Charlie sees: ProjectedContinuation (skips this step)
```

### 2. Choice/Branch Operations

**Rule**: For `Choice<Role, Branches>`:

- For the choosing `Role`: Project to a `Choose` operation
- For other roles: Project to a `Branch` operation with all possible branches

```rust
// Global: Choice<Alice, (Label1, Protocol1), (Label2, Protocol2)>
// Alice sees: Choose<(Label1, ProjectedProtocol1), (Label2, ProjectedProtocol2)>
// Others see: Branch<Alice, (Label1, ProjectedProtocol1), (Label2, ProjectedProtocol2)>
```

### 3. Parallel Composition

**Rule**: For `Parallel<Protocol1, Protocol2>`:

- Project both protocols independently
- Combine results with `Parallel<ProjectedP1, ProjectedP2>`

### 4. Recursion

**Rule**: For `Rec<Protocol>` and `Var<N>`:

- `Rec<P>` projects to `Rec<ProjectedP>`
- `Var<N>` projects to `Var<N>` (variable references are preserved)

### 5. End Protocol

**Rule**: `End` always projects to `End` for all roles.

## Implementation Details

The projection system uses several helper traits to handle complex cases:

### ProjectHelper Trait

```rust
pub trait ProjectHelper<P, Role> {
    type Output;
}
```

This trait provides specialized implementations for different protocol types, enabling the main `Project` trait to delegate to appropriate handlers.

### Role-Specific Projection Logic

The system distinguishes between:

- **Sending roles**: Get `Send` operations
- **Receiving roles**: Get `Recv` operations  
- **Observing roles**: Skip irrelevant communications

### Handling Complex Protocols

For nested protocols like `Choice` and `Parallel`, the system:

1. Analyzes the structure recursively
2. Applies projection rules at each level
3. Combines results according to the protocol semantics

## Examples

### Basic Communication

```rust
use besedarium::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Alice;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]  
struct Bob;

// Global: Alice sends greeting to Bob
type GreetingProtocol = Send<Alice, Bob, String, End>;

// Projections
type AliceLocal = <GreetingProtocol as Project<GreetingProtocol, Alice>>::Output;
// Result: Send<Bob, String, End>

type BobLocal = <GreetingProtocol as Project<GreetingProtocol, Bob>>::Output;
// Result: Recv<Alice, String, End>
```

### Choice Protocol

```rust
use besedarium::prelude::*;

// Global: Alice chooses between greeting or farewell
type ChoiceProtocol = Choice<Alice, (
    ("greeting", Send<Alice, Bob, String, End>),
    ("farewell", Send<Alice, Bob, String, End>)
)>;

// Alice's view: Make a choice
type AliceChoice = <ChoiceProtocol as Project<ChoiceProtocol, Alice>>::Output;
// Result: Choose<(("greeting", Send<Bob, String, End>), ("farewell", Send<Bob, String, End>))>

// Bob's view: Handle either branch
type BobBranch = <ChoiceProtocol as Project<ChoiceProtocol, Bob>>::Output;  
// Result: Branch<Alice, (("greeting", Recv<Alice, String, End>), ("farewell", Recv<Alice, String, End>))>
```

### Multi-Party Protocol

```rust
// Three-party protocol: Alice coordinates between Bob and Charlie
type CoordinationProtocol = Send<Alice, Bob, String,
                           Send<Alice, Charlie, String,
                           Recv<Bob, Alice, bool,
                           Recv<Charlie, Alice, bool, End>>>>;

// Each participant gets their specific view:
type AliceCoordinator = <CoordinationProtocol as Project<CoordinationProtocol, Alice>>::Output;
// Result: Send<Bob, String, Send<Charlie, String, Recv<Bob, bool, Recv<Charlie, bool, End>>>>

type BobParticipant = <CoordinationProtocol as Project<CoordinationProtocol, Bob>>::Output;
// Result: Recv<Alice, String, Send<Alice, bool, End>>

type CharlieParticipant = <CoordinationProtocol as Project<CoordinationProtocol, Charlie>>::Output;
// Result: Recv<Alice, String, Send<Alice, bool, End>>
```

## Debugging Projections

### Common Issues

1. **Infinite Recursion**: Occurs with malformed recursive protocols

   ```rust
   // Problematic: Direct self-reference without progress
   type BadRec = Rec<Var<0>>; // This will cause compilation issues
   ```

2. **Role Mismatches**: Using undefined roles in projections

   ```rust
   // Error: UnknownRole is not defined in the protocol
   type BadProjection = <GreetingProtocol as Project<GreetingProtocol, UnknownRole>>::Output;
   ```

3. **Complex Type Errors**: Deep nesting can lead to confusing error messages

### Debugging Strategies

#### 1. Use Type Assertions

```rust
use besedarium::protocol::testing::assert_type_eq;

// Verify projection results match expectations
assert_type_eq::<
    <GreetingProtocol as Project<GreetingProtocol, Alice>>::Output,
    Send<Bob, String, End>
>();
```

#### 2. Build Incrementally

Start with simple protocols and add complexity gradually:

```rust
// Step 1: Simple message
type Step1 = Send<Alice, Bob, String, End>;

// Step 2: Add response  
type Step2 = Send<Alice, Bob, String, Recv<Bob, Alice, bool, End>>;

// Step 3: Add choice
type Step3 = Choice<Alice, (("option1", Step2), ("option2", End))>;
```

#### 3. Use Compiler Diagnostics

Enable helpful compiler flags for better error messages:

```bash
cargo check --verbose
RUST_BACKTRACE=1 cargo check
```

## Advanced Patterns

### 1. Conditional Projections

Some protocols may require different projections based on role relationships:

```rust
// Protocol where behavior depends on role hierarchy
type ConditionalProtocol<Leader, Follower> = 
    Send<Leader, Follower, Command, 
    Recv<Follower, Leader, Response, End>>;
```

### 2. Dynamic Role Assignment

For protocols with variable role participation:

```rust
// Protocol supporting different numbers of participants
type ScalableProtocol<Roles> = 
    Parallel<
        Broadcast<Coordinator, Roles, Message>,
        Gather<Roles, Coordinator, Response>
    >;
```

### 3. Protocol Composition

Combining multiple projected protocols:

```rust
// Sequential composition of projected protocols
type ComposedLocal<P1, P2, Role> = 
    Sequence<
        <P1 as Project<P1, Role>>::Output,
        <P2 as Project<P2, Role>>::Output
    >;
```

## Integration with Other Components

### Runtime System

Projected protocols integrate seamlessly with Besedarium's runtime:

```rust
use besedarium::runtime::{Session, SessionManager};

// Create session using projected protocol
let alice_session = Session::<AliceLocal>::new();
let manager = SessionManager::new();
```

### Dual Generation

Projections work with automatically generated dual protocols:

```rust
use besedarium::protocol::duality::Dual;

// Get dual of projected protocol
type AliceDual = <AliceLocal as Dual>::Output;
```

### Diagram Generation

Projected protocols can be visualized independently:

```rust
use besedarium::diagrams::ProtocolFlow;

// Generate diagram for Alice's local view
let alice_diagram = AliceLocal::generate_diagram();
```

## Conclusion

Protocol projections are essential for translating global communication patterns into local participant behavior. Besedarium's type-level projection system ensures that:

- **Correctness**: Invalid projections are caught at compile time
- **Efficiency**: All computation happens during compilation
- **Compositionality**: Complex protocols project correctly through recursive rules
- **Integration**: Projected protocols work seamlessly with all library components

Understanding projections enables you to design robust distributed systems where each participant has a clear, type-safe view of their responsibilities in the overall communication protocol.

For more information, see:

- [Duality Documentation](./duality.md) - Understanding protocol duality
- [Recursion Documentation](./recursion.md) - Working with recursive protocols
- [Protocol Examples](./protocol-examples.md) - Complete protocol implementations
