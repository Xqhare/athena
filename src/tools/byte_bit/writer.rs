use crate::error::AthenaError;

use super::encoder::byte_bit_encoder;

/// Writes bits to a io::Writer
///
/// The bits will be in the order of least significant bit to most significant bit
/// 
/// # Example
/// ```
/// # use athena::byte_bit::byte_bit_writer;
/// let bytes = vec![[0, 0, 0, 0, 1, 1, 1, 1], [1, 1, 1, 1, 0, 0, 0, 0]];
/// let mut writer = Vec::new();
/// byte_bit_writer(&mut writer, bytes).unwrap();
/// assert_eq!(writer, vec![0b11110000, 0b00001111]);
/// ```
pub fn byte_bit_writer<W: std::io::Write>(writer: &mut W, bytes: Vec<[u8; 8]>) -> Result<(), AthenaError> {
    for byte in bytes {
        writer.write_all(&[byte_bit_encoder(&byte)]).map_err(AthenaError::IoError)?;
    }
    Ok(())
}

#[test]
fn writer_err() {
    // buffer too small for bytes
    let mut writer: &mut [u8] = &mut [0u8; 0];
    let bytes = vec![[0, 0, 0, 0, 1, 1, 1, 1], [1, 1, 1, 1, 0, 0, 0, 0]];
    let result = byte_bit_writer(&mut writer, bytes);
    assert!(result.is_err());
}

#[test]
fn byte_bit_writer_basics() {
    let byte0: Vec<[u8; 8]> = vec![[0, 1, 0, 1, 0, 1, 0, 1], [0, 1, 0, 1, 0, 1, 0, 1]];
    let byte1: Vec<[u8; 8]> = vec![[1, 1, 1, 1, 1, 1, 1, 1], [1, 1, 1, 1, 1, 1, 1, 1]];
    let byte2: Vec<[u8; 8]> = vec![[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]];
    let byte3: Vec<[u8; 8]> = vec![[1, 0, 1, 0, 1, 0, 1, 0], [1, 0, 1, 0, 1, 0, 1, 0]];
    let byte4: Vec<[u8; 8]> = vec![[0, 0, 0, 0, 1, 1, 1, 1], [0, 0, 0, 0, 1, 1, 1, 1]];
    let byte5: Vec<[u8; 8]> = vec![[1, 1, 1, 1, 0, 0, 0, 0], [1, 1, 1, 1, 0, 0, 0, 0]];
    let byte6: Vec<[u8; 8]> = vec![[0, 0, 0, 0, 0, 0, 0, 1], [0, 0, 0, 0, 0, 0, 0, 1]];
    let byte7: Vec<[u8; 8]> = vec![[1, 0, 0, 0, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0, 0, 0]];
    let mut writer0 = Vec::new();
    let mut writer1 = Vec::new();
    let mut writer2 = Vec::new();
    let mut writer3 = Vec::new();
    let mut writer4 = Vec::new();
    let mut writer5 = Vec::new();
    let mut writer6 = Vec::new();
    let mut writer7 = Vec::new();
    let bits0 = byte_bit_writer(&mut writer0, byte0.as_slice().try_into().unwrap());
    let bits1 = byte_bit_writer(&mut writer1, byte1.as_slice().try_into().unwrap());
    let bits2 = byte_bit_writer(&mut writer2, byte2.as_slice().try_into().unwrap());
    let bits3 = byte_bit_writer(&mut writer3, byte3.as_slice().try_into().unwrap());
    let bits4 = byte_bit_writer(&mut writer4, byte4.as_slice().try_into().unwrap());
    let bits5 = byte_bit_writer(&mut writer5, byte5.as_slice().try_into().unwrap());
    let bits6 = byte_bit_writer(&mut writer6, byte6.as_slice().try_into().unwrap());
    let bits7 = byte_bit_writer(&mut writer7, byte7.as_slice().try_into().unwrap());
    assert!(bits0.is_ok());
    assert!(bits1.is_ok());
    assert!(bits2.is_ok());
    assert!(bits3.is_ok());
    assert!(bits4.is_ok());
    assert!(bits5.is_ok());
    assert!(bits6.is_ok());
    assert!(bits7.is_ok());
    assert_eq!(writer0, vec![0b10101010, 0b10101010]);
    assert_eq!(writer1, vec![0b11111111, 0b11111111]);
    assert_eq!(writer2, vec![0b00000000, 0b00000000]);
    assert_eq!(writer3, vec![0b01010101, 0b01010101]);
    assert_eq!(writer4, vec![0b11110000, 0b11110000]);
    assert_eq!(writer5, vec![0b00001111, 0b00001111]);
    assert_eq!(writer6, vec![0b10000000, 0b10000000]);
    assert_eq!(writer7, vec![0b00000001, 0b00000001]);
}

