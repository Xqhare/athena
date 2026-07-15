use crate::error::{AthenaError, AthenaResult};

const CONTINUATION_BIT: u8 = 0b10000000;
const SIGN_BIT_V3: u8 = 0b01000000; // Bit 6 of the first byte

/// Serializes a signed integer using the XFF v3 custom LEB128 encoding into a provided buffer.
/// Returns the number of bytes written, or `None` if the buffer is too small.
pub fn serialize_leb128_signed_v3_buf(value: i64, buf: &mut [u8]) -> Option<usize> {
    if buf.is_empty() {
        return None;
    }
    let is_negative = value < 0;
    let mut abs_value = value.unsigned_abs();

    // Process first byte (6 bits of value)
    let mut first_byte = (abs_value & 0x3F) as u8;
    abs_value >>= 6;

    if is_negative {
        first_byte |= SIGN_BIT_V3;
    }

    if abs_value > 0 {
        first_byte |= CONTINUATION_BIT;
    }
    buf[0] = first_byte;
    let mut idx = 1;

    // Process subsequent bytes (7 bits of value)
    while abs_value > 0 {
        if idx >= buf.len() {
            return None;
        }
        let mut byte = (abs_value & 0x7F) as u8;
        abs_value >>= 7;
        if abs_value > 0 {
            byte |= CONTINUATION_BIT;
        }
        buf[idx] = byte;
        idx += 1;
    }

    Some(idx)
}

/// Serializes a signed integer using the XFF v3 custom LEB128 encoding.
///
/// First Byte: [Continuation (1bit)] [Sign (1bit)] [Value (6bits)]
/// Subsequent Bytes: [Continuation (1bit)] [Value (7bits)]
#[must_use]
pub fn serialize_leb128_signed_v3(value: i64) -> Vec<u8> {
    let mut buf = [0u8; 10];
    let len = serialize_leb128_signed_v3_buf(value, &mut buf).unwrap();
    buf[..len].to_vec()
}


/// Deserializes a signed integer using the XFF v3 custom LEB128 encoding.
pub fn deserialize_leb128_signed_v3(data: &[u8]) -> AthenaResult<(i64, u8)> {
    if data.is_empty() {
        return Err(AthenaError::ContinuationBitInLastByte.into());
    }

    let first_byte = data[0];
    let is_negative = (first_byte & SIGN_BIT_V3) != 0;

    // Fast-path: single byte
    if (first_byte & CONTINUATION_BIT) == 0 {
        let val = i64::from(first_byte & 0x3F);
        let final_val = if is_negative { -val } else { val };
        return Ok((final_val, 1));
    }

    let mut result = u64::from(first_byte & 0x3F);
    let mut num_of_bytes = 1;

    let mut shift = 6;
    loop {
        if num_of_bytes >= data.len() {
            return Err(AthenaError::ContinuationBitInLastByte.into());
        }
        let byte = data[num_of_bytes];
        num_of_bytes += 1;

        result |= u64::from(byte & 0x7F) << shift;
        shift += 7;

        if (byte & CONTINUATION_BIT) == 0 {
            break;
        }
        if shift >= 64 {
            return Err(AthenaError::Overflow.into());
        }
    }

    let final_val = if is_negative {
        (-i128::from(result)) as i64
    } else {
        result as i64
    };

    Ok((final_val, num_of_bytes as u8))
}

/// Serializes a signed i128 integer using the XFF v3 custom LEB128 encoding into a provided buffer.
/// Returns the number of bytes written, or `None` if the buffer is too small.
pub fn serialize_leb128_signed_i128_buf(value: i128, buf: &mut [u8]) -> Option<usize> {
    if buf.is_empty() {
        return None;
    }
    let is_negative = value < 0;
    let mut abs_value = value.unsigned_abs();

    // Process first byte (6 bits of value)
    let mut first_byte = (abs_value & 0x3F) as u8;
    abs_value >>= 6;

    if is_negative {
        first_byte |= SIGN_BIT_V3;
    }

    if abs_value > 0 {
        first_byte |= CONTINUATION_BIT;
    }
    buf[0] = first_byte;
    let mut idx = 1;

    // Process subsequent bytes (7 bits of value)
    while abs_value > 0 {
        if idx >= buf.len() {
            return None;
        }
        let mut byte = (abs_value & 0x7F) as u8;
        abs_value >>= 7;
        if abs_value > 0 {
            byte |= CONTINUATION_BIT;
        }
        buf[idx] = byte;
        idx += 1;
    }

    Some(idx)
}

