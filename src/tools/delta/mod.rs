mod tests;

/// Returns the delta compressed version of the given data
/// Returned data is in binary, all integers are 
///
/// # Arguments
/// * `data` - The data to compress
///
/// # Example
/// ```
/// # use athena::utils::compression::delta::delta_encoder;
/// 
/// let data = "The quick brown fox jumps over the lazy dog".as_bytes().to_vec();
/// let compressed = delta_encoder(&data);
/// assert_eq!(compressed.len(), 43);
/// assert_eq!(data.len(), 43);
/// ```
pub fn delta_encoder(data: &[u8]) -> Vec<i16> {
    let mut out: Vec<i16> = Vec::with_capacity(data.len());
    out.push(data[0] as i16);

    let mut last: u8 = data[0];

    for entry in data.iter().skip(1) {
        out.push(last as i16 - *entry as i16);
        last = *entry;
    }

    out
}

/// Returns the delta decompressed version of the given data
///
/// # Arguments
/// * `data` - The data to decompress
///
/// # Example
/// ```
/// # use athena::utils::compression::delta::delta_decoder;
/// 
/// let data = vec![1, -1, -1, -1, -1];
/// let decompressed = delta_decoder(&data);
/// assert_eq!(decompressed.len(), 5);
/// assert_eq!(data.len(), 5);
/// assert_eq!(decompressed, vec![1, 2, 3, 4, 5]);
/// ```
pub fn delta_decoder(data: &[i16]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(data.len());
    debug_assert!(data[0] >= 0);
    out.push(data[0] as u8);

    let mut last: i16 = data[0];

    for entry in data.iter().skip(1) {
        let decoded = last - *entry;
        debug_assert!(decoded >= 0);
        out.push(decoded as u8);
        last = decoded;
    }

    out
}
