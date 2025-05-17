# Implementation Overview

## Besedarium: Session Types in Rust — Implementation Overview and Analysis

### Introduction & Goals

Besedarium is a Rust library for building, composing, and verifying communication protocols at the
type level using session types. The primary goal is to catch protocol mistakes at compile time,
ensuring that distributed systems and networked applications follow the correct message flow. The
library aims for a balance between strong static guarantees, ergonomic protocol construction, and
extensibility for multiparty and advanced session type features.

### What Are Session Types?

Session types are a type-theoretic framework for describing and verifying structured communication
between concurrent or distributed processes. They allow developers to specify the sequence and
structure of messages exchanged between roles (participants) in a protocol, ensuring properties
like linearity (no double use of channels), progress (no deadlocks), and protocol fidelity (no
unexpected messages).

In practice, session types provide a way to encode communication protocols as types, so that the
compiler can check for protocol violations. This is especially valuable in distributed systems,
where mismatches in message order or structure can lead to subtle, hard-to-debug errors. Session
types originated in the study of process calculi and have been adopted in several programming
languages, including Haskell, Scala, and Rust, to provide strong compile-time guarantees for
communication safety.

### High-Level Architecture

Besedarium models protocols as global types (describing the whole protocol) and provides machinery
to project these to local (endpoint) types for each role. The core is a set of combinators
(type-level building blocks) and traits for composing, analyzing, and projecting protocols. Macros
are used for ergonomic construction of n-ary choices and parallel branches. Compile-time assertions
and trybuild tests ensure that safety properties are enforced and violations are caught early.

---

## Global Session Type Combinators

### 1. `TInteract`

- **Purpose:** Models a single interaction (send/receive) by a role in a protocol.
- **Implementation:**

  ```rust
  pub struct TInteract<IO, Lbl: ProtocolLabel, R, H, T: TSession<IO>>(
      PhantomData<(IO, Lbl, R, H, T)>,
  );
  ```

  - `IO`: Protocol marker (e.g., Http, Mqtt)
  - `Lbl`: Label for this interaction (for projection and debugging)
  - `R`: Role performing the action (sender or receiver)
  - `H`: Message type being sent or received
  - `T`: Continuation protocol after this interaction
- **Pros:**
  - Simple, compositional building block for protocols.
  - Encodes both the actor and the message at the type level.
  - Labeling supports protocol projection and debugging.
- **Cons:**
  - Requires explicit role, label, and message types for each step.
- **Properties Ensured:**
  - Linearity (each step is explicit and unique in the protocol).
  - Type safety for message and role.
- **Example:**

  ```rust
  type Handshake = TInteract<Http, HandshakeLabel, TClient, Message,
      TInteract<Http, ResponseLabel, TServer, Response, TEnd<Http>>>;
  // Projects to local types for each role using the projection machinery (see below).
  ```

  - **Diagram:**

  ```mermaid
  sequenceDiagram
      participant Client
      participant Server
      Client->>Server: Message
      Server-->>Client: Response
  ```

### 2. `TChoice`

- **Purpose:** Models a branching point in the protocol (choice between alternatives).
- **Implementation:**

  ```rust
  pub struct TChoice<IO, Lbl: ProtocolLabel, L: TSession<IO>, R: TSession<IO>>(
      PhantomData<(IO, Lbl, L, R)>,
  );
  ```

  - `IO`: Protocol marker type
  - `Lbl`: Label for this choice (for projection and debugging)
  - `L`, `R`: The two protocol branches
- **Pros:**
  - Expressive for modeling protocol alternatives.
  - N-ary choices supported via macros and type-level lists.
  - Labeling supports protocol projection and debugging.
- **Cons:**
  - Manual construction of deeply nested choices can be verbose (mitigated by macros).
- **Properties Ensured:**
  - Exhaustiveness (all branches are explicit).
