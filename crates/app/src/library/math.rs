use std::ops::{Add, Mul};

pub fn add<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}

pub fn square<T>(x: T) -> T
where
    T: Mul<Output = T> + Copy,
{
    x * x
}
