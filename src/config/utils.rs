use std::ops::*;

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: PartialEq<usize> + RemAssign<T> + Copy,
{
    loop {
        if a == 0 {
            return b;
        }
        b %= a;
        if b == 0 {
            return a;
        }
        a %= b;
    }
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Mul<T, Output = T> + Div<T, Output = T> + PartialEq<usize> + RemAssign<T> + Copy,
{
    (a * b) / gcd(a, b)
}

pub fn max<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}

pub struct Vertical<T>(pub T);
pub struct Horizontal<T>(pub T);
