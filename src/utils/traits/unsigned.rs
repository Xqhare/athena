use std::fmt::Debug;

pub trait Unsigned: Copy + Clone + Debug + Eq + Ord + PartialEq + PartialOrd {
}

impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for usize {}
