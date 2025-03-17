
#[inline]
/// Reads bits from a byte
///
/// The bits will be in the order of least significant bit to most significant bit
///
/// # Example
/// ```
/// # use athena::tools::bitreader::bitreader;
/// let byte = 0b11110000;
/// let bits = bitreader(byte);
/// assert_eq!(bits, vec![0, 0, 0, 0, 1, 1, 1, 1]);
/// ```
pub fn bitreader(byte: u8) -> Vec<u8> {
    let mut out: Vec<u8> = Default::default();
    for i in 0..8 {
        out.push((byte >> i) & 1);
    }
    out
}

#[test]
fn basics() {
    let byte0 = 0b10101010;
    let byte1 = 0b11111111;
    let byte2 = 0b00000000;
    let byte3 = 0b01010101;
    let byte4 = 0b11110000;
    let byte5 = 0b00001111;
    let byte6 = 0b10000000;
    let byte7 = 0b00000001;
    let bits0 = bitreader(byte0);
    let bits1 = bitreader(byte1);
    let bits2 = bitreader(byte2);
    let bits3 = bitreader(byte3);
    let bits4 = bitreader(byte4);
    let bits5 = bitreader(byte5);
    let bits6 = bitreader(byte6);
    let bits7 = bitreader(byte7);
    assert_eq!(bits0, vec![0, 1, 0, 1, 0, 1, 0, 1]);
    assert_eq!(bits1, vec![1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(bits2, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(bits3, vec![1, 0, 1, 0, 1, 0, 1, 0]);
    assert_eq!(bits4, vec![0, 0, 0, 0, 1, 1, 1, 1]);
    assert_eq!(bits5, vec![1, 1, 1, 1, 0, 0, 0, 0]);
    assert_eq!(bits6, vec![0, 0, 0, 0, 0, 0, 0, 1]);
    assert_eq!(bits7, vec![1, 0, 0, 0, 0, 0, 0, 0]);
}
