use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let (sol1, sol2) = read_to_string("input/day02.txt").unwrap()
        .lines()
        .map(solve_line)
        .fold((0, 0), |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2));

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_line(line: &str) -> (usize, usize) {
    let mut chars = line.chars();
    let opp = chars.next().unwrap() as usize - 'A' as usize;
    let us = chars.nth(1).unwrap() as usize - 'X' as usize;
    
    let sol1 = (us + 1) + SCORES_TABLE[opp][us];
    let sol2 = VALUES_TABLE[opp][us] + (us * 3);
    (sol1, sol2)
}

// Scores for a given combination of Rock/Paper/Scissors
static SCORES_TABLE: [[usize; 3]; 3] = [ 
//     Us
//   R  P  S
    [3, 6, 0], // R
    [0, 3, 6], // P  Opponent
    [6, 0 ,3], // S
];

// Value of the shape that achieves a given result against the opponent's shape
static VALUES_TABLE: [[usize; 3]; 3] = [ 
//   Result
//   L  D  W
    [3, 1, 2], // R
    [1, 2, 3], // P  Opponent
    [2, 3 ,1], // S
];