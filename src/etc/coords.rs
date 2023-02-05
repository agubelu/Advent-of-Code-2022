use std::ops::{Add, Sub, AddAssign};
use num_traits::int::PrimInt;
use num_traits::sign::Signed;

/** A pair of signed integers representing 2D coordinates */
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coords<T: PrimInt + Signed> {
    pub x: T,
    pub y: T,
}

pub struct SegmentIter<T: PrimInt + Signed> {
    delta: Coords<T>,
    current: Coords<T>,
    end: Coords<T>,
}

impl<T: PrimInt + Signed> Coords<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    // Iterates from this coordinate to the target coordinate, both inclusive
    // Only works for coordinates that share at least one coordinate
    pub fn iter_to(&self, other: &Self) -> SegmentIter<T> {
        assert!(self.x == other.x || self.y == other.y, "At least one coordinate must be shared");
        let dx = (other.x - self.x).signum();
        let dy = (other.y - self.y).signum();
        let delta = Self { x: dx, y: dy };
        SegmentIter { delta: Self { x: dx, y: dy }, current: *self - delta, end: *other }
    }

    pub fn manhattan_dist(&self, other: &Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl<T: PrimInt + Signed> Iterator for SegmentIter<T> {
    type Item = Coords<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            x if x == self.end => None,
            _ => {
                self.current += self.delta;
                Some(self.current)
            },
        }
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

impl <T: PrimInt + Signed> AddAssign<Coords<T>> for Coords<T> {
    fn add_assign(&mut self, rhs: Coords<T>) {
        *self = *self + rhs;
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

impl <T: PrimInt + Signed> Add<(T, T)> for Coords<T> {
    type Output = Self;

    fn add(self, rhs: (T, T)) -> Self::Output {
        Self::new(self.x + rhs.0, self.y + rhs.1)
    }
}


impl <T: PrimInt + Signed> Add<&(T, T)> for Coords<T> {
    type Output = Self;

    fn add(self, rhs: &(T, T)) -> Self::Output {
        Self::new(self.x + rhs.0, self.y + rhs.1)
    }
}
