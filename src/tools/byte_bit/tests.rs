use crate::byte_bit::{byte_bit_decoder, byte_bit_encoder};

#[test]
fn test_generic_bit_ops() {
    let value: u32 = 0x12345678;
    let bits = byte_bit_decoder(value);
    assert_eq!(bits.len(), 32);
    let decoded: u32 = byte_bit_encoder(&bits);
    assert_eq!(decoded, value);
}

#[test]
fn test_u8_ops() {
    let value: u8 = 0b10101010;
    let bits = byte_bit_decoder(value);
    assert_eq!(bits, vec![0, 1, 0, 1, 0, 1, 0, 1]);
    let encoded: u8 = byte_bit_encoder(&bits);
    assert_eq!(encoded, value);
}
