mod error;
mod utils;
mod tools;

// Reexports for convenience

pub mod compression {
    pub use crate::utils::compression::lzw::{compress_lzw, decompress_lzw};
    pub use crate::utils::compression::delta::{delta_encoder, delta_decoder};
    pub use crate::utils::compression::run_length::{run_length_encoder, run_length_decoder};
}
pub mod checksum {
    pub mod crc32 {
        pub use crate::tools::checksum::crc32::*;
    }
}
pub mod encoding {
    pub mod leb128 {
        pub use crate::tools::leb128::*;
    }
}
pub mod byte_bit {
    pub use crate::tools::byte_bit::*;
}
