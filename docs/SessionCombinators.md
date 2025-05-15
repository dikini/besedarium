# Session Types in Rust: Design Discussion Summary

This document summarizes the key points from our discussion on implementing session types in Rust,
focusing on recursion, choices, parallelism, and shared channels.

---

## Recursion in Session Types

- **Simple Recursion:**
  The `TRecS` combinator wraps a session type `S` to indicate recursion. This is a type-level
  marker; actual unrolling or repetition is handled at runtime (e.g., via a loop in the protocol
  interpreter).

- **Nested Recursion:**
  You can nest `TRec` combinators to express recursive structures within recursive structures,
  e.g. `TRec<TInteract<..., TRec<...>>>`.

- **Mutual Recursion:**
  Mutual recursion (where two or more session types refer to each other) is not directly supported
  with just `TRec`. It typically requires more advanced constructs, such as recursion variables
  (`TVar`) and fixpoint binders (`TMu`).
  Alternatively, mutual recursion can sometimes be simulated using parallel composition (`TPar`) and
  communication between processes, though this is more complex and not as direct as explicit mutual
  recursion at the type level.

---

## Choices

- **Choice Combinator (`TChoice`):**
  Represents a branching point in the protocol, similar to sum types or protocol choices.
  Example implementation:

  ```rust
  pub struct TChoice<L: TSession, R: TSession>(PhantomData<(L, R)>);
  ```

  This allows the protocol to proceed along one of several possible branches.

  ```rust
  pub struct TChoice<L: TSession, R: TSession>(PhantomData<(L, R)>);

  impl<L: TSession, R: TSession> Sealed for TChoice<L, R> {}

  impl<L: TSession, R: TSession> TSession for TChoice<L, R> {
      type Compose<Rhs: TSession> = TChoice<L::Compose<Rhs>, R::Compose<Rhs>>;
      const IS_EMPTY: bool = false;
  }
  ```

---

## Parallel Processes

- **Parallel Combinator (`TPar`):**
  Represents concurrent or interleaved processes, similar to product types or concurrent branches.
  Example implementation:

  ```rust
  pub struct TPar<L: TSession, R: TSession>(PhantomData<(L, R)>);
  ```

  This enables modeling of protocols where two or more processes proceed independently or interact
  concurrently.

  ```rust
  pub struct TPar<L: TSession, R: TSession>(PhantomData<(L, R)>);

  impl<L: TSession, R: TSession> Sealed for TPar<L, R> {}

  impl<L: TSession, R: TSession> TSession for TPar<L, R> {
      type Compose<Rhs: TSession> = TPar<L::Compose<Rhs>, R::Compose<Rhs>>;
      const IS_EMPTY: bool = false;
  }
  ```

---

## Shared Channels

- **Type-Level Representation:**
  Shared channels can be indicated using phantom types or combinators,
  e.g. `TShared<Channel, S>`, to mark that a session uses a particular channel.

  ```rust
  pub struct ChannelA;
  pub struct ChannelB;

  pub struct TInteract<R: TRole, H, T: TSession, C>(PhantomData<(R, H, T, C)>);
  ```

- **Channel Environments:**
  For more complex scenarios, a type-level environment mapping channel names to session types can
  be used, but this is advanced and not trivial in Rust.

  ```rust
  pub struct TShared<C, S: TSession>(PhantomData<(C, S)>);

  impl<C, S: TSession> Sealed for TShared<C, S> {}
  impl<C, S: TSession> TSession for TShared<C, S> {
      type Compose<Rhs: TSession> = TShared<C, S::Compose<Rhs>>;
      const IS_EMPTY: bool = false;
  }
  ```

- **Runtime Handling:**
  In practice, shared channels are often managed at runtime (e.g., using `Arc<Mutex<...>>` or async
  channels), with the type system ensuring protocol correctness.

---

## Summary Table

| Feature            | Supported? | Notes                                                 |
|--------------------|------------|-------------------------------------------------------|
| Simple Recursion   | Yes        | With `TRecS`                                        |
| Nested Recursion   | Yes        | By nesting `TRec`                                     |
| Mutual Recursion   | No*        | Requires advanced constructs (`TVar`, `TMu`) or       |
|                    |            | can be simulated with `TPar`                          |
| Choices            | Yes        | With `TChoice<L, R>`                                  |
| Parallelism        | Yes        | With `TPar<L, R>`                                     |
| Shared Channels    | Partial    | Phantom types/combinators at type level;              |
|                    |            | runtime management required                           |

---

## Example: Session Chain

```rust
type Chain = Compose<Compose<ClientServer, ServerBroker>, TRec<BrokerWorker>>;
```

This represents a protocol where:

1. `ClientServer` session runs,
2. followed by `ServerBroker`,
3. followed by a recursive wrapper around `BrokerWorker`.

---

## Further Reading

- [Session Types in Rust (blog post)](https://blog.sessiontypes.com/)
- [Multiparty Session Types](https://www.cs.kent.ac.uk/people/staff/srm25/research/multiparty/)

---
