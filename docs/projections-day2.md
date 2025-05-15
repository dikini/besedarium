# Session Type Projections - Day 2 Progress

## Current Status

We are implementing projections from global multiparty session types to local endpoint types. The
key breakthrough was realizing that our existing implementation already models global multiparty
session types through:

- `TInteract<IO, R, H, T>` carrying role information in `R`
- `TPar` enforcing role disjointness
- Examples like `Workflow` and `MqttPubSub` demonstrating multi-role interactions

## Current Approach

Our current implementation uses a type-level map to project global types into an n-dimensional
structure:

- We maintain a type-level list of all roles: `AllRoles`

```rust
type AllRoles = Cons<TClient, Cons<TServer, Cons<TBroker, Cons<TWorker, Nil>>>>;
```

- For each role, we maintain a list to collect their endpoint types: `EmptyLists`

```rust
type EmptyLists = Cons<Nil, Cons<Nil, Cons<Nil, Cons<Nil, Nil>>>>;
```

- We use type-level natural numbers (Z, SN) to track positions in these lists:

```rust
pub struct Z;  // Zero position
pub struct S<N>(PhantomData<N>);  // Successor
```

- The core projection machinery:
  - `Project<IO>` trait maps over the session graph
  - `PositionR` trait finds a role's position in AllRoles
  - `UpdateAt<Pos, Item>` trait updates lists at specific positions

- The current implementation can project `TEnd` and simple `TInteract` nodes, producing local
endpoint types:
  - `EpEnd<IO>` for protocol termination
  - `EpSend<IO, Msg, Cont>` for sending
  - `EpRecv<IO, Msg, Cont>` for receiving

## Implementation Challenges

1. **Current Issue**: Our projection only handles the sender side of `TInteract`. When projecting
`TInteract<IO, R1, Msg, Cont>`, we need to:
  - Add `EpSend` to R1's list (sender)
  - Add `EpRecv` to the receiving role's list
  - Leave other role lists unchanged

2. **Complex Structure**: The n-dimensional output structure makes it tricky to update multiple
positions for a single `TInteract` node.

## Ideas for Moving Forward

### Approach 1: Two-Phase Projection

- First pass: Build a type-level map of roles to their communications
- Second pass: Convert the communication map into local endpoint types

This could make it easier to handle both sides of each interaction.

### Approach 2: Communication-Centric View

Instead of focusing on roles first, we could:

- Treat each `TInteract` as a communication event
- Project the event to both sender and receiver simultaneously
- Use type-level machinery to merge these projections into role-specific views

### Approach 3: Revised Structure

We could simplify by:

- Making the n-dimensional structure more explicit:

```rust
pub struct ProjectionLists<ClientList, ServerList, BrokerList, WorkerList>;
```

- Using type families to update multiple lists at once:

```rust
pub type ProjectComm<IO, Sender, Receiver, Msg, Lists> = ...;
```

### Questions to Consider

1. Should we maintain the current approach with position-based updates, or switch to a more direct
structure?

2. How do we best handle the receiving side of interactions? Options:
  - Add another position-based update in the current implementation
  - Switch to a two-phase approach
  - Use a different structure entirely

3. What's the cleanest way to compose these projections when we add other combinators (TChoice,
TPar, TRec)?

## Next Steps

1. Decide on whether to continue with current approach or switch to a different structure

2. Implement proper bi-directional projection for TInteract:
  - Handle both sender and receiver
  - Maintain correct ordering of communications
  - Preserve protocol structure

3. Add more compile-time assertions to verify projections:
  - Test multiple interactions
  - Verify both send and receive endpoints
  - Check ordering preservation

4. Plan approach for other combinators:
  - How TChoice affects projection
  - How TPar's disjointness carries through
  - How to handle TRec's recursive structure

## Implementation Notes

The current implementation uses type-level position tracking and list updates. Example projection:

```rust
type SimpleProtocol = TInteract<IO, TClient, Message, TEnd<IO>>;

// Projects to:
type Projection = Cons<
  Cons<EpSend<IO, Message, EpEnd<IO>>, Nil>,  // Client's list
  Cons<Nil,                                    // Server's list
    Cons<Nil,                                // Broker's list
      Cons<Nil, Nil>                       // Worker's list
    >
  >
>;
```

