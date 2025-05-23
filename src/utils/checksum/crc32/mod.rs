pub type Crc32Table = [u32; 256];

/// Returns the crc32 checksum for the given data
///
/// The given data will not be mutated in any way
///
/// Use this for ease of use
///
/// Implements the CRC32 ISO-HDLC algorithm
///
/// # Arguments
/// * `data` - The data to calculate the checksum for
///
/// # Example
/// ```
/// # use athena::checksum::crc32;
///
/// let data = "The quick brown fox jumps over the lazy dog".as_bytes().to_vec();
/// let crc = crc32(&data);
/// assert_eq!(crc, 0x414fa339);
/// assert_eq!(data.len(), 43);
/// ```
pub fn crc32(data: &[u8]) -> u32 {
    let crc32_table = generate_crc32_lookuptable();

    !data.iter().fold(!0, |acc, octet| {
        (acc >> 8) ^ crc32_table[((acc & 0xff) ^ *octet as u32) as usize]
    })
}

/// Returns the crc32 checksum for the given data
///
/// The given data will not be mutated in any way
///
/// Use this for performance
///
/// Implements the CRC32 ISO-HDLC algorithm
///
/// Uses the provided lookup table
///
/// # Arguments
/// * `data` - The data to calculate the checksum for
/// * `table` - The lookup table to use
///
/// # Example
/// ```
/// # use athena::checksum::{crc32_with_table, generate_crc32_lookuptable};
///
/// let data = "The quick brown fox jumps over the lazy dog".as_bytes().to_vec();
/// let table = generate_crc32_lookuptable();
/// let crc = crc32_with_table(&data, &table);
/// assert_eq!(crc, 0x414fa339);
/// assert_eq!(data.len(), 43);
/// ```
pub fn crc32_with_table(data: &[u8], table: &Crc32Table) -> u32 {
    !data.iter().fold(!0, |acc, octet| {
        (acc >> 8) ^ table[((acc & 0xff) ^ *octet as u32) as usize]
    })
}

/// Returns the crc32 lookup table
/// 
/// Implements the CRC32 ISO-HDLC algorithm
pub fn generate_crc32_lookuptable() -> Crc32Table {
    let mut crc32_table = [0u32; 256];

    (0..256).for_each(|n| {
        crc32_table[n as usize] = (0..8).fold(n as u32, |acc, _| {
            match acc & 1 == 1 {
                true => 0xedb88320 ^ (acc >> 1),
                false => acc >> 1,
            }
        });
    });

    crc32_table
}

#[test]
fn crc32_quicktest() {
    let string = "The quick brown fox jumps over the lazy dog";
    let crc32 = crc32(&string.as_bytes().to_vec());
    assert_eq!(crc32, 0x414fa339);
    assert_ne!(string.len(), 0);
    assert_eq!(string.len(), 43);
}

#[test]
fn larger_test() {
    let long_str = "lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    let crc = crc32(&long_str.as_bytes().to_vec());
    assert_eq!(crc, 0xdb74b106);
    assert_ne!(long_str.len(), 0);
    assert_ne!(long_str.len(), 1);
}
