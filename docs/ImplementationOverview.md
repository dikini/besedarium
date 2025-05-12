# Implementation Overview

## Besedarium: Session Types in Rust — Implementation Overview and Analysis

### Introduction & Goals

Besedarium is a Rust library for building, composing, and verifying communication protocols at the type level using session types. The primary goal is to catch protocol mistakes at compile time, ensuring that distributed systems and networked applications follow the correct message flow. The library aims for a balance between strong static guarantees, ergonomic protocol construction, and extensibility for multiparty and advanced session type features.

### What Are Session Types?

Session types are a type-theoretic framework for describing and verifying structured communication between concurrent or distributed processes. They allow developers to specify the sequence and structure of messages exchanged between roles (participants) in a protocol, ensuring properties like linearity (no double use of channels), progress (no deadlocks), and protocol fidelity (no unexpected messages).

In practice, session types provide a way to encode communication protocols as types, so that the compiler can check for protocol violations. This is especially valuable in distributed systems, where mismatches in message order or structure can lead to subtle, hard-to-debug errors. Session types originated in the study of process calculi and have been adopted in several programming languages, including Haskell, Scala, and Rust, to provide strong compile-time guarantees for communication safety.

### High-Level Architecture

Besedarium models protocols as global types (describing the whole protocol) and provides machinery to project these to local (endpoint) types for each role. The core is a set of combinators (type-level building blocks) and traits for composing, analyzing, and projecting protocols. Macros are used for ergonomic construction of n-ary choices and parallel branches. Compile-time assertions and trybuild tests ensure that safety properties are enforced and violations are caught early.

---

## Global Session Type Combinators

### 1. `TInteract`

- **Purpose:** Models a single interaction (send/receive) by a role in a protocol.
- **Implementation:**

  ```rust
  pub struct TInteract<IO, R, H, T: TSession<IO>>(PhantomData<(IO, R, H, T)>);
  ```

  - `IO`: protocol marker (e.g., Http, Mqtt)
  - `R`: role performing the action
  - `H`: message type
  - `T`: continuation
- **Pros:**
  - Simple, compositional building block for protocols.
  - Encodes both the actor and the message at the type level.
- **Cons:**
  - Requires explicit role and message types for each step.
- **Properties Ensured:**
  - Linearity (each step is explicit and unique in the protocol).
  - Type safety for message and role.
- **Example:**

  ```rust
  type Handshake = TInteract<Http, TClient, Message, TInteract<Http, TServer, Response, TEnd<Http>>>;
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
  pub struct TChoice<IO, L: TSession<IO>, R: TSession<IO>>(PhantomData<(IO, L, R)>);
  ```

  - Used recursively for n-ary choices via macros.
- **Pros:**
  - Expressive for modeling protocol alternatives.
  - N-ary choices supported via macros.
- **Cons:**
  - Manual construction of deeply nested choices can be verbose (mitigated by macros).
- **Properties Ensured:**
  - Exhaustiveness (all branches are explicit).
- **Example:**

  ```rust
  type Choice = tchoice!(Http;
      TInteract<Http, TClient, Message, TEnd<Http>>,
      TInteract<Http, TServer, Response, TEnd<Http>>,
  );
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

##@ 3. `TPar`

- **Purpose:** Models parallel (concurrent) composition of protocol branches.
- **Implementation:**

  ```rust
  pub struct TPar<IO, L: TSession<IO>, R: TSession<IO>, IsDisjoint>(PhantomData<(IO, L, R, IsDisjoint)>);
  ```

  - `IsDisjoint` is a type-level boolean indicating if the branches are disjoint in their roles.
- **Pros:**
  - Enables modeling of concurrent or independent protocol flows.
  - Disjointness is enforced at compile time via traits and macros.
- **Cons:**
  - Requires explicit disjointness checks (assert_disjoint!).
  - N-ary parallel composition can be verbose without macros.
- **Properties Ensured:**
  - Disjointness of roles (no role appears in more than one branch).
  - Linearity and progress for parallel branches.
- **Example:**

  ```rust
  type Workflow = tpar!(Http;
      TInteract<Http, TClient, Message, TInteract<Http, TServer, Response, TEnd<Http>>>,
      TInteract<Http, TBroker, Publish, TEnd<Http>>,
      TInteract<Http, TWorker, Notify, TEnd<Http>>
  );
  assert_disjoint!(par Workflow);
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
  pub struct TRec<IO, S: TSession<IO>>(PhantomData<(IO, S)>);
  ```

- **Pros:**
  - Enables modeling of streaming, loops, or repeated protocol fragments.
- **Cons:**
  - No mutual recursion (only single recursion supported).
- **Properties Ensured:**
  - Well-formedness of recursive protocols.
- **Example:**
  ```rust
  type Streaming = TRec<Http, TInteract<Http, TClient, Message, TEnd<Http>>>;
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
  pub struct TEnd<IO>(PhantomData<IO>);
  ```
- **Pros:**
  - Simple, unambiguous protocol termination.
- **Cons:**
  - None.
- **Properties Ensured:**
  - Explicit protocol termination.

---

## Local (Endpoint) Session Types and Projection

## Overview

Local (endpoint) types describe the protocol from the perspective of a single role. Besedarium provides machinery to project a global protocol to the local type for any role, ensuring that each participant follows the correct sequence of actions.

## Endpoint Combinators

- `EpSend<IO, R, H, T>`: Send action for role R
- `EpRecv<IO, R, H, T>`: Receive action for role R
- `EpChoice<IO, R, L, Rb>`: Local choice/branching
- `EpPar<IO, R, L, Rb>`: Local parallel composition
- `EpEnd<IO, R>`: Local protocol termination

### Projection Machinery

- `ProjectRole<Me, IO, G>`: Trait to project a global protocol G to the local session type for role Me.
- Uses type-level role equality (`RoleEq`) to determine send/receive.
- Compile-time assertions (e.g., `assert_type_eq!`) ensure correctness.

### Example: Projection

```rust
struct Alice;
struct Bob;
impl Role for Alice {}
impl Role for Bob {}
impl RoleEq<Alice> for Alice { type Output = True; }
impl RoleEq<Bob> for Alice { type Output = False; }
impl RoleEq<Alice> for Bob { type Output = False; }
impl RoleEq<Bob> for Bob { type Output = True; }

