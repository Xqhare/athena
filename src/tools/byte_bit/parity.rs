/// Checks if a byte has even parity (even number of set bits)
/// 
/// # Example
/// ```
/// use athena::byte_bit::is_even_parity;
/// 
/// assert!(is_even_parity(0b00000000)); // 0 ones
/// assert!(is_even_parity(0b00000011)); // 2 ones
/// assert!(!is_even_parity(0b00000001)); // 1 one
/// ```
pub fn is_even_parity(byte: u8) -> bool {
    (byte.count_ones() % 2) == 0
}

/// Ensures a byte has even parity by toggling the most significant bit (MSB) if necessary.
/// 
/// If the byte (excluding MSB) has odd parity, sets MSB to 1.
/// If the byte (excluding MSB) has even parity, sets MSB to 0.
/// 
/// # Example
/// ```
/// use athena::byte_bit::ensure_even_parity;
/// 
/// // 0x41 is 0100 0001 (2 ones). MSB is 0. Parity is even. No change.
/// assert_eq!(ensure_even_parity(0x41), 0x41); 
/// 
/// // 0x01 is 0000 0001 (1 one). To make even, MSB must be 1.
/// // 0x01 | 0x80 = 0x81 (1000 0001) which has 2 ones.
/// assert_eq!(ensure_even_parity(0x01), 0x81);
/// ```
pub fn ensure_even_parity(byte: u8) -> u8 {
    let low_bits = byte & 0x7F;
    if (low_bits.count_ones() % 2) == 0 {
        low_bits // MSB should be 0
    } else {
        low_bits | 0x80 // MSB should be 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even_parity() {
        assert!(is_even_parity(0x00)); // 0
        assert!(is_even_parity(0x03)); // 2
        assert!(is_even_parity(0x0F)); // 4
        assert!(!is_even_parity(0x01)); // 1
        assert!(!is_even_parity(0x07)); // 3
    }

    #[test]
    fn test_ensure_even_parity() {
        for i in 0..128 {
            let parity_byte = ensure_even_parity(i);
            assert!(is_even_parity(parity_byte));
            assert_eq!(parity_byte & 0x7F, i);
        }
    }
}
