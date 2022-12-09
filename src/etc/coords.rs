use std::ops::{Add, Sub};
use num_traits::int::PrimInt;
use num_traits::sign::Signed;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coords<T: PrimInt + Signed> {
    pub x: T,
    pub y: T,
}

impl<T: PrimInt + Signed> Coords<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: PrimInt + Signed> From<(T, T)> for Coords<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl <T: PrimInt + Signed> Add<Coords<T>> for Coords<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl <T: PrimInt + Signed> Sub<Coords<T>> for Coords<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl <T: PrimInt + Signed> Add<&Coords<T>> for Coords<T> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl <T: PrimInt + Signed> Sub<&Coords<T>> for Coords<T> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}