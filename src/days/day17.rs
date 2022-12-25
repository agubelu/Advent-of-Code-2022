use std::cmp::max;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::{Solution, SolutionPair};
use crate::etc::coords::Coords;
use Direction::*;

///////////////////////////////////////////////////////////////////////////////

type Pos = Coords<i64>;
type CaveMap = FxHashSet<Pos>;

const ROCK_TYPES: [char; 5] = ['-', '+', 'L', 'I', 'O'];
const ROCKS_P1: i64 = 2022;
const ROCKS_P2: i64 = 1000000000000;

struct Rock {
    coords: Vec<Pos>,
    left_hitbox: Vec<usize>,
    right_hitbox: Vec<usize>,
    bottom_hitbox: Vec<usize>,
}

// Struct to store some info about the moment when a rock lands, to help with
// detecting a repetition pattern. This struct also stores the rock index but
// it's not hashed or compared for equality by it.
#[derive(Copy, Clone)]
struct RockLanding {
    i: i64,
    rock_type: char,
    x_pos: i64,
    wind_moment: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Left, Right
}

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let jet_patterns = read_to_string("input/day17.txt").unwrap()
        .chars()
        .map(|c| match c {
            '>' => Right,
            '<' => Left,
             _  => unreachable!()
        }).collect_vec();

    // Part 1: Just run the simulation normally for the first 2022 rocks
    let sol1 = run_simulation(&jet_patterns, 0, ROCKS_P1, &mut None);

    // Part 2: First, determine the moment when the repetition starts happening
    let mut rep_i = [0; 2];
    run_simulation(&jet_patterns, 0, i64::MAX, &mut Some(&mut rep_i));
    let [cycle_start, cycle_end] = rep_i;

    // Determine how many times the cycle repeat and what's the leftover
    let in_cycle = ROCKS_P2 - cycle_start;
    let cycle_len = cycle_end - cycle_start;
    let (reps, remainder) = (in_cycle / cycle_len, in_cycle % cycle_len);

    // Calculate the total height: up until the first end of the cycle, the
    // remaining iterations, and the final part
    let h1 = run_simulation(&jet_patterns, 0, cycle_end, &mut None);
    let h2 = run_simulation(&jet_patterns, cycle_end, cycle_end + cycle_len, &mut None) * (reps-1);
    let h3 = run_simulation(&jet_patterns, cycle_end, cycle_end + remainder, &mut None);

    let sol2 = h1 + h2 + h3;

    (Solution::from(sol1), Solution::from(sol2))
}

fn run_simulation(jetstreams: &[Direction], start: i64, end: i64, cycle_indices: &mut Option<&mut [i64]>) -> i64 {
    if start >= end {
        return 0;
    }

    let mut cave = FxHashSet::default();
    let mut states: FxHashSet<RockLanding> = FxHashSet::default();

    // Populate the cave's floor
    for x in 0..7 {
        cave.insert(Pos::new(x, -1));
    }

    let mut rock_types = ROCK_TYPES.iter().cycle();
    let mut jet_directions = jetstreams.iter().enumerate().cycle().peekable();
    let mut repetitions = 0;
    let mut start_y = 0;
    let mut max_y = -1;

    for i in 0..end {
        if i == start {
            start_y = max_y;
        }

        // Spawn a new rock, determining its initial position
        let mut still_moving = true;
        let rock_type = *rock_types.next().unwrap();
        let y_spawn = max_y + 4;
        let mut rock = Rock::spawn_new(Pos::new(2, y_spawn), rock_type);

        // Simulate the rock's movement through the cave
        while still_moving {
            let jetstream = jet_directions.next().unwrap().1;
            rock.move_horizontally(*jetstream, &cave);
            still_moving = rock.move_down(&cave);
        }

        // If we are looking for the moment when the cycle starts, update the set
        // with the info on this rock's landing instant
        if let Some(ptr) = cycle_indices {
            let x_pos = rock.coords[0].x;
            let wind_moment = jet_directions.peek().unwrap().0;
            let state = RockLanding { i, x_pos, wind_moment, rock_type };

            // Has the state occured before?
            if let Some(prev_state) = states.get(&state) {
                repetitions += 1;
                // Wait until we have 5 repetitions in a row to consider the cycle started
                if repetitions == 5 {
                    ptr[0] = prev_state.i - 4;
                    ptr[1] = i - 4;
                    break;
                }
            } else {
                // Otherwise, just store this state, reset the repetition counter and keep going
                repetitions = 0;
                states.insert(state);
            }
        }

        // The rock stopped moving, add its positions to the cave map
        max_y = max(max_y, rock.coords.last().unwrap().y);

        for pos in rock.coords.into_iter() {
            cave.insert(pos);
        }
    }

    max_y - start_y
}

