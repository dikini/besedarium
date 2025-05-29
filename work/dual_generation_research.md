# Dual Protocol Generation Research

## Research Phase: Automatic Dual Protocol Generation

This document captures the research and prototyping work for Task 1.4.3, exploring approaches to automatically generate dual protocols from existing protocol specifications.

## Literature Review

### Academic Foundation

The concept of duality in session types originates from linear logic and π-calculus research:

1. **Honda, Vasconcelos, and Kubo (1998)**: Introduced basic session types with send/receive duality

2. **Gay and Hole (2005)**: Formalized duality rules for choice and branching

3. **Carbone, Honda, and Yoshida (2007)**: Extended to multiparty session types with projection

### Duality Rules Summary

From our `docs/duality.md` and existing implementation:

| Construct | Dual |
|-----------|------|
| `TChanEnd` | `TChanEnd` (self-dual) |
| `TChanSend<S,R,C,L,Msg,P,AIO>` | `TChanRecv<R,S,C,L,Msg,Dual(P),AIO>` |
| `TChanRecv<R,S,C,L,Msg,P,AIO>` | `TChanSend<S,R,C,L,Msg,Dual(P),AIO>` |
| `TChanChoice<R,C,L,Left,Right,AIO>` | `TChanOffer<R,C,L,Dual(Left),Dual(Right),AIO>` |
| `TChanOffer<R,C,L,Left,Right,AIO>` | `TChanChoice<R,C,L,Dual(Left),Dual(Right),AIO>` |
| `TChanPar<C,L,Left,Right,IsDisjoint,AIO>` | `TChanPar<C,L,Dual(Left),Dual(Right),IsDisjoint,AIO>` |

## Approach 1: Type-Level Dual Generation

### Core Trait Design

```rust
/// Trait for automatically generating the dual of a protocol
pub trait GenerateDual<P> {
    type Output;
}

/// Helper type alias for cleaner usage
pub type Dual<P> = <() as GenerateDual<P>>::Output;
```

### Implementation Strategy

The key insight is that each protocol type needs its own implementation that follows the duality rules. The challenge is handling the type parameter transformations correctly.

### Prototype Implementation

```rust
// TChanEnd is self-dual
impl<C, L, AIO> GenerateDual<TChanEnd<C, L, AIO>> for ()
where
    C: ChanId,
    L: MsgLbl,
    AIO: ActionIOTMarker,
{
    type Output = TChanEnd<C, L, AIO>;
}

// TChanSend becomes TChanRecv with role reversal
impl<S, R, C, L, Msg, P, AIO> GenerateDual<TChanSend<S, R, C, L, Msg, P, AIO>> for ()
where
    S: Role,
    R: Role,
    C: ChanId,
    L: MsgLbl,
    Msg: Message,
    P: GlobalProtocol,
    AIO: ActionIOTMarker,
    (): GenerateDual<P>,
{
    type Output = TChanRecv<R, S, C, L, Msg, Dual<P>, AIO>;
}
```

## Approach 2: Macro-Based Generation

### Procedural Macro Design

```rust
#[proc_macro]
pub fn generate_dual(input: TokenStream) -> TokenStream {
    // Parse the input protocol type
    // Apply transformation rules
    // Generate the dual protocol type
}

// Usage:
// type DualProtocol = generate_dual!(MyProtocol);
```

### Benefits

- More flexible type transformations
- Better error messages
- Can handle complex nested structures
- Integration with IDE tooling

### Drawbacks

- Compile-time dependency
- Less integrated with type system
- Harder to compose with other type-level operations

## Technical Challenges

### 1. Role Parameter Handling

The biggest challenge is automatically swapping role parameters in `TChanSend`/`TChanRecv`. In Rust's type system, we need the trait implementation to "know" which type parameters are roles and need swapping.

### 2. Recursive Protocol Handling

For recursive protocols, we need to ensure that:

- Recursive references are preserved
- The dual generation terminates
- Type bounds are maintained

### 3. Integration with Existing `IsDual` System

The generated dual should satisfy:

```rust
fn verify_dual_generation<P>() 
where 
    (): GenerateDual<P>,
    (): IsDual<P, Dual<P>>,
    <() as IsDual<P, Dual<P>>>::Output: EqualsTrue,
{
    // This should always compile
}
```

## Rust Type System Constraints

### Limitations in Stable Rust

1. **No Specialization**: Cannot provide more specific implementations for subsets of types
2. **No Negative Bounds**: Cannot constrain what types are NOT
3. **No Associated Types as Generic Parameters**: Limited in how we can use computed types
4. **No Overlapping Implementations**: Implementation sets must be disjoint

### Workarounds

1. **Marker Type Dispatch**: Use phantom type parameters to distinguish cases
2. **Helper Trait Cascading**: Use multiple traits with different bounds
3. **Type-Level Programming**: Leverage associated types and where clauses

## Feasibility Analysis

### Type-Level Generation Feasibility: ✅ PROMISING

The type-level approach appears feasible with current Rust stable features:

- We can implement `GenerateDual` for each protocol type
- Role swapping can be handled through explicit implementations
- Recursive types can be handled with proper trait bounds

### Integration Feasibility: ✅ GOOD

Integration with existing system looks straightforward:

- `GenerateDual` can coexist with `IsDual`
- Type aliases provide clean user interface
- Backward compatibility maintained

### Performance Considerations: ⚠️ NEEDS TESTING

Compile-time impact needs measurement:

- Type-level computation adds to compile times
- Complex nested protocols may be expensive
- Need benchmarking on realistic protocols

## Next Steps

1. **Implement Prototype**: Create working implementation of `GenerateDual` trait
2. **Comprehensive Testing**: Verify all duality rules are correctly implemented
3. **Performance Benchmarking**: Measure compile-time impact
4. **Integration Testing**: Ensure compatibility with existing `IsDual` system
5. **User Experience**: Design clean APIs for protocol authors

## Implementation Roadmap

### Phase 1: Core Trait Implementation

- [ ] Define `GenerateDual` trait
- [ ] Implement for basic protocol types (`TChanEnd`, `TChanSend`, `TChanRecv`)
- [ ] Add helper type alias `Dual<P>`

### Phase 2: Complex Protocol Support

- [ ] Implement for choice/offer constructs
- [ ] Implement for parallel composition
- [ ] Handle nested protocol structures

### Phase 3: Integration and Testing

- [ ] Verify integration with `IsDual` system
- [ ] Add comprehensive test suite
- [ ] Performance benchmarking

### Phase 4: Advanced Features

- [ ] Error message improvement
- [ ] Macro-based convenience functions
- [ ] Documentation and examples

## Research Conclusions

The type-level dual generation approach appears both technically feasible and practically useful for the Besedarium project. The main challenges are manageable within Rust's stable type system constraints, and the integration with existing duality verification would provide a comprehensive solution for protocol duality management.
