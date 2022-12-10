use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day10.txt").unwrap();

    let mut reg = 1;
    let mut cycle = 1;
    let mut sol1 = 0;
    let mut screen = [[' '; 40]; 6];

    for line in input.lines() {
        let spl: Vec<&str> = line.split(' ').collect();
        for _ in 0..spl.len() { // conveniently, the number of cycles is the same as the instruction size
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

        if spl.len() == 2 {
            reg += spl[1].parse::<i32>().unwrap();
        }
    }

    let display = screen.iter().map(|row| row.iter().collect::<String>()).join("\n");
    let sol2 = format!("\n{}", display);

    (Solution::from(sol1), Solution::from(sol2))
}
