#![allow(unused_imports)]
use crate::compression::{compress_lzw, decompress_lzw};

#[test]
fn compression_check() {
    let (data, len) = {
        let tmp = std::fs::read_to_string("src/utils/compression/lzw/mod.rs").unwrap();
        (tmp.as_bytes().to_vec(), tmp.len())
    };
    let compressed = compress_lzw(&data);
    assert!(compressed.len() < len);

    let (data, len) = {
        let tmp = std::fs::read_to_string("Session.vim").unwrap();
        (tmp.as_bytes().to_vec(), tmp.len())
    };
    let compressed = compress_lzw(&data);
    assert!(compressed.len() < len);

    let (data, len) = {
        let tmp = std::fs::read_to_string("README.md").unwrap();
        (tmp.as_bytes().to_vec(), tmp.len())
    };
    let compressed = compress_lzw(&data);
    assert!(compressed.len() < len);

    let (data, len) = {
        let tmp = std::fs::read_to_string("LICENSE").unwrap();
        (tmp.as_bytes().to_vec(), tmp.len())
    };
    let compressed = compress_lzw(&data);
    assert!(compressed.len() < len);

    let (data, len) = {
        let tmp = std::fs::read_to_string("Cargo.toml").unwrap();
        (tmp.as_bytes().to_vec(), tmp.len())
    };
    let compressed = compress_lzw(&data);
    assert!(compressed.len() < len);
}

#[test]
fn compression_decompression() {
    let (data, string) = {
        let tmp = std::fs::read_to_string("src/utils/compression/lzw/mod.rs").unwrap();
        (tmp.as_bytes().to_vec(), tmp)
    };
    let compressed = compress_lzw(&data);
    let decompressed = decompress_lzw(&compressed);
    assert_eq!(String::from_utf8(decompressed).unwrap(), string);

    let (data, string) = {
        let tmp = std::fs::read_to_string("Session.vim").unwrap();
        (tmp.as_bytes().to_vec(), tmp)
    };
    let compressed = compress_lzw(&data);
    let decompressed = decompress_lzw(&compressed);
    assert_eq!(String::from_utf8(decompressed).unwrap(), string);

    let (data, string) = {
        let tmp = std::fs::read_to_string("README.md").unwrap();
        (tmp.as_bytes().to_vec(), tmp)
    };
    let compressed = compress_lzw(&data);
    let decompressed = decompress_lzw(&compressed);
    assert_eq!(String::from_utf8(decompressed).unwrap(), string);

    let (data, string) = {
        let tmp = std::fs::read_to_string("LICENSE").unwrap();
        (tmp.as_bytes().to_vec(), tmp)
    };
    let compressed = compress_lzw(&data);
    let decompressed = decompress_lzw(&compressed);
    assert_eq!(String::from_utf8(decompressed).unwrap(), string);

    let (data, string) = {
        let tmp = std::fs::read_to_string("Cargo.toml").unwrap();
        (tmp.as_bytes().to_vec(), tmp)
    };
    let compressed = compress_lzw(&data);
    let decompressed = decompress_lzw(&compressed);
    assert_eq!(String::from_utf8(decompressed).unwrap(), string);
}
