use crate::error::AthenaError;

const CONTINUATION_BIT: u8 = 0b10000000;

#[inline]
fn low_bits_of_byte(b: u8) -> u8 {
    b & !CONTINUATION_BIT
}

#[inline]
fn low_bits_of_u64(b: u64) -> u8 {
    let byte = b & (u8::MAX as u64);
    low_bits_of_byte(byte as u8)
}

/// Reads a LEB128 encoded unsigned integer
/// 
/// Returns the value and the number of bytes read
/// Does not mutate the buffer
///
/// # Arguments
/// * `data` - The buffer to read from
///
/// # Example
/// ```
/// # use athena::tools::leb128::deserialize_leb128_unsigned;
/// # use std::collections::VecDeque;
/// 
/// let u8max: Vec<u8> = vec![0b11111111, 0b00000001]; // 255 in binary LEB128
/// let (result, num_of_bytes) = deserialize_leb128_unsigned(&u8max).unwrap();
/// assert_eq!(result, 255);
/// assert_eq!(num_of_bytes, 2);
/// assert_ne!(u8max.len(), 0);
/// ```
pub fn deserialize_leb128_unsigned(data: &[u8]) -> Result<(u64, u8), AthenaError> {
    let mut result: u64 = 0;
    let mut shift = 0;
    let mut num_of_bytes: u8 = 0;
    loop {
        if num_of_bytes as usize >= data.len() {
            return Err(AthenaError::InsufficientBuffer);
        }
        let byte = data[num_of_bytes as usize];
        num_of_bytes += 1;
        let low_bits = low_bits_of_byte(byte) as u64;
        result |= low_bits << shift;
        if byte & CONTINUATION_BIT == 0 {
            return Ok((result, num_of_bytes));
        }
        shift += 7;
    }
}

/// Serializes a LEB128 encoded unsigned integer
///
/// Does not mutate the input
///
/// # Arguments
/// * `value` - The value to serialize
///
/// # Example
/// ```
/// # use athena::tools::leb128::serialize_leb128_unsigned;
/// let value = 255;
/// let serialized = serialize_leb128_unsigned(value);
/// assert_eq!(serialized, vec![0b11111111, 0b00000001]);
/// assert_eq!(value, 255);
/// ```
pub fn serialize_leb128_unsigned(value: u64) -> Vec<u8> {
    let mut val = value;
    let val_len = {
        if val <= u8::MAX as u64 {
            2
        } else if val <= u16::MAX as u64 {
            3
        } else if val <= u32::MAX as u64 {
            5
        } else {
            10
        }
    };
    let mut out = Vec::with_capacity(val_len);
    loop {
        let mut byte = low_bits_of_u64(val);
        val >>= 7;
        if val != 0 {
            byte |= CONTINUATION_BIT;
        }
        out.push(byte);
        if val == 0 {
            return out;
        }
    }
}

// Still takes 2.6 sec! -> optimized in refactor of api, now 1.3/1.4 sec
// rughly 12mil numbers per sec
#[test]
fn serde_all_the_numbers_u24() {
    let wtf_compiler: u32 = 16_777_215; // u24::MAX
    for i in 0..=wtf_compiler {
        let serialized = serialize_leb128_unsigned(i.into());
        let (result, len) = deserialize_leb128_unsigned(&serialized).unwrap();
        assert_eq!(result, i.into());
        assert!(len <= 4);
        /* if i % 1000 == 0 {
            println!("{}", i);
        } */
    }
}

// 420 sec last time in non release mode, 78 sec in release mode
// rughly 10mil numbers per sec for non release
// rughly 55mil numbers per sec for release
#[ignore = "Takes a long time; 150s, < 30 in release"]
#[test]
fn serde_all_the_numbers_low_u32() {
    for i in 0..=u32::MAX / 3 {
        let serialized = serialize_leb128_unsigned(i.into());
        let (result, _) = deserialize_leb128_unsigned(&serialized).unwrap();
        assert_eq!(result, i.into());
        assert_ne!(serialized.len(), 0);
    }
}

