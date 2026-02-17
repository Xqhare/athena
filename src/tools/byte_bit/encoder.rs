use crate::utils::traits::unsigned::Unsigned;

/// Encodes bits into a value
///
/// The bits will be in the order of least significant bit to most significant bit
pub fn byte_bit_encoder<T: Unsigned>(bits: &[u8]) -> T {
    let mut value: T = T::zero();
    for (i, &bit) in bits.iter().enumerate() {
        if bit != 0 {
            value = value | (T::one() << i);
        }
    }
    value
}
