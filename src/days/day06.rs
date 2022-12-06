use itertools::Itertools;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day06.txt").unwrap().chars().collect_vec();

    let sol1 = solve_for_length(&input, 4);
    let sol2 = solve_for_length(&input, 14);
    
    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_for_length(chars: &[char], len: usize) -> usize {
    chars.windows(len)
         .position(|window| window.iter().unique().count() == len)
         .unwrap() + len
}
