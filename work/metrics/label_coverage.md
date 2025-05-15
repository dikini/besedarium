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
- At least 2 custom label types (e.g., `L1`, `L2`, `L3`)

### 4. Label Edge Case Coverage

**Definition**: The percentage of identified edge cases related to labels that have been tested.

**Edge cases**:
- Nested compositions with multiple label types
- Interactions between different combinators with different label types
- Complex protocol structures with mixed label usage

**Target**: At least 80% of identified edge cases should have tests.

## Current Measurements (as of May 14, 2025)

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
| `TInteract`| 3 (`L1`, `L2`, `L3`) | In `test_tinteract_label_preserved`, `test_tinteract_l2_label_preserved`, `test_tinteract_l3_label_preserved` |
| `TRec`     | 3 (`L1`, `L2`, `L3`) | In `test_trec_label_preserved`, `test_trec_l2_label_preserved`, `test_trec_l3_label_preserved` |
| `TChoice`  | 3 (`L1`, `L2`, `L3`) | In `test_tchoice_label_preserved` and edge case tests |
| `TPar`     | 3 (`L1`, `L2`, `L3`) | In `test_tpar_label_preserved` and edge case tests |

**Current Status**:
- 5/5 combinators meet target (≥ 3 label types)

### 4. Label Edge Case Coverage

| Edge Case | Has Tests | Notes |
|-----------|-----------|-------|
| Nested compositions | ✅ Yes | In `test_nested_composition_label_preservation` |
| Mixed combinator interactions | ✅ Yes | In `test_mixed_combinator_interactions` |
| Complex protocol structures | ✅ Yes | In `test_complex_protocol_structure` |

**Current Coverage**: 3/3 = 100%

## Summary

| Metric | Previous | Current Value | Target | Status |
|--------|----------|--------------|--------|--------|
| Combinator Coverage | 100% | 100% | 100% | ✅ Met |
| Composition Operation Coverage | 100% | 100% | 100% | ✅ Met |
| Custom Label Type Coverage | 60% | 100% | 100% | ✅ Met |
| Label Edge Case Coverage | 0% | 100% | 80% | ✅ Exceeded |

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
       pub custom_label_types_tested: usize,
       pub target_custom_label_types: usize,
       pub edge_cases_tested: usize,
       pub target_edge_cases: usize,
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
       println!("Combinators meeting custom label type target: {}/{} ({}%)",
                CURRENT_COVERAGE.custom_label_types_tested,
                CURRENT_COVERAGE.target_custom_label_types,
                CUSTOM_LABEL_TYPES_PCT);
       println!("Edge cases tested: {}/{} ({}%)",
                CURRENT_COVERAGE.edge_cases_tested,
                CURRENT_COVERAGE.target_edge_cases,
                EDGE_CASES_PCT);
   }
   ```

## Next Steps for Phase 2 Refactoring

With all test coverage metrics now meeting or exceeding targets, we are ready to proceed with the actual label parameter refactoring in Phase 2. The implementation plan is:

1. Begin with refactoring `TEnd<IO, L>` to `TEnd<IO, Lbl>` since it has the simplest implementation
2. Run the tests to verify the refactoring works as expected
3. Continue with refactoring the remaining combinators in this order:
   - `TInteract<IO, L, R, H, T>` to `TInteract<IO, Lbl, R, H, T>`
   - `TRec<IO, L, S>` to `TRec<IO, Lbl, S>`
   - Keep `TChoice<IO, Lbl, L, R>` as is (already using `Lbl`)
   - Keep `TPar<IO, Lbl, L, R, IsDisjoint>` as is (already using `Lbl`)

---

*Last updated: May 14, 2025*