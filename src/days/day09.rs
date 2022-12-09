use crate::{Solution, SolutionPair};
use crate::etc::utils::{UP, DOWN, LEFT, RIGHT};
use crate::etc::coords::Coords;
use rustc_hash::FxHashSet;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

type Instr = (Coords<i32>, u32);

pub fn solve() -> SolutionPair {
    let instructions: Vec<Instr> = read_to_string("input/day09.txt").unwrap()
        .lines()
        .map(line_to_instr)
        .collect();

    let sol1 = solve_for_length::<2>(&instructions);
    let sol2 = solve_for_length::<10>(&instructions);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_for_length<const N: usize>(instrs: &[Instr]) -> usize {
    let mut visited = FxHashSet::default();
    let mut rope = [Coords::new(0, 0); N];

    for (direction, times) in instrs {
        for _ in 0..*times {
            rope[0] = rope[0] + direction;

            for i in 1..N {
                rope[i] = new_tail_pos(rope[i-1], rope[i]);
            }

            visited.insert(rope[N-1]);
        }
    }

    visited.len()
}

fn new_tail_pos(cur_head: Coords<i32>, prev_tail: Coords<i32>) -> Coords<i32> {
    match cur_head - prev_tail {
        Coords{x, y} if x.abs() <= 1 && y.abs() <= 1 => prev_tail, // The tail is still touching
        Coords{x, y} => Coords::new(prev_tail.x + x.signum(), prev_tail.y + y.signum()) // Move 1 in every direction in which the head moves
    }
}

pub fn line_to_instr(line: &str) -> Instr {
    let (d, n) = line.split_once(' ').unwrap();
    let dir = match d {
        "U" => UP,
        "D" => DOWN,
        "L" => LEFT,
        "R" => RIGHT,
        _ => unreachable!(),
    };

    (Coords::from(dir), n.parse().unwrap())
}
