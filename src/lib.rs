//! Athena: Rust Utility Toolbox
//!
//! Athena is a collection of low-level utilities and tools developed by Xqhare.
//! It focuses on efficiency, minimal dependencies, and providing building blocks for other projects in the ecosystem.

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
    feature = "sorting"
))]
mod utils;
mod value;

// Reexports for convenience

/// Process utilities
#[cfg(any(doc, feature = "process"))]
pub mod process {
    pub use crate::utils::process::*;
}
/// Compression utilities like LZW
#[cfg(any(doc, feature = "compression"))]
pub mod compression {
    pub use crate::utils::compression::lzw::*;
}
/// Checksum utilities like CRC32
#[cfg(any(doc, feature = "checksum"))]
pub mod checksum {
    pub use crate::utils::checksum::crc32::*;
}
/// Encoding and decoding utilities like LEB128 and Delta encoding
#[cfg(any(doc, feature = "encoding_decoding"))]
pub mod encoding_and_decoding {
    pub use crate::tools::delta::*;
    pub use crate::tools::leb128::bit_chain::*;
    pub use crate::tools::leb128::signed_v3::*;
    pub use crate::tools::leb128::*;
    pub use crate::tools::run_length::*;
}
/// Byte and bit level manipulation utilities
#[cfg(any(doc, feature = "byte_bit"))]
pub mod byte_bit {
    pub use crate::tools::byte_bit::decoder::*;
    pub use crate::tools::byte_bit::encoder::*;
    pub use crate::tools::byte_bit::parity::*;
    pub use crate::tools::byte_bit::reader::*;
    pub use crate::tools::byte_bit::writer::*;
}
/// Generic traits for numeric types
#[cfg(any(doc, feature = "traits"))]
pub mod traits {
    pub use crate::utils::traits::signed::Signed;
    pub use crate::utils::traits::unsigned::Unsigned;
}
/// Bit flag utilities
#[cfg(any(doc, feature = "bit_flags"))]
pub mod bit_flags {
    pub use crate::tools::bit_flags::{BitFlag, U8Flag, U16Flag, U32Flag};
}
/// Sorting algorythms
#[cfg(any(doc, feature = "sorting"))]
pub mod sorting {
    pub use crate::utils::sorting::topological_sort::kahns::*;
}

// Reexports
pub use aequa::value::*;

pub mod hp_float {
    pub use aequa::hp_float::*;
}
