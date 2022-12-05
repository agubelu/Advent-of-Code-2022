use scanf::sscanf;
use itertools::Itertools;
use crate::{Solution, SolutionPair};
use crate::etc::utils::DOUBLE_NEWLINE;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

type Instruction = (usize, usize, usize);

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day05.txt").unwrap();
    let (header_text, body_text) = input.split_once(DOUBLE_NEWLINE).unwrap();
    let header = header_text.lines().collect_vec();

    let n_stacks = (header.last().unwrap().len() + 1) / 4;
    let stacks = (0..n_stacks).map(|i| read_stack(i, &header)).collect_vec();
    let instrs = body_text.lines().map(line_to_instr).collect_vec();

    let sol1 = process_stacks(&stacks, &instrs, false);
    let sol2 = process_stacks(&stacks, &instrs, true);
    
    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////
// Runs all the instructions on the given stacks, returning the final string
fn process_stacks(stacks: &[Vec<char>], instructions: &[Instruction], new_model: bool) -> String {
    let mut stacks = stacks.to_owned();

    for &(amount, from, to) in instructions {
        let i = stacks[from].len() - amount;
        let mut tmp = stacks[from].split_off(i);
        if !new_model { tmp.reverse(); }
        stacks[to].extend(tmp); 
    }

    stacks.iter().map(|st| st.last().unwrap()).collect()
}

// Creates the nth stack by reading its contents from the header in the input
fn read_stack(n: usize, header: &[&str]) -> Vec<char> {
    header.iter()
          .rev()
          .skip(1)  // Skip the stack numbers
          .map(|line| line.chars().nth(4*n + 1).unwrap())
          .take_while(|ch| *ch != ' ')
          .collect()
}

pub fn line_to_instr(line: &str) -> Instruction {
    let (mut amount, mut from, mut to) = (0, 0, 0);
    sscanf!(line, "move {} from {} to {}", amount, from, to).expect("Error reading instruction");
    (amount, from - 1, to - 1)
}
