use std::{fmt::Debug, ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Shl, Shr, Sub}};

pub trait Signed:
    Copy
    + Clone
    + Debug
    + Default
    + Eq
    + Ord
    + PartialEq + PartialOrd
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

impl Signed for i8 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Signed for i16 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Signed for i32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Signed for i64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Signed for isize {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
