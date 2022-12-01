use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let mut calories: Vec<u32> = read_to_string("input/day01.txt").unwrap()
        .split("\n\n")
        .map(|elf| elf.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect();

    calories.sort_unstable_by(|a, b| b.cmp(a));
    let sol1 = calories[0];
    let sol2 = calories[0..3].iter().sum();

    (Solution::U32(sol1), Solution::U32(sol2))
}
