
/// Encodes bits into a byte
///
/// The bits will be in the order of least significant bit to most significant bit
///
/// # Example
/// ```
/// # use athena::tools::byte_bit::byte_bit_encoder;
/// let bits = [0, 0, 0, 0, 1, 1, 1, 1];
/// let byte = byte_bit_encoder(bits);
/// assert_eq!(byte, 0b11110000);
/// ```
pub fn byte_bit_encoder(bits: &[u8; 8]) -> u8 {
    let mut byte: u8 = 0;
    for i in 0..8 {
        byte |= bits[i] << i;
    }
    byte
}

#[test]
fn byte_bit_encoder_basics() {
    let byte0: Vec<u8> = vec![0, 1, 0, 1, 0, 1, 0, 1];
    let byte1: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1];
    let byte2: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let byte3: Vec<u8> = vec![1, 0, 1, 0, 1, 0, 1, 0];
    let byte4: Vec<u8> = vec![0, 0, 0, 0, 1, 1, 1, 1];
    let byte5: Vec<u8> = vec![1, 1, 1, 1, 0, 0, 0, 0];
    let byte6: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 1];
    let byte7: Vec<u8> = vec![1, 0, 0, 0, 0, 0, 0, 0];
    let bits0 = byte_bit_encoder(byte0.as_slice().try_into().unwrap());
    let bits1 = byte_bit_encoder(byte1.as_slice().try_into().unwrap());
    let bits2 = byte_bit_encoder(byte2.as_slice().try_into().unwrap());
    let bits3 = byte_bit_encoder(byte3.as_slice().try_into().unwrap());
    let bits4 = byte_bit_encoder(byte4.as_slice().try_into().unwrap());
    let bits5 = byte_bit_encoder(byte5.as_slice().try_into().unwrap());
    let bits6 = byte_bit_encoder(byte6.as_slice().try_into().unwrap());
    let bits7 = byte_bit_encoder(byte7.as_slice().try_into().unwrap());
    assert_eq!(bits0, 0b10101010);
    assert_eq!(bits1, 0b11111111);
    assert_eq!(bits2, 0b00000000);
    assert_eq!(bits3, 0b01010101);
    assert_eq!(bits4, 0b11110000);
    assert_eq!(bits5, 0b00001111);
    assert_eq!(bits6, 0b10000000);
    assert_eq!(bits7, 0b00000001);
}