// now this is what I would call 100% test coverage!
#[test]
fn byte_bit_writer_all_u8() {
    let all_u8_as_vec: Vec<[u8; 8]> = vec![
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 1, 1],
        [0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 1],
        [0, 0, 0, 0, 0, 1, 1, 0],
        [0, 0, 0, 0, 0, 1, 1, 1],
        [0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 1],
        [0, 0, 0, 0, 1, 0, 1, 0],
        [0, 0, 0, 0, 1, 0, 1, 1],
        [0, 0, 0, 0, 1, 1, 0, 0],
        [0, 0, 0, 0, 1, 1, 0, 1],
        [0, 0, 0, 0, 1, 1, 1, 0],
        [0, 0, 0, 0, 1, 1, 1, 1],
        [0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 1],
        [0, 0, 0, 1, 0, 0, 1, 0],
        [0, 0, 0, 1, 0, 0, 1, 1],
        [0, 0, 0, 1, 0, 1, 0, 0],
        [0, 0, 0, 1, 0, 1, 0, 1],
        [0, 0, 0, 1, 0, 1, 1, 0],
        [0, 0, 0, 1, 0, 1, 1, 1],
        [0, 0, 0, 1, 1, 0, 0, 0],
        [0, 0, 0, 1, 1, 0, 0, 1],
        [0, 0, 0, 1, 1, 0, 1, 0],
        [0, 0, 0, 1, 1, 0, 1, 1],
        [0, 0, 0, 1, 1, 1, 0, 0],
        [0, 0, 0, 1, 1, 1, 0, 1],
        [0, 0, 0, 1, 1, 1, 1, 0],
        [0, 0, 0, 1, 1, 1, 1, 1],
        [0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 1],
        [0, 0, 1, 0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0, 0, 1, 1],
        [0, 0, 1, 0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0, 1, 0, 1],
        [0, 0, 1, 0, 0, 1, 1, 0],
        [0, 0, 1, 0, 0, 1, 1, 1],
        [0, 0, 1, 0, 1, 0, 0, 0],
        [0, 0, 1, 0, 1, 0, 0, 1],
        [0, 0, 1, 0, 1, 0, 1, 0],
        [0, 0, 1, 0, 1, 0, 1, 1],
        [0, 0, 1, 0, 1, 1, 0, 0],
        [0, 0, 1, 0, 1, 1, 0, 1],
        [0, 0, 1, 0, 1, 1, 1, 0],
        [0, 0, 1, 0, 1, 1, 1, 1],
        [0, 0, 1, 1, 0, 0, 0, 0],
        [0, 0, 1, 1, 0, 0, 0, 1],
        [0, 0, 1, 1, 0, 0, 1, 0],
        [0, 0, 1, 1, 0, 0, 1, 1],
        [0, 0, 1, 1, 0, 1, 0, 0],
        [0, 0, 1, 1, 0, 1, 0, 1],
        [0, 0, 1, 1, 0, 1, 1, 0],
        [0, 0, 1, 1, 0, 1, 1, 1],
        [0, 0, 1, 1, 1, 0, 0, 0],
        [0, 0, 1, 1, 1, 0, 0, 1],
        [0, 0, 1, 1, 1, 0, 1, 0],
        [0, 0, 1, 1, 1, 0, 1, 1],
        [0, 0, 1, 1, 1, 1, 0, 0],
        [0, 0, 1, 1, 1, 1, 0, 1],
        [0, 0, 1, 1, 1, 1, 1, 0],
        [0, 0, 1, 1, 1, 1, 1, 1],
        [0, 1, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 1],
        [0, 1, 0, 0, 0, 0, 1, 0],
        [0, 1, 0, 0, 0, 0, 1, 1],
        [0, 1, 0, 0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0, 1, 0, 1],
        [0, 1, 0, 0, 0, 1, 1, 0],
        [0, 1, 0, 0, 0, 1, 1, 1],
        [0, 1, 0, 0, 1, 0, 0, 0],
        [0, 1, 0, 0, 1, 0, 0, 1],
        [0, 1, 0, 0, 1, 0, 1, 0],
        [0, 1, 0, 0, 1, 0, 1, 1],
        [0, 1, 0, 0, 1, 1, 0, 0],
        [0, 1, 0, 0, 1, 1, 0, 1],
        [0, 1, 0, 0, 1, 1, 1, 0],
        [0, 1, 0, 0, 1, 1, 1, 1],
        [0, 1, 0, 1, 0, 0, 0, 0],
        [0, 1, 0, 1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0, 0, 1, 0],
        [0, 1, 0, 1, 0, 0, 1, 1],
        [0, 1, 0, 1, 0, 1, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 1],
        [0, 1, 0, 1, 0, 1, 1, 0],
        [0, 1, 0, 1, 0, 1, 1, 1],
        [0, 1, 0, 1, 1, 0, 0, 0],
        [0, 1, 0, 1, 1, 0, 0, 1],
        [0, 1, 0, 1, 1, 0, 1, 0],
        [0, 1, 0, 1, 1, 0, 1, 1],
        [0, 1, 0, 1, 1, 1, 0, 0],
        [0, 1, 0, 1, 1, 1, 0, 1],
        [0, 1, 0, 1, 1, 1, 1, 0],
        [0, 1, 0, 1, 1, 1, 1, 1],
        [0, 1, 1, 0, 0, 0, 0, 0],
        [0, 1, 1, 0, 0, 0, 0, 1],
        [0, 1, 1, 0, 0, 0, 1, 0],
        [0, 1, 1, 0, 0, 0, 1, 1],
        [0, 1, 1, 0, 0, 1, 0, 0],
        [0, 1, 1, 0, 0, 1, 0, 1],
        [0, 1, 1, 0, 0, 1, 1, 0],
        [0, 1, 1, 0, 0, 1, 1, 1],
        [0, 1, 1, 0, 1, 0, 0, 0],
        [0, 1, 1, 0, 1, 0, 0, 1],
        [0, 1, 1, 0, 1, 0, 1, 0],
        [0, 1, 1, 0, 1, 0, 1, 1],
        [0, 1, 1, 0, 1, 1, 0, 0],
        [0, 1, 1, 0, 1, 1, 0, 1],
        [0, 1, 1, 0, 1, 1, 1, 0],
        [0, 1, 1, 0, 1, 1, 1, 1],
        [0, 1, 1, 1, 0, 0, 0, 0],
        [0, 1, 1, 1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0, 0, 1, 0],
        [0, 1, 1, 1, 0, 0, 1, 1],
        [0, 1, 1, 1, 0, 1, 0, 0],
        [0, 1, 1, 1, 0, 1, 0, 1],
        [0, 1, 1, 1, 0, 1, 1, 0],
        [0, 1, 1, 1, 0, 1, 1, 1],
        [0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0, 1],
        [0, 1, 1, 1, 1, 0, 1, 0],
        [0, 1, 1, 1, 1, 0, 1, 1],
        [0, 1, 1, 1, 1, 1, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 1],
        [0, 1, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 1, 0],
        [1, 0, 0, 0, 0, 0, 1, 1],
        [1, 0, 0, 0, 0, 1, 0, 0],
        [1, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 1, 1, 0],
        [1, 0, 0, 0, 0, 1, 1, 1],
        [1, 0, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 0, 1, 0, 1, 0],
        [1, 0, 0, 0, 1, 0, 1, 1],
        [1, 0, 0, 0, 1, 1, 0, 0],
        [1, 0, 0, 0, 1, 1, 0, 1],
        [1, 0, 0, 0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1, 1, 1, 1],
        [1, 0, 0, 1, 0, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 1, 0],
        [1, 0, 0, 1, 0, 0, 1, 1],
        [1, 0, 0, 1, 0, 1, 0, 0],
        [1, 0, 0, 1, 0, 1, 0, 1],
        [1, 0, 0, 1, 0, 1, 1, 0],
        [1, 0, 0, 1, 0, 1, 1, 1],
        [1, 0, 0, 1, 1, 0, 0, 0],
        [1, 0, 0, 1, 1, 0, 0, 1],
        [1, 0, 0, 1, 1, 0, 1, 0],
        [1, 0, 0, 1, 1, 0, 1, 1],
        [1, 0, 0, 1, 1, 1, 0, 0],
        [1, 0, 0, 1, 1, 1, 0, 1],
        [1, 0, 0, 1, 1, 1, 1, 0],
        [1, 0, 0, 1, 1, 1, 1, 1],
        [1, 0, 1, 0, 0, 0, 0, 0],
        [1, 0, 1, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 0, 0, 1, 0],
        [1, 0, 1, 0, 0, 0, 1, 1],
        [1, 0, 1, 0, 0, 1, 0, 0],
        [1, 0, 1, 0, 0, 1, 0, 1],
        [1, 0, 1, 0, 0, 1, 1, 0],
        [1, 0, 1, 0, 0, 1, 1, 1],
        [1, 0, 1, 0, 1, 0, 0, 0],
        [1, 0, 1, 0, 1, 0, 0, 1],
        [1, 0, 1, 0, 1, 0, 1, 0],
        [1, 0, 1, 0, 1, 0, 1, 1],
        [1, 0, 1, 0, 1, 1, 0, 0],
        [1, 0, 1, 0, 1, 1, 0, 1],
        [1, 0, 1, 0, 1, 1, 1, 0],
        [1, 0, 1, 0, 1, 1, 1, 1],
        [1, 0, 1, 1, 0, 0, 0, 0],
        [1, 0, 1, 1, 0, 0, 0, 1],
        [1, 0, 1, 1, 0, 0, 1, 0],
        [1, 0, 1, 1, 0, 0, 1, 1],
        [1, 0, 1, 1, 0, 1, 0, 0],
        [1, 0, 1, 1, 0, 1, 0, 1],
        [1, 0, 1, 1, 0, 1, 1, 0],
        [1, 0, 1, 1, 0, 1, 1, 1],
        [1, 0, 1, 1, 1, 0, 0, 0],
        [1, 0, 1, 1, 1, 0, 0, 1],
        [1, 0, 1, 1, 1, 0, 1, 0],
        [1, 0, 1, 1, 1, 0, 1, 1],
        [1, 0, 1, 1, 1, 1, 0, 0],
        [1, 0, 1, 1, 1, 1, 0, 1],
        [1, 0, 1, 1, 1, 1, 1, 0],
        [1, 0, 1, 1, 1, 1, 1, 1],
        [1, 1, 0, 0, 0, 0, 0, 0],
        [1, 1, 0, 0, 0, 0, 0, 1],
        [1, 1, 0, 0, 0, 0, 1, 0],
        [1, 1, 0, 0, 0, 0, 1, 1],
        [1, 1, 0, 0, 0, 1, 0, 0],
        [1, 1, 0, 0, 0, 1, 0, 1],
        [1, 1, 0, 0, 0, 1, 1, 0],
        [1, 1, 0, 0, 0, 1, 1, 1],
        [1, 1, 0, 0, 1, 0, 0, 0],
        [1, 1, 0, 0, 1, 0, 0, 1],
        [1, 1, 0, 0, 1, 0, 1, 0],
        [1, 1, 0, 0, 1, 0, 1, 1],
        [1, 1, 0, 0, 1, 1, 0, 0],
        [1, 1, 0, 0, 1, 1, 0, 1],
        [1, 1, 0, 0, 1, 1, 1, 0],
        [1, 1, 0, 0, 1, 1, 1, 1],
        [1, 1, 0, 1, 0, 0, 0, 0],
        [1, 1, 0, 1, 0, 0, 0, 1],
        [1, 1, 0, 1, 0, 0, 1, 0],
        [1, 1, 0, 1, 0, 0, 1, 1],
        [1, 1, 0, 1, 0, 1, 0, 0],
        [1, 1, 0, 1, 0, 1, 0, 1],
        [1, 1, 0, 1, 0, 1, 1, 0],
        [1, 1, 0, 1, 0, 1, 1, 1],
        [1, 1, 0, 1, 1, 0, 0, 0],
        [1, 1, 0, 1, 1, 0, 0, 1],
        [1, 1, 0, 1, 1, 0, 1, 0],
        [1, 1, 0, 1, 1, 0, 1, 1],
        [1, 1, 0, 1, 1, 1, 0, 0],
        [1, 1, 0, 1, 1, 1, 0, 1],
        [1, 1, 0, 1, 1, 1, 1, 0],
        [1, 1, 0, 1, 1, 1, 1, 1],
        [1, 1, 1, 0, 0, 0, 0, 0],
        [1, 1, 1, 0, 0, 0, 0, 1],
        [1, 1, 1, 0, 0, 0, 1, 0],
        [1, 1, 1, 0, 0, 0, 1, 1],
        [1, 1, 1, 0, 0, 1, 0, 0],
        [1, 1, 1, 0, 0, 1, 0, 1],
        [1, 1, 1, 0, 0, 1, 1, 0],
        [1, 1, 1, 0, 0, 1, 1, 1],
        [1, 1, 1, 0, 1, 0, 0, 0],
        [1, 1, 1, 0, 1, 0, 0, 1],
        [1, 1, 1, 0, 1, 0, 1, 0],
        [1, 1, 1, 0, 1, 0, 1, 1],
        [1, 1, 1, 0, 1, 1, 0, 0],
        [1, 1, 1, 0, 1, 1, 0, 1],
        [1, 1, 1, 0, 1, 1, 1, 0],
        [1, 1, 1, 0, 1, 1, 1, 1],
        [1, 1, 1, 1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0, 0, 0, 1],
        [1, 1, 1, 1, 0, 0, 1, 0],
        [1, 1, 1, 1, 0, 0, 1, 1],
        [1, 1, 1, 1, 0, 1, 0, 0],
        [1, 1, 1, 1, 0, 1, 0, 1],
        [1, 1, 1, 1, 0, 1, 1, 0],
        [1, 1, 1, 1, 0, 1, 1, 1],
        [1, 1, 1, 1, 1, 0, 0, 0],
        [1, 1, 1, 1, 1, 0, 0, 1],
        [1, 1, 1, 1, 1, 0, 1, 0],
        [1, 1, 1, 1, 1, 0, 1, 1],
        [1, 1, 1, 1, 1, 1, 0, 0],
        [1, 1, 1, 1, 1, 1, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 0],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];

    for i in 0..=255 {
        let mut bits = vec![all_u8_as_vec[i as usize], all_u8_as_vec[i as usize]];
        bits[0].reverse();
        bits[1].reverse();
        let mut writer = Vec::new();
        let result = byte_bit_writer(&mut writer, bits.as_slice().try_into().unwrap());
        assert!(result.is_ok());
        assert_eq!(writer, vec![i, i]);
    }
}