- **Example:**

  ```rust
  type Choice = TChoice<Http, ChoiceLabel,
      TInteract<Http, L1, TClient, Message, TEnd<Http>>,
      TInteract<Http, L2, TServer, Response, TEnd<Http>>
  >;
  // Each branch is projected to the local type for each role.
  ```

  - **Diagram:**

  ```mermaid
  flowchart TD
      Start((Start))
      A[Client: Message]
      B[Server: Response]
      Start -->|choose| A
      Start -->|choose| B
      A --> End1((End))
      B --> End2((End))
  ```

### 3. `TPar`

- **Purpose:** Models parallel (concurrent) composition of protocol branches.
- **Implementation:**

  ```rust
  pub struct TPar<IO, Lbl: ProtocolLabel, L: TSession<IO>, R: TSession<IO>, IsDisjoint>(
      PhantomData<(IO, Lbl, L, R, IsDisjoint)>,
  );
  ```

  - `IO`: Protocol marker type
  - `Lbl`: Label for this parallel composition
  - `L`, `R`: The two protocol branches to run in parallel
  - `IsDisjoint`: Type-level boolean indicating if branches are disjoint
- **Pros:**
  - Enables modeling of concurrent or independent protocol flows.
  - Disjointness is enforced at compile time via traits and macros.
  - Labeling supports protocol projection and debugging.
- **Cons:**
  - Requires explicit disjointness checks (assert_disjoint!).
  - N-ary parallel composition can be verbose without macros.
- **Properties Ensured:**
  - Disjointness of roles (no role appears in more than one branch).
  - Linearity and progress for parallel branches.
- **Example:**

  ```rust
  type Workflow = TPar<Http, ParLabel,
      TInteract<Http, L1, TClient, Message, TInteract<Http, L2, TServer, Response, TEnd<Http>>>,
      TInteract<Http, L3, TBroker, Publish, TEnd<Http>>,
      TrueB // or FalseB, depending on disjointness check
  >;
  // Each branch is projected independently to local types for each role.
  ```

  - **Diagram:**

  ```mermaid
  flowchart TD
      subgraph Parallel
          direction LR
          A[Client: Message] --> End1((End))
          B[Broker: Publish] --> End2((End))
          C[Worker: Notify] --> End3((End))
      end
  ```

### 4. `TRec`

- **Purpose:** Models recursion (repetition) in protocols.
- **Implementation:**

  ```rust
  pub struct TRec<IO, Lbl: ProtocolLabel, S: TSession<IO>>(
      PhantomData<(IO, Lbl, S)>,
  );
  ```

  - `IO`: Protocol marker type
  - `Lbl`: Label for this recursion (for projection and debugging)
  - `S`: The protocol fragment to repeat (may refer to itself)
- **Pros:**
  - Enables modeling of streaming, loops, or repeated protocol fragments.
  - Labeling supports protocol projection and debugging.
- **Cons:**
  - No mutual recursion (only single recursion supported).
- **Properties Ensured:**
  - Well-formedness of recursive protocols.
- **Example:**

  ```rust
  type Streaming = TRec<Http, StreamLabel, TInteract<Http, L1, TClient, Message, TEnd<Http>>>;
  // Recursion is preserved in the local projection.
  ```

  - **Diagram:**

  ```mermaid
  flowchart TD
      Start((Start))
      A[Client: Message]
      Start --> A
      A --> Start
  ```

### 5. `TEnd`

- **Purpose:** Marks the end of a protocol branch.
- **Implementation:**

  ```rust
  pub struct TEnd<IO, Lbl = EmptyLabel>(PhantomData<(IO, Lbl)>);
  ```

  - `IO`: Protocol marker type
  - `Lbl`: Label for this end (default: EmptyLabel)
- **Pros:**
  - Simple, unambiguous protocol termination.
- **Cons:**
  - None.
- **Properties Ensured:**
  - Explicit protocol termination.
- **Example:**

  ```rust
  type Done = TEnd<Http>;
  ```

---

## Local (Endpoint) Session Types and Projection

## Overview

