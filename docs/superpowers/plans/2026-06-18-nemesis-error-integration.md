# Nemesis Error Integration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Update the `athena` crate to use the `nemesis` error handling crate by redefining `AthenaResult` and implementing automatic conversion from `AthenaError` to `NemesisError`.

**Architecture:** Redefine `AthenaResult<T>` as `Result<T, nemesis::NemesisError>` and implement `From<AthenaError> for nemesis::NemesisError`. Update signatures and error return paths across the library, using `?` for automatic conversion and `.into()` for direct returns of `AthenaError`.

**Tech Stack:** Rust (Edition 2024), nemesis

---

### Task 1: Redefine `AthenaResult` and `From` Conversion

**Files:**
- Modify: `src/error/mod.rs`

- [ ] **Step 1: Modify `src/error/mod.rs`**
  Modify [src/error/mod.rs](file:///home/xqhare/Adytum/Programming/rust/athena/src/error/mod.rs) to:
  1. Add `use nemesis::NemesisError;` at the top.
  2. Redefine `AthenaResult` as `Result<T, NemesisError>`.
  3. Implement `From<AthenaError> for NemesisError`.
  4. Add a test case `test_nemesis_error_conversion` verifying that `AthenaError` converts to `NemesisError` with source `"Athena"`, and can be downcasted back.

  ```rust
  // Modify imports at the top
  use nemesis::NemesisError;

  // Redefine AthenaResult
  pub type AthenaResult<T> = Result<T, nemesis::NemesisError>;

  // Implement From
  impl From<AthenaError> for nemesis::NemesisError {
      fn from(err: AthenaError) -> Self {
          nemesis::NemesisError::new("Athena", err)
      }
  }

  // Add at the bottom of the file (or inside mod tests)
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_nemesis_error_conversion() {
          let err = AthenaError::ParityError;
          let nemesis_err: nemesis::NemesisError = err.into();
          assert_eq!(nemesis_err.source_name(), "Athena");
          let downcasted = nemesis_err.downcast_ref::<AthenaError>();
          assert!(downcasted.is_some());
          assert!(matches!(downcasted.unwrap(), AthenaError::ParityError));
      }
  }
  ```

- [ ] **Step 2: Try to run `cargo check` to verify initial changes compile**
  Run: `cargo check`
  Expected: Success in compiling `nemesis` and `src/error/mod.rs`, but errors in other modules due to mismatching signatures (expected signature changes).

- [ ] **Step 3: Commit**
  Run:
  ```bash
  git add src/error/mod.rs
  git commit -m "refactor(error): redefine AthenaResult and impl From<AthenaError> for NemesisError"
  ```

---

### Task 2: Update `byte_bit` tools

**Files:**
- Modify: `src/tools/byte_bit/reader.rs`
- Modify: `src/tools/byte_bit/writer.rs`

- [x] **Step 1: Modify `src/tools/byte_bit/reader.rs`**
  Update the imports to include `AthenaResult` and change the return type of `byte_bit_reader` to `AthenaResult<Vec<Vec<u8>>>`, and update direct `Err(AthenaError)` instantiation to call `.into()`.

  ```rust
  use crate::error::{AthenaError, AthenaResult};
  // ...
  pub fn byte_bit_reader<R: std::io::Read>(reader: R) -> AthenaResult<Vec<Vec<u8>>> {
      let mut out: Vec<Vec<u8>> = Vec::with_capacity(64);
      for byte in reader.bytes() {
          match byte {
              Ok(byte) => out.push(byte_bit_decoder(byte)),
              Err(e) => return Err(AthenaError::IoError(e).into()),
          }
      }
      Ok(out)
  }
  ```

- [x] **Step 2: Modify `src/tools/byte_bit/writer.rs`**
  Update the imports to include `AthenaResult` and update the return signature of `byte_bit_writer` to `AthenaResult<()>`.

  ```rust
  use crate::error::{AthenaError, AthenaResult};
  // ...
  pub fn byte_bit_writer<W: std::io::Write>(
      writer: &mut W,
      bytes: Vec<[u8; 8]>,
  ) -> AthenaResult<()> {
      for byte in bytes {
          writer
              .write_all(&[byte_bit_encoder(&byte)])
              .map_err(AthenaError::IoError)?;
      }
      Ok(())
  }
  ```

- [x] **Step 3: Verify the changes by building the `byte_bit` tests**
  Run: `cargo test --lib tools::byte_bit`
  Expected: PASS

- [x] **Step 4: Commit**
  Run:
  ```bash
  git add src/tools/byte_bit/reader.rs src/tools/byte_bit/writer.rs
  git commit -m "refactor(byte_bit): update signatures to use NemesisError via AthenaResult"
  ```

---

### Task 3: Update `leb128` tools

**Files:**
- Modify: `src/tools/leb128/bit_chain.rs`
- Modify: `src/tools/leb128/mod.rs`
- Modify: `src/tools/leb128/signed_v3.rs`
- Modify: `src/tools/leb128/tests.rs`

- [x] **Step 1: Modify `src/tools/leb128/bit_chain.rs`**
  Update `deserialize_version_bit_chain` return type to `AthenaResult<(usize, u8)>` and update direct return error statement to use `.into()`.

  ```rust
  use crate::error::{AthenaError, AthenaResult};
  // ...
  pub fn deserialize_version_bit_chain(data: &[u8]) -> AthenaResult<(usize, u8)> {
      let mut version = 0;
      let mut num_of_bytes = 0;

      loop {
          if num_of_bytes >= data.len() {
              return Err(AthenaError::ContinuationBitInLastByte.into());
          }
          // ...
  ```

- [x] **Step 2: Modify `src/tools/leb128/mod.rs`**
  Update the signatures for `deserialize_leb128_unsigned` and `deserialize_leb128_signed` to use `AthenaResult`, and append `.into()` to the explicit error returns.

  ```rust
  use crate::error::{AthenaError, AthenaResult};
  // ...
  pub fn deserialize_leb128_unsigned(data: &[u8]) -> AthenaResult<(u128, u8)> {
      let mut result: u128 = 0;
      let mut shift = 0;
      let mut num_of_bytes: u8 = 0;
      loop {
          if num_of_bytes as usize >= data.len() {
              return Err(AthenaError::ContinuationBitInLastByte.into());
          }
          let byte = data[num_of_bytes as usize];
          num_of_bytes += 1;
          let low_bits = u128::from(low_bits_of_byte(byte));
          if shift < 128 {
              result |= low_bits << shift;
          } else if low_bits != 0 {
              return Err(AthenaError::Overflow.into());
          }
          if byte & CONTINUATION_BIT == 0 {
              return Ok((result, num_of_bytes));
          }
          shift += 7;
      }
  }

  // ...
  pub fn deserialize_leb128_signed(data: &[u8]) -> AthenaResult<(i128, u8)> {
      let mut result: i128 = 0;
      let mut shift = 0;
      let size = 128;
      let mut byte;
      let mut index: u8 = 0;

      loop {
          if index as usize >= data.len() {
              return Err(AthenaError::ContinuationBitInLastByte.into());
          }

          byte = data[index as usize];
          index += 1;
          // ...
          if shift >= 133 {
              return Err(AthenaError::Overflow.into());
          }
      }
      // ...
  ```

- [x] **Step 3: Modify `src/tools/leb128/signed_v3.rs`**
  Update signatures for `deserialize_leb128_signed_v3` and `deserialize_leb128_signed_i128` to return `AthenaResult`, and append `.into()` to the explicit error returns.

  ```rust
  use crate::error::{AthenaError, AthenaResult};
  // ...
  pub fn deserialize_leb128_signed_v3(data: &[u8]) -> AthenaResult<(i64, u8)> {
      if data.is_empty() {
          return Err(AthenaError::ContinuationBitInLastByte.into());
      }
      // ...
      if (first_byte & CONTINUATION_BIT) != 0 {
          let mut shift = 6;
          loop {
              if num_of_bytes >= data.len() {
                  return Err(AthenaError::ContinuationBitInLastByte.into());
              }
              // ...
              if shift >= 64 {
                  return Err(AthenaError::Overflow.into());
              }
          }
      }
      // ...
  }

  pub fn deserialize_leb128_signed_i128(data: &[u8]) -> AthenaResult<(i128, u8)> {
      if data.is_empty() {
          return Err(AthenaError::ContinuationBitInLastByte.into());
      }
      // ...
      if (first_byte & CONTINUATION_BIT) != 0 {
          let mut shift = 6;
          loop {
              if num_of_bytes >= data.len() {
                  return Err(AthenaError::ContinuationBitInLastByte.into());
              }
              // ...
              if shift >= 128 {
                  return Err(AthenaError::Overflow.into());
              }
          }
      }
      // ...
  }
  ```

- [x] **Step 4: Modify `src/tools/leb128/tests.rs`**
  Update imports to use `AthenaResult` if needed.
  ```rust
  // Modify imports at line 78-81:
  use crate::{
      error::{AthenaError, AthenaResult},
      tools::leb128::{deserialize_leb128_signed, serialize_leb128_signed},
  };
  ```

- [x] **Step 5: Run tests for `leb128`**
  Run: `cargo test --lib tools::leb128`
  Expected: PASS

- [x] **Step 6: Commit**
  Run:
  ```bash
  git add src/tools/leb128/bit_chain.rs src/tools/leb128/mod.rs src/tools/leb128/signed_v3.rs src/tools/leb128/tests.rs
  git commit -m "refactor(leb128): update signatures to return AthenaResult"
  ```

---

### Task 4: Update `process` and `system` tools

**Files:**
- Modify: `src/tools/process/mod.rs`
- Modify: `src/tools/process/unix.rs`
- Modify: `src/tools/system/mod.rs`

- [x] **Step 1: Modify `src/tools/process/mod.rs`**
  No logic changes needed since return types are already `AthenaResult`, but we must update signature direct error returns in the mock `not(unix)` functions:

  ```rust
  #[cfg(not(unix))]
  use crate::error::{AthenaError, AthenaResult};
  // ...
  #[cfg(not(unix))]
  pub fn set_nice_value(_priority: i32) -> AthenaResult<()> {
      Err(AthenaError::UnsupportedPlatform.into())
  }

  #[cfg(not(unix))]
  pub fn set_scheduler(_policy: SchedulerPolicy, _priority: i32) -> AthenaResult<()> {
      Err(AthenaError::UnsupportedPlatform.into())
  }

  #[cfg(not(unix))]
  pub fn set_ionice_value(_class: IoNiceClass, _class_data: u32) -> AthenaResult<()> {
      Err(AthenaError::UnsupportedPlatform.into())
  }
  ```

- [x] **Step 2: Modify `src/tools/process/unix.rs`**
  Update explicit `Err` returns to use `.into()` to convert `AthenaError` to `NemesisError`.

  ```rust
  pub fn set_nice_value(priority: i32) -> AthenaResult<()> {
      unsafe {
          if libc::setpriority(libc::PRIO_PROCESS, 0, priority) == -1 {
              return Err(AthenaError::IoError(std::io::Error::last_os_error()).into());
          }
      }
      Ok(())
  }

  pub fn set_scheduler(policy: SchedulerPolicy, priority: i32) -> AthenaResult<()> {
      // ...
      unsafe {
          if libc::sched_setscheduler(0, libc_policy, &raw const param) == -1 {
              return Err(AthenaError::IoError(std::io::Error::last_os_error()).into());
          }
      }
      // ...
  }

  pub fn set_ionice_value(class: IoNiceClass, class_data: u32) -> AthenaResult<()> {
      // ...
      if libc_class != 0 && libc_class != 3 {
          if class_data > 7 {
              return Err(AthenaError::InvalidInput.into());
          }
          command.arg("-n").arg(format!("{class_data}"));
      }
      // ...
      if !status.success() {
          return Err(AthenaError::IoError(std::io::Error::other(
              "ionice command failed",
          )).into());
      }
      Ok(())
  }

  pub fn get_ionice_value() -> AthenaResult<(IoNiceClass, u32)> {
      // ...
      if stdout.is_empty() {
          return Err(AthenaError::InvalidOutput.into());
      }
      // ...
      let io_class = if class_str.contains("none") {
          IoNiceClass::None
      } else if class_str.contains("real") {
          IoNiceClass::RealTime
      } else if class_str.contains("best") {
          IoNiceClass::BestEffort
      } else if class_str.contains("idle") {
          IoNiceClass::Idle
      } else {
          return Err(AthenaError::InvalidOutput.into());
      };

      Ok((io_class, prio))
  }
  ```

- [x] **Step 3: Modify `src/tools/system/mod.rs`**
  Update direct returns to use `AthenaError::from(...).into()` (or `AthenaError::IoError(...).into()`) since `std::io::Error` does not automatically convert to `NemesisError`.

  ```rust
  use crate::error::{AthenaError, AthenaResult};
  // ...
  pub fn terminal_size(fd_stdout: RawFd) -> AthenaResult<(u16, u16)> {
      unsafe {
          let mut winsize: winsize = zeroed();
          if ioctl(fd_stdout, TIOCGWINSZ, &mut winsize) != 0 {
              return Err(AthenaError::from(std::io::Error::last_os_error()).into());
          }
          Ok((winsize.ws_row, winsize.ws_col))
      }
  }
  ```

- [x] **Step 4: Run process/system tests**
  Run: `cargo test --lib tools::process --lib tools::system`
  Expected: PASS

- [x] **Step 5: Commit**
  Run:
  ```bash
  git add src/tools/process/mod.rs src/tools/process/unix.rs src/tools/system/mod.rs
  git commit -m "refactor(process,system): return NemesisError via AthenaResult"
  ```

---

### Task 5: Update `lzw` compression

**Files:**
- Modify: `src/utils/compression/lzw/mod.rs`

- [ ] **Step 1: Modify `src/utils/compression/lzw/mod.rs`**
  Import `AthenaResult` and update `decompress_lzw_decode_leb128` return signature.

  ```rust
  use crate::{
      encoding_and_decoding::{deserialize_leb128_unsigned, serialize_leb128_unsigned},
      error::{AthenaError, AthenaResult},
  };
  // ...
  pub fn decompress_lzw_decode_leb128(data: &[u8]) -> AthenaResult<Vec<u8>> {
      // ...
  ```

- [ ] **Step 2: Run compression tests**
  Run: `cargo test --lib utils::compression::lzw`
  Expected: PASS

- [ ] **Step 3: Commit**
  Run:
  ```bash
  git add src/utils/compression/lzw/mod.rs
  git commit -m "refactor(lzw): update decompress return type to AthenaResult"
  ```

---

### Task 6: Final Verification & Clippy

**Files:**
- None

- [ ] **Step 1: Run all tests with all features**
  Run: `cargo test --all-features`
  Expected: PASS

- [ ] **Step 2: Run clippy**
  Run: `cargo clippy --all-features`
  Expected: No errors

- [ ] **Step 3: Commit**
  Run:
  ```bash
  git commit --allow-empty -m "test(all): verify all tests pass with nemesis integration"
  ```
