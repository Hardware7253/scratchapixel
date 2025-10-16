use std::ops::{Add, Sub, Mul, Neg};

pub trait Num: Copy + Mul<Output = Self> + Neg<Output = Self> + Add<Output = Self> + Sub<Output = Self> + PartialEq + PartialOrd {}

impl Num for f64{}
impl Num for f32 {}

// impl Num for u128 {}
// impl Num for u64 {}
// impl Num for u32 {}
// impl Num for u16 {}
// impl Num for u8 {}

impl Num for i128{}
impl Num for i64 {}
impl Num for i32 {}
impl Num for i16 {}
impl Num for i8 {}