#[ignore = "Takes a long time; 150s, < 30 in release"]
#[test]
fn serde_all_the_numbers_mid_u32() {
    let upper_bound: u64 = (u32::MAX as u64 * 2) / 3;
    for i in (u32::MAX / 3) as u64..=upper_bound {
        let serialized = serialize_leb128_unsigned(i.into());
        let (result, _) = deserialize_leb128_unsigned(&serialized).unwrap();
        assert_eq!(result, i.into());
        assert_ne!(serialized.len(), 0);
    }
}

#[ignore = "Takes a long time; 150s, < 30 in release"]
#[test]
fn serde_all_the_numbers_high_u32() {
    let upper_bound: u64 = (u32::MAX as u64 * 2) / 3;
    for i in upper_bound..=u32::MAX  as u64 {
        let serialized = serialize_leb128_unsigned(i.into());
        let (result, _) = deserialize_leb128_unsigned(&serialized).unwrap();
        assert_eq!(result, i.into());
        assert_ne!(serialized.len(), 0);
    }
}

#[test]
fn serialize_leb_unsigned() {
    let data = vec![255, 65535, 4294967295, 18446744073709551615, 16777215, 1099511627775, 281474976710655, 72057594037927935];
    let u8max = vec![0b11111111, 0b00000001]; // 1
    let u16max = vec![0b11111111, 0b11111111, 0b00000011]; // 2
    let u32max = vec![0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00001111]; // 4
    let u64max = vec![0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00000001]; // 8
    let u24max = vec![0b11111111, 0b11111111, 0b11111111, 0b00000111]; // 3
    let u40max = vec![0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00011111]; // 5
    let u48max = vec![0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00111111]; // 6
    let u56max = vec![0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b01111111]; // 7
    let all_u = vec![u8max, u16max, u32max, u64max, u24max, u40max, u48max, u56max];
    for (i, d) in data.iter().enumerate() {
        let serialized = serialize_leb128_unsigned(*d);
        let ser_check: Vec<u8> = serialized.clone().into();
        let deserialized = deserialize_leb128_unsigned(&serialized).unwrap();
        assert_eq!(all_u[i], ser_check);
        assert_eq!(d, &deserialized.0);
        assert_ne!(serialized.len(), 0);
    }
}

#[test]
fn deserialize_leb_unsigned() {
    let mut data = Vec::new();
    let u8max = vec![0b11111111, 0b00000001]; // 1
    let u16max = vec![0b11111111, 0b11111111, 0b00000011]; // 2
    let u32max = vec![0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00001111]; // 4
    let u64max = vec![0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00000001]; // 8
    let all_u = vec![u8max, u16max, u32max, u64max];
    let all_u_res: Vec<(u64, u8)> = vec![(255, 2), (65535, 3), (4_294_967_295, 5), (18_446_744_073_709_551_615, 10)];
    let mut count = 0;
    for (i, u) in all_u.iter().enumerate() {
        data.clear();
        data.extend(u.iter().cloned());
        let (result, num_of_bytes) = deserialize_leb128_unsigned(&data).unwrap();
        assert_eq!(result, all_u_res[i].0);
        assert_eq!(num_of_bytes, all_u_res[i].1);
        count += 1;
    }
    assert_eq!(count, 4);
    let u24max = vec![0b11111111, 0b11111111, 0b11111111, 0b00000111]; // 3
    let u40max = vec![0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00011111]; // 5
    let u48max = vec![0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00111111]; // 6
    let u56max = vec![0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b01111111]; // 7
    let new_u = vec![u24max, u40max, u48max, u56max];
    let new_u_res: Vec<(u64, u8)> = vec![(16777215, 4), (1099511627775, 6), (281474976710655, 7), (72057594037927935, 8)];
    count = 0;
    for (i, u) in new_u.iter().enumerate() {
        data.clear();
        data.extend(u.iter().cloned());
        let (result, num_of_bytes) = deserialize_leb128_unsigned(&data).unwrap();
        assert_eq!(result, new_u_res[i].0);
        assert_eq!(num_of_bytes, new_u_res[i].1);
        count += 1;
    }
    assert_eq!(count, 4);
}