Local (endpoint) types describe the protocol from the perspective of a single role. Besedarium provides type-level machinery to project a global protocol to the local type for any role, ensuring that each participant follows the correct sequence of actions. This projection is performed entirely at the type level, using type-level lists, booleans, and trait dispatch.

### Endpoint Combinators

- **EpSend**: Endpoint sending operation

  ```rust
  pub struct EpSend<IO, Lbl: ProtocolLabel, R, H, T>(PhantomData<(IO, Lbl, R, H, T)>);
  ```

  - `IO`: Protocol marker type
  - `Lbl`: Label for this interaction (for traceability and debugging)
  - `R`: Role performing the send
  - `H`: Message type being sent
  - `T`: Continuation after sending

- **EpRecv**: Endpoint receiving operation

  ```rust
  pub struct EpRecv<IO, Lbl: ProtocolLabel, R, H, T>(PhantomData<(IO, Lbl, R, H, T)>);
  ```

  - `IO`: Protocol marker type
  - `Lbl`: Label for this interaction (for traceability and debugging)
  - `R`: Role performing the receive
  - `H`: Message type being received
  - `T`: Continuation after receiving

- **EpChoice**: Endpoint protocol choice (branching/offer)

  ```rust
  pub struct EpChoice<IO, Lbl: ProtocolLabel, Me, L, R>(PhantomData<(IO, Lbl, Me, L, R)>);
  ```

  - `IO`: Protocol marker type
  - `Lbl`: Label for this choice (for traceability and debugging)
  - `Me`: The role being projected
  - `L`, `R`: The two local protocol branches

- **EpPar**: Endpoint parallel composition

  ```rust
  pub struct EpPar<IO, Lbl: ProtocolLabel, Me, L, R>(PhantomData<(IO, Lbl, Me, L, R)>);
  ```

  - `IO`: Protocol marker type
  - `Lbl`: Label for this parallel composition (for traceability and debugging)
  - `Me`: The role being projected
  - `L`, `R`: The two local protocol branches

- **EpEnd**: Endpoint protocol termination

  ```rust
  pub struct EpEnd<IO, Lbl: ProtocolLabel, R>(PhantomData<(IO, Lbl, R)>);
  ```

  - `IO`: Protocol marker type
  - `Lbl`: Label for this endpoint (for traceability and debugging)
  - `R`: Role for which the protocol ends

- **EpSkip**: No-op type for roles not involved in a branch

  ```rust
  pub struct EpSkip<IO, Lbl: ProtocolLabel, R>(PhantomData<(IO, Lbl, R)>);
  ```

  - `IO`: Protocol marker type
  - `Lbl`: Label for this skip operation (for traceability and debugging)
  - `R`: Role that is skipping this branch
  - Used to improve type-level precision for projections (not always enabled in runtime code)

### Notes

- All endpoint combinators implement the `EpSession<IO, R>` trait for the appropriate role.
- Labels are used throughout for traceability, debugging, and to support correct projection from global types.
- The design supports type-level checks for whether a local type is an `EpSkip` or `EpEnd` variant, which can be useful for endpoint interpreters and code generation.

---

## Projection Machinery: From Global to Local Types

### Overview

The core of Besedarium's type-level protocol analysis is the machinery for projecting a global protocol
 type to the local (endpoint) type for a given role. This is achieved entirely at the type level using
 traits, type-level booleans, and recursive trait dispatch. The projection machinery ensures that each
 participant in a protocol receives a local type that precisely describes its required actions, and that
 uninvolved roles are handled correctly.

#### Key Traits and Helpers

- **`ProjectRole`**: The main trait for projecting a global protocol onto a specific role, producing the local endpoint type for that role.

- **`ProjectInteract`**: Helper trait for projecting a single interaction, dispatching on whether the role is the sender or receiver.

- **`ProjectChoice` / `ProjectChoiceCase`**: Helpers for projecting protocol choices, handling all cases of role presence in branches.

