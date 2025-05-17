## Label Preservation in Type-Level Protocol Projection

### Pattern: Preserving Metadata Across Type Transformations

**Problem:** When projecting from global to local protocol types, we need to preserve label information to maintain traceability and enhance debugging.

**Solution:** Extend all endpoint types to carry label information, and create traits for accessing this metadata:

```rust
// Before: Types without label parameters
pub struct EpSend<IO, R, H, T> { /* ... */ }
pub struct EpEnd<IO, R> { /* ... */ }

// After: Types with label parameters
pub struct EpSend<IO, Lbl: ProtocolLabel, R, H, T> { /* ... */ }
pub struct EpEnd<IO, Lbl: ProtocolLabel, R> { /* ... */ }
```

### Pattern: Metadata Extraction Traits

**Problem:** Need a standardized way to extract label information from protocol types.

**Solution:** Create specialized extraction traits:

```rust
// Extract label from global protocol types
pub trait GetProtocolLabel {
    type Label: types::ProtocolLabel;
}

// Extract label from local endpoint types
pub trait GetLocalLabel {
    type Label: types::ProtocolLabel;
}

// Implementation for endpoint type
impl<IO, Lbl: types::ProtocolLabel, R, H, T> GetLocalLabel for EpSend<IO, Lbl, R, H, T> {
    type Label = Lbl;
}
```

### Pattern: Ensuring Metadata Propagation in Type-Level Functions

**Problem:** During projection, label information must be correctly propagated from source to destination types.

**Solution:** Update trait implementations to preserve and propagate labels:

```rust
// Before: ProjectRole implementation without label handling
impl<Me, IO, R, H, T> ProjectRole<Me, IO, TInteract<IO, Lbl, R, H, T>> for ()
where /* bounds */ {
    type Out = <() as ProjectInteract</* params */>::Out;
}

// After: ProjectRole implementation with label handling
impl<Me, IO, Lbl, R, H, T> ProjectRole<Me, IO, TInteract<IO, Lbl, R, H, T>> for ()
where /* bounds */ {
    type Out = <() as ProjectInteract</* params with Lbl */>::Out;
}
```

### Key Insights

1. **Type Parameters for Metadata:** Use dedicated type parameters to carry metadata like labels through type-level transformations.

2. **Access Traits:** Create specialized traits for metadata access that work with the type system, not runtime values.

3. **Trait Dependency Chains:** Labels flow through a chain of trait implementations, from global types through projection traits to local types.

4. **Consistent Parameter Ordering:** Maintain consistent type parameter ordering across related types to reduce confusion:
   ```
   <IO, Label, Role, MessageType, Continuation>
   ```

5. **Helper Traits for Extraction:** Use helper traits like `GetLocalLabel` to make metadata accessible to code using the resulting types without exposing implementation details.

This approach enables debugging, tracing, and relating local to global protocol points without runtime overhead.
