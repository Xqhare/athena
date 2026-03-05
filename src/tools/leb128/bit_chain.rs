use crate::error::AthenaError;

const CONTINUATION_BIT: u8 = 0b10000000;

/// Serializes a version number using the bit-chain encoding.
///
/// All bits set to '1' are added together to form the version number.
/// The last version byte is always padded to a full width byte with '0'.
///
/// # Example
/// ```
/// # use athena::encoding_and_decoding::serialize_version_bit_chain;
/// assert_eq!(serialize_version_bit_chain(0), vec![0x00]);
/// assert_eq!(serialize_version_bit_chain(1), vec![0x01]);
/// assert_eq!(serialize_version_bit_chain(3), vec![0x07]); // 3 ones: 0000 0111
/// assert_eq!(serialize_version_bit_chain(8), vec![0xFF, 0x01]); // 7 ones + 1 one
/// ```
pub fn serialize_version_bit_chain(version: usize) -> Vec<u8> {
    if version == 0 {
        return vec![0];
    }

    let mut out = Vec::new();
    let mut remaining = version;

    while remaining > 0 {
        let take = std::cmp::min(remaining, 7);
        let mut byte = (1u8 << take) - 1; // Set 'take' number of bits to 1
        remaining -= take;

        if remaining > 0 {
            byte |= CONTINUATION_BIT;
        }
        out.push(byte);
    }

    out
}

/// Deserializes a version number from a bit-chain encoded byte array.
///
/// Returns the version and the number of bytes read.
pub fn deserialize_version_bit_chain(data: &[u8]) -> Result<(usize, u8), AthenaError> {
    let mut version = 0;
    let mut num_of_bytes = 0;

    loop {
        if num_of_bytes >= data.len() {
            return Err(AthenaError::ContinuationBitInLastByte);
        }
        let byte = data[num_of_bytes];
        num_of_bytes += 1;

        version += (byte & !CONTINUATION_BIT).count_ones() as usize;

        if byte & CONTINUATION_BIT == 0 {
            return Ok((version, num_of_bytes as u8));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_bit_chain_serde() {
        for v in 0..100 {
            let serialized = serialize_version_bit_chain(v);
            let (deserialized, len) = deserialize_version_bit_chain(&serialized).unwrap();
            assert_eq!(v, deserialized);
            assert_eq!(serialized.len(), len as usize);
        }
    }

    #[test]
    fn test_specific_examples() {
        assert_eq!(serialize_version_bit_chain(0), vec![0x00]);
        assert_eq!(serialize_version_bit_chain(1), vec![0x01]);
        assert_eq!(serialize_version_bit_chain(2), vec![0x03]);
        assert_eq!(serialize_version_bit_chain(3), vec![0x07]);
        assert_eq!(serialize_version_bit_chain(7), vec![0x7F]);
        assert_eq!(serialize_version_bit_chain(8), vec![0xFF, 0x01]);
    }
}
