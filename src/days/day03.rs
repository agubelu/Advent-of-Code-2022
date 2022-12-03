use crate::{Solution, SolutionPair};
use itertools::Itertools;
use rustc_hash::FxHashSet; // slightly faster than std::collections::HashSet
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
    // Convert the string iterator to a collection of character hashsets
    let mut sets: Vec<FxHashSet<char>> = str_iter.into_iter().map(|line| line.chars().collect()).collect();

    // Find the character that is shared among all sets
    let (intersect, others) = sets.split_at_mut(1);
    let intersect = &mut intersect[0];
    for other in others {
        intersect.retain(|x| other.contains(x));
    }

    // It is guaranteed to contain exactly one character, convert it into its priority value
    char2prio(*intersect.iter().next().unwrap())
}

fn char2prio(ch: char) -> u32 {
    match ch {
        'a'..='z' => ch as u32 - 'a' as u32 + 1,
        'A'..='Z' => ch as u32 - 'A' as u32 + 27,
        _ => unreachable!()
    }
}