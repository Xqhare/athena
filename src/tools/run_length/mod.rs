//! Run-Length Encoding (RLE) and decoding.
//!
//! RLE is a simple form of data compression in which runs of data (sequences in 
//! which the same data value occurs in many consecutive data elements) are stored 
//! as a single data value and count.

use crate::utils::traits::unsigned::Unsigned;

mod tests;

/// Returns the run length encoded version of the given data.
///
/// # Arguments
/// * `data` - The slice of data to compress.
///
/// # Returns
/// * `Vec<(usize, D)>` - The encoded data as a vector of (count, value) pairs.
///
/// # Example
/// ```
/// # use athena::encoding_and_decoding::run_length_encoder;
///
/// let data = b"AAAABBBCCDEEEE".to_vec();
/// let compressed = run_length_encoder(&data);
/// assert_eq!(compressed, vec![(4, b'A'), (3, b'B'), (2, b'C'), (1, b'D'), (4, b'E')]);
/// ```
pub fn run_length_encoder<D: Unsigned>(data: &[D]) -> Vec<(usize, D)> {
    if data.is_empty() {
        return Vec::new();
    }
    let mut out: Vec<(usize, D)> = Vec::with_capacity(data.len());
    let mut last = data[0];
    let mut count: usize = 1;
    for entry in data.iter().skip(1) {
        if *entry == last {
            count += 1;
        } else {
            out.push((count, last));
            last = *entry;
            count = 1;
        }
    }
    out.push((count, last));
    out
}

/// Returns the run length decoded version of the given data.
///
/// # Arguments
/// * `data` - The slice of (count, value) pairs to decompress.
///
/// # Returns
/// * `Vec<D>` - The reconstructed original data sequence.
///
/// # Example
/// ```
/// # use athena::encoding_and_decoding::run_length_decoder;
///
/// let data: Vec<(usize, u8)> = vec![(4, b'a'), (2, b'b'), (2, b'c')];
/// let decompressed = run_length_decoder(&data);
/// assert_eq!(decompressed, b"aaaabbcc".to_vec());
/// ```
pub fn run_length_decoder<D: Unsigned>(data: &[(usize, D)]) -> Vec<D> {
    let mut out: Vec<D> = Vec::with_capacity(data.len());
    for (count, value) in data {
        if *count == 0 {
            out.push(*value);
        } else {
            for _ in 0..*count {
                out.push(*value);
            }
        }
    }
    out
}
