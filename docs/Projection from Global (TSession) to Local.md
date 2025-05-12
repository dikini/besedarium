## Projection from Global (TSession) to Local Session Types

This document summarizes the discussion on implementing a type-level fold to project a global session (`TSession`) onto local session types for each role. It outlines the necessary traits, type-level lists, and the technique to avoid overlapping trait implementations.

---

### 1. Type-Level List of Roles (HList)

```rust
pub struct Nil;
pub struct Cons<Head, Tail>(PhantomData<(Head, Tail)>);
```

Use `Cons<Role, Tail>` to build a finite list of all roles, ending with `Nil`.

---

### 2. Per-Role Projection Trait (`ProjectRole`)

Define a trait that projects a global `TSession` onto a single role `Me`:

```rust
pub trait ProjectRole<Me, IO, G: TSession<IO>> {
    type Out: LocalSession<IO>;
}

// Base case: end of protocol
impl<Me, IO> ProjectRole<Me, IO, TEnd<IO>> for () {
    type Out = LEnd<IO>;
}
```

#### 2.1 Handling `TInteract` (Send/Receive) without Overlap

##### 2.1.1 Define Type-Level Booleans

```rust
pub struct True;
pub struct False;

pub trait Bool {}
impl Bool for True {}
impl Bool for False {}
```

##### 2.1.2 Compute Role Equality at Type Level

```rust
pub trait RoleEq<Other: Role> {
    type Output: Bool;
}

// Same role
impl<R: Role> RoleEq<R> for R {
    type Output = True;
}

// Different roles (sealed in private module to avoid overlap with the above)
impl<A: Role, B: Role> RoleEq<B> for A {
    type Output = False;
}
```

##### 2.1.3 Helper Trait to Dispatch on the Boolean Flag

```rust
pub trait ProjectInteract<Flag: Bool, Me: Role, IO, R: Role, H, T: TSession<IO>> {
    type Out: LocalSession<IO>;
}

// Send-case when Flag = True
impl<Me, IO, R: Role, H, T: TSession<IO>>
    ProjectInteract<True, Me, IO, R, H, T> for ()
where
    (): ProjectRole<Me, IO, T>,
{
    type Out = LSend<IO, H, <() as ProjectRole<Me, IO, T>>::Out>;
}

// Recv-case when Flag = False
impl<Me, IO, R: Role, H, T: TSession<IO>>
    ProjectInteract<False, Me, IO, R, H, T> for ()
where
    (): ProjectRole<Me, IO, T>,
{
    type Out = LRecv<IO, H, <() as ProjectRole<Me, IO, T>>::Out>;
}
```

##### 2.1.4 Single `ProjectRole` Impl for `TInteract`

```rust
impl<Me, IO, R: Role, H, T: TSession<IO>>
    ProjectRole<Me, IO, TInteract<IO, R, H, T>> for ()
where
    Flag: Bool,
    Me: RoleEq<R, Output = Flag>,
    (): ProjectInteract<Flag, Me, IO, R, H, T>,
{
    type Out = <() as ProjectInteract<Flag, Me, IO, R, H, T>>::Out;
}
```

This avoids overlapping impls by dispatching inside the helper trait based on the computed `Flag`.

---

### 3. Projection for Other Global Combinators

For each global combinator, add a `ProjectRole` impl:

```rust
// Binary choice
impl<Me, IO, L: TSession<IO>, Rb: TSession<IO>>
    ProjectRole<Me, IO, TChoice<IO, L, Rb>> for ()
where
    (): ProjectRole<Me, IO, L, Out = OutL>,
    (): ProjectRole<Me, IO, Rb, Out = OutR>,
{
    type Out = LChoice<IO, OutL, OutR>;
}

// Parallel composition
impl<Me, IO, L: TSession<IO>, Rb: TSession<IO>, B>
    ProjectRole<Me, IO, TPar<IO, L, Rb, B>> for ()
where
    (): ProjectRole<Me, IO, L, Out = OutL>,
    (): ProjectRole<Me, IO, Rb, Out = OutR>,
{
    type Out = LPar<IO, OutL, OutR>;
}

// Recursion
impl<Me, IO, S: TSession<IO>>
    ProjectRole<Me, IO, TRec<IO, S>> for ()
where
    (): ProjectRole<Me, IO, S, Out = OutS>,
{
    type Out = LRec<IO, OutS>;
}
```

---

### 4. Type-Level Map Over Roles (`MapRoles`)

```rust
pub trait MapRoles<Roles, IO, G: TSession<IO>> {
    type Outputs;
}

// Empty list
impl<IO, G: TSession<IO>> MapRoles<Nil, IO, G> for () {
    type Outputs = Nil;
}

// Cons: project `Head` then recurse
impl<Head, Tail, IO, G, P, MappedTail>
    MapRoles<Cons<Head, Tail>, IO, G> for ()
where
    (): ProjectRole<Head, IO, G, Out = P>,
    (): MapRoles<Tail, IO, G, Outputs = MappedTail>,
{
    type Outputs = Cons<(Head, P), MappedTail>;
}
```

Usage:

```rust
type AllRoles = Cons<TClient, Cons<TServer, Cons<TBroker, Cons<TWorker, Nil>>>>;
type Projections = <() as MapRoles<AllRoles, IO, MyGlobalProtocol>>::Outputs;
```

`Projections` is now a type-level list of `(Role, LocalSession)` pairs for every role.