type Global = TInteract<Http, Alice, Message, TInteract<Http, Bob, Response, TEnd<Http>>>;
type AliceLocal = <() as ProjectRole<Alice, Http, Global>>::Out;
// AliceLocal = EpSend<Http, Alice, Message, EpRecv<Http, Alice, Response, EpEnd<Http, Alice>>>
```

---

## Compile-Time Properties & Testing

- **Disjointness:** Enforced for parallel branches via traits and macros.
- **Type Equality:** Compile-time assertions ensure that projected types match expected types.
- **Negative Tests:** Trybuild tests ensure that violations (e.g., duplicate roles, mixed IO) fail to compile.

---

## Comparison with Other Implementations

### Haskell (Session Types Libraries)

- Haskell libraries (e.g., [session-types](https://hackage.haskell.org/package/session-types), [mpst](https://hackage.haskell.org/package/mpst)) use type classes and GADTs for similar guarantees.
- Rust’s trait system is less expressive for some advanced features (e.g., mutual recursion, dependent types), but macros and type-level programming provide strong guarantees.

### Scala (Effpi, Scribble)

- Scala libraries use implicits and type-level programming for session types.
- [Effpi](https://github.com/effpi/effpi) is a Scala library for concurrent and distributed programming with session types.
- [Scribble](https://www.scribble.org/) is a protocol language and toolchain, with Scala integration for multiparty session types.
- Rust’s approach is more explicit and less reliant on inference, but offers similar compile-time safety.

### Other Rust

- Some Rust crates (e.g., [session-types](https://crates.io/crates/session-types), [ferrite](https://github.com/ferrite-rs/ferrite)) focus on binary session types or runtime representations.
- Besedarium emphasizes multiparty, global-to-local projection, and compile-time protocol construction.

---

### Extensibility & Future Work

- **Easy to Extend:** New combinators and macros can be added for more protocol features.
- **Limitations:** No runtime choreography, no mutual recursion, no dynamic role handling (yet).
- **Planned:** Improved endpoint combinators (e.g., Silent, Branch, Select), better error messages, and runtime integration.

---

### Summary Table

| Combinator | Purpose         | Properties Ensured         | Pros                        | Cons                       |
|------------|----------------|----------------------------|-----------------------------|----------------------------|
| TInteract  | Interaction    | Linearity, type safety     | Simple, compositional       | Explicit for each step     |
| TChoice    | Branching      | Exhaustiveness             | Expressive, n-ary via macro | Verbose without macros     |
| TPar       | Parallelism    | Disjointness, progress     | Models concurrency          | Requires explicit checks   |
| TRec       | Recursion      | Well-formedness            | Loops, streaming            | No mutual recursion        |
| TEnd       | Termination    | Explicit end               | Simple                      | -                          |
| Ep*        | Endpoint types | Local protocol correctness | Ensures local fidelity      | No runtime choreography    |

---

## References

- [Multiparty Asynchronous Session Types (Honda et al., 2008)](https://www.cs.kent.ac.uk/people/staff/srm25/research/multiparty/)
- [Linear type theory for asynchronous session types (Gay & Vasconcelos, 2010)](https://www.dcs.gla.ac.uk/~simon/publications/linear-session-types.pdf)
- [Propositions as sessions (Wadler, 2012)](https://homepages.inf.ed.ac.uk/wadler/papers/propositions-as-sessions/propositions-as-sessions.pdf)
- [Session Types in Rust (blog post)](https://blog.sessiontypes.com/)

---

*Prepared by GitHub Copilot, 2025*
