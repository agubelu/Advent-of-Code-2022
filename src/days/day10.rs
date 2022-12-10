use crate::{Solution, SolutionPair};
use Instruction::*;
use itertools::Itertools;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

enum Instruction {
    Noop,
    Addx(i32)
}

pub fn solve() -> SolutionPair {
    let instrs = read_to_string("input/day10.txt").unwrap()
        .lines()
        .map(Instruction::from_line)
        .collect_vec();

    let mut reg = 1;
    let mut cycle = 1;
    let mut sol1 = 0;
    let mut screen = [[' '; 40]; 6];

    for instr in &instrs {
        for _ in 0..instr.cycles() {
            // Update signal strength for part 1
            if (cycle - 20) % 40 == 0 {
                sol1 += cycle * reg;
            }

            // Update screen for part 2
            let i = cycle - 1;
            let (row, col) = (i / 40, i % 40);

            if col >= reg - 1 && col <= reg + 1 {
                screen[row as usize][col as usize] = '█';
            }

            // End of cycle
            cycle += 1;
        }

        if let Addx(n) = instr {
            reg += n;
        }
    }

    let display = screen.iter().map(|row| row.iter().collect::<String>()).join("\n");
    let sol2 = format!("\n{}", display);

    (Solution::from(sol1), Solution::from(sol2))
}

impl Instruction {
    pub fn from_line(line: &str) -> Self {
        let spl = line.split(' ').collect_vec();
        match spl[0] {
            "noop" => Noop,
            "addx" => Addx(spl[1].parse().unwrap()),
            _ => unreachable!()
        }
    }

    pub fn cycles(&self) -> u32 {
        match self {
            Noop => 1,
            Addx(_) => 2,
        }
    }
}