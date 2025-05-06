mod error;
mod utils;
mod tools;

// Reexports for convenience

pub mod compression {
    pub use crate::utils::compression::lzw::*;
}
pub mod checksum {
    pub use crate::utils::checksum::crc32::*;
}
pub mod encoding_and_decoding {
    pub use crate::tools::leb128::*;
    pub use crate::tools::delta::*;
    pub use crate::tools::run_length::*;
}
pub mod byte_bit {
    pub use crate::tools::byte_bit::reader::byte_bit_reader;
    pub use crate::tools::byte_bit::writer::byte_bit_writer;
    pub use crate::tools::byte_bit::decoder::byte_bit_decoder;
    pub use crate::tools::byte_bit::encoder::byte_bit_encoder;
}
