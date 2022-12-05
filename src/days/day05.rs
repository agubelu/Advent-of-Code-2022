use itertools::Itertools;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

type Instruction = (usize, usize, usize);

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day05.txt").unwrap();
    let (n_stacks, height) = get_stacks_info(&input);
    let header: Vec<&str> = input.lines().take(height).collect();

    let stacks = (0..n_stacks).map(|i| read_stack(i, &header)).collect_vec();
    let instrs = input.lines().skip(height + 2).map(line_to_instr).collect_vec();

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

// Finds the number of stacks and the maximum initial height of the stacks
// by finding the line that starts with " 1  2 ..." and processing it
fn get_stacks_info(input: &str) -> (usize, usize) {
    let (height, line) = input.lines().enumerate()
        .find(|(_, line)| line.starts_with(" 1"))
        .unwrap();

    let n_stacks = line.trim().split(' ').last().unwrap().parse().unwrap();
    (n_stacks, height)
}

// Creates the nth stack by reading its contents from the header in the input
fn read_stack(n: usize, header: &[&str]) -> Vec<char> {
    header.iter()
          .map(|line| line.chars().nth(4*n + 1).unwrap())
          .filter(|ch| *ch != ' ')
          .rev()
          .collect()
}

// Reads an instruction from a line by splitting by spaces and parsing every other element
pub fn line_to_instr(line: &str) -> Instruction {
    let (amount, from, to) = line.split(' ').skip(1).step_by(2).map(|x| x.parse().unwrap()).next_tuple().unwrap();
    (amount, from - 1, to - 1)
}
