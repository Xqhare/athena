mod error;
mod utils;
mod tools;

// Reexports for convenience

pub mod compression {
    pub use crate::utils::compression::lzw::*;
}
pub mod checksum {
    pub use crate::tools::checksum::crc32::*;
}
pub mod encoding_and_decoding {
    pub use crate::tools::leb128::*;
    pub use crate::utils::compression::delta::*;
    pub use crate::utils::compression::run_length::*;
}
pub mod byte_bit {
    pub use crate::tools::byte_bit::*;
}
