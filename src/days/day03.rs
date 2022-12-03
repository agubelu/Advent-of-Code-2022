use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day03.txt").unwrap();

    let sol1: u32 = input.lines().map(|line| {
        let half = line.len() / 2;
        find_repeated([&line[..half], &line[half..]])
    }).sum();
    
    let sol2: u32 = input.lines().chunks(3).into_iter().map(find_repeated).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

fn find_repeated<'a, I>(str_iter: I) -> u32 
where I: IntoIterator<Item = &'a str> {
    // Turns each string into an u64 where a set bit indicates that it contains the item,
    // and then ANDs them together to find the only common item
    str_iter.into_iter()
        .map(|string| string.chars().map(|ch| 1 << char2prio(ch)).fold(0, |a, b| a | b))
        .fold(u64::MAX, |a, b| a & b)
        .trailing_zeros()
}

fn char2prio(ch: char) -> u32 {
    match ch {
        'a'..='z' => ch as u32 - 'a' as u32 + 1,
        'A'..='Z' => ch as u32 - 'A' as u32 + 27,
        _ => unreachable!()
    }
}