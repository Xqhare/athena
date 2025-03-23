#![allow(unused_imports)]
use crate::tools::byte_bit::{byte_bit_decoder, byte_bit_encoder, byte_bit_reader};

use super::byte_bit_writer;

#[test]
fn encode_decode_all_u8() {
    for n in 0..256 {
        let bits = byte_bit_decoder(n as u8);
        let encoded = byte_bit_encoder(&bits);
        assert_eq!(encoded, n as u8);
    }
}

#[test]
fn writer_reader_all_u8() {
    let mut buffer = Vec::new();
    for n in 0..256 {
        buffer.clear();
        let tmp = byte_bit_decoder(n as u8);
        byte_bit_writer(&mut buffer, vec![tmp]).unwrap();
        assert_eq!(byte_bit_decoder(buffer[0]), tmp);
        let read = byte_bit_reader(buffer.as_slice()).unwrap();
        assert_eq!(read[0], tmp);
    }
}

#[test]
fn writer_reader_all_u16() {
    let mut buffer = Vec::new();
    for n in 0..=u16::MAX {
        buffer.clear();
        let tt = n.to_le_bytes();
        let tmp = vec![byte_bit_decoder(tt[0]), byte_bit_decoder(tt[1])];
        let ok = byte_bit_writer(&mut buffer, tmp.clone());
        assert!(ok.is_ok());
        let read = byte_bit_reader(buffer.as_slice()).unwrap();
        assert_eq!(read, tmp);
    }
}

#[ignore = "Checks all u32, Takes a long time"]
#[test]
fn writer_reader_all_u32() {
    let mut buffer = Vec::with_capacity(4);
    for n in 0..=u32::MAX {
        buffer.clear();
        let tt = n.to_le_bytes();
        let tmp = vec![byte_bit_decoder(tt[0]), byte_bit_decoder(tt[1]), byte_bit_decoder(tt[2]), byte_bit_decoder(tt[3])];
        let ok = byte_bit_writer(&mut buffer, tmp.clone());
        assert!(ok.is_ok());
        let read = byte_bit_reader(buffer.as_slice()).unwrap();
        assert_eq!(read, tmp);
    }
}