- **`ProjectPar` / `ProjectParCase`**: Helpers for projecting parallel compositions, handling all cases of role presence in branches.

- **`ProjectRoleOrSkip`**: Helper for projecting a branch or producing an `EpSkip` if the role is not present.

- **`ComposeProjectedParBranches` / `ComposeProjectedParBranchesCase`**: Helpers for composing projected parallel branches, handling `EpSkip` and `EpEnd` cases.

- **`ContainsRole` / `NotContainsRole`**: Type-level predicates to check if a role is present in a protocol branch.

- **`GetProtocolLabel` / `GetLocalLabel`**: Extractors for protocol and endpoint labels, used for label preservation.

### Main Projection Trait: `ProjectRole`

```rust
pub trait ProjectRole<Me, IO, G: TSession<IO>> {
    type Out: EpSession<IO, Me>;
}
```

- `Me`: The role being projected

- `IO`: Protocol marker type

- `G`: The global protocol type

- `Out`: The resulting local endpoint type for `Me`

#### Example: Projecting a Simple Protocol

```rust
// Global protocol: Alice sends Message, then Bob sends Response
// type Global = TInteract<Http, EmptyLabel, Alice, Message,
//     TInteract<Http, EmptyLabel, Bob, Response, TEnd<Http, EmptyLabel>>>;
// Project onto Alice:
type AliceLocal = <() as ProjectRole<Alice, Http, Global>>::Out;
// Result: EpSend<..., EpRecv<..., EpEnd<...>>>
```

### Projection for Each Global Combinator

#### 1. `TEnd`

- Projects to `EpEnd` for the role, preserving the label.

```rust
impl<Me, IO, Lbl> ProjectRole<Me, IO, TEnd<IO, Lbl>> for () {
    type Out = EpEnd<IO, Lbl, Me>;
}
```

#### 2. `TInteract`

- If the role is the sender, projects to `EpSend`.

- If the role is not the sender, projects to `EpRecv`.

- Uses `ProjectInteract` to dispatch based on role equality.

```rust
impl<Me, IO, Lbl, R, H, T> ProjectRole<Me, IO, TInteract<IO, Lbl, R, H, T>> for ()
where
    Me: RoleEq<R>,
    (): ProjectInteract<<Me as RoleEq<R>>::Output, Me, IO, Lbl, R, H, T>,
{
    type Out = <() as ProjectInteract<<Me as RoleEq<R>>::Output, Me, IO, Lbl, R, H, T>>::Out;
}
```

#### 3. `TChoice`

- Projects to `EpChoice` if the role is present in either branch.

- If the role is present in only one branch, the other branch is projected as `EpSkip`.

- If the role is in neither branch, projects to `EpSkip`.

- Uses `ContainsRole` and `ProjectChoiceCase` for case analysis.

```rust
impl<Me, IO, Lbl, L, R> ProjectRole<Me, IO, TChoice<IO, Lbl, L, R>> for ()
where
    L: ContainsRole<Me>,
    R: ContainsRole<Me>,
    (): ProjectChoiceCase<Me, IO, Lbl, L, R, <L as ContainsRole<Me>>::Output, <R as ContainsRole<Me>>::Output>,
{
    type Out = <() as ProjectChoiceCase<Me, IO, Lbl, L, R, <L as ContainsRole<Me>>::Output, <R as ContainsRole<Me>>::Output>>::Out;
}
```

#### 4. `TPar`

- Projects to `EpPar` if the role is present in both branches.

- If the role is present in only one branch, projects that branch directly.

- If the role is in neither branch, projects to `EpSkip`.

- Uses `ContainsRole` and `ProjectParCase` for case analysis.

```rust
impl<Me, IO, Lbl, L, R, IsDisjoint> ProjectRole<Me, IO, TPar<IO, Lbl, L, R, IsDisjoint>> for ()
where
    L: ContainsRole<Me>,
    R: ContainsRole<Me>,
    (): ProjectParCase<Me, IO, Lbl, L, R, <L as ContainsRole<Me>>::Output, <R as ContainsRole<Me>>::Output>,
{
    type Out = <() as ProjectParCase<Me, IO, Lbl, L, R, <L as ContainsRole<Me>>::Output, <R as ContainsRole<Me>>::Output>>::Out;
}
```

