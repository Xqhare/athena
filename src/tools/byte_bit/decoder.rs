use crate::utils::traits::unsigned::Unsigned;

/// Reads bits from a value
///
/// The bits will be in the order of least significant bit to most significant bit
pub fn byte_bit_decoder<T: Unsigned>(value: T) -> Vec<u8> {
    let size = std::mem::size_of::<T>() * 8;
    let mut out: Vec<u8> = Vec::with_capacity(size);
    for i in 0..size {
        let bit = (value >> i) & T::one();
        out.push(if bit != T::zero() { 1 } else { 0 });
    }
    out
}
