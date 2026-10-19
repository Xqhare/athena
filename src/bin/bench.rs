#![cfg(any(doc, feature = "encoding_decoding", feature = "checksum"))]
use athena::checksum::crc32;
use athena::encoding_and_decoding::{
    deserialize_leb128_signed_v3, deserialize_leb128_unsigned, serialize_leb128_signed_v3,
    serialize_leb128_unsigned,
};
use std::time::Instant;

fn main() {
    println!("=== Running Performance Benchmark ===");

    // 1. CRC32 Performance
    let data = vec![0u8; 1024 * 1024]; // 1MB buffer
    let start = Instant::now();
    let mut sum = 0;
    for _ in 0..100 {
        sum ^= crc32(&data);
    }
    let duration = start.elapsed();
    println!("CRC-32 on 1MB (100 iterations) took: {:?}", duration);
    println!("Checksum value (to prevent optimization): {:x}", sum);

    // 2. LEB128 Unsigned single-byte (Fast Path)
    let start = Instant::now();
    let mut accum = 0;
    let serialized = serialize_leb128_unsigned(42);
    for _ in 0..1_000_000 {
        let (val, _) = deserialize_leb128_unsigned(&serialized).unwrap();
        accum ^= val;
    }
    let duration = start.elapsed();
    println!(
        "LEB128 Unsigned single-byte deserialize (1M iterations) took: {:?}",
        duration
    );
    println!("Accum value: {}", accum);

    // 3. LEB128 Signed v3 multi-byte
    let start = Instant::now();
    let mut accum_v3 = 0;
    let serialized_v3 = serialize_leb128_signed_v3(1234567890);
    for _ in 0..1_000_000 {
        let (val, _) = deserialize_leb128_signed_v3(&serialized_v3).unwrap();
        accum_v3 ^= val;
    }
    let duration = start.elapsed();
    println!(
        "LEB128 Signed V3 multi-byte deserialize (1M iterations) took: {:?}",
        duration
    );
    println!("Accum V3 value: {}", accum_v3);
}
