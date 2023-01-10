// This is the first time in my 4 years of doing AoC that I hardcode something
// according to my specific input, but all the generic cube folding algorithms
// that I found felt more annoying than fun to code. So, this solution assumes
// that the cube is folded in the specific way found in my input (which I think 
// is the same for everyone) but will not work for any other shape, such as the
// example. It doesn't however make any assumptions about the size of the cube,
// so in theory it should work for cubes of any size as long as they are folded
// in this particular way.

use std::fs::read_to_string;
use regex::Regex;

use crate::{Solution, SolutionPair};
use crate::etc::{Coords, VecMat, DOUBLE_NEWLINE};
use Action::*;

///////////////////////////// Typedefs and stuff ///////////////////////////////

type Pos = Coords<i32>;
type PosTransform = fn(Pos, i32) -> Pos;
type FaceIndex = usize;
type Direction = usize;

const RIGHT: Direction = 0;
const DOWN: Direction = 1;
const LEFT: Direction = 2;
const UP: Direction = 3;

// Directions of movement, in the same order as the previous consts (clockwise turns)
static DIRECTIONS: [Pos; 4] = [Pos::new(1, 0), Pos::new(0, 1), Pos::new(-1, 0), Pos::new(0, -1)];

enum Action {
    Advance(u32),
    Turn(i32),
}

struct Face {
    grid: VecMat<char>,
    max: i32, // == size - 1, comes in handy later
    transitions_flat: [(FaceIndex, Direction, PosTransform); 4],
    transitions_cube: [(FaceIndex, Direction, PosTransform); 4],
}

/////////////////////////////////// Main ///////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day22.txt").unwrap();
    let (cube_str, actions_str) = input.split_once(DOUBLE_NEWLINE).unwrap();

    let actions = parse_actions(actions_str);
    let faces = parse_faces(cube_str);

    let sol1 = run_simulation::<false>(&actions, &faces);
    let sol2 = run_simulation::<true>(&actions, &faces);

    (Solution::from(sol1), Solution::from(sol2))
}

/////////////////////////// Simulation functions ///////////////////////////////

// Main simulation runner for both parts
fn run_simulation<const IS_CUBE: bool>(actions: &[Action], faces: &[Face]) -> i32 {
    let mut cur_face = 0;
    let mut direction = RIGHT;
    let mut position = Pos::new(0, 0);

    for action in actions {
        match action {
            Advance(n) => {
                // Try to advance as many times as specified or until we hit a wall
                for _ in 0..*n {
                    if let Some((face, pos, dir)) = try_advance::<IS_CUBE>(faces, position, cur_face, direction) {
                        cur_face = face;
                        position = pos;
                        direction = dir;
                    } else {
                        break;
                    }
                }
            },
            // Update the current direction, wrapping the direction index to modulo 4
            Turn(rot) => direction = (direction as i32 + rot).rem_euclid(4) as Direction,
        }
    }

    // Compute the score from the final position
    let offset = get_face_position(cur_face);
    let size = faces[0].max + 1;

    let global_x = offset.x * size + position.x + 1;
    let global_y = offset.y * size + position.y + 1;
    1000 * global_y + 4 * global_x + direction as i32
}

// Tries to advance one step in the current position and face in a given direction.
// If the step is successful, it returns the new position, face and direction after moving.
// If it runs into a wall, it returns None
fn try_advance<const IS_CUBE: bool>(faces: &[Face], pos: Pos, mut cur_face: FaceIndex, mut direction: Direction) 
    -> Option<(FaceIndex, Pos, Direction)> {
    // Try to advance one in the specified direction
    let face = &faces[cur_face];
    let mut new_pos = pos + DIRECTIONS[direction];

    // Is the new position outside this face's bounds?
    if new_pos.x < 0 || new_pos.y < 0 || new_pos.x > face.max || new_pos.y > face.max {
        // Wrap around to the new face and update the positional information
        let (new_face, new_dir, func) = if IS_CUBE {
            face.transitions_cube[direction]
        } else {
            face.transitions_flat[direction]
        };

        cur_face = new_face;
        direction = new_dir;
        new_pos = func(pos, face.max);
    }

    // Return the updated position if it's not a wall
    if faces[cur_face].grid[new_pos] == '.' {
        Some((cur_face, new_pos, direction))
    } else {
        None
    }
}

//////////////////////////////// Parsers ///////////////////////////////////////

