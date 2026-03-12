# athena
My rusty toolbox of useful tools.

## Features
- [Utilities](#utilities)
    - [Compression](#compression)
        - [LZW](#lzw)
        - [Delta encoding](#delta-encoding)
        - [Run-length encoding](#run-length-encoding)
    - [Tools](#tools)
        - [LEB128](#leb128)
        - [Bit Flags](#bit-flags)
        - [Parity](#parity)
    - [Checksums](#checksums)
        - [CRC-32](#crc-32)
    - [Bitreader](#bitreader)

## Naming
Athena is named after the Greek goddess of wisdom, warfare and handicraft.

## TODO

- [x] LEB128 signed integers
- [ ] Run-length encoding with custom pattern length
- [ ] Delta encoding writer (using LEB128)
- [ ] Refactor `value/num.rs` to use `Signed` and `Unsigned` traits for generic internal operations.
- [ ] Use AthenaResult more
- [ ] Rework Doc

---

## Utilities

### Compression
You can call the functions in the `compression` module one by one, or use `Delta` and `RunLength` in one loop.

#### LZW
A simple implementation of the Lempel-Ziv-Welch algorithm.

#### Delta encoding
A simple implementation of delta encoding.

#### Run-length encoding
A simple implementation of run-length encoding.

## Tools

### LEB128
Gated behind the `encoding_decoding` feature.
Provides standard LEB128 and XFF v3 specific variations.

```rust
use athena::encoding_and_decoding::{serialize_leb128_unsigned, deserialize_leb128_unsigned};

let bytes = serialize_leb128_unsigned(255);
let (val, len) = deserialize_leb128_unsigned(&bytes).unwrap();
assert_eq!(val, 255);
```

### Bit Flags
Gated behind the `bit_flags` feature.
A simple implementation of bit flags for `u8`, `u16`, and `u32`.

### Parity
Gated behind the `byte_bit` feature.
Provides utilities for even parity.

```rust
use athena::byte_bit::{ensure_even_parity, is_even_parity};

let marker = 0x01; // 0000 0001 (1 set bit - odd)
let parity_marker = ensure_even_parity(marker); // 1000 0001 (2 set bits - even)
assert!(is_even_parity(parity_marker));
```

### Checksums

#### CRC-32
Gated behind the `checksum` feature.
A implementation of CRC-32 (ISO-HDLC) used to detect errors in data.

```rust
use athena::checksum::crc32;

let data = b"Hello World";
let checksum = crc32(data);
```

### Bitreader
A simple function to read bits from a single byte.
