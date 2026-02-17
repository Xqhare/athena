use std::{fmt::Debug, ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Shl, Shr, Sub}};

pub trait Unsigned:
    Copy
    + Clone
    + Debug
    + Default
    + Eq
    + Ord
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}

impl Unsigned for u8 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Unsigned for u16 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Unsigned for u32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Unsigned for u64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Unsigned for usize {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
