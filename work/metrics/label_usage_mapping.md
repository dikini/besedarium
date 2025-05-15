# Label Parameter Usage Mapping

This document provides a comprehensive mapping of all label parameter usages throughout the codebase, which will guide the label parameter refactoring effort.

## Core Type Definitions

| Type | Parameter | Constraint | Default | Location |
|------|-----------|------------|---------|----------|
| `TEnd<IO, L>` | `L` | None | `EmptyLabel` | `protocol.rs:64` |
| `TInteract<IO, L, R, H, T>` | `L` | `L: types::ProtocolLabel` | None | `protocol.rs:109` |
| `TRec<IO, L, S>` | `L` | `L: types::ProtocolLabel` | None | `protocol.rs:132` |
| `TChoice<IO, Lbl, L, R>` | `Lbl` | `Lbl: types::ProtocolLabel` | None | `protocol.rs:148` |
| `TPar<IO, Lbl, L, R, IsDisjoint>` | `Lbl` | `Lbl: types::ProtocolLabel` | None | `protocol.rs:199` |

## TSession Implementations

Each combinator implements `TSession<IO>` with composition behavior that preserves the label parameter:

### TEnd

```rust
impl<IO, L> TSession<IO> for TEnd<IO, L> {
    type Compose<Rhs: TSession<IO>> = Rhs;  // Label not preserved (end is replaced)
    const IS_EMPTY: bool = true;
}
```

### TInteract

```rust
impl<IO, L: types::ProtocolLabel, R, H, T: TSession<IO>> TSession<IO> for TInteract<IO, L, R, H, T> {
    type Compose<Rhs: TSession<IO>> = TInteract<IO, L, R, H, T::Compose<Rhs>>;
    // Label L preserved ^
    const IS_EMPTY: bool = false;
}
```

### TRec

```rust
impl<IO, L: types::ProtocolLabel, S: TSession<IO>> TSession<IO> for TRec<IO, L, S> {
    type Compose<Rhs: TSession<IO>> = TRec<IO, L, S::Compose<Rhs>>;
    // Label L preserved ^
    const IS_EMPTY: bool = false;
}
```

### TChoice

```rust
impl<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>> TSession<IO> 
    for TChoice<IO, Lbl, L, R> 
{
    type Compose<Rhs: TSession<IO>> = TChoice<IO, Lbl, L::Compose<Rhs>, R::Compose<Rhs>>;
    // Label Lbl preserved ^
    const IS_EMPTY: bool = false;
}
```

### TPar

```rust
impl<IO, Lbl: types::ProtocolLabel, L: TSession<IO>, R: TSession<IO>, IsDisjoint> TSession<IO> 
    for TPar<IO, Lbl, L, R, IsDisjoint> 
{
    type Compose<Rhs: TSession<IO>> = TPar<IO, Lbl, L::Compose<Rhs>, R::Compose<Rhs>, IsDisjoint>;
    // Label Lbl preserved ^
    const IS_EMPTY: bool = false;
}
```

## ToTChoice and ToTPar Implementations

When constructing `TChoice` and `TPar` from type lists, `EmptyLabel` is used as the default label:

```rust
// In ToTChoice implementation
impl<IO, H: TSession<IO>, T: ToTChoice<IO>> ToTChoice<IO> for Cons<H, T> {
    type Output = TChoice<IO, types::EmptyLabel, H, <T as ToTChoice<IO>>::Output>;
    //                       ^^^^^^^^^^^^^^^ Default label used here
}

// In ToTPar implementation
impl<IO, H: TSession<IO>, T: ToTPar<IO>> ToTPar<IO> for Cons<H, T> {
    type Output = TPar<IO, types::EmptyLabel, H, <T as ToTPar<IO>>::Output, types::False>;
    //                    ^^^^^^^^^^^^^^^ Default label used here
}
```

## Introspection Module

The `introspection.rs` module extensively uses label parameters for collecting and manipulating protocol metadata:

### RolesOf Implementations

```rust
impl<IO, L: types::ProtocolLabel, R, H, T: protocol::TSession<IO> + RolesOf> RolesOf
    for protocol::TInteract<IO, L, R, H, T>
{
    type Roles = protocol::Cons<R, <T as RolesOf>::Roles>;
}

// Similar for other types
```

### LabelsOf Implementations

```rust
impl<IO, L> LabelsOf for protocol::TEnd<IO, L> {
    type Labels = protocol::Cons<L, protocol::Nil>;
}

impl<IO, L: types::ProtocolLabel, R, H, T: protocol::TSession<IO> + LabelsOf> LabelsOf
    for protocol::TInteract<IO, L, R, H, T>
{
    type Labels = protocol::Cons<L, <T as LabelsOf>::Labels>;
}

impl<IO, Lbl: types::ProtocolLabel, L: protocol::TSession<IO> + LabelsOf, R: protocol::TSession<IO> + LabelsOf> 
    LabelsOf for protocol::TChoice<IO, Lbl, L, R>
{
    type Labels = protocol::Cons<Lbl, <L as LabelsOf>::Labels>;
}

impl<IO, Lbl: types::ProtocolLabel, L: protocol::TSession<IO> + LabelsOf, R: protocol::TSession<IO> + LabelsOf, IsDisjoint> 
    LabelsOf for protocol::TPar<IO, Lbl, L, R, IsDisjoint>
{
    type Labels = protocol::Cons<Lbl, <L as LabelsOf>::Labels>;
}

impl<IO, L: types::ProtocolLabel, S: protocol::TSession<IO> + LabelsOf> LabelsOf
    for protocol::TRec<IO, L, S>
{
    type Labels = protocol::Cons<L, <S as LabelsOf>::Labels>;
}
```