/// Serializes a signed i128 integer using the XFF v3 custom LEB128 encoding.
///
/// First Byte: [Continuation (1bit)] [Sign (1bit)] [Value (6bits)]
/// Subsequent Bytes: [Continuation (1bit)] [Value (7bits)]
#[must_use]
pub fn serialize_leb128_signed_i128(value: i128) -> Vec<u8> {
    let mut buf = [0u8; 19];
    let len = serialize_leb128_signed_i128_buf(value, &mut buf).unwrap();
    buf[..len].to_vec()
}

/// Deserializes a signed i128 integer using the XFF v3 custom LEB128 encoding.
pub fn deserialize_leb128_signed_i128(data: &[u8]) -> AthenaResult<(i128, u8)> {
    if data.is_empty() {
        return Err(AthenaError::ContinuationBitInLastByte.into());
    }

    let first_byte = data[0];
    let is_negative = (first_byte & SIGN_BIT_V3) != 0;

    // Fast-path: single byte
    if (first_byte & CONTINUATION_BIT) == 0 {
        let val = i128::from(first_byte & 0x3F);
        let final_val = if is_negative { -val } else { val };
        return Ok((final_val, 1));
    }

    let mut result = u128::from(first_byte & 0x3F);
    let mut num_of_bytes = 1;

    let mut shift = 6;
    loop {
        if num_of_bytes >= data.len() {
            return Err(AthenaError::ContinuationBitInLastByte.into());
        }
        let byte = data[num_of_bytes];
        num_of_bytes += 1;

        result |= u128::from(byte & 0x7F) << shift;
        shift += 7;

        if (byte & CONTINUATION_BIT) == 0 {
            break;
        }
        if shift >= 128 {
            return Err(AthenaError::Overflow.into());
        }
    }

    let final_val = if is_negative {
        -(result as i128)
    } else {
        result as i128
    };

    Ok((final_val, num_of_bytes as u8))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signed_v3_serde() {
        let test_values = vec![
            0,
            1,
            -1,
            63,
            -63,
            64,
            -64,
            127,
            -127,
            128,
            -128,
            1000,
            -1000,
            i64::MAX,
            i64::MIN,
            i64::MIN + 1,
        ];
        for v in test_values {
            let serialized = serialize_leb128_signed_v3(v);
            let (deserialized, len) = deserialize_leb128_signed_v3(&serialized).unwrap();
            assert_eq!(v, deserialized, "Failed for value {}", v);
            assert_eq!(serialized.len(), len as usize);
        }
    }

    #[test]
    fn test_signed_v3_buf() {
        let mut buf = [0u8; 10];
        let len = serialize_leb128_signed_v3_buf(-64, &mut buf).unwrap();
        // continuation bit 0 | sign bit 1 | 0 value (abs_val for -64 is 64; 64 >> 6 is 1; so first byte is (0 & 0x3F) | 0x40 = 0x40. Since remainder is > 0, it also has continuation bit 0x80. So first byte is 0xC0. The next byte is (1 & 0x7F) = 1. So we get [0xC0, 0x01].
        assert_eq!(&buf[..len], &[0xC0, 0x01]);
        
        let too_small = serialize_leb128_signed_v3_buf(-64, &mut buf[..0]);
        assert!(too_small.is_none());

        let mut buf_i128 = [0u8; 19];
        let len_i128 = serialize_leb128_signed_i128_buf(-64, &mut buf_i128).unwrap();
        assert_eq!(&buf_i128[..len_i128], &[0xC0, 0x01]);
    }
}
