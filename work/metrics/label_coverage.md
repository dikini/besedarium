# Label Parameter Test Coverage Metrics

This document tracks the test coverage metrics for label parameters in the Besedarium project, specifically related to the label parameter refactoring effort.

## Defined Metrics

We have established the following metrics to track the completeness of our label-related testing:

### 1. Combinator Coverage

**Definition**: The percentage of session type combinators that have specific tests for their label parameter behavior.

**Formula**: 
```
(Number of combinators with custom label tests / Total number of combinators) × 100%
```

**Target**: 100% of combinators should have dedicated label tests.

### 2. Composition Operation Coverage

**Definition**: The percentage of composition operations (`TSession::Compose`) that have tests verifying label preservation.

**Formula**: 
```
(Number of composition operations with label preservation tests / Total number of composition operations) × 100%
```

**Target**: 100% of composition operations should have label preservation tests.

### 3. Custom Label Type Coverage

**Definition**: For each combinator, the number of different custom label types used in tests.

**Target**: Each combinator should be tested with at least 3 different label types:
- `EmptyLabel` (default)
- At least 2 custom label types (e.g., `L1`, `L2`)

### 4. Label Edge Case Coverage

**Definition**: The percentage of identified edge cases related to labels that have been tested.

**Edge cases**:
- Nested compositions with multiple label types
- Interactions between different combinators with different label types
- Complex protocol structures with mixed label usage

**Target**: At least 80% of identified edge cases should have tests.

## Current Baseline Measurements (as of May 14, 2025)

### 1. Combinator Coverage

| Combinator | Has Custom Label Tests | Notes |
|------------|------------------------|-------|
| `TEnd`     | ✅ Yes                | In `label_tests.rs` |
| `TInteract`| ✅ Yes                | In `label_tests.rs` |
| `TRec`     | ✅ Yes                | In `label_tests.rs` |
| `TChoice`  | ✅ Yes                | In `label_tests.rs` |
| `TPar`     | ✅ Yes                | In `label_tests.rs` |

**Current Coverage**: 5/5 = 100%

### 2. Composition Operation Coverage

| Combinator | Has Label Preservation Tests | Notes |
|------------|------------------------------|-------|
| `TEnd`     | ✅ Yes                       | In `label_tests.rs` (shows label is replaced) |
| `TInteract`| ✅ Yes                       | In `label_tests.rs` |
| `TRec`     | ✅ Yes                       | In `label_tests.rs` |
| `TChoice`  | ✅ Yes                       | In `label_tests.rs` |
| `TPar`     | ✅ Yes                       | In `label_tests.rs` |

**Current Coverage**: 5/5 = 100%

### 3. Custom Label Type Coverage

| Combinator | Different Label Types | Notes |
|------------|----------------------|-------|
| `TEnd`     | 3 (`L1`, `L2`, `L3`) | In `test_tend_label_replaced` |
| `TInteract`| 2 (`L1`, `L2`)       | In `test_tinteract_label_preserved` |
| `TRec`     | 2 (`L1`, `L2`)       | In `test_trec_label_preserved` |
| `TChoice`  | 3 (`L1`, `L2`, `L3`) | In `test_tchoice_label_preserved` |
| `TPar`     | 3 (`L1`, `L2`, `L3`) | In `test_tpar_label_preserved` |

**Current Status**:
- 3/5 combinators meet target (≥ 3 label types)
- 2/5 combinators need additional label type tests

### 4. Label Edge Case Coverage

| Edge Case | Has Tests | Notes |
|-----------|-----------|-------|
| Nested compositions | ❌ No | Not yet implemented |
| Mixed combinator interactions | ❌ No | Not yet implemented |
| Complex protocol structures | ❌ No | Not yet implemented |

**Current Coverage**: 0/3 = 0%

## Summary

| Metric | Current Value | Target | Status |
|--------|--------------|--------|--------|
| Combinator Coverage | 100% | 100% | ✅ Met |
| Composition Operation Coverage | 100% | 100% | ✅ Met |
| Custom Label Type Coverage | 60% | 100% | ⚠️ Partially met |
| Label Edge Case Coverage | 0% | 80% | ❌ Not met |

## Test Coverage Tracking Implementation

The test coverage is tracked both manually in this document and programmatically in `tests/label_tests.rs` through:

1. **Type-level tracking traits**:
   ```rust
   trait TestedWithCustomLabel {}
   trait TestedLabelPreservation {}
   ```

2. **Coverage statistics structure**:
   ```rust
   struct LabelTestCoverage {
       pub combinators_with_custom_labels: usize,
       pub total_combinators: usize,
       pub composition_operations_tested: usize,
       pub total_composition_operations: usize,
   }
   ```

3. **Coverage report test** that outputs current metrics during test runs:
   ```rust
   #[test]
   fn report_label_test_coverage() {
       println!("Label Test Coverage Report:");
       println!("Combinators with custom label tests: {}/{} ({}%)", 
                CURRENT_COVERAGE.combinators_with_custom_labels, 
                CURRENT_COVERAGE.total_combinators,
                COMBINATOR_COVERAGE_PCT);
       println!("Composition operations with label preservation tests: {}/{} ({}%)", 
                CURRENT_COVERAGE.composition_operations_tested, 
                CURRENT_COVERAGE.total_composition_operations,
                COMPOSITION_COVERAGE_PCT);
   }
   ```

## Next Steps for Improving Coverage

1. Add tests for `TInteract` and `TRec` with at least one additional custom label type
2. Implement tests for the identified edge cases:
   - Nested compositions with multiple label types
   - Mixed combinator interactions
   - Complex protocol structures

These improvements will ensure comprehensive test coverage before proceeding with the actual label parameter refactoring.

---

*Last updated: May 14, 2025*