## Label Uniqueness Checking

The codebase has utilities for checking label uniqueness at the type level:

```rust
pub trait UniqueList {}
impl UniqueList for Nil {}
impl<H, T> UniqueList for Cons<H, T> where T: NotInList<H> + UniqueList {}

// Used in assert_unique_labels! macro
#[macro_export]
macro_rules! assert_unique_labels {
    ($T:ty) => {
        const _: fn() = || {
            fn _assert_unique_labels()
            where
                <$T as $crate::LabelsOf>::Labels: $crate::UniqueList,
            {
            }
        };
    };
}
```

## Projection Machinery

In the projection code, labels are used in trait bounds but not preserved in the endpoint types:

```rust
impl<Me, IO, L> ProjectRole<Me, IO, TEnd<IO, L>> for ()
where
    Me: Role,
{
    type Out = EpEnd<IO, Me>;  // Label L not preserved
}

impl<Me, IO, L, R, H, T> ProjectRole<Me, IO, TInteract<IO, L, R, H, T>> for ()
where
    Me: Role,
    L: types::ProtocolLabel,  // Label L used as a bound
    R: Role,
    T: TSession<IO>,
    Me: RoleEq<R>,
    <Me as RoleEq<R>>::Output: types::Bool,
    (): ProjectInteract<<Me as RoleEq<R>>::Output, Me, IO, R, H, T>,
{
    type Out = <() as ProjectInteract<<Me as RoleEq<R>>::Output, Me, IO, R, H, T>>::Out;
    // Label L not included in ProjectInteract call
}

// Similar pattern for TChoice, TPar, TRec
```

## Local Endpoint Types

The endpoint types representing local projections lack label parameters entirely:

```rust
pub struct EpSend<IO, R, H, T>(PhantomData<(IO, R, H, T)>);
pub struct EpRecv<IO, R, H, T>(PhantomData<(IO, R, H, T)>);
pub struct EpEnd<IO, R>(PhantomData<(IO, R)>);
pub struct EpChoice<IO, Me, L, R>(PhantomData<(IO, Me, L, R)>);
pub struct EpPar<IO, Me, L, R>(PhantomData<(IO, Me, L, R)>);
pub struct EpSkip<IO, R>(PhantomData<(IO, R)>);
```

## Macro System

Labels are used in macros that construct protocol combinations:

```rust
// In tchoice! macro from lib.rs
#[macro_export]
macro_rules! tchoice {
    ($io:ty; $($branch:ty),+ $(,)?) => {
        <tlist!($($branch),*) as ToTChoice<$io>>::Output
    };
}

// In tpar! macro from lib.rs
#[macro_export]
macro_rules! tpar {
    ($io:ty; $($branch:ty),* $(,)?) => {
        <tlist!($($branch),*) as ToTPar<$io>>::Output
    };
}
```

## Test and Example Usage

Labels are used extensively in tests and examples:

```rust
// Example from a protocol test
type Global = TInteract<
    Http,
    EmptyLabel,  // Using default empty label
    Alice,
    Message,
    TInteract<Http, EmptyLabel, Bob, Response, TEnd<Http, EmptyLabel>>
>;
```

## Refactoring Impact

Based on this detailed mapping, the refactoring will impact:

1. **Type Definitions (3)**: 
   - `TEnd<IO, L>` → `TEnd<IO, Lbl>`
   - `TInteract<IO, L, R, H, T>` → `TInteract<IO, Lbl, R, H, T>`
   - `TRec<IO, L, S>` → `TRec<IO, Lbl, S>`

2. **Trait Implementations (7+)**:
   - `TSession` implementations for each combinator
   - `LabelsOf` implementations in `introspection.rs`
   - `ProjectRole` implementations for projection

3. **Tests and Examples (Many)**:
   - Type aliases in test files
   - Examples in documentation
   - Compile-time assertions

4. **Unique Challenges**:
   - In `TChoice` and `TPar`, both `Lbl` (label) and `L` (left branch) parameters exist, which requires careful handling to avoid confusion
   
5. **Areas Requiring Special Attention**:
   - The `introspection.rs` module relies heavily on label parameters
   - Projection machinery uses label parameters in bounds
   - Compile-fail tests that check error messages

This mapping will serve as a guide during the implementation of the refactoring to ensure all usages of label parameters are consistently updated.

---

Last updated: May 14, 2025