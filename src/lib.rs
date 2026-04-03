#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::all)]

#[cfg(any(
    doc,
    feature = "encoding_decoding",
    feature = "byte_bit",
    feature = "bit_flags",
    feature = "compression",
    feature = "checksum",
    feature = "traits",
    feature = "process"
))]
mod error;
#[cfg(any(
    doc,
    feature = "encoding_decoding",
    feature = "byte_bit",
    feature = "bit_flags"
))]
mod tools;
#[cfg(any(
    doc,
    feature = "compression",
    feature = "checksum",
    feature = "traits",
    feature = "process",
    feature = "sorting",
    feature = "rng_api",
))]
mod utils;

// Reexports for convenience

/// Process utilities for UNIX systems, including priority and scheduling management.
#[cfg(any(doc, feature = "process"))]
pub mod process {
    pub use crate::utils::process::*;
}

/// Compression utilities including LZW and combined LEB128 implementations.
#[cfg(any(doc, feature = "compression"))]
pub mod compression {
    pub use crate::utils::compression::lzw::*;
    // pub use crate::utils::compression::delta_with_run_length::*;
}

/// Checksum utilities, featuring a robust CRC-32 (ISO 3309) implementation.
#[cfg(any(doc, feature = "checksum"))]
pub mod checksum {
    pub use crate::utils::checksum::crc32::*;
}

/// Encoding and decoding utilities, including LEB128 variants, Delta, and RLE.
#[cfg(any(doc, feature = "encoding_decoding"))]
pub mod encoding_and_decoding {
    pub use crate::tools::delta::*;
    pub use crate::tools::leb128::bit_chain::*;
    pub use crate::tools::leb128::signed_v3::*;
    pub use crate::tools::leb128::*;
    pub use crate::tools::run_length::*;
}

/// Bit and byte level manipulation tools, including readers, writers, and parity checks.
#[cfg(any(doc, feature = "byte_bit"))]
pub mod byte_bit {
    pub use crate::tools::byte_bit::decoder::*;
    pub use crate::tools::byte_bit::encoder::*;
    pub use crate::tools::byte_bit::parity::*;
    pub use crate::tools::byte_bit::reader::*;
    pub use crate::tools::byte_bit::writer::*;
}

/// Generic numeric traits for signed and unsigned types.
#[cfg(any(doc, feature = "traits"))]
pub mod traits {
    pub use crate::utils::traits::signed::Signed;
    pub use crate::utils::traits::unsigned::Unsigned;
}

/// Bit flag utilities for ergonomic management of u8, u16, and u32 flags.
#[cfg(any(doc, feature = "bit_flags"))]
pub mod bit_flags {
    pub use crate::tools::bit_flags::{BitFlag, U8Flag, U16Flag, U32Flag};
}

/// Sorting algorithms, including a deterministic Kahn's Topological Sort.
#[cfg(any(doc, feature = "sorting"))]
pub mod sorting {
    pub use crate::utils::sorting::topological_sort::kahns::*;
}

/// Random Number Generator API
#[cfg(any(doc, feature = "rng_api"))]
pub mod rng_api {
    pub use crate::utils::rng_api::*;
}

// Ecosystem Reexports
pub use aequa::value::*;

/// Reexports High Precision Float from the aequa crate
pub mod float {
    pub use aequa::hp_float::*;
}

/// Reexports Graph from the aequa crate
pub mod graph {
    pub use aequa::graph::*;
}
