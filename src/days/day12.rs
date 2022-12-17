use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::{Solution, SolutionPair};
use crate::etc::vecmat::VecMat;
use crate::etc::utils::{UP, DOWN, LEFT, RIGHT};
use crate::etc::coords::Coords;

use std::collections::BinaryHeap;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

type Pos = Coords<i32>;

#[derive(Copy, Clone, Eq)]
struct SearchState {
    pub cost: u32,
    pub node: Pos,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day12.txt").unwrap();
    let (heights, start, end) = parse(&input);

    let sol1 = shortest_path(start, end, &heights);
    let sol2 = heights.indexed_iter()
                      .filter(|(_, val)| *val == 0)
                      .map(|(pos, _)| shortest_path(Pos::new(pos.0 as i32, pos.1 as i32), end, &heights))
                      .min().unwrap();

    (Solution::from(sol1), Solution::from(sol2))
}

fn shortest_path(source: Pos, target: Pos, heights: &VecMat<u8>) -> u32 {
    let mut to_visit = BinaryHeap::new();
    let mut visited_costs = FxHashMap::default();

    to_visit.push(SearchState{ node: source, cost: 0 });

    while let Some(SearchState { node, cost }) = to_visit.pop() {
        if node == target {
            return cost;
        }

        for next_node in ok_neighbors(node, heights) {
            let new_cost = cost + 1;
            let prev_cost = match visited_costs.get(&next_node) {
                Some(val) => *val,
                None => u32::MAX,
            };

            if new_cost < prev_cost {
                visited_costs.insert(next_node, new_cost);
                to_visit.push(SearchState { cost: new_cost, node: next_node });
            }

        }
    }

    u32::MAX
}

fn ok_neighbors(pos: Pos, mat: &VecMat<u8>) -> Vec<Pos> {
    let cur_height = mat[(pos.x, pos.y)];
    [UP, DOWN, LEFT, RIGHT].iter()
        .map(|delta| pos + delta)
        .filter(|new| new.x >= 0 && new.x < mat.width() as i32 && new.y >= 0 && new.y < mat.height() as i32
                && mat[(new.x, new.y)] <= cur_height + 1) 
        .collect()
}

fn parse(input: &str) -> (VecMat<u8>, Pos, Pos) {
    let lines = input.lines().collect_vec();
    let height = lines.len();
    let width = lines[0].len();

    let mut mat = VecMat::new(width, height, 0);
    let mut start = Pos::new(0, 0);
    let mut end = Pos::new(0, 0);

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let val = match ch {
                'a'..='z' => ch as u8 - b'a',
                'S' => {
                    start = Pos::new(x as i32, y as i32);
                    0
                },
                'E' => {
                    end = Pos::new(x as i32, y as i32);
                    b'z' - b'a'
                },
                _ => unreachable!()
            };

            mat[(x, y)] = val;
        }
    }

    (mat, start, end)
}

impl PartialEq for SearchState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}