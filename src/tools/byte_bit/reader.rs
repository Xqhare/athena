use crate::error::AthenaError;

use super::decoder::byte_bit_decoder;

/// Reads the bits from a io::Reader
///
/// The bits will be in the order of least significant bit to most significant bit
///
/// # Example
/// ```
/// # use athena::byte_bit::byte_bit_reader;
/// let bytes = vec![0b11110000, 0b00001111];
/// let bits = byte_bit_reader(bytes.as_slice()).unwrap();
/// let expected: Vec<[u8; 8]> = vec![[0, 0, 0, 0, 1, 1, 1, 1], [1, 1, 1, 1, 0, 0, 0, 0]];
/// assert_eq!(bits.to_vec(), expected);
/// ```
pub fn byte_bit_reader<R: std::io::Read>(reader: R) -> Result<Vec<Vec<u8>>, AthenaError> {
    // min size 16 would only be enough for 2 bytes
    let mut out: Vec<Vec<u8>> = Vec::with_capacity(64);
    for byte in reader.bytes() {
        match byte {
            Ok(byte) => out.push(byte_bit_decoder(byte)),
            Err(e) => return Err(AthenaError::IoError(e)),
        }
    }
    Ok(out)
}

#[test]
fn byte_bit_reader_basics() {
    let byte0 = vec![0b10101010, 0b10101010];
    let bits0 = byte_bit_reader(byte0.as_slice());
    assert!(bits0.is_ok());
    assert_eq!(bits0.unwrap(), vec![vec![0, 1, 0, 1, 0, 1, 0, 1], vec![0, 1, 0, 1, 0, 1, 0, 1]]);
}


