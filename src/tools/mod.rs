//! Low-level bit and byte manipulation tools.
//!
//! This module contains utilities for precise data handling, including:
//! - Bit-level reading and writing (`byte_bit`)
//! - Variable-length integer encodings (`leb128`)
//! - Data-specific encodings like Delta and Run-Length Encoding (RLE)
//! - Ergonomic bit flag management (`bit_flags`)

#[cfg(any(doc, feature = "bit_flags"))]
pub mod bit_flags;
#[cfg(any(doc, feature = "byte_bit"))]
pub mod byte_bit;
#[cfg(any(doc, feature = "encoding_decoding"))]
pub mod delta;
#[cfg(any(doc, feature = "encoding_decoding"))]
pub mod leb128;
#[cfg(any(doc, feature = "process"))]
pub mod process;
#[cfg(any(doc, feature = "encoding_decoding"))]
pub mod run_length;
#[cfg(any(doc, feature = "system"))]
pub mod system;
