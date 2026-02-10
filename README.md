# athena
My rusty toolbox of useful tools.

## Features
- [Utilities](#utilities)
    - [Compression](#compression)
        - [LZW](#lzw)
        - [Delta encoding](#delta-encoding)
        - [Run-length encoding](#run-length-encoding)
    - [Tools](#tools)
    - [Checksums](#checksums)
        - [CRC-32](#crc-32)
    - [Bitreader](#bitreader)

## Naming
Athena is named after the Greek goddess of wisdom, warfare and handicraft.

## TODO

- [x] LEB128 signed integers
- [ ] Run-length encoding with custom pattern length
- [ ] Delta encoding writer (using LEB128)

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

### Bit Flags

A simple implementation of bit flags.

Implemented for:

- `u8`
- `u16`
- `u32`

Gated behind the `bit_flags` feature.

### Checksums

#### CRC-32
A implementation of CRC-32, a checksum algorithm that is used to detect errors in data.

It should be in accordance with ISO 3309.

### Bitreader
A simple function to read bits from a single byte.
