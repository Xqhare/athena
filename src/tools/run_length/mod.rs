use crate::utils::traits::unsigned::Unsigned;

mod tests;

/// Returns the run length encoded version of the given data
/// Returned data is in binary, returned count includes 1 
/// 
/// # Arguments
/// * `data` - The data to compress
/// 
/// # Returns
/// * `Vec<(usize, u8)>` - The run length encoded data in format (count, value)
///
/// # Example
/// ```
/// # use athena::encoding_and_decoding::run_length_encoder;
/// 
/// let data = "AAAABBBCCDEEEEFFGGGGHHIJKLLMMNNNPQQRRSTUUVVVVWWXYYZ".as_bytes().to_vec();
/// let compressed = run_length_encoder(&data);
/// assert_eq!(compressed.len(), 25);
/// assert_eq!(data.len(), 51);
/// ```
pub fn run_length_encoder<D: Unsigned>(data: &[D]) -> Vec<(usize, D)> {
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

/// Returns the run length decoded version of the given data
/// Requires the data to be in the format (count, value)
/// Count = 0 is the same as Count = 1
///
/// # Arguments
/// * `data` - The data to decompress
///     * `count` - The number of times to repeat the value
///     * `value` - The value to repeat
/// 
/// # Returns
/// * `Vec<u8>` - The decompressed data
///
/// # Errors
/// * `AthenaError::InvalidRunLength` - If any count is 0
///
/// # Example
/// ```
/// # use athena::encoding_and_decoding::run_length_decoder;
/// 
/// let data: Vec<(usize, u8)> = vec![(4, 97), (1, 98), (2, 99), (3, 100)];
/// let decompressed = run_length_decoder(&data);
/// assert_eq!(decompressed.len(), 10);
/// assert_eq!(data.len(), 4);
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
