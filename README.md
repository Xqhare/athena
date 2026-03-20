# Athena: Rust Utility Toolbox

Athena is a collection of low-level utilities and tools developed by Xqhare. It focuses on efficiency, minimal dependencies, and providing building blocks for other projects in the ecosystem (especially those using the XFF file format).

## Features

- **Data & Values:** Dynamic type system (`XffValue`) for XFF v2/v3, including `Table`, `Uuid`, and metadata handling.
- **Compression:** LZW, Delta, and Run-Length Encoding (RLE) implementations.
- **Encoding:** LEB128 (unsigned, signed v2, signed v3) and Bit-Chain encoding.
- **Bit Manipulation:** Bit-level reader/writer, bit flags (`u8`, `u16`, `u32`), and Even Parity utilities.
- **Checksums:** CRC-32 (ISO 3309) implementation.
- **Minimal Dependencies:** Built primarily using the Rust standard library.

## Installation

Add Athena to your `Cargo.toml`:

```toml
[dependencies]
athena = { git = "https://github.com/Xqhare/athena" }
```

### Feature Flags

Athena uses feature gates to keep the footprint small:

- `encoding_decoding`: Enables LEB128, Delta, and RLE tools.
- `compression`: Enables LZW and other compression utilities.
- `checksum`: Enables CRC-32.
- `byte_bit`: Enables bit-level reader/writer and parity tools.
- `traits`: Enables custom numeric traits (`Signed`, `Unsigned`).
- `bit_flags`: Enables bit flag utilities.
- `process`: Enables UNIX-specific process utilities.

## Usage Examples

### XffValue (Dynamic Types)

```rust
use athena::{XffValue, Number, Object};

let val = XffValue::from(42.69);
assert!(val.is_number());

let obj = XffValue::from(Object::from(vec![
    ("key".to_string(), XffValue::from("value"))
]));
```

### LEB128 Encoding

```rust
use athena::encoding_and_decoding::{serialize_leb128_unsigned, deserialize_leb128_unsigned};

let bytes = serialize_leb128_unsigned(255);
let (val, len) = deserialize_leb128_unsigned(&bytes).unwrap();
assert_eq!(val, 255);
```

### Compression (LZW)

```rust
use athena::compression::lzw_compress;

let data = b"TOBEORNOTTOBEORTOBEORNOT";
let compressed = lzw_compress(data);
```

### Bit Flags

```rust
use athena::bit_flags::U8Flag;

let mut flags = U8Flag::new();
flags.set(0);
assert!(flags.read(0));
```

## Testing

When using the `process` feature, some tests modify process-wide state (such as CPU and I/O scheduling priorities). Since Rust's default test runner executes tests in parallel threads within the same process, these tests may interfere with each other.

To ensure consistent results, it is recommended to run these tests serially:

```bash
cargo test --all-features -- --test-threads=1
```

Note: Some process priority tests (like `RealTime` priority) may fail or be skipped if the test runner does not have sufficient privileges (e.g., `CAP_SYS_NICE` or root access).

## Naming
Named after Athena, the Greek goddess of wisdom and handicraft, reflecting the library's role as a "toolbox" for building complex systems.

## License
This project is licensed under the MIT License.
