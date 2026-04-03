# Athena: Rust Utility Toolbox

Athena is a collection of low-level utilities and tools that serve as fundamental building blocks for my ecosystem. It focuses on efficiency, minimal dependencies, and providing high-performance implementations of core algorithms, particularly those required for the XFF (Xqhare File Format) ecosystem.

## Project Overview

Athena acts as the "computational layer" of the ecosystem, sitting between the logical data representation provided by **Aequa** and the physical on-disk serialization handled by **Nabu**.

- **Purpose**: A "rusty toolbox" featuring compression, bit manipulation, checksums, and other low-level utilities.
- **Core Technology**: Built with Rust (Edition 2024), maintaining a strict "no external dependencies" rule (standard library and libc only).
- **Naming**: Named after the Greek goddess of wisdom and handicraft, reflecting its role as a versatile set of tools for building complex systems.
- **Architecture**: Modularized into `tools` (lower level code, like bit-level operations and encodings) and `utils` (higher-level code, like algorithms like compression and sorting).

## Key Features

### 1. Bit Manipulation & Encoding (`src/tools/`)

Athena provides precise control over data at the bit and byte level, essential for efficient file formats and network protocols.

- **`byte_bit`**: Bit-level reader and writer implementations, including support for **Even Parity** generation and validation as used in XFF v3.
- **`bit_flags`**: Ergonomic utilities for managing bit flags on `u8`, `u16`, and `u32` types.
- **`leb128`**: 
    - **Unsigned**: Standard variable-length integer encoding.
    - **Signed (v2)**: DWARF-style signed variable-length integers.
    - **Signed (v3)**: Custom XFF v3 encoding where the sign bit is stored in the first byte for faster decoding.
    - **Bit-Chain**: A custom unary bit-chain encoding used for version metadata.

### 2. Compression & Encoding (`src/utils/compression/`)

High-performance implementations of classic and custom compression algorithms.

- **LZW**: A complete Lempel-Ziv-Welch implementation, including combined LEB128 encoding/decoding.
- **Delta Encoding**: Simple and efficient delta encoding for numerical sequences.
- **Run-Length Encoding (RLE)**: Standard RLE for repetitive data patterns.
- **Combined Utilities**: Specialized combinations like `delta_with_run_length` for specific data profiles.

### 3. System & Core Utilities (`src/utils/`)

Low-level system interactions and fundamental algorithms.

- **Checksums**: A robust CRC-32 (ISO 3309) implementation for data integrity verification.
- **Topological Sort**: A deterministic implementation of Kahn's algorithm for dependency resolution.
- **Process Utilities**: UNIX-specific tools for system monitoring and process priority management (e.g., CPU and I/O scheduling).
- **Generic Traits**: Custom `Signed` and `Unsigned` traits to facilitate generic numeric operations across different bit-widths.
- **RngApi**: A unified interface for random number generation, for implementations like [fortuna](https://github.com/xqhare/fortuna) and [tyche](https://github.com/xqhare/tyche.

### 4. Ecosystem Integration (via `Aequa`)

Athena re-exports core types from **Aequa** to provide a unified interface for data manipulation.

- **`XffValue`**: The central dynamic enum for XFF data types.
- **`HpFloat`**: High-precision decimal floating-point arithmetic.
- **`Table`**: Schema-based multi-row data structures for efficient storage.

## Philosophy: No External Dependencies

Following the "The Pantheon" convention, Athena adheres to a strict "no external dependencies" rule. All algorithms are implemented from scratch or utilize only the Rust standard library and `libc`. This ensures:

- **Security**: Zero supply-chain risk from third-party crates.
- **Stability**: Complete control over the codebase and its evolution.
- **Portability**: Minimal requirements for integration into other projects.

## Installation

Add Athena to your `Cargo.toml`:

```toml
[dependencies]
athena = { git = "https://github.com/xqhare/athena" }
```

### Feature Flags

Athena uses feature gates to keep the library lean. Only the utilities you need are compiled.

- `encoding_decoding`: Enables LEB128, Delta, and RLE tools.
- `compression`: Enables LZW and other compression utilities.
- `checksum`: Enables CRC-32.
- `byte_bit`: Enables bit-level reader/writer and parity tools.
- `traits`: Enables custom numeric traits.
- `bit_flags`: Enables bit flag utilities.
- `process`: Enables UNIX-specific process utilities.
- `sorting`: Enables sorting algorithms (e.g., Kahn's Topological Sort).

## Usage Examples

### LEB128 Encoding
```rust
use athena::encoding_and_decoding::{serialize_leb128_unsigned, deserialize_leb128_unsigned};

let bytes = serialize_leb128_unsigned(255);
let (val, len) = deserialize_leb128_unsigned(&bytes).unwrap();
assert_eq!(val, 255);
```

### LZW Compression
```rust
use athena::compression::compress_lzw_encode_leb128;

let data = b"TOBEORNOTTOBEORTOBEORNOT";
let compressed = compress_lzw_encode_leb128(data);
```

### Bit Flags
```rust
use athena::bit_flags::U8Flag;

let mut flags = U8Flag::new();
flags.set(0);
assert!(flags.read(0));
```

## Testing and Validation

Athena maintains a rigorous testing suite. Because some process utilities modify system-wide state, it is recommended to run tests serially.

- **Run all tests**: `cargo test --all-features -- --test-threads=1`
- **Doc tests**: `cargo test --doc`

Note: Certain `process` tests (like real-time priority) may require elevated privileges (`CAP_SYS_NICE` or root) to pass.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
