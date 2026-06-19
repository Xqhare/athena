# Design Spec: Testing and Exporting Kahn's Weighted Topological Sort

## Overview
This document specifies the design for exporting and adding comprehensive unit tests to the `kahns_weighted` module inside the Athena library. Currently, this function is defined but not exported outside its submodule, resulting in unused code warnings during compilation. In addition, there is an unused variable warning for `children` inside the `build_libraries` function.

## Goals & Success Criteria
1. **Export Functionality**: Export `kahns_weighted` from the top-level `sorting` module in [src/lib.rs](file:///home/xqhare/Adytum/Programming/rust/athena/src/lib.rs).
2. **Resolve Compiler Warnings**: Ensure `cargo build --all-features` and `cargo test --all-features` compile cleanly without warnings for `kahns_weighted`.
3. **Robust Unit Tests**: Cover edge cases, priority ordering, alphabetical tie-breaking, cycle detection, duplicate node definitions, and default priority fallbacks.
4. **No External Dependencies**: Keep standard library alignment, using no external crates (only standard Rust features).

## Proposed Changes

### 1. Library Interface Update
Re-export the function in the public API in [src/lib.rs](file:///home/xqhare/Adytum/Programming/rust/athena/src/lib.rs):
```rust
/// Sorting algorithms, including a deterministic Kahn's Topological Sort.
#[cfg(any(doc, feature = "sorting"))]
pub mod sorting {
    pub use crate::utils::sorting::topological_sort::kahns::*;
    pub use crate::utils::sorting::topological_sort::kahns_weighted::*;
}
```

### 2. Warn-Clean Implementation
Modify the destructuring loop in `build_libraries` within [kahns_weighted.rs](file:///home/xqhare/Adytum/Programming/rust/athena/src/utils/sorting/topological_sort/kahns_weighted.rs) to prevent compiler warnings:
```rust
    // 1. Ensure all mentioned nodes exist with default priorities
    for (name, priority, _) in input {
```

### 3. Inline Unit Tests
Add a `#[cfg(test)] mod tests` block at the bottom of [kahns_weighted.rs](file:///home/xqhare/Adytum/Programming/rust/athena/src/utils/sorting/topological_sort/kahns_weighted.rs) containing:
- `test_basic_sort`: Tests a simple chain `C <- B <- A`.
- `test_priority_ordering`: Tests nodes with different priority weights (where lower values mean higher priority).
- `test_alphabetical_tie_breaking`: Tests that nodes with identical priority resolve alphabetically.
- `test_priority_and_alphabetical_mix`: Mixes priority difference and alphabetical resolving.
- `test_cycle_detection`: Asserts that an error is returned when a cycle is present.
- `test_duplicate_node_definition`: Asserts that a duplicate definition returns an error.
- `test_missing_child_filled_with_default_priority`: Verifies default fallback logic (default priority of 0).