// now this is what I would call 100% test coverage!
#[test]
fn byte_bit_decoder_all_u8() {
    let all_u8_as_vec = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 1],
        vec![0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 1, 0, 1],
        vec![0, 0, 0, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 0, 0, 1, 1, 1],
        vec![0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 1],
        vec![0, 0, 0, 0, 1, 0, 1, 0],
        vec![0, 0, 0, 0, 1, 0, 1, 1],
        vec![0, 0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 0, 1],
        vec![0, 0, 0, 0, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 1, 1, 1, 1],
        vec![0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 1],
        vec![0, 0, 0, 1, 0, 0, 1, 0],
        vec![0, 0, 0, 1, 0, 0, 1, 1],
        vec![0, 0, 0, 1, 0, 1, 0, 0],
        vec![0, 0, 0, 1, 0, 1, 0, 1],
        vec![0, 0, 0, 1, 0, 1, 1, 0],
        vec![0, 0, 0, 1, 0, 1, 1, 1],
        vec![0, 0, 0, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 1],
        vec![0, 0, 0, 1, 1, 0, 1, 0],
        vec![0, 0, 0, 1, 1, 0, 1, 1],
        vec![0, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 1, 1, 1, 0, 1],
        vec![0, 0, 0, 1, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 1, 1, 1, 1],
        vec![0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 0, 1],
        vec![0, 0, 1, 0, 0, 0, 1, 0],
        vec![0, 0, 1, 0, 0, 0, 1, 1],
        vec![0, 0, 1, 0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0, 1, 0, 1],
        vec![0, 0, 1, 0, 0, 1, 1, 0],
        vec![0, 0, 1, 0, 0, 1, 1, 1],
        vec![0, 0, 1, 0, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 1, 0, 0, 1],
        vec![0, 0, 1, 0, 1, 0, 1, 0],
        vec![0, 0, 1, 0, 1, 0, 1, 1],
        vec![0, 0, 1, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 0, 1, 1, 0, 1],
        vec![0, 0, 1, 0, 1, 1, 1, 0],
        vec![0, 0, 1, 0, 1, 1, 1, 1],
        vec![0, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0, 1],
        vec![0, 0, 1, 1, 0, 0, 1, 0],
        vec![0, 0, 1, 1, 0, 0, 1, 1],
        vec![0, 0, 1, 1, 0, 1, 0, 0],
        vec![0, 0, 1, 1, 0, 1, 0, 1],
        vec![0, 0, 1, 1, 0, 1, 1, 0],
        vec![0, 0, 1, 1, 0, 1, 1, 1],
        vec![0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0, 1],
        vec![0, 0, 1, 1, 1, 0, 1, 0],
        vec![0, 0, 1, 1, 1, 0, 1, 1],
        vec![0, 0, 1, 1, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 1, 1, 0, 1],
        vec![0, 0, 1, 1, 1, 1, 1, 0],
        vec![0, 0, 1, 1, 1, 1, 1, 1],
        vec![0, 1, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0, 0, 1],
        vec![0, 1, 0, 0, 0, 0, 1, 0],
        vec![0, 1, 0, 0, 0, 0, 1, 1],
        vec![0, 1, 0, 0, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 0, 1, 0, 1],
        vec![0, 1, 0, 0, 0, 1, 1, 0],
        vec![0, 1, 0, 0, 0, 1, 1, 1],
        vec![0, 1, 0, 0, 1, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 0, 0, 1],
        vec![0, 1, 0, 0, 1, 0, 1, 0],
        vec![0, 1, 0, 0, 1, 0, 1, 1],
        vec![0, 1, 0, 0, 1, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 1],
        vec![0, 1, 0, 0, 1, 1, 1, 0],
        vec![0, 1, 0, 0, 1, 1, 1, 1],
        vec![0, 1, 0, 1, 0, 0, 0, 0],
        vec![0, 1, 0, 1, 0, 0, 0, 1],
        vec![0, 1, 0, 1, 0, 0, 1, 0],
        vec![0, 1, 0, 1, 0, 0, 1, 1],
        vec![0, 1, 0, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0, 1, 1, 0],
        vec![0, 1, 0, 1, 0, 1, 1, 1],
        vec![0, 1, 0, 1, 1, 0, 0, 0],
        vec![0, 1, 0, 1, 1, 0, 0, 1],
        vec![0, 1, 0, 1, 1, 0, 1, 0],
        vec![0, 1, 0, 1, 1, 0, 1, 1],
        vec![0, 1, 0, 1, 1, 1, 0, 0],
        vec![0, 1, 0, 1, 1, 1, 0, 1],
        vec![0, 1, 0, 1, 1, 1, 1, 0],
        vec![0, 1, 0, 1, 1, 1, 1, 1],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 1],
        vec![0, 1, 1, 0, 0, 0, 1, 0],
        vec![0, 1, 1, 0, 0, 0, 1, 1],
        vec![0, 1, 1, 0, 0, 1, 0, 0],
        vec![0, 1, 1, 0, 0, 1, 0, 1],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 1],
        vec![0, 1, 1, 0, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 1],
        vec![0, 1, 1, 0, 1, 0, 1, 0],
        vec![0, 1, 1, 0, 1, 0, 1, 1],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 1],
        vec![0, 1, 1, 0, 1, 1, 1, 0],
        vec![0, 1, 1, 0, 1, 1, 1, 1],
        vec![0, 1, 1, 1, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 0, 0, 1],
        vec![0, 1, 1, 1, 0, 0, 1, 0],
        vec![0, 1, 1, 1, 0, 0, 1, 1],
        vec![0, 1, 1, 1, 0, 1, 0, 0],
        vec![0, 1, 1, 1, 0, 1, 0, 1],
        vec![0, 1, 1, 1, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 0, 1, 1, 1],
        vec![0, 1, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 0, 1],
        vec![0, 1, 1, 1, 1, 0, 1, 0],
        vec![0, 1, 1, 1, 1, 0, 1, 1],
        vec![0, 1, 1, 1, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 0, 1],
        vec![0, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 0, 0, 1, 1],
        vec![1, 0, 0, 0, 0, 1, 0, 0],
        vec![1, 0, 0, 0, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 1, 1, 1],
        vec![1, 0, 0, 0, 1, 0, 0, 0],
        vec![1, 0, 0, 0, 1, 0, 0, 1],
        vec![1, 0, 0, 0, 1, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0, 1, 1],
        vec![1, 0, 0, 0, 1, 1, 0, 0],
        vec![1, 0, 0, 0, 1, 1, 0, 1],
        vec![1, 0, 0, 0, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 1, 1, 1, 1],
        vec![1, 0, 0, 1, 0, 0, 0, 0],
        vec![1, 0, 0, 1, 0, 0, 0, 1],
        vec![1, 0, 0, 1, 0, 0, 1, 0],
        vec![1, 0, 0, 1, 0, 0, 1, 1],
        vec![1, 0, 0, 1, 0, 1, 0, 0],
        vec![1, 0, 0, 1, 0, 1, 0, 1],
        vec![1, 0, 0, 1, 0, 1, 1, 0],
        vec![1, 0, 0, 1, 0, 1, 1, 1],
        vec![1, 0, 0, 1, 1, 0, 0, 0],
        vec![1, 0, 0, 1, 1, 0, 0, 1],
        vec![1, 0, 0, 1, 1, 0, 1, 0],
        vec![1, 0, 0, 1, 1, 0, 1, 1],
        vec![1, 0, 0, 1, 1, 1, 0, 0],
        vec![1, 0, 0, 1, 1, 1, 0, 1],
        vec![1, 0, 0, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 1, 1, 1, 1, 1],
        vec![1, 0, 1, 0, 0, 0, 0, 0],
        vec![1, 0, 1, 0, 0, 0, 0, 1],
        vec![1, 0, 1, 0, 0, 0, 1, 0],
        vec![1, 0, 1, 0, 0, 0, 1, 1],
        vec![1, 0, 1, 0, 0, 1, 0, 0],
        vec![1, 0, 1, 0, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 0, 1, 1, 0],
        vec![1, 0, 1, 0, 0, 1, 1, 1],
        vec![1, 0, 1, 0, 1, 0, 0, 0],
        vec![1, 0, 1, 0, 1, 0, 0, 1],
        vec![1, 0, 1, 0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1, 0, 1, 1],
        vec![1, 0, 1, 0, 1, 1, 0, 0],
        vec![1, 0, 1, 0, 1, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 1, 1, 0],
        vec![1, 0, 1, 0, 1, 1, 1, 1],
        vec![1, 0, 1, 1, 0, 0, 0, 0],
        vec![1, 0, 1, 1, 0, 0, 0, 1],
        vec![1, 0, 1, 1, 0, 0, 1, 0],
        vec![1, 0, 1, 1, 0, 0, 1, 1],
        vec![1, 0, 1, 1, 0, 1, 0, 0],
        vec![1, 0, 1, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 1, 0, 1, 1, 0],
        vec![1, 0, 1, 1, 0, 1, 1, 1],
        vec![1, 0, 1, 1, 1, 0, 0, 0],
        vec![1, 0, 1, 1, 1, 0, 0, 1],
        vec![1, 0, 1, 1, 1, 0, 1, 0],
        vec![1, 0, 1, 1, 1, 0, 1, 1],
        vec![1, 0, 1, 1, 1, 1, 0, 0],
        vec![1, 0, 1, 1, 1, 1, 0, 1],
        vec![1, 0, 1, 1, 1, 1, 1, 0],
        vec![1, 0, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 0, 0, 0, 0, 1, 0],
        vec![1, 1, 0, 0, 0, 0, 1, 1],
        vec![1, 1, 0, 0, 0, 1, 0, 0],
        vec![1, 1, 0, 0, 0, 1, 0, 1],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 1],
        vec![1, 1, 0, 0, 1, 0, 0, 0],
        vec![1, 1, 0, 0, 1, 0, 0, 1],
        vec![1, 1, 0, 0, 1, 0, 1, 0],
        vec![1, 1, 0, 0, 1, 0, 1, 1],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 1],
        vec![1, 1, 0, 0, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 1, 1],
        vec![1, 1, 0, 1, 0, 0, 0, 0],
        vec![1, 1, 0, 1, 0, 0, 0, 1],
        vec![1, 1, 0, 1, 0, 0, 1, 0],
        vec![1, 1, 0, 1, 0, 0, 1, 1],
        vec![1, 1, 0, 1, 0, 1, 0, 0],
        vec![1, 1, 0, 1, 0, 1, 0, 1],
        vec![1, 1, 0, 1, 0, 1, 1, 0],
        vec![1, 1, 0, 1, 0, 1, 1, 1],
        vec![1, 1, 0, 1, 1, 0, 0, 0],
        vec![1, 1, 0, 1, 1, 0, 0, 1],
        vec![1, 1, 0, 1, 1, 0, 1, 0],
        vec![1, 1, 0, 1, 1, 0, 1, 1],
        vec![1, 1, 0, 1, 1, 1, 0, 0],
        vec![1, 1, 0, 1, 1, 1, 0, 1],
        vec![1, 1, 0, 1, 1, 1, 1, 0],
        vec![1, 1, 0, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 0, 0, 0, 1, 0],
        vec![1, 1, 1, 0, 0, 0, 1, 1],
        vec![1, 1, 1, 0, 0, 1, 0, 0],
        vec![1, 1, 1, 0, 0, 1, 0, 1],
        vec![1, 1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 1, 0, 0, 1, 1, 1],
        vec![1, 1, 1, 0, 1, 0, 0, 0],
        vec![1, 1, 1, 0, 1, 0, 0, 1],
        vec![1, 1, 1, 0, 1, 0, 1, 0],
        vec![1, 1, 1, 0, 1, 0, 1, 1],
        vec![1, 1, 1, 0, 1, 1, 0, 0],
        vec![1, 1, 1, 0, 1, 1, 0, 1],
        vec![1, 1, 1, 0, 1, 1, 1, 0],
        vec![1, 1, 1, 0, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 0, 0, 1, 1],
        vec![1, 1, 1, 1, 0, 1, 0, 0],
        vec![1, 1, 1, 1, 0, 1, 0, 1],
        vec![1, 1, 1, 1, 0, 1, 1, 0],
        vec![1, 1, 1, 1, 0, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 0, 1, 0],
        vec![1, 1, 1, 1, 1, 0, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
    ];

    for i in 0..=255 {
        let mut bits = all_u8_as_vec[i as usize].clone();
        bits.reverse();
        let result = byte_bit_encoder(bits.as_slice().try_into().unwrap());
        assert_eq!(result, i);
    }
}

