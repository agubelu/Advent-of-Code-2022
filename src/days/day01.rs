use crate::{Solution, SolutionPair};
use crate::etc::utils::DOUBLE_NEWLINE;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let mut calories: Vec<u32> = read_to_string("input/day01.txt").unwrap()
        .split(DOUBLE_NEWLINE)
        .map(|elf| elf.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect();

    calories.sort_by(|a, b| b.cmp(a));
    let sol1 = calories[0];
    let sol2: u32 = calories[0..3].iter().sum();

    (Solution::from(sol1), Solution::from(sol2))
}