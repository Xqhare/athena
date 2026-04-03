//! Delta encoding and decoding for numerical sequences.
//!
//! Delta encoding stores the difference between consecutive values rather than the
//! values themselves. This is highly effective for data that changes slowly or
//! follows a predictable pattern.

use crate::utils::traits::{signed::Signed, unsigned::Unsigned};

mod tests;

/// Returns the delta compressed version of the given data.
///
/// Each element in the output (except the first) is the difference between the 
/// previous element and the current element in the input.
///
/// # Arguments
/// * `data` - The slice of unsigned integers to compress.
///
/// # Returns
/// * `Vec<S>` - A vector of signed integers representing the deltas.
///
/// # Example
/// ```
/// # use athena::encoding_and_decoding::delta_encoder;
/// let data: Vec<u8> = vec![100, 105, 110, 108];
/// let encoded: Vec<i16> = delta_encoder(&data);
/// assert_eq!(encoded, vec![100, -5, -5, 2]);
/// ```
pub fn delta_encoder<U: Unsigned, S: Signed + From<U>>(data: &[U]) -> Vec<S> {
    if data.is_empty() {
        return Vec::new();
    }
    let mut out: Vec<S> = Vec::with_capacity(data.len());
    out.push(S::from(data[0]));

    let mut last: U = data[0];

    for entry in data.iter().skip(1) {
        out.push(S::from(last) - S::from(*entry));
        last = *entry;
    }

    out
}

/// Returns the delta decompressed version of the given data.
///
/// Reconstructs the original sequence from the first value and subsequent deltas.
///
/// # Arguments
/// * `data` - The slice of signed deltas to decompress.
///
/// # Returns
/// * `Vec<U>` - The original sequence of unsigned integers.
///
/// # Example
/// ```
/// # use athena::encoding_and_decoding::{delta_encoder, delta_decoder};
/// let data: Vec<u8> = vec![100, 105, 110, 108];
/// let encoded: Vec<i16> = delta_encoder(&data);
/// let decoded: Vec<u8> = delta_decoder(&encoded);
/// assert_eq!(decoded, data);
/// ```
pub fn delta_decoder<U: Unsigned + TryFrom<S>, S: Signed>(data: &[S]) -> Vec<U>
where
    <U as TryFrom<S>>::Error: std::fmt::Debug,
{
    if data.is_empty() {
        return Vec::new();
    }
    let mut out: Vec<U> = Vec::with_capacity(data.len());
    out.push(U::try_from(data[0]).unwrap());

    let mut last: S = data[0];

    for entry in data.iter().skip(1) {
        let decoded_s = last - *entry;
        let decoded = U::try_from(decoded_s).unwrap();
        out.push(decoded);
        last = decoded_s;
    }

    out
}
