use std::fs::read_to_string;
use itertools::Itertools;
use pathfinding::prelude::astar;
use crate::{Solution, SolutionPair};
use crate::etc::Coords;

////////////////////////////////////////////////////////////////////////////////

type Pos = Coords<i32>;
type BlizzardIndex = Vec<Vec<Blizzard>>;

const UP: Pos = Pos::new(0, -1);
const DOWN: Pos = Pos::new(0, 1);
const LEFT: Pos = Pos::new(-1, 0);
const RIGHT: Pos = Pos::new(1, 0);
const NOOP: Pos = Pos::new(0, 0);

// A blizzard, represented by its initial position, direction and wrapping amount
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Blizzard {
    position: Pos,
    direction: Pos,
    max: i32
}

// Aux struct to hold information about the map
struct FieldInfo {
    width: i32,
    height: i32,
    start: Pos,
    goal: Pos,
    blz_rows: BlizzardIndex,
    blz_cols: BlizzardIndex
}

// Any given instant in the search through the blizzard
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct SearchState {
    position: Pos,
    minute: i32
}

////////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day24.txt").unwrap();
    let field_info = parse(&input);

    let start = field_info.start;
    let end = field_info.goal;

    let sol1 = search_best_path(&field_info, start, end, 0);
    let aux = sol1 + search_best_path(&field_info, end, start, sol1);
    let sol2 = aux + search_best_path(&field_info, start, end, aux);

    (Solution::from(sol1), Solution::from(sol2))
}

////////////////////////////////////////////////////////////////////////////////

// Finds the minimum amount of time required to go from a start point to
// a goal point in the field, starting at a given minute
fn search_best_path(field: &FieldInfo, start: Pos, goal: Pos, initial_min: i32) -> i32 {
    let initial = SearchState { position: start, minute: initial_min };
    let path = astar(&initial,
         |state| get_neighbors(state, field),  // Navigation function
         |state| state.position.manhattan_dist(&goal),  // Heuristic
         |state| state.position == goal  // Success condition
    );

    path.unwrap().1
}

// Calculates the possible actions for a search state
fn get_neighbors(state: &SearchState, field: &FieldInfo) -> impl IntoIterator<Item = (SearchState, i32)> {
    [RIGHT, DOWN, UP, LEFT, NOOP].into_iter()
        .map(|diff| state.position + diff)
        .filter(|new_pos| can_visit(*new_pos, state, field))
        .map(|new_pos| (SearchState { position: new_pos, minute: state.minute + 1 }, 1))
        .collect_vec()
}

// Determines if a given position can be visited for a specific state. The
// position must either be the goal position, or in bounds and free from blizzards
fn can_visit(position: Pos, state: &SearchState, field: &FieldInfo) -> bool {
    if position == field.goal || position == field.start {
        return true;
    }

    if position.x < 0 || position.x >= field.width || position.y < 0 || position.y >= field.height {
        return false;
    }

    // We are in bounds, we can only visit if the position will be blizzard-free the next minute
    // We consider only the blizzards that move along the position's row and column
    let blizzards_ver = &field.blz_cols[position.x as usize];
    let blizzards_hor = &field.blz_rows[position.y as usize];

    !any_blizzard_present(blizzards_ver, position, state.minute + 1) &&
    !any_blizzard_present(blizzards_hor, position, state.minute + 1)
}

// Determines if, in a given list of blizzards representing a row or a
// column, any of them will be present in a specified position and time
fn any_blizzard_present(ls: &[Blizzard], position: Pos, minute: i32) -> bool {
    ls.iter().any(|blz| blizzard_position_at(blz, minute) == position)
}

// Calculates where a given blizzard will be in a specific minute
fn blizzard_position_at(blz: &Blizzard, minute: i32) -> Pos {
    match blz.direction {
        Pos { x: 0, y } => Pos::new(blz.position.x, (blz.position.y + y * minute).rem_euclid(blz.max)),
        Pos { x, y: 0 } => Pos::new((blz.position.x + x * minute).rem_euclid(blz.max), blz.position.y),
        _ => unreachable!(),
    }
}

// Parses the input data. The grid starts at (-1, -1) because it makes a bunch of stuff easier.
fn parse(input: &str) -> FieldInfo {
    let lines = input.lines().collect_vec();

    let height = lines.len() as i32 - 2;
    let width = lines[0].len() as i32 - 2;

    // Find out the X position of the start and goal points, which are
    // the only dots in the top and bottom rows
    let start_x = lines[0].chars().position(|ch| ch == '.').unwrap() as i32 - 1;
    let start = Pos::new(start_x, -1);

    let end_x = lines.last().unwrap().chars().position(|ch| ch == '.').unwrap() as i32 - 1;
    let goal = Pos::new(end_x, height);

    // Parse the blizzards in every row and column
    let mut blz_rows = vec![vec![]; height as usize];
    let mut blz_cols = vec![vec![]; width as usize];

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let position = Pos::new(col as i32 - 1, row as i32 - 1);

            let (direction, max, collection, index) = match ch {
                '>' => (RIGHT, width, &mut blz_rows, position.y),
                '<' => (LEFT, width, &mut blz_rows, position.y),
                '^' => (UP, height, &mut blz_cols, position.x),
                'v' => (DOWN, height, &mut blz_cols, position.x),
                 _  => continue,
            };

            let blizzard = Blizzard { position, direction, max };
            collection[index as usize].push(blizzard);
        }
    }

    FieldInfo { width, height, start, goal, blz_cols, blz_rows }
}
