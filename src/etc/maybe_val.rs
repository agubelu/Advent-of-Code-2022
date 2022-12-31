use std::ops::{Add, Sub, Mul, Div};

// Represents a value that can be known or unknown and implements
// basic math operations on it
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaybeVal {
    Known(i64),
    Unknown
}

impl MaybeVal {
    pub fn unwrap(&self) -> i64 {
        match self {
            MaybeVal::Known(x) => *x,
            MaybeVal::Unknown => panic!("Tried to unwrap an unknown value"),
        }
    }

    pub fn as_option(&self) -> Option<i64> {
        match self {
            MaybeVal::Known(x) => Some(*x),
            MaybeVal::Unknown => None,
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl Add for MaybeVal {
    type Output = MaybeVal;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MaybeVal::Known(x), MaybeVal::Known(y)) => MaybeVal::Known(x + y),
            _ => MaybeVal::Unknown,
        }
    }
}

impl Sub for MaybeVal {
    type Output = MaybeVal;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MaybeVal::Known(x), MaybeVal::Known(y)) => MaybeVal::Known(x - y),
            _ => MaybeVal::Unknown,
        }
    }
}

impl Mul for MaybeVal {
    type Output = MaybeVal;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MaybeVal::Known(x), MaybeVal::Known(y)) => MaybeVal::Known(x * y),
            _ => MaybeVal::Unknown,
        }
    }
}

impl Div for MaybeVal {
    type Output = MaybeVal;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MaybeVal::Known(x), MaybeVal::Known(y)) => MaybeVal::Known(x / y),
            _ => MaybeVal::Unknown,
        }
    }
}
