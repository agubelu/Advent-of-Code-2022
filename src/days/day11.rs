use crate::etc::utils::DOUBLE_NEWLINE;
use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::collections::VecDeque;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct Monkey {
    items: VecDeque<i64>,
    update_op: Operation,
    divide_by: i64,
    targets: [usize; 2],
    inspected: u64,
}

#[derive(Clone)]
enum Operation {
    Add(i64),
    Mul(i64),
    Pow
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day11.txt").unwrap();

    let mut monkeys_p1 = input.split(DOUBLE_NEWLINE).map(Monkey::from_str).collect_vec();
    let mut monkeys_p2 = monkeys_p1.clone();

    let sol1 = solve_for_params(&mut monkeys_p1, 20, 3);
    let sol2 = solve_for_params(&mut monkeys_p2, 10_000, 1);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_for_params(monkeys: &mut [Monkey], iters: u32, worry_red: i64) -> u64 {
    let modulus: i64 = monkeys.iter().map(|x| x.divide_by).product();

    // Do some monkey business
    for _ in 0..iters {
        for i in 0..monkeys.len() {
            while let Some((mut item, target)) = monkeys[i].process_next(worry_red) {
                if worry_red == 1 { item %= modulus };
                monkeys[target].items.push_back(item);
            }
        }
    }

    // Look for the top 2 monkeys to get the solution
    let mut inspections = monkeys.iter().map(|x| x.inspected).collect_vec();
    inspections.sort_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

impl Monkey {
    // Parse monkey data from lines
    pub fn from_str(string: &str) -> Self {
        let lines = string.lines().collect_vec();

        let items = lines[1][18..].split(", ").map(|x| x.parse().unwrap()).collect();
        let update_op = Operation::from_str(&lines[2][19..]);
        let divide_by = lines[3][21..].parse().unwrap();
        let if_true = lines[4][29..].parse().unwrap();
        let if_false = lines[5][30..].parse().unwrap();

        Self { items, update_op, divide_by, targets: [if_false, if_true], inspected: 0 }
    }

    // Processes the first item in the list, and returns its new worry value
    // and the monkey to throw it to. Returns None if we have no items left.
    pub fn process_next(&mut self, relief_ratio: i64) -> Option<(i64, usize)> {
        self.items.pop_front().map(|item| {
            self.inspected += 1;
            let new_worry = self.update_op.apply(item) / relief_ratio;
            let test = new_worry % self.divide_by == 0;
            let target = self.targets[test as usize];
            (new_worry, target)
        })
    }
}

impl Operation {
    pub fn from_str(s: &str) -> Self {
        let spl = s.split(' ').collect_tuple().unwrap();
        match spl {
            ("old", "*", "old") => Self::Pow,
            ("old", "+", x) => Self::Add(x.parse().unwrap()),
            ("old", "*", x) => Self::Mul(x.parse().unwrap()),
            _ => unreachable!()
        }
    }

    pub fn apply(&self, item: i64) -> i64 {
        match self {
            Self::Pow => item * item,
            Self::Add(x) => item + x,
            Self::Mul(x) => item * x
        }
    }
}
