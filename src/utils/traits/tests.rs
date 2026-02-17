#[cfg(test)]
mod tests {
    use crate::utils::traits::signed::Signed;
    use crate::utils::traits::unsigned::Unsigned;

    fn add_unsigned<T: Unsigned>(a: T, b: T) -> T {
        a + b
    }

    fn sub_signed<T: Signed>(a: T, b: T) -> T {
        a - b
    }

    fn bitwise_unsigned<T: Unsigned>(a: T, b: T) -> T {
        (a & b) | (a ^ b)
    }

    #[test]
    fn test_unsigned_impls() {
        assert_eq!(add_unsigned(1u8, 2u8), 3u8);
        assert_eq!(add_unsigned(1u16, 2u16), 3u16);
        assert_eq!(add_unsigned(1u32, 2u32), 3u32);
        assert_eq!(add_unsigned(1u64, 2u64), 3u64);
        assert_eq!(add_unsigned(1usize, 2usize), 3usize);
    }

    #[test]
    fn test_unsigned_bitwise() {
        assert_eq!(bitwise_unsigned(0b1010u8, 0b1100u8), 0b1110u8);
    }

    #[test]
    fn test_signed_impls() {
        assert_eq!(sub_signed(1i8, 2i8), -1i8);
        assert_eq!(sub_signed(1i16, 2i16), -1i16);
        assert_eq!(sub_signed(1i32, 2i32), -1i32);
        assert_eq!(sub_signed(1i64, 2i64), -1i64);
        assert_eq!(sub_signed(1isize, 2isize), -1isize);
    }
}
