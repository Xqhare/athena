/// A 256-element lookup table for CRC32 calculations
pub type Crc32Table = [u32; 256];

/// A type representing the 8 lookup tables for the slice-by-8 CRC32 algorithm.
pub type Crc32Slice8Table = [[u32; 256]; 8];

/// Static lookup table for standard CRC32.
pub const CRC32_TABLE: Crc32Table = generate_crc32_lookuptable();

/// Static lookup tables for slice-by-8 CRC32.
pub const CRC32_SLICE8_TABLE: Crc32Slice8Table = generate_crc32_slice8_table();

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
#[must_use]
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = !0u32;
    let chunks = data.chunks_exact(8);
    let remainder = chunks.remainder();

    for chunk in chunks {
        let bytes: [u8; 8] = chunk.try_into().unwrap();
        let term = crc ^ u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        
        crc = CRC32_SLICE8_TABLE[7][(term & 0xff) as usize]
            ^ CRC32_SLICE8_TABLE[6][((term >> 8) & 0xff) as usize]
            ^ CRC32_SLICE8_TABLE[5][((term >> 16) & 0xff) as usize]
            ^ CRC32_SLICE8_TABLE[4][(term >> 24) as usize]
            ^ CRC32_SLICE8_TABLE[3][bytes[4] as usize]
            ^ CRC32_SLICE8_TABLE[2][bytes[5] as usize]
            ^ CRC32_SLICE8_TABLE[1][bytes[6] as usize]
            ^ CRC32_SLICE8_TABLE[0][bytes[7] as usize];
    }

    for &octet in remainder {
        crc = (crc >> 8) ^ CRC32_SLICE8_TABLE[0][((crc & 0xff) ^ u32::from(octet)) as usize];
    }

    !crc
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
#[must_use]
pub fn crc32_with_table(data: &[u8], table: &Crc32Table) -> u32 {
    !data.iter().fold(!0, |acc, octet| {
        (acc >> 8) ^ table[((acc & 0xff) ^ u32::from(*octet)) as usize]
    })
}

/// Returns the crc32 lookup table
///
/// Implements the CRC32 ISO-HDLC algorithm
#[must_use]
pub const fn generate_crc32_lookuptable() -> Crc32Table {
    let mut crc32_table = [0u32; 256];
    let mut n = 0;
    while n < 256 {
        let mut acc = n as u32;
        let mut i = 0;
        while i < 8 {
            if acc & 1 == 1 {
                acc = 0xedb88320 ^ (acc >> 1);
            } else {
                acc >>= 1;
            }
            i += 1;
        }
        crc32_table[n] = acc;
        n += 1;
    }
    crc32_table
}

/// Generates the 8 lookup tables for the slice-by-8 CRC32 algorithm.
#[must_use]
pub const fn generate_crc32_slice8_table() -> Crc32Slice8Table {
    let mut slice8_table = [[0u32; 256]; 8];
    let base_table = generate_crc32_lookuptable();
    
    let mut n = 0;
    while n < 256 {
        slice8_table[0][n] = base_table[n];
        n += 1;
    }
    
    let mut i = 1;
    while i < 8 {
        let mut j = 0;
        while j < 256 {
            let val = slice8_table[i - 1][j];
            slice8_table[i][j] = (val >> 8) ^ base_table[(val & 0xff) as usize];
            j += 1;
        }
        i += 1;
    }
    
    slice8_table
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

#[test]
fn test_crc32_various_sizes() {
    let data = (0..100).collect::<Vec<u8>>();
    for size in 0..=100 {
        let sub_data = &data[..size];
        let table = generate_crc32_lookuptable();
        let expected = crc32_with_table(sub_data, &table);
        let actual = crc32(sub_data);
        assert_eq!(expected, actual, "Failed for size {}", size);
    }
}

