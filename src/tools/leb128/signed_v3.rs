use crate::error::AthenaError;

const CONTINUATION_BIT: u8 = 0b10000000;
const SIGN_BIT_V3: u8 = 0b01000000; // Bit 6 of the first byte

/// Serializes a signed integer using the XFF v3 custom LEB128 encoding.
///
/// First Byte: [Continuation (1bit)] [Sign (1bit)] [Value (6bits)]
/// Subsequent Bytes: [Continuation (1bit)] [Value (7bits)]
#[must_use]
pub fn serialize_leb128_signed_v3(value: i64) -> Vec<u8> {
    let mut out = Vec::new();
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
    out.push(first_byte);

    // Process subsequent bytes (7 bits of value)
    while abs_value > 0 {
        let mut byte = (abs_value & 0x7F) as u8;
        abs_value >>= 7;
        if abs_value > 0 {
            byte |= CONTINUATION_BIT;
        }
        out.push(byte);
    }

    out
}

/// Deserializes a signed integer using the XFF v3 custom LEB128 encoding.
pub fn deserialize_leb128_signed_v3(data: &[u8]) -> Result<(i64, u8), AthenaError> {
    if data.is_empty() {
        return Err(AthenaError::ContinuationBitInLastByte);
    }

    let first_byte = data[0];
    let is_negative = (first_byte & SIGN_BIT_V3) != 0;
    let mut result = u64::from(first_byte & 0x3F);
    let mut num_of_bytes = 1;

    if (first_byte & CONTINUATION_BIT) != 0 {
        let mut shift = 6;
        loop {
            if num_of_bytes >= data.len() {
                return Err(AthenaError::ContinuationBitInLastByte);
            }
            let byte = data[num_of_bytes];
            num_of_bytes += 1;

            result |= u64::from(byte & 0x7F) << shift;
            shift += 7;

            if (byte & CONTINUATION_BIT) == 0 {
                break;
            }
            if shift >= 64 {
                return Err(AthenaError::Overflow);
            }
        }
    }

    let final_val = if is_negative {
        (-i128::from(result)) as i64
    } else {
        result as i64
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
}
