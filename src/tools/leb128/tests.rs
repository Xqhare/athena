#![allow(unused_imports)]

mod unsigned {
    use crate::encoding_and_decoding::{deserialize_leb128_unsigned, serialize_leb128_unsigned};
    
    #[test]
    fn deser_err() {
        // continuation bit set in last byte
        let serialized = vec![0b10000000];
        let err = deserialize_leb128_unsigned(&serialized);
        assert!(err.is_err());
    }

    // Still takes 2.6 sec! -> optimized in refactor of api, now 1.3/1.4 sec
    // rughly 12mil numbers per sec
    #[test]
    fn serde_all_the_numbers_u24() {
        let wtf_compiler: u32 = 16_777_215; // u24::MAX
        for i in 0..=wtf_compiler {
            let serialized = serialize_leb128_unsigned(i.try_into().unwrap());
            let (result, len) = deserialize_leb128_unsigned(&serialized).unwrap();
            assert_eq!(result, i.try_into().unwrap());
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
            let serialized = serialize_leb128_unsigned(i.try_into().unwrap());
            let (result, _) = deserialize_leb128_unsigned(&serialized).unwrap();
            assert_eq!(result, i.try_into().unwrap());
            assert_ne!(serialized.len(), 0);
        }
    }

    #[ignore = "Takes a long time; 150s, < 30 in release"]
    #[test]
    fn serde_all_the_numbers_mid_u32() {
        let upper_bound: u64 = (u32::MAX as u64 * 2) / 3;
        for i in (u32::MAX / 3) as u64..=upper_bound {
            let serialized = serialize_leb128_unsigned(i.try_into().unwrap());
            let (result, _) = deserialize_leb128_unsigned(&serialized).unwrap();
            assert_eq!(result, i.try_into().unwrap());
            assert_ne!(serialized.len(), 0);
        }
    }

    #[ignore = "Takes a long time; 150s, < 30 in release"]
    #[test]
    fn serde_all_the_numbers_high_u32() {
        let upper_bound: u64 = (u32::MAX as u64 * 2) / 3;
        for i in upper_bound..=u32::MAX  as u64 {
            let serialized = serialize_leb128_unsigned(i.try_into().unwrap());
            let (result, _) = deserialize_leb128_unsigned(&serialized).unwrap();
            assert_eq!(result, i.try_into().unwrap());
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
            assert_eq!(*d, *&deserialized.0 as usize);
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
            assert_eq!(result, all_u_res[i].0 as usize);
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
            assert_eq!(result, new_u_res[i].0 as usize);
            assert_eq!(num_of_bytes, new_u_res[i].1);
            count += 1;
        }
        assert_eq!(count, 4);
    }
}

mod signed {
    use crate::{error::AthenaError, tools::leb128::{deserialize_leb128_signed, serialize_leb128_signed}};

    #[test]
    fn serialize_leb_signed() {
        let i8min = vec![0x80, 0x7F];
        let i16min = vec![0x80, 0x80, 0x7E];
        let i32min = vec![0x80, 0x80, 0x80, 0x80, 0x78];
        let i64min = vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7F];

        let all_i = vec![i8min, i16min, i32min, i64min];
        let all_i_res: Vec<(i64, u8)> = vec![(-128, 2), (-32768, 3), (-2_147_483_648, 5), (-9_223_372_036_854_775_808, 10)];
        let mut count = 0;
        for (i, int) in all_i.iter().enumerate() {
            let result = serialize_leb128_signed(all_i_res[i].0);
            assert_eq!(result, *int);
            count += 1;
        }
        assert_eq!(count, 4);
    }

    #[test]
    fn deserialize_leb_signed() {
        let i8min = vec![0x80, 0x7F];
        let i16min = vec![0x80, 0x80, 0x7E];
        let i32min = vec![0x80, 0x80, 0x80, 0x80, 0x78];
        let i64min = vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7F];

        let all_i = vec![i8min, i16min, i32min, i64min];
        let all_i_res: Vec<(i64, u8)> = vec![(-128, 2), (-32768, 3), (-2_147_483_648, 5), (-9_223_372_036_854_775_808, 10)];
        let mut count = 0;
        for (i, int) in all_i.iter().enumerate() {
            let (result, num_of_bytes) = deserialize_leb128_signed(int).unwrap();
            assert_eq!(result, all_i_res[i].0);
            assert_eq!(num_of_bytes, all_i_res[i].1);
            count += 1;
        }
        assert_eq!(count, 4);
    }

    #[test]
    fn deser_err() {
        let data = vec![0b10000000];
        let err = deserialize_leb128_signed(&data);
        assert!(err.is_err());

        let data2 = vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7F];
        let err = deserialize_leb128_signed(&data2);
        assert!(err.is_err());
    }

    #[test]
    fn serde_all_the_numbers_i16() {
        for i in i16::MIN..=i16::MAX {
            let serialized = serialize_leb128_signed(i as i64);
            let (result, len) = deserialize_leb128_signed(&serialized).unwrap();
            assert_eq!(result, i as i64);
            assert!(len <= 3);
        }
    }
}
