//! Bit-level and byte-level stream manipulation.
//!
//! This module provides low-level abstractions for reading and writing data at the
//! bit level, which is critical for implementing efficient encodings and verifying
//! data integrity with parity bits.

/// Bit-stream decoder for reading bits from bytes.
pub mod decoder;
/// Bit-stream encoder for writing bits to bytes.
pub mod encoder;
/// Parity bit generation and validation (Even Parity).
pub mod parity;
/// High-level bit reader.
pub mod reader;
mod tests;
/// High-level bit writer.
pub mod writer;
