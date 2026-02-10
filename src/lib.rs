mod value;
mod error;
#[cfg(any(doc, feature = "compression", feature = "checksum", feature = "traits"))]
mod utils;
#[cfg(any(doc, feature = "encoding_decoding", feature = "byte_bit", feature = "bit_flags"))]
mod tools;

// Reexports for convenience

#[cfg(any(doc, feature = "compression"))]
pub mod compression {
    pub use crate::utils::compression::lzw::*;
}
#[cfg(any(doc, feature = "checksum"))]
pub mod checksum {
    pub use crate::utils::checksum::crc32::*;
}
#[cfg(any(doc, feature = "encoding_decoding"))]
pub mod encoding_and_decoding {
    pub use crate::tools::leb128::*;
    pub use crate::tools::delta::*;
    pub use crate::tools::run_length::*;
}
#[cfg(any(doc, feature = "byte_bit"))]
pub mod byte_bit {
    pub use crate::tools::byte_bit::reader::*;
    pub use crate::tools::byte_bit::writer::*;
    pub use crate::tools::byte_bit::decoder::*;
    pub use crate::tools::byte_bit::encoder::*;
}
#[cfg(any(doc, feature = "traits"))]
pub mod traits {
    pub use crate::utils::traits::signed::Signed;
    pub use crate::utils::traits::unsigned::Unsigned;
}
#[cfg(any(doc, feature = "bit_flags"))]
pub mod bit_flags {
    pub use crate::tools::bit_flags::u8_flag::U8Flag;
    pub use crate::tools::bit_flags::u16_flag::U16Flag;
    pub use crate::tools::bit_flags::u32_flag::U32Flag;
}

pub use crate::value::*;
