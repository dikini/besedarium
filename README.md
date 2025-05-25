# Besedarium

Welcome to the Session Types Playground! This project is a Rust library for building, composing,
and verifying communication protocols at the type level. If you've ever wanted to make sure your
distributed systems or networked applications follow the right message flow—at compile time—this
is for you. With clear protocol entry points (`TStart`) and comprehensive safety checks, Besedarium
helps you design robust communication protocols.

## Background: Session Types

Session types provide a formal, type-based approach to describing and verifying communication
protocols between concurrent or distributed processes. By encoding the permitted sequences of
message exchanges in types, they guarantee properties such as protocol fidelity, progress, and
deadlock freedom.

Key research works:

- K. Honda, V. T. Vasconcelos, M. Kubo. "Language primitives and type discipline for structured
  communication-based programming." ESOP '98.
- N. Yoshida, M. H. Carbone. "Multiparty asynchronous session types." POPL '15.
- R. Gay and N. Vasconcelos. "Linear type theory for asynchronous session types." JFP '10.

Notable implementations:

- Rust:
  - besedarium (this library)
  - `session-types` crate
  ([<https://crates.io/crates/session-types](https://crates.io/crates/session-types>))
- Scala:
  - Effpi library ([<https://github.com/effpi/effpi](https://github.com/effpi/effpi>))
- Haskell:
  - `session` package
  ([<https://hackage.haskell.org/package/session](https://hackage.haskell.org/package/session>))

## What is this?

Session types let you describe the structure of conversations between different parts of your
system. With this library, you can:

- Define protocols as types (like a handshake, a publish/subscribe, or a workflow)
- Compose protocols using ergonomic macros
- Get helpful compile-time errors if you make a mistake (like mixing up roles or leaving out a
  branch)
- See real-world protocol examples in the `tests/protocols/` folder

## Why should I care?

- **Catch protocol mistakes early:** No more runtime surprises when two services disagree on what
  comes next.
- **Readable and reusable:** Protocols are just Rust types—easy to read, share, and reuse.
- **Great for learning:** The examples and tests are designed to be easy to follow, so you can
  learn session types by example.

## How do I use it?

1. Add this crate to your project (see [Cargo.toml](Cargo.toml)).
2. Define your roles and messages as Rust types.
3. Use the provided macros (`tchoice!`, `tpar!`, etc.) to build your protocol.
4. Check out the examples in `tests/protocols/` for inspiration.


## Where do I find more?

- **Protocol examples:** See `tests/protocols/` for real-world patterns.
- **Negative tests:** See `tests/trybuild/` for compile-fail cases and macro edge cases.
- **Docs:** Build and read the docs with `cargo doc --open`.

## Contributing

Contributions, questions, and protocol ideas are welcome! Open an issue or PR, or just try out
the library and let us know what you think.

---

*Session Types Playground: making protocols safer, one type at a time.*

## ⚠️ Doctest/Test Failure Note

> **Note:**
> Some code blocks in this README use macros (e.g., `tchoice!`, `tpar!`) or type-level assertions (e.g., `assert_type_eq!`) that are not available in the Rust doctest context. As a result, running `cargo test --doc` or CI doctests may fail due to macro visibility or Rust's type identity limitations. All real protocol and projection tests are covered in integration tests and `tests/compile.rs`.
