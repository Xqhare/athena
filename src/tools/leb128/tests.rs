#![allow(unused_imports)]

mod unsigned {
    use crate::tools::leb128::{deserialize_leb128_unsigned, serialize_leb128_unsigned};

    #[test]
    fn deser_err() {
        // continuation bit set in last byte
        let serialized = vec![0b10000000];
        let err = deserialize_leb128_unsigned(&serialized);
        assert!(err.is_err());
    }

    #[test]
    fn serde_all_the_numbers_u24() {
        let wtf_compiler: u32 = 16_777_215; // u24::MAX
        for i in 0..=wtf_compiler {
            let serialized = serialize_leb128_unsigned(i as u128);
            let (result, len) = deserialize_leb128_unsigned(&serialized).unwrap();
            assert_eq!(result, i as u128);
            assert!(len <= 4);
        }
    }

    #[test]
    fn serialize_leb_unsigned() {
        let data: Vec<u128> = vec![
            255,
            65535,
            4294967295,
            18446744073709551615,
            16777215,
            1099511627775,
            281474976710655,
            72057594037927935,
        ];
        let u8max = vec![0b11111111, 0b00000001]; // 255
        let u16max = vec![0b11111111, 0b11111111, 0b00000011]; // 65535
        let u32max = vec![0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00001111]; // 4294967295
        let u64max = vec![
            0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
            0b11111111, 0b11111111, 0b00000001,
        ]; // 18446744073709551615
        let u24max = vec![0b11111111, 0b11111111, 0b11111111, 0b00000111]; // 16777215
        let u40max = vec![
            0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00011111,
        ]; // 1099511627775
        let u48max = vec![
            0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00111111,
        ]; // 281474976710655
        let u56max = vec![
            0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
            0b01111111,
        ]; // 72057594037927935
        let all_u = vec![
            u8max, u16max, u32max, u64max, u24max, u40max, u48max, u56max,
        ];
        for (i, d) in data.iter().enumerate() {
            let serialized = serialize_leb128_unsigned(*d as u128);
            let deserialized = deserialize_leb128_unsigned(&serialized).unwrap();
            assert_eq!(all_u[i], serialized);
            assert_eq!(*d as u128, deserialized.0);
        }
    }

    #[test]
    fn serde_u128() {
        let val: u128 = u128::MAX;
        let serialized = serialize_leb128_unsigned(val);
        let (deserialized, len) = deserialize_leb128_unsigned(&serialized).unwrap();
        assert_eq!(val, deserialized);
        assert_eq!(serialized.len(), len as usize);
        assert_eq!(len, 19);
    }
}

mod signed {
    use crate::{
        error::AthenaError,
        tools::leb128::{deserialize_leb128_signed, serialize_leb128_signed},
    };

    #[test]
    fn serialize_leb_signed() {
        let i8min = vec![0x80, 0x7F];
        let i16min = vec![0x80, 0x80, 0x7E];
        let i32min = vec![0x80, 0x80, 0x80, 0x80, 0x78];
        let i64min = vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7F];

        let all_i = vec![i8min, i16min, i32min, i64min];
        let all_i_res: Vec<(i128, u8)> = vec![
            (-128, 2),
            (-32768, 3),
            (-2_147_483_648, 5),
            (-9_223_372_036_854_775_808, 10),
        ];
        for (i, int) in all_i.iter().enumerate() {
            let result = serialize_leb128_signed(all_i_res[i].0);
            assert_eq!(result, *int);
        }
    }

    #[test]
    fn deserialize_leb_signed() {
        let i8min = vec![0x80, 0x7F];
        let i16min = vec![0x80, 0x80, 0x7E];
        let i32min = vec![0x80, 0x80, 0x80, 0x80, 0x78];
        let i64min = vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7F];

        let all_i = vec![i8min, i16min, i32min, i64min];
        let all_i_res: Vec<(i128, u8)> = vec![
            (-128, 2),
            (-32768, 3),
            (-2_147_483_648, 5),
            (-9_223_372_036_854_775_808, 10),
        ];
        for (i, int) in all_i.iter().enumerate() {
            let (result, num_of_bytes) = deserialize_leb128_signed(int).unwrap();
            assert_eq!(result, all_i_res[i].0);
            assert_eq!(num_of_bytes, all_i_res[i].1);
        }
    }

    #[test]
    fn serde_i128() {
        let val: i128 = i128::MIN;
        let serialized = serialize_leb128_signed(val);
        let (deserialized, len) = deserialize_leb128_signed(&serialized).unwrap();
        assert_eq!(val, deserialized);
        assert_eq!(serialized.len(), len as usize);
        
        let val2: i128 = i128::MAX;
        let serialized2 = serialize_leb128_signed(val2);
        let (deserialized2, len2) = deserialize_leb128_signed(&serialized2).unwrap();
        assert_eq!(val2, deserialized2);
        assert_eq!(serialized2.len(), len2 as usize);
    }

    #[test]
    fn deser_err() {
        let data = vec![0b10000000];
        let err = deserialize_leb128_signed(&data);
        assert!(err.is_err());

        // 20 bytes of 0x80 with a trailing 0x00 should overflow i128
        let mut data2 = vec![0x80; 20];
        data2.push(0x00);
        let err = deserialize_leb128_signed(&data2);
        assert!(err.is_err());
    }

    #[test]
    fn serde_all_the_numbers_i16() {
        for i in i16::MIN..=i16::MAX {
            let serialized = serialize_leb128_signed(i as i128);
            let (result, len) = deserialize_leb128_signed(&serialized).unwrap();
            assert_eq!(result, i as i128);
            assert!(len <= 3);
        }
    }
}