We need to extend this to handle the receiving side and maintain correct communication structure
through the projection.

## Additional Insights

### Type-Level Programming Patterns

1. **Natural Transformation Pattern**
  - Our projection can be viewed as a natural transformation between two type-level functors:
    - Source: The global protocol structure (TInteract, TEnd, etc.)
    - Target: The n-dimensional local view structure
  - This suggests we might benefit from making the functor structure more explicit

2. **Zipper Pattern**
  - Instead of treating our protocol as just a recursive structure, we could use a type-level
  zipper
  - This would give us a "focus point" in the protocol, making it easier to:
    - Track the current sender/receiver context
    - Maintain proper causality in projections
    - Handle recursive structures later

3. **Phase Distinction**
  - We could separate our projection into distinct phases:
    - Structure traversal (mapping over the protocol graph)
    - Role resolution (determining sender/receiver relationships)
    - Local type construction (building endpoint types)
  - This separation might make the code clearer and more maintainable

### Alternative Implementation Ideas

1. **Role Sets as Type-Level Sets**

  ```rust
  // Instead of position-based lookups:
  pub struct RoleSet<Roles>(PhantomData<Roles>);
  pub trait Contains<R: Role> { type HasRole: Bool; }
  ```

  This could make role membership checks more direct than position-based lookups.

2. **Communication Events as Primary Structure**

  ```rust
  // Model each interaction as a communication event
  pub struct CommEvent<Sender, Receiver, Msg>;

  // Project protocol into sequence of events
  type Events = Cons<
    CommEvent<TClient, TServer, Message>,
    Cons<CommEvent<TServer, TClient, Response>, Nil>
  >;
  ```

  This makes the communication structure more explicit and could simplify projection.

3. **Type-Level State Machines**
  - Model each role's behavior as a type-level state machine
  - Use type-level transitions to ensure protocol consistency
  - This could help with handling TPar and TChoice later

### Deeper Insights on Current Approach

1. **Position Tracking Trade-offs**
  - Pros:
    - Clear mapping between roles and their projections
    - Easier to maintain role-projection correspondence
    - Natural way to handle multiple roles
  - Cons:
    - Complex type-level machinery for position updates
    - Harder to handle bi-directional communication
    - May get unwieldy with TPar and TChoice

2. **List Structure Considerations**
  - Current approach builds a list of lists
  - Each inner list represents one role's view
  - Alternative: We could use a more structured type:

  ```rust
  pub struct ProjectionViews<IO> {
    client: EpView<IO, TClient>,
    server: EpView<IO, TServer>,
    broker: EpView<IO, TBroker>,
    worker: EpView<IO, TWorker>,
  }
  ```

3. **Composition and Recursion**
  - Our current position-based approach might make it harder to handle:
    - Recursive protocols (TRec)
    - Parallel composition (TPar)
    - Choice composition (TChoice)
  - We might need a different structure for these cases

### Mathematical Observations

1. **Category Theory Connection**
  - Global protocols form a category:
    - Objects are protocol states
    - Morphisms are protocol transitions
  - Projection is a functor from this category to the category of local types
  - This suggests we should preserve certain structural properties

2. **Order Theory**
  - The communication events in our protocol form a partial order
  - Projection should preserve this ordering
  - Current position-based approach makes this harder to verify

3. **Type-Level Algebra**
  - We're effectively doing type-level algebraic manipulation
  - Could benefit from more algebraic structure in our types
  - Might help simplify complex projections

## Next Research Directions

1. **Algebraic Effects View**
  - Model protocol actions as type-level effects
  - Use effect handlers for projection
  - This could give us a cleaner separation of concerns

2. **Session Type Combinators**
  - Build higher-level combinators for common patterns
  - Make projection compositional
  - Could simplify complex protocol definitions

3. **Dependent Types Simulation**
  - Use more advanced type-level programming
  - Encode more properties at the type level
  - Better static guarantees for projections

These ideas suggest several possible directions for improving our implementation while maintaining
its mathematical foundations and correctness properties.
