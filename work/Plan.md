# Implementation Plan: Label Preservation

## Motivation

Currently, when global protocol types are projected to local endpoint types, labels are not preserved. This leads to two significant issues:

1. Loss of traceability between global and local protocol points
2. Difficulty in debugging and reasoning about the relationship between global and local types

## Goals

1. Preserve labels during projection from global to local types
2. Maintain connection between corresponding global and local protocol points
3. Improve debugging capabilities
4. Enhance type safety by preserving more type information

## Implementation Strategy

### 1. Update Local Endpoint Type Definitions

First, we need to modify the local endpoint types to include label parameters:

```rust
// Current (without labels)
struct EpSend<IO, R, H, T> { ... }
struct EpRecv<IO, R, H, T> { ... }
struct EpChoice<IO, R, L, R> { ... }
struct EpPar<IO, R, L, R> { ... }
struct EpEnd<IO> { ... }
struct EpSkip<IO> { ... }

// Proposed (with labels)
struct EpSend<IO, Lbl, R, H, T> { ... }
struct EpRecv<IO, Lbl, R, H, T> { ... }
struct EpChoice<IO, Lbl, R, L, R> { ... } 
struct EpPar<IO, Lbl, R, L, R> { ... }
struct EpEnd<IO, Lbl> { ... }
struct EpSkip<IO, Lbl> { ... }
```

### 2. Update ProjectRole Trait Implementation

Modify the `ProjectRole` trait and its implementations to propagate labels from global to local types:

```rust
// Current
trait ProjectRole<Me, IO, G> {
    type Endpoint;
}

// Proposed
trait ProjectRole<Me, IO, G> {
    type Endpoint;
}

// Implementation example for TInteract
impl<Me, IO, Lbl, R1, R2, H, T> ProjectRole<Me, IO, TInteract<IO, Lbl, R1, R2, H, T>> 
where
    // existing bounds...
{
    // Now preserves Lbl in the output endpoint type
    type Endpoint = /* ... */;
}
```

### 3. Update Edge Cases and Helper Traits

Modify all helper traits used in projection to properly handle labels:

- `ProjectInteract`
- `ProjectChoice`
- `ProjectPar`
- etc.

### 4. Update Protocol Combinators

For any protocol combinators that generate local types, ensure they propagate label information.

### 5. Add Utility Traits for Label Access

Implement traits to access and compare labels in endpoint types:

```rust
trait GetLabel<IO> {
    type Label;
}

impl<IO, Lbl, R, H, T> GetLabel<IO> for EpSend<IO, Lbl, R, H, T> {
    type Label = Lbl;
}

// Implementations for other endpoint types...
```

### 6. Update Macros and Helper Functions

Update any macros or helper functions that construct endpoint types to include label parameters.

## Testing Plan

1. Create test cases verifying that labels are preserved during projection
2. Test complex nested protocol structures
3. Test label preservation in recursive protocols
4. Add compile-time assertions to verify label preservation

## Advantages of Clean Implementation

Since there are no released versions yet, we can implement label preservation without backward compatibility concerns. This allows us to:

1. Design the cleanest possible API
2. Make breaking changes as needed
3. Focus on correctness and completeness rather than migration strategies
4. Implement the feature more efficiently without compatibility layers

## Timeline

1. Update endpoint type definitions (1-2 days)
2. Update projection implementation (2-3 days)
3. Update helper traits and utilities (1-2 days)
4. Add tests (1 day)
5. Update documentation (1 day)

Total estimated time: 6-9 days
