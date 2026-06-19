# Kahn's Weighted Topological Sort Testing Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Export `kahns_weighted` and add a comprehensive unit test suite to verify its correctness.

**Architecture:** Export `kahns_weighted` from the top-level `sorting` module, resolve the compiler warnings in `kahns_weighted.rs` for unused fields, and add inline unit tests.

**Tech Stack:** Rust (Edition 2024), Cargo.

---

### Task 1: Resolve Unused Variable Warning

**Files:**
- Modify: `src/utils/sorting/topological_sort/kahns_weighted.rs:63-63`

- [ ] **Step 1: Replace unused binding**

Replace line 63:
```rust
    for (name, priority, children) in input {
```
with:
```rust
    for (name, priority, _) in input {
```

- [ ] **Step 2: Verify compile warning is resolved**

Run: `cargo check --all-features`
Expected output: No warnings for unused variable `children`.

- [ ] **Step 3: Commit**

```bash
git add src/utils/sorting/topological_sort/kahns_weighted.rs
git commit -m "fix(sorting): resolve unused variable warning in kahns_weighted"
```

---

### Task 2: Export `kahns_weighted`

**Files:**
- Modify: `src/lib.rs:101-105`

- [ ] **Step 1: Re-export in public module**

Replace lines 101-105:
```rust
/// Sorting algorithms, including a deterministic Kahn's Topological Sort.
#[cfg(any(doc, feature = "sorting"))]
pub mod sorting {
    pub use crate::utils::sorting::topological_sort::kahns::*;
}
```
with:
```rust
/// Sorting algorithms, including a deterministic Kahn's Topological Sort.
#[cfg(any(doc, feature = "sorting"))]
pub mod sorting {
    pub use crate::utils::sorting::topological_sort::kahns::*;
    pub use crate::utils::sorting::topological_sort::kahns_weighted::*;
}
```

- [ ] **Step 2: Verify compilation passes**

Run: `cargo check --all-features`
Expected output: Compiles cleanly with no unused struct/function warnings for `Library`, `HeapElement`, `kahns_weighted`, `build_libraries`, and `topological_sort`.

- [ ] **Step 3: Commit**

```bash
git add src/lib.rs
git commit -m "feat(sorting): export kahns_weighted from public API"
```

---

### Task 3: Implement Positive Unit Test Cases

**Files:**
- Modify: `src/utils/sorting/topological_sort/kahns_weighted.rs:147-147`

- [ ] **Step 1: Add positive unit tests**

Append the tests module and positive test cases at the bottom of the file (after line 147):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_sort() {
        // Simple chain: A depends on B, B depends on C
        let input = vec![
            ("A", 0, vec!["B"]),
            ("B", 0, vec!["C"]),
            ("C", 0, vec![]),
        ];
        let result = kahns_weighted(&input);
        assert_eq!(result, Ok(vec!["C", "B", "A"]));
    }

    #[test]
    fn test_priority_ordering() {
        // No dependencies. A (priority 2), B (priority 1), C (priority 3).
        // Since smaller priority value is higher priority:
        // Priority order: B (1), A (2), C (3)
        let input = vec![
            ("A", 2, vec![]),
            ("B", 1, vec![]),
            ("C", 3, vec![]),
        ];
        let result = kahns_weighted(&input);
        assert_eq!(result, Ok(vec!["B", "A", "C"]));
    }

    #[test]
    fn test_alphabetical_tie_breaking() {
        // No dependencies. Same priority (0).
        // A, B, C should sort alphabetically (A before Z)
        let input = vec![
            ("C", 0, vec![]),
            ("A", 0, vec![]),
            ("B", 0, vec![]),
        ];
        let result = kahns_weighted(&input);
        assert_eq!(result, Ok(vec!["A", "B", "C"]));
    }

    #[test]
    fn test_priority_and_alphabetical_mix() {
        // A (priority 2), B (priority 1), C (priority 1).
        // B and C have same priority, so alphabetical tie-breaker: B before C.
        // A has lower priority (2), so it comes after.
        let input = vec![
            ("A", 2, vec![]),
            ("C", 1, vec![]),
            ("B", 1, vec![]),
        ];
        let result = kahns_weighted(&input);
        assert_eq!(result, Ok(vec!["B", "C", "A"]));
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test --all-features --test athena -- utils::sorting::topological_sort::kahns_weighted`
Expected output: 4 passed; 0 failed.

- [ ] **Step 3: Commit**

```bash
git add src/utils/sorting/topological_sort/kahns_weighted.rs
git commit -m "test(sorting): add positive test cases for kahns_weighted"
```

---

### Task 4: Implement Negative and Fallback Unit Test Cases

**Files:**
- Modify: `src/utils/sorting/topological_sort/kahns_weighted.rs:194-194` (at the end of `mod tests`)

- [ ] **Step 1: Add cycle, duplicate, and default priority tests**

Append these tests inside `mod tests`:
```rust
    #[test]
    fn test_cycle_detection() {
        // A depends on B, B depends on A
        let input = vec![
            ("A", 0, vec!["B"]),
            ("B", 0, vec!["A"]),
        ];
        let result = kahns_weighted(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cycle detected"));
    }

    #[test]
    fn test_duplicate_node_definition() {
        let input = vec![
            ("A", 0, vec![]),
            ("A", 1, vec![]),
        ];
        let result = kahns_weighted(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Duplicate node definition"));
    }

    #[test]
    fn test_missing_child_filled_with_default_priority() {
        // A depends on B. B is not explicitly defined in the input.
        // B should be filled in with a default priority of 0.
        // Since B has no dependencies, it runs first.
        let input = vec![
            ("A", 1, vec!["B"]),
        ];
        let result = kahns_weighted(&input);
        assert_eq!(result, Ok(vec!["B", "A"]));
    }
```

- [ ] **Step 2: Run all tests**

Run: `cargo test --all-features`
Expected output: All 52 unit tests and 25 doc-tests pass successfully.

- [ ] **Step 3: Commit**

```bash
git add src/utils/sorting/topological_sort/kahns_weighted.rs
git commit -m "test(sorting): add negative and fallback test cases for kahns_weighted"
```
