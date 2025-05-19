# Implementation Plan: Adding TStart Combinator as Protocol Entry Point

## PROPOSED EDIT PLAN
Working with: Multiple files in the besedarium codebase
Total planned edits: 7
---

## 1. Background & Context

The goal of this task is to implement a `TStart` combinator to serve as the initial entry point for global protocols. Based on our analysis of the codebase, this would fit into the existing architecture alongside other global protocol combinators like `TEnd`, `TSend`, `TRecv`, `TChoice`, and `TPar`.

The `TStart` will provide a clear and explicit starting point for protocols, making the protocol structure more consistent and improving readability. It will also help with protocol composition since it will serve as a well-defined entry point.

## 2. Technical Requirements

1. The `TStart` combinator must adhere to the protocol label invariant: it must have a label parameter and implement the `GetProtocolLabel` trait.
2. It must be compatible with the existing projection machinery to ensure proper conversion from global to local protocols.
3. It must work with stable Rust features (no specialization, negative bounds, etc.).
4. It should maintain type safety and preserve the integrity of the session type system.

## 3. Detailed Design

### 3.1. Structure of `TStart`

The `TStart` combinator will have the following structure:
```rust
pub struct TStart<IO, Lbl: types::ProtocolLabel, S: TSession<IO>>(PhantomData<(IO, Lbl, S)>);
```

where:
- `IO`: Protocol marker type (e.g., Http, Mqtt)
- `Lbl`: Label for this start point (for projection and debugging)
- `S`: The continuation protocol after this start point

### 3.2. Trait Implementations

The `TStart` combinator will need the following trait implementations:

1. `sealed::Sealed` trait (to restrict trait implementations)
2. `TSession<IO>` trait (core trait for all protocol combinators)
3. `GetProtocolLabel` trait (to adhere to the protocol label invariant)

### 3.3. Projection Implementation

We'll need to implement `ProjectRole` for `TStart` to properly project it to local protocols. The projection will:
1. Map `TStart<IO, Lbl, S>` to a new corresponding `EpStart<IO, Lbl, Me>` for each role `Me`
2. Chain this with the projection of the inner protocol `S`
3. Preserve the label from the global protocol through to the local protocol

## 4. Implementation Plan

### 4.1. Add `TStart` Combinator in global.rs

First, we'll define the `TStart` struct and implement the basic traits for it in the `global.rs` file.

### 4.2. Update GetProtocolLabel Implementation

Ensure the `TStart` combinator adheres to the protocol label invariant by implementing `GetProtocolLabel`.

### 4.3. Implement Projection Logic

Add an implementation of `ProjectRole` for `TStart` in the projection module to handle its projection to local protocols.

### 4.4. Add Unit Tests

Create unit tests to verify that `TStart` works correctly, including:
- Basic type-level functioning
- Proper projection to local protocols
- Label preservation

### 4.5. Add Integration Tests

Add integration tests to verify that `TStart` works correctly in more complex protocol scenarios.

### 4.6. Update Documentation

Update documentation in key files to explain the purpose and usage of `TStart`.

### 4.7. Update Examples

Update existing examples to use `TStart` as their entry point where appropriate.

## 5. Step-by-Step Plan

1. **Edit 1**: Add `TStart` struct definition and trait implementations to `src/protocol/global.rs`
   - Define the struct with proper generic parameters
   - Implement required traits (`sealed::Sealed`, `TSession<IO>`)
   - Implement composition rules for `TStart`

2. **Edit 2**: Add label implementation in `transforms/util.rs` or appropriate module
   - Implement `GetProtocolLabel` for `TStart` to adhere to the protocol label invariant

3. **Edit 3**: Implement projection logic in `transforms/projection.rs`
   - Add `ProjectRole` implementation to project `TStart` to local protocols

4. **Edit 4**: Create unit tests in `tests/` directory
   - Create basic tests for `TStart` functionality
   - Test projection and label preservation

5. **Edit 5**: Add or update integration tests in `tests/protocols/` directory
   - Update an existing protocol test to use `TStart`
   - Verify that it works correctly in a real protocol scenario

6. **Edit 6**: Update documentation in key files
   - Update docstrings in `global.rs`
   - Update module-level documentation as appropriate

7. **Edit 7**: Update README.md if necessary to mention `TStart`

## 6. Coding Guidelines

- Follow Rust's naming conventions and coding style
- Add comprehensive docstrings to all public items
- Ensure all changes pass all tests (`cargo test`)
- Ensure code passes clippy (`cargo clippy`) without warnings
- Format all code with rustfmt (`cargo fmt`)

## 7. Testing Strategy

### 7.1. Unit Tests

1. Test that `TStart` is properly defined and can be used in type definitions
2. Test that `TStart` projects correctly to local protocols
3. Test that labels are preserved through projection
4. Test that `TStart` composes correctly with other protocol combinators

### 7.2. Integration Tests

1. Update an existing protocol test to use `TStart` as its entry point
2. Create a new protocol example that uses `TStart` to verify its applicability

## 8. Documentation

The following documentation updates will be needed:

1. Add detailed docstrings to `TStart` definition
2. Update module-level documentation in `global.rs` to mention `TStart`
3. Update ImplementationOverview.md to include `TStart` in the list of global session type combinators

## 9. Risks and Mitigations

### 9.1. Potential Risks

1. **Projection Complexity**: Adding a new global combinator may require updates to the projection system.
   - *Mitigation*: Follow the existing pattern of projection implementation.

2. **Backward Compatibility**: Existing tests may need to be updated.
   - *Mitigation*: Ensure that `TStart` is optional for existing protocols or update tests to use it.

3. **Composition Rules**: Composition with other combinators needs careful consideration.
   - *Mitigation*: Define composition rules clearly and test them exhaustively.

## 10. Future Considerations

1. Consider adding a specific local projection type for `TStart` if needed, like `EpStart`
2. Consider enhancing `TStart` with additional metadata about the protocol
3. Consider adding support for multiple starting points in complex protocols