// Parses the faces on the cube, using the hardcoded info about
// their relative position, connections and transitions
fn parse_faces(faces_str: &str) -> Vec<Face> {
    // How big is the side of a face? Calculate it by obtaining the total
    // area of the cube, dividing it by 6 to get the area of one face,
    // and computing its square root
    let area = faces_str.chars().filter(|&ch| ch == '.' || ch == '#').count() as i32;
    let face_size = ((area / 6) as f32).sqrt() as i32;
    
    // Build all six faces of the cube
    (0..6).map(|index| {
        let grid = read_face_grid(faces_str, index, face_size);

        // Adjacency data for part 1. Only the neighbors of each
        // face in the cardinal directions are relevant, since the direction
        // doesn't change when traversing between faces and the transformation
        // function is always the same for each direction.
        let neighbors_p1 = match index {
            0 => [1, 2, 1, 4],
            1 => [0, 1, 0, 1],
            2 => [2, 4, 2, 0],
            3 => [4, 5, 4, 5],
            4 => [3, 0, 3, 2],
            5 => [5, 3, 5, 3],
            _ => unreachable!(),
        };
        
        let transitions_flat: [(_, _, PosTransform); 4] = [
            (neighbors_p1[0], RIGHT, |pos, ___| (0, pos.y).into()),
            (neighbors_p1[1], DOWN,  |pos, ___| (pos.x, 0).into()),
            (neighbors_p1[2], LEFT,  |pos, max| (max, pos.y).into()),
            (neighbors_p1[3], UP,    |pos, max| (pos.x, max).into()),
        ];

        // Adjacency data for part 2. For each face, the neighboring face,
        // direction change and transformation function for every direction.
        // Lovingly hand-crafted thanks to my IRL cube, which was actually
        // my Rubik's cube with some pieces of paper blutack'd to the faces :^)
        let transitions_cube: [(_, _, PosTransform); 4] = match index {
            0 => [
                (1, RIGHT,  |pos, ___| (0, pos.y).into()),
                (2, DOWN,   |pos, ___| (pos.x, 0).into()),
                (3, RIGHT,  |pos, max| (0, max - pos.y).into()),
                (5, RIGHT,  |pos, ___| (0, pos.x).into()),
            ],
            1 => [
                (4, LEFT,  |pos, max| (max, max - pos.y).into()),
                (2, LEFT,  |pos, max| (max, pos.x).into()),
                (0, LEFT,  |pos, max| (max, pos.y).into()),
                (5, UP,    |pos, max| (pos.x, max).into()),
            ],
            2 => [
                (1, UP,    |pos, max| (pos.y, max).into()),
                (4, DOWN,  |pos, ___| (pos.x, 0).into()),
                (3, DOWN,  |pos, ___| (pos.y, 0).into()),
                (0, UP,    |pos, max| (pos.x, max).into()),
            ],
            3 => [
                (4, RIGHT, |pos, ___| (0, pos.y).into()),
                (5, DOWN,  |pos, ___| (pos.x, 0).into()),
                (0, RIGHT, |pos, max| (0, max - pos.y).into()),
                (2, RIGHT, |pos, ___| (0, pos.x).into()),
            ], 
            4 => [
                (1, LEFT,  |pos, max| (max, max - pos.y).into()),
                (5, LEFT,  |pos, max| (max, pos.x).into()),
                (3, LEFT,  |pos, max| (max, pos.y).into()),
                (2, UP,    |pos, max| (pos.x, max).into()),
            ],
            5 => [
                (4, UP,    |pos, max| (pos.y, max).into()),
                (1, DOWN,  |pos, ___| (pos.x, 0).into()),
                (0, DOWN,  |pos, ___| (pos.y, 0).into()),
                (3, UP,    |pos, max| (pos.x, max).into()),
            ],
            _ => unreachable!(),
        };

        Face { grid, transitions_flat, transitions_cube, max: face_size - 1 }
    }).collect()
}

// Parses the content of the face with a given index
fn read_face_grid(faces_str: &str, index: FaceIndex, size: i32) -> VecMat<char> {
    let coords = get_face_position(index);
    let size = size as usize;

    let data = faces_str.lines().skip(size * coords.y as usize).take(size).flat_map(|line| {
        line.chars().skip(size * coords.x as usize).take(size)
    }).collect();

    VecMat::from_data(size, size, data)
}

// Parses the list of actions from the input string
fn parse_actions(actions_str: &str) -> Vec<Action> {
    let re = Regex::new(r"L|R|\d+").unwrap();

    re.find_iter(actions_str).map(|m| {
        match m.as_str() {
            "L" => Turn(-1),
            "R" => Turn(1),
             x  => Advance(x.parse().unwrap())
        }
    }).collect()
}

/////////////////////////////// Aux stuff //////////////////////////////////////

// Returns the 2D position of a given face in the higher-order grid of faces
fn get_face_position(face: FaceIndex) -> Pos {
    match face {
        0 => (1, 0).into(),
        1 => (2, 0).into(),
        2 => (1, 1).into(),
        3 => (0, 2).into(),
        4 => (1, 2).into(),
        5 => (0, 3).into(),
        _ => unreachable!()
    }
}
