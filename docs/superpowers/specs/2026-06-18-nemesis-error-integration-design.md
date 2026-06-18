# Spec: Nemesis Error Integration in Athena

**Date:** 2026-06-18  
**Status:** Approved  
**Topic:** Integrating the new `nemesis` error handling crate into `athena`.

---

## 1. Overview
The goal of this change is to update `athena` to use the `nemesis` error handling crate. We will preserve the structured domain-specific error enumeration `AthenaError`, but redefine `AthenaResult<T>` to wrap `nemesis::NemesisError`. This will allow `athena` to accumulate error contexts, isolate boundary layers, and propagate errors with structured trace information.

---

## 2. Architecture & Design Decisions

### Redefined Result Type
`AthenaResult<T>` will be changed from `Result<T, AthenaError>` to `Result<T, nemesis::NemesisError>`. This ensures all public APIs in `athena` return `NemesisError` as the top-level error.

### Retaining `AthenaError` as Leaf Error
`AthenaError` remains a public enum implementing `std::error::Error`, representing the local domain-specific failures (e.g. `ParityError`, `ContinuationBitInLastByte`). When an `athena` operation fails, the resulting `AthenaError` is wrapped inside a `NemesisError` using `NemesisError::new("Athena", err)`.

### Automatic Conversion
An implementation of `From<AthenaError>` for `nemesis::NemesisError` will map any local `AthenaError` automatically to `NemesisError` with a source of `"Athena"`:

```rust
impl From<AthenaError> for nemesis::NemesisError {
    fn from(err: AthenaError) -> Self {
        nemesis::NemesisError::new("Athena", err)
    }
}
```

This ensures that we can use the `?` operator on functions returning `Result<T, AthenaError>` to convert them automatically into `Result<T, NemesisError>`.

---

## 3. Specific File Changes

### `athena/src/error/mod.rs`
1. Add import: `use nemesis::NemesisError;`
2. Update `AthenaResult<T>`:
   ```rust
   pub type AthenaResult<T> = Result<T, nemesis::NemesisError>;
   ```
3. Implement `From<AthenaError> for nemesis::NemesisError`.
4. Keep standard trait implementations like `From<std::io::Error> for AthenaError` and `From<std::num::ParseIntError> for AthenaError` to facilitate quick error coercion inside internal modules.

### Internal Crate Modules
Any module that uses `AthenaResult` or returns an error must be compatible with the new signature. In most cases, internal methods returning `Result<T, AthenaError>` will automatically convert to `AthenaResult<T>` at boundaries when caller functions use `?`.

---

## 4. Test & Verification Plan

### Automated Tests
1. Run `cargo test` in the `athena` repository.
2. Add a new unit test in `athena/src/error/mod.rs` (or equivalent test module) to verify:
   - Successful wrapping of an `AthenaError` inside a `NemesisError`.
   - The error source name is `"Athena"`.
   - Downcasting of `NemesisError` back to `AthenaError` works correctly.
3. Validate that existing tests compile and run properly with the updated result signatures.
