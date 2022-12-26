use std::fs::read_to_string;
use std::cmp::{min, max};

use rustc_hash::FxHashSet;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Coords3D = [i32; 3];
type CoordsSet = FxHashSet<Coords3D>;
const DELTAS: [Coords3D; 6] = [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]];

pub fn solve() -> SolutionPair {
    let cubes: CoordsSet = read_to_string("input/day18.txt").unwrap()
        .lines()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap())
        .collect();

    let sol1: usize = cubes.iter().map(|cube| visible_sides(*cube, &cubes)).sum();
    let sol2 = expand_gas(&cubes);

    (Solution::from(sol1), Solution::from(sol2))
}

fn visible_sides([x, y, z]: Coords3D, others: &CoordsSet) -> usize {
    DELTAS.iter()
          .map(|[dx, dy, dz]| [x + dx, y + dy, z + dz])
          .filter(|pos| !others.contains(pos))
          .count()
}

fn expand_gas(cubes: &CoordsSet) -> usize {
    // First, determine the minimum and maximum coordinate for each axis
    let mut mins = [0, 0, 0];
    let mut maxs = [0, 0, 0];

    for i in 0..3 {
        let (min, max) = cubes.iter()
            .fold((i32::MAX, i32::MIN), |acc, cube| (min(acc.0, cube[i]), max(acc.1, cube[i])));

        mins[i] = min - 1;
        maxs[i] = max + 1;
    }

    // Next, do a breadth-first search to simulate the steam traversing the
    // 3D space and count how many cube edges we touch along the way
    let mut res = 0;
    let mut to_visit = FxHashSet::default();
    let mut visited = FxHashSet::default();

    to_visit.insert(mins);

    while !to_visit.is_empty() {
        let mut new_to_visit = FxHashSet::default();

        for pos in to_visit.drain() {
            visited.insert(pos);
            let (neighbors, touching) = get_neighbors(pos, cubes, &visited, maxs, mins);
            res += touching;
            new_to_visit.extend(neighbors);
        }

        to_visit = new_to_visit;
    }

    res
}

// Determines which neighbors of the current position are OK to visit,
// and how many cube sides it touches
fn get_neighbors(pos: Coords3D, cubes: &CoordsSet, visited: &CoordsSet, maxs: Coords3D, mins: Coords3D) -> (Vec<Coords3D>, usize) {
    let mut neighbors_ok = Vec::with_capacity(6);
    let mut touched = 0;

    'outer: for [dx, dy, dz] in DELTAS {
        let neighbor = [pos[0] + dx, pos[1] + dy, pos[2] + dz];

        // Check that the neighbor is within bounds
        for i in 0..3 {
            if neighbor[i] < mins[i] || neighbor[i] > maxs[i] {
                continue 'outer;
            }
        }

        // Is the neighbor an existing cube? If so, we're next to an exposed side,
        // add it to the touched count (note that, since every position is evaluated
        // exactly once, this side will not be counted again)
        if cubes.contains(&neighbor) {
            touched += 1;
        // Otherwise, we can visit this neighbor only if we haven't visited it before
        } else if !visited.contains(&neighbor) {
            neighbors_ok.push(neighbor);
        }
    }

    (neighbors_ok, touched)
}
