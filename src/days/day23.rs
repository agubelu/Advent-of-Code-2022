use std::fs::read_to_string;
use itertools::Itertools;
use rustc_hash::{FxHashSet, FxHashMap};
use crate::{Solution, SolutionPair};
use crate::etc::Coords;

////////////////////////////////////////////////////////////////////////////////

type Pos = Coords<i32>;
type PositionSet = FxHashSet<Pos>;
type MoveSuggestions = FxHashMap<Pos, Option<Pos>>;
type MoveCounter = FxHashMap<Pos, u32>;

const NORTH: Pos = Pos::new(0, -1);
const SOUTH: Pos = Pos::new(0, 1);
const WEST: Pos = Pos::new(-1, 0);
const EAST: Pos = Pos::new(1, 0);

////////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("input/day23.txt").unwrap();
    let map = parse_elves_positions(&input);

    let sol1 = simulate_n_rounds(&map, 10);
    let sol2 = simulate_until_finish(&map);

    (Solution::from(sol1), Solution::from(sol2))
}

// Does N rounds of movement, taking care of updating the priorities of
// the movement directions after every one
fn simulate_n_rounds(map: &PositionSet, rounds: usize) -> i32 {
    let mut main_directions = [NORTH, SOUTH, WEST, EAST];
    let mut map = map.clone();
    
    for _ in 0..rounds {
        do_one_round(&mut map, &main_directions);
        main_directions.rotate_left(1);
    }

    // Find out the area of the minimum rectangle and substract the number of 
    // elves from it to get the number of empty spaces
    let min_x = map.iter().map(|p| p.x).min().unwrap();
    let max_x = map.iter().map(|p| p.x).max().unwrap();
    let min_y = map.iter().map(|p| p.y).min().unwrap();
    let max_y = map.iter().map(|p| p.y).max().unwrap();

    (max_x - min_x + 1) * (max_y - min_y + 1) - map.len() as i32
}

// Simulate the elves' movement until they all stop, returns the number of rounds
fn simulate_until_finish(map: &PositionSet) -> i32 {
    let mut main_directions = [NORTH, SOUTH, WEST, EAST];
    let mut map = map.clone();
    let mut rounds = 1;
    
    while !do_one_round(&mut map, &main_directions) {
        rounds += 1;
        main_directions.rotate_left(1);
    }

    rounds
}

// Performs one round of movement, updating the positions set in place
// Returns a boolean that is true only when no elf moved in this round
fn do_one_round(map: &mut PositionSet, main_directions: &[Pos]) -> bool {
    // For every elf, check out where they want to move, keeping track of their original
    // position and the amount of other elves that want to move to the same spot
    let mut movements = MoveSuggestions::default();
    let mut counters = MoveCounter::default();

    for pos in map.iter().copied() {
        let suggestion = get_movement_proposal(pos, map, main_directions);
        movements.insert(pos, suggestion);

        if let Some(new_pos) = suggestion {
            *counters.entry(new_pos).or_insert(0) += 1;
        }
    }

    // Move only those elves...
    let ok_moves = movements.iter()
        .filter_map(|(old, new_opt)| new_opt.map(|new| (old, new))) // ...that have a suggestion...
        .filter(|(_, new)| counters[new] <= 1) // ...and don't have a conflict with any other elves
        .collect_vec();

    for (old_pos, new_pos) in &ok_moves {
        map.remove(old_pos);
        map.insert(*new_pos);
    }

    return ok_moves.is_empty();
}

// Determines the position to which a given elf will propose to move, if any
fn get_movement_proposal(pos: Pos, map: &PositionSet, main_directions: &[Pos]) -> Option<Pos> {
    // If there are no other elves around this one, do not move
    if (-1..=1).cartesian_product(-1..=1).all(|diff| diff == (0, 0) || !map.contains(&(pos + diff))) {
        return None;
    }

    // Otherwise, try to move in the following directions, in order
    for main_dir in main_directions {
        // The auxiliary direction is orthogonal to the main direction of
        // movement, and we use it to check that the diagonals are also free
        let aux_dirs = match main_dir {
            Pos { x: 0, y: _ } => [(-1, 0), (0, 0), (1, 0)],
            Pos { x: _, y: 0 } => [(0, -1), (0, 0), (0, 1)],
            _ => unreachable!(),
        };

        // The way to move in this direction is free, we can stop looking
        if aux_dirs.iter().all(|aux| !map.contains(&(pos + main_dir + aux))) {
            return Some(pos + main_dir);
        }
    }

    None
}

fn parse_elves_positions(input: &str) -> PositionSet {
    let mut positions = PositionSet::default();

    for (y, line) in input.lines().enumerate() {
        for x in line.chars().enumerate().positions(|(_, ch)| ch == '#') {
            positions.insert(Pos::new(x as i32, y as i32));
        }
    }

    positions
}
