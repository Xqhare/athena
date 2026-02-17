use std::{fmt::Debug, ops::{Add, Div, Mul, Sub}};

pub trait Unsigned: Copy + Clone + Debug + Eq + Ord + PartialEq + PartialOrd + Add + Sub + Mul + Div {
}

impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for usize {}
