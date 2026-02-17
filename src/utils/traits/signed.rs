use std::{fmt::Debug, ops::{Add, Div, Mul, Sub}};

pub trait Signed: Copy + Clone + Debug + Eq + Ord + PartialEq + PartialOrd + Add + Sub + Mul + Div {
}

impl Signed for i8 {}
impl Signed for i16 {}
impl Signed for i32 {}
impl Signed for i64 {}
impl Signed for isize {}
