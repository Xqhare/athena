use std::collections::HashMap;

/// Compresses the given data using the LZW algorithm
/// 
/// # Arguments
/// * `data` - The data to compress
/// 
/// # Returns
/// * `Vec<u32>` - The compressed data
///
/// # Example
/// ```
/// # use athena::utils::compression::lzw::compress_lzw;
/// 
/// let data = "The quick brown fox jumps over the lazy dog".as_bytes().to_vec();
/// let compressed = compress_lzw(&data);
/// assert_eq!(compressed.len(), 42);
/// assert_eq!(data.len(), 43);
/// ```
pub fn compress_lzw(data: &[u8]) -> Vec<u32> {
    let mut dict: HashMap<Vec<u8>, u32> = (0u32..=255).map(|i| (vec![i as u8], i)).collect();
    let mut tmp = Vec::new();
    let mut comp = Vec::new();

    for &byte in data {
        let mut work_tmp = tmp.clone();
        work_tmp.push(byte);
        if dict.contains_key(&work_tmp) {
            tmp = work_tmp;
        } else {
            comp.push(dict[&tmp]);
            dict.insert(work_tmp, dict.len() as u32);
            tmp = vec![byte];
        }
    }

    if !tmp.is_empty() {
        comp.push(dict[&tmp]);
    }

    comp
}

/// Decompresses the given data using the LZW algorithm
/// 
/// # Arguments
/// * `data` - The data to decompress
/// 
/// # Returns
/// * `Vec<u8>` - The decompressed data
/// 
/// # Example
/// ```
/// # use athena::utils::compression::lzw::{decompress_lzw, compress_lzw};
/// 
/// let data = "The quick brown fox jumps over the lazy dog".as_bytes().to_vec();
/// let compressed = compress_lzw(&data);
/// let decompressed = decompress_lzw(&compressed);
/// assert_eq!(String::from_utf8(decompressed).unwrap(), "The quick brown fox jumps over the lazy dog".to_string());
/// ```
pub fn decompress_lzw(mut data: &[u32]) -> Vec<u8> {
    let mut dict: HashMap<u32, Vec<u8>> = (0u32..=255).map(|i| (i, vec![i as u8])).collect();
    let mut tmp = dict[&data[0]].clone();
    data = &data[1..];
    let mut out = tmp.clone();

    for &unit in data {
        let entry = if dict.contains_key(&unit) {
            dict[&unit].clone()
        } else if unit == dict.len() as u32 {
            let mut entry = tmp.clone();
            entry.push(tmp[0]);
            entry
        } else {
            panic!("Invalid unit {}", unit);
        };

        out.extend_from_slice(&entry);
        tmp.push(entry[0]);
        dict.insert(dict.len() as u32, tmp);

        tmp = entry;
    }

    out
}
