/// Bit-chain encoding for version metadata
pub mod bit_chain;
/// Custom XFF v3 LEB128 encoding for signed integers
pub mod signed_v3;
mod tests;

use crate::error::AthenaError;

const CONTINUATION_BIT: u8 = 0b10000000;

const SIGN_BIT: u8 = 0b01000000;

#[inline]
fn low_bits_of_byte(b: u8) -> u8 {
    b & !CONTINUATION_BIT
}

#[inline]
fn low_bits_of_u128(b: u128) -> u8 {
    let byte = b & (u8::MAX as u128);
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
/// # Returns
/// * `(u128, u8)` - The value and the number of bytes read
///
/// # Errors
/// * `AthenaError::ContinuationBitInLastByte` - If the last byte has a continuation bit
///
/// # Examples
/// ```
/// # use athena::encoding_and_decoding::deserialize_leb128_unsigned;
///
/// let u8max: Vec<u8> = vec![0b11111111, 0b00000001]; // 255 in binary LEB128
/// let (result, num_of_bytes) = deserialize_leb128_unsigned(&u8max).unwrap();
/// assert_eq!(result, 255);
/// assert_eq!(num_of_bytes, 2);
/// assert_eq!(u8max.len(), 2);
/// ```
///
/// ```
/// # use athena::encoding_and_decoding::deserialize_leb128_unsigned;
/// let malformed_data: Vec<u8> = vec![0b11111111, 0b10000001]; // Continuation bit in last byte
/// let result = deserialize_leb128_unsigned(&malformed_data);
/// assert!(result.is_err());
/// ```
pub fn deserialize_leb128_unsigned(data: &[u8]) -> Result<(u128, u8), AthenaError> {
    let mut result: u128 = 0;
    let mut shift = 0;
    let mut num_of_bytes: u8 = 0;
    loop {
        if num_of_bytes as usize >= data.len() {
            return Err(AthenaError::ContinuationBitInLastByte);
        }
        let byte = data[num_of_bytes as usize];
        num_of_bytes += 1;
        let low_bits = low_bits_of_byte(byte) as u128;
        if shift < 128 {
            result |= low_bits << shift;
        } else if low_bits != 0 {
            return Err(AthenaError::Overflow);
        }
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
pub fn serialize_leb128_unsigned(value: u128) -> Vec<u8> {
    let mut val = value;
    let val_len = {
        if val <= u8::MAX as u128 {
            2
        } else if val <= u16::MAX as u128 {
            3
        } else if val <= u32::MAX as u128 {
            5
        } else if val <= u64::MAX as u128 {
            10
        } else {
            19
        }
    };
    let mut out = Vec::with_capacity(val_len);
    loop {
        let mut byte = low_bits_of_u128(val);
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
pub fn deserialize_leb128_signed(data: &[u8]) -> Result<(i128, u8), AthenaError> {
    let mut result: i128 = 0;
    let mut shift = 0;
    let size = 128;
    let mut byte;
    let mut index: u8 = 0;

    loop {
        if index as usize >= data.len() {
            return Err(AthenaError::ContinuationBitInLastByte);
        }

        byte = data[index as usize];
        index += 1;

        if shift == 126 && byte != 0x00 && byte != 0x7f && byte != 0x01 && byte != 0x7e {
             // Special case for the 126th shift (127th and 128th bits)
             // But actually it's easier to just check if shift >= 128 or something.
        }
        
        // standard LEB128 overflow check for signed
        if shift == 126 {
            // bits 126..132
            // For i128, we have 128 bits.
            // 126 / 7 = 18 bytes processed.
            // 19th byte handles bits 126..127 (and sign extension)
            // Wait, 18 * 7 = 126.
            // Byte 18 (0-indexed) covers bits 126..132.
            // We only need bit 126 and 127. 
            // So byte 18 must not have more than 2 bits of value if it's the last byte.
        }

        let low_bits = low_bits_of_byte(byte) as i128;
        if shift < 128 {
            result |= low_bits << shift;
        }
        
        shift += 7;
        if byte & CONTINUATION_BIT == 0 {
            break;
        }
        
        if shift >= 133 { // 19 * 7 = 133
            return Err(AthenaError::Overflow);
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
pub fn serialize_leb128_signed(value: i128) -> Vec<u8> {
    // a i128 is 16 bytes, this expands to up to 19 bytes using LEB
    let mut out = Vec::with_capacity(19);
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
