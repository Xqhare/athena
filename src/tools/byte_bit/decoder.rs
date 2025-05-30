
/// Reads bits from a byte
///
/// The bits will be in the order of least significant bit to most significant bit
///
/// # Example
/// ```
/// # use athena::byte_bit::byte_bit_decoder;
/// let byte = 0b11110000;
/// let bits = byte_bit_decoder(byte);
/// assert_eq!(bits.to_vec(), vec![0, 0, 0, 0, 1, 1, 1, 1]);
/// ```
pub fn byte_bit_decoder(byte: u8) -> [u8; 8] {
    let mut out: [u8; 8] = [0; 8];
    for (i, out_slice) in out.iter_mut().enumerate() {
        *out_slice = (byte >> i) & 1;
    }
    out
}

#[test]
fn byte_bit_decoder_basics() {
    let byte0 = 0b10101010;
    let byte1 = 0b11111111;
    let byte2 = 0b00000000;
    let byte3 = 0b01010101;
    let byte4 = 0b11110000;
    let byte5 = 0b00001111;
    let byte6 = 0b10000000;
    let byte7 = 0b00000001;
    let bits0 = byte_bit_decoder(byte0);
    let bits1 = byte_bit_decoder(byte1);
    let bits2 = byte_bit_decoder(byte2);
    let bits3 = byte_bit_decoder(byte3);
    let bits4 = byte_bit_decoder(byte4);
    let bits5 = byte_bit_decoder(byte5);
    let bits6 = byte_bit_decoder(byte6);
    let bits7 = byte_bit_decoder(byte7);
    assert_eq!(bits0.to_vec(), vec![0, 1, 0, 1, 0, 1, 0, 1]);
    assert_eq!(bits1.to_vec(), vec![1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(bits2.to_vec(), vec![0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(bits3.to_vec(), vec![1, 0, 1, 0, 1, 0, 1, 0]);
    assert_eq!(bits4.to_vec(), vec![0, 0, 0, 0, 1, 1, 1, 1]);
    assert_eq!(bits5.to_vec(), vec![1, 1, 1, 1, 0, 0, 0, 0]);
    assert_eq!(bits6.to_vec(), vec![0, 0, 0, 0, 0, 0, 0, 1]);
    assert_eq!(bits7.to_vec(), vec![1, 0, 0, 0, 0, 0, 0, 0]);
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
        let bits = byte_bit_decoder(i);
        let mut expected_vec = all_u8_as_vec[i as usize].clone();
        expected_vec.reverse();
        assert_eq!(bits.to_vec(), expected_vec);
    }
}

