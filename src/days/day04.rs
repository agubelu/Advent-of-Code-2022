use itertools::Itertools;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

struct RangePair {
    start_a: i32,
    end_a: i32,
    start_b: i32,
    end_b: i32,
}

pub fn solve() -> SolutionPair {
    let pairs = read_to_string("input/day04.txt").unwrap()
        .lines()
        .map(RangePair::from_line)
        .collect_vec();

    let sol1 = pairs.iter().filter(|p| p.has_full_overlap()).count();
    let sol2 = pairs.iter().filter(|p| p.has_partial_overlap()).count();

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

impl RangePair {
    pub fn from_line(line: &str) -> Self {
        let (left, right) = line.split(',').next_tuple().unwrap();
        let (start_a, end_a) = left.split('-').map(|x| x.parse().unwrap()).next_tuple().unwrap();
        let (start_b, end_b) = right.split('-').map(|x| x.parse().unwrap()).next_tuple().unwrap();
        Self { start_a, end_a, start_b, end_b }
    }

    pub fn has_full_overlap(&self) -> bool {
        (self.start_a <= self.start_b && self.end_a >= self.end_b) ||
        (self.start_b <= self.start_a && self.end_b >= self.end_a)
    }

    pub fn has_partial_overlap(&self) -> bool {
        self.start_a <= self.end_b && self.start_b <= self.end_a
    }
}
