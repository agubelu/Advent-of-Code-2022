use itertools::Itertools;
use rustc_hash::FxHashSet;
use crate::etc::coords::Coords;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

type Pos = Coords<i32>;

pub const SPAWN: Pos = Pos::new(500, 0);
pub const DOWN: Pos = Pos::new(0, 1);
pub const DOWN_LEFT: Pos = Pos::new(-1, 1);
pub const DOWN_RIGHT: Pos = Pos::new(1, 1);

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day14.txt").unwrap();
    let mut map = FxHashSet::default(); 

    input.lines().for_each(|line| add_rocks(line, &mut map));
    
    let sol1 = simulate(&map, true);
    let sol2 = simulate(&map, false);

    (Solution::from(sol1), Solution::from(sol2))
}

fn simulate(map: &FxHashSet<Pos>, bottomless: bool) -> usize {
    let mut map = map.clone();
    let n_rocks = map.len();
    let max_y = map.iter().map(|pos| pos.y).max().unwrap();

    let mut overflowing = false;

    while !overflowing {
        let mut particle_pos = SPAWN;

        loop {
            // Determine if the particle can keep moving
            let next_position = [DOWN, DOWN_LEFT, DOWN_RIGHT].iter()
                .map(|diff| particle_pos + diff)
                .find(|pos| !map.contains(pos) && pos.y < max_y + 2);

            if let Some(pos) = next_position {
                // We can move, update the position and check if we are falling to the void
                particle_pos = pos;
                overflowing = bottomless && particle_pos.y > max_y;
                if overflowing {
                    break;
                }
            } else {
                // No possible movements, the particle comes to a stop where it is
                map.insert(particle_pos);
                break;
            }
        }

        // Stop if we have filled the whole thing
        if particle_pos == SPAWN {
            break;
        }
    }

    map.len() - n_rocks
}

fn add_rocks(line: &str, map: &mut FxHashSet<Pos>) {
    line.split(" -> ").map(|coords| {
        let mut spl = coords.split(',').map(|v| v.parse().unwrap());
        Pos::new(spl.next().unwrap(), spl.next().unwrap())
    })
    .tuple_windows()
    .for_each(|(start, end)| {
        start.iter_to(&end).for_each(|pos| {
            map.insert(pos);
        });
    });
}