#### 5. `TRec`

- Recursion is preserved in the local projection.

### Role Presence: `ContainsRole` and `NotContainsRole`

- `ContainsRole<R>`: Type-level predicate returning `True` if role `R` is present in a protocol branch.

- `NotContainsRole<R>`: True if role `R` is not present.

- Used to determine whether to project a branch or produce an `EpSkip`.

#### Example

```rust
impl<IO, Lbl, R> ContainsRole<R> for TEnd<IO, Lbl> {
    type Output = types::False;
}
// For TInteract, all roles are considered present (sender or receiver)
impl<IO, Lbl, H, T, R1, R2> ContainsRole<R2> for TInteract<IO, Lbl, R1, H, T> {
    type Output = types::True;
}
```

### Handling `EpSkip` and `EpEnd` in Endpoint Composition

- `EpSkip` is used when a role is not involved in a branch (e.g., in a parallel or choice branch).

- `EpEnd` marks protocol termination for a role.

- The machinery includes helpers (`ComposeProjectedParBranches`, etc.) to combine branches and handle cases where one or both are `EpSkip` or `EpEnd`.

#### Example: Composing Parallel Branches

```rust
// If both branches are EpSkip, output EpSkip
impl<IO, Me, Lbl1, Lbl2> ComposeProjectedParBranchesCase<types::True, types::True, types::False, types::False, IO, Me, EpSkip<IO, Lbl1, Me>, EpSkip<IO, Lbl2, Me>> for () {
    type Out = EpSkip<IO, Lbl1, Me>;
}
// If one branch is EpSkip, return the other branch
// If both are projected, create EpPar
```

### Type-Level Dispatch and Label Preservation

- All projection logic is implemented using trait dispatch and type-level booleans.

- Labels from the global protocol are preserved in the local types for traceability and debugging.

- Helper traits (`GetProtocolLabel`, `GetLocalLabel`) extract labels for use in endpoint combinators.

### Example: Full Projection Flow

```rust
// Given a global protocol:
type Global = TPar<Http, ParLabel,
    TInteract<Http, L1, Alice, Msg, TEnd<Http>>,
    TInteract<Http, L2, Bob, Ack, TEnd<Http>>,
    TrueB>;
// Project onto Alice:
type AliceLocal = <() as ProjectRole<Alice, Http, Global>>::Out;
// Result: EpSend<Http, L1, Alice, Msg, EpEnd<Http, L1, Alice>>
```

### Summary

- **Type safe:** All projection is checked at compile time.

- **Compositional:** Each protocol combinator has a corresponding projection rule.

- **Extensible:** New combinators and projection rules can be added as needed.

- **Traceable:** Labels are preserved for debugging and code generation.

For more details, see the source code in `src/protocol/transforms.rs` and the projection tests.

---

## Discussion: EpSkip (Silent/No-op) Combinator

### What is EpSkip?

EpSkip (also called EpSilent or EpNoOp) is a local endpoint combinator representing a "do nothing"
or silent step in a protocol. It is used in session type systems to indicate that a role is
uninvolved in a particular protocol fragment (e.g., not present in any branch of a parallel
composition).

#### Pros of EpSkip

- **Type-level precision:** Clearly indicates uninvolved roles in the local type, improving
protocol clarity and correctness.
- **Simplifies endpoint code:** Allows endpoint interpreters to treat uninvolved roles as true
no-ops, potentially reducing boilerplate.
- **Explicit intent:** Makes the protocol structure and role participation explicit in the type
system.

#### Cons of EpSkip

