mod tests;

use crate::error::AthenaError;

const CONTINUATION_BIT: u8 = 0b10000000;

const SIGN_BIT: u8 = 0b01000000;

#[inline]
fn low_bits_of_byte(b: u8) -> u8 {
    b & !CONTINUATION_BIT
}

#[inline]
fn low_bits_of_usize(b: usize) -> u8 {
    let byte = b & (u8::MAX as usize);
    low_bits_of_byte(byte as u8)
}

/// Deserializes a unsigned integer from a LEB128 encoded byte array
/// 
/// Returns the value and the number of bytes read
/// Does not mutate the buffer
///
/// # Arguments
/// * `data` - The buffer to read from
///
/// # Example
/// ```
/// # use athena::encoding_and_decoding::deserialize_leb128_unsigned;
/// # use std::collections::VecDeque;
/// 
/// let u8max: Vec<u8> = vec![0b11111111, 0b00000001]; // 255 in binary LEB128
/// let (result, num_of_bytes) = deserialize_leb128_unsigned(&u8max).unwrap();
/// assert_eq!(result, 255);
/// assert_eq!(num_of_bytes, 2);
/// assert_ne!(u8max.len(), 0);
/// ```
pub fn deserialize_leb128_unsigned(data: &[u8]) -> Result<(usize, u8), AthenaError> {
    let mut result: usize = 0;
    let mut shift = 0;
    let mut num_of_bytes: u8 = 0;
    loop {
        if num_of_bytes as usize >= data.len() {
            return Err(AthenaError::ContinuationBitInLastByte);
        }
        let byte = data[num_of_bytes as usize];
        num_of_bytes += 1;
        let low_bits = low_bits_of_byte(byte) as usize;
        result |= low_bits << shift;
        if byte & CONTINUATION_BIT == 0 {
            return Ok((result, num_of_bytes));
        }
        shift += 7;
    }
}

/// Serializes a unsigned integer to a LEB128 encoded byte array
///
/// Does not mutate the input
///
/// # Arguments
/// * `value` - The value to serialize
///
/// # Example
/// ```
/// # use athena::encoding_and_decoding::serialize_leb128_unsigned;
/// let value = 255;
/// let serialized = serialize_leb128_unsigned(value);
/// assert_eq!(serialized, vec![0b11111111, 0b00000001]);
/// assert_eq!(value, 255);
/// ```
pub fn serialize_leb128_unsigned(value: usize) -> Vec<u8> {
    let mut val = value;
    let val_len = {
        if val <= u8::MAX as usize {
            2
        } else if val <= u16::MAX as usize {
            3
        } else if val <= u32::MAX as usize {
            5
        } else {
            10
        }
    };
    let mut out = Vec::with_capacity(val_len);
    loop {
        let mut byte = low_bits_of_usize(val);
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

/// Deserializes a signed integer from a LEB128 encoded byte array
/// 
/// Returns the value and the number of bytes read
/// Does not mutate the buffer
///
/// # Arguments
/// * `data` - The buffer to read from
///
/// # Example
/// ```
/// # use athena::encoding_and_decoding::deserialize_leb128_signed;
/// # use std::collections::VecDeque;
/// 
/// let i8min: Vec<u8> = vec![0b10000000, 0b01111111]; // -128 in binary LEB128
/// let (result, num_of_bytes) = deserialize_leb128_signed(&i8min).unwrap();
/// assert_eq!(result, -128);
/// assert_eq!(num_of_bytes, 2);
/// assert_eq!(i8min.len(), 2);
/// ```
pub fn deserialize_leb128_signed(data: &[u8]) -> Result<(i64, u8), AthenaError> {
    let mut result: i64 = 0;
    let mut shift = 0;
    let size = 64;
    let mut byte;
    let mut index: u8 = 0;

    loop {
        if index as usize >= data.len() {
            return Err(AthenaError::ContinuationBitInLastByte);
        }

        let mut buf = [0];
        buf[0] = data[index as usize];
        index += 1;

        byte = buf[0];
        if shift == 63 && byte != 0x00 && byte != 0x7f {
            return Err(AthenaError::Overflow);
        }

        let low_bits = low_bits_of_byte(byte) as i64;
        result |= low_bits << shift;
        shift += 7;
        if byte & CONTINUATION_BIT == 0 {
            break;
        }
    }

    if shift < size && (SIGN_BIT & byte) == SIGN_BIT {
        result |= !0 << shift;
    }
    Ok((result, index))
}

/// Serializes a signed integer to a LEB128 encoded byte array
///
/// Does not mutate the input
///
/// # Arguments
/// * `value` - The value to serialize
///
/// # Example
/// ```
/// # use athena::encoding_and_decoding::serialize_leb128_signed;
/// let value = -1;
/// let serialized = serialize_leb128_signed(value);
/// assert_eq!(serialized, vec![0b01111111]);
/// ```
pub fn serialize_leb128_signed(value: i64) -> Vec<u8> {
    // a i64 is 8 bytes, this expands to 10 bytes using LEB
    let mut out = Vec::with_capacity(10);
    let mut val = value;
    loop {
        let mut byte = val as u8;
        val >>= 6;
        let done = val == 0 || val == -1;
        if done {
            byte &= !CONTINUATION_BIT;
        } else {
            val >>= 1;
            byte |= CONTINUATION_BIT;
        }
        out.push(byte);
        if done {
            return out;
        }
    }
}