///////////////////////////////////////////////////////////////////////////////

impl Rock {
    // Creates a new rock of a certain type spawning in the given position
    pub fn spawn_new(bottom_left: Pos, rock_type: char) -> Self {
        let (x, y) = (bottom_left.x, bottom_left.y);
        let (positions, left_hitbox, right_hitbox, bottom_hitbox) = match rock_type {
            '+' => (vec![(x+1, y), (x, y+1), (x+1, y+1), (x+2, y+1), (x+1, y+2)],
                    vec![0, 1, 4],
                    vec![0, 3, 4],
                    vec![0, 1, 3]),
            '-' => (vec![(x, y), (x+1, y), (x+2, y), (x+3, y)],
                    vec![0],
                    vec![3],
                    vec![0, 1, 2, 3]),
            'I' => (vec![(x, y), (x, y+1), (x, y+2), (x, y+3)],
                    vec![0, 1, 2, 3],
                    vec![0, 1, 2, 3],
                    vec![0]),
            'O' => (vec![(x, y), (x+1, y), (x, y+1), (x+1, y+1)],
                    vec![0, 2],
                    vec![1, 3],
                    vec![0, 1]),
            'L' => (vec![(x, y), (x+1, y), (x+2, y), (x+2, y+1), (x+2, y+2)],
                    vec![0],
                    vec![2, 3, 4],
                    vec![0, 1, 2]),
            _ => unreachable!(),
        };

        let coords = positions.into_iter().map(Pos::from).collect_vec();
        Self { coords, left_hitbox, right_hitbox, bottom_hitbox }
    }

    pub fn move_horizontally(&mut self, dir: Direction, map: &CaveMap) {
        // Check that the way in which we're moving is clear
        let (delta, hitbox) = match dir {
            Left => (Pos::new(-1, 0), &self.left_hitbox),
            Right => (Pos::new(1, 0), &self.right_hitbox),
        };

        let can_move = hitbox.iter().map(|i| self.coords[*i]).all(|p| {
            // 1) Don't move past a wall
            let clear_walls = dir == Left && p.x > 0 || dir == Right && p.x < 6;
            // 2) Don't move past an existing rock
            let clear_rocks = !map.contains(&(p + delta));
            clear_walls && clear_rocks
        });

        if can_move {
            self.update_coords(delta);
        }
    }

    pub fn move_down(&mut self, map: &CaveMap) -> bool {
        // Check if there is a rock under any of the bottom hitboxes of this rock
        let delta = Pos::new(0, -1);
        let can_move = self.bottom_hitbox.iter().map(|i| self.coords[*i]).all(|p| !map.contains(&(p + delta)));

        if can_move {
            self.update_coords(delta);
        }

        can_move
    }

    fn update_coords(&mut self, delta: Pos) {
        self.coords.iter_mut().for_each(|pos| *pos += delta);
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Eq for RockLanding { }
impl PartialEq for RockLanding {
    fn eq(&self, other: &Self) -> bool {
        self.rock_type == other.rock_type && self.x_pos == other.x_pos && self.wind_moment == other.wind_moment
    }
}

impl Hash for RockLanding {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rock_type.hash(state);
        self.x_pos.hash(state);
        self.wind_moment.hash(state);
    }
}