- **Increased runtime complexity:** The endpoint state machine must recognize and handle EpSkip
states, even though they do nothing.
- **More verbose state machines:** The presence of explicit skip states can make the runtime
control flow harder to follow, especially in complex protocols.
- **Potential confusion:** Developers may not immediately distinguish between EpSkip (no-op) and
EpEnd (termination), leading to ambiguity if not documented and handled carefully.
- **Maintenance burden:** All endpoint interpreters, code generators, and runtime systems must
consistently handle EpSkip, increasing the surface area for bugs or inconsistencies.

### Rationale for Not Implementing EpSkip (for Now)

While EpSkip improves type-level expressiveness, we have decided not to implement it at this stage
due to the increased costs at runtime. The main reasons are:

- **State machine complexity:** Every endpoint runtime must implement and handle EpSkip states,
which adds to the number of possible states and transitions, even though most are no-ops.
- **Control flow clarity:** The presence of explicit skip states can make the runtime logic more
verbose and harder to reason about, especially when debugging or tracing protocol execution.
- **Potential for confusion:** Without careful documentation and discipline, EpSkip may be mistaken
for protocol termination (EpEnd), leading to subtle bugs.
- **Implementation overhead:** All endpoint interpreters and code generators must be updated to
recognize and correctly process EpSkip, increasing maintenance and testing effort.

For these reasons, the current design omits EpSkip and instead allows uninvolved roles to project
to simple terminal or empty endpoint types (e.g., EpEnd), accepting some loss of type-level
precision in favor of runtime simplicity and maintainability.

---

## Compile-Time Properties & Testing

- **Disjointness:** Enforced for parallel branches via traits and macros.
- **Type Equality:** Compile-time assertions ensure that projected types match expected types.
- **Negative Tests:** Trybuild tests ensure that violations (e.g., duplicate roles, mixed IO) fail
to compile.

---

## Comparison with Other Implementations

### Haskell (Session Types Libraries)

- Haskell libraries (e.g., [session-types](https://hackage.haskell.org/package/session-types),
[mpst](https://hackage.haskell.org/package/mpst)) use type classes and GADTs for similar guarantees.
- Rust’s trait system is less expressive for some advanced features (e.g., mutual recursion,
dependent types), but macros and type-level programming provide strong guarantees.

### Scala (Effpi, Scribble)

- Scala libraries use implicits and type-level programming for session types.
- [Effpi](https://github.com/effpi/effpi) is a Scala library for concurrent and distributed
programming with session types.
- [Scribble](https://www.scribble.org/) is a protocol language and toolchain, with Scala
integration for multiparty session types.
- Rust’s approach is more explicit and less reliant on inference, but offers similar compile-time
safety.

### Other Rust

- Some Rust crates (e.g., [session-types](https://crates.io/crates/session-types),
[ferrite](https://github.com/ferrite-rs/ferrite)) focus on binary session types or runtime
representations.
- Besedarium emphasizes multiparty, global-to-local projection, and compile-time protocol
construction.

---

### Extensibility & Future Work

- **Easy to Extend:** New combinators and macros can be added for more protocol features.
- **Limitations:** No runtime choreography, no mutual recursion, no dynamic role handling (yet).
- **Planned:** Improved endpoint combinators (e.g., Silent, Branch, Select), better error messages,
and runtime integration.

---

### Summary Table

| Combinator | Purpose         | Properties Ensured         | Pros                        | Cons
                   |
|------------|----------------|----------------------------|-----------------------------|----------
------------------|
| TInteract  | Interaction    | Linearity, type safety     | Simple, compositional       | Explicit
for each step     |
| TChoice    | Branching      | Exhaustiveness             | Expressive, n-ary via macro | Verbose
without macros     |
| TPar       | Parallelism    | Disjointness, progress     | Models concurrency          | Requires
explicit checks   |
| TRec       | Recursion      | Well-formedness            | Loops, streaming            | No
mutual recursion        |
| TEnd       | Termination    | Explicit end               | Simple                      | -
                  |
| Ep*        | Endpoint types | Local protocol correctness | Ensures local fidelity      | No
runtime choreography    |

---
