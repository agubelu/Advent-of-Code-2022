use crate::{Solution, SolutionPair};
use crate::etc::vecmat::VecMat;
use crate::etc::utils::{Pos2D, UP, DOWN, RIGHT, LEFT};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let data: Vec<Vec<i32>> = read_to_string("input/day08.txt").unwrap()
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as i32).collect())
        .collect();

    let height = data.len();
    let width = data[0].len();

    // Dump the data into our beautiful 2D matrix structure
    let matrix = VecMat::from_data(width, height, data.into_iter().flatten().collect());

    // Transform each tree into its scenic score and visibility
    let tree_data: Vec<(usize, bool)> = matrix.indexed_iter()
        .map(|(pos, elem)| score_and_visibility(pos, elem, &matrix))
        .collect();

    let sol1 = tree_data.iter().filter(|x| x.1).count();
    let sol2 = tree_data.iter().map(|x| x.0).max().unwrap();

    (Solution::from(sol1), Solution::from(sol2))
}

// Calculates the scenic score of a position and whether or not it's visible from outside
fn score_and_visibility(pos: Pos2D, max_height: i32, mat: &VecMat<i32>) -> (usize, bool) {
    let mut score = 1;
    let mut visible = false;

    for direction in [UP, DOWN, LEFT, RIGHT] {
        let trees = walk(pos, direction, mat, max_height);
        let n_trees = trees.len();
        // For part 1: this tree is visible from outside if all trees from
        // this point of view are strictly smaller (if n_trees == 0 then .all() is true)
        visible |= trees.iter().all(|&x| x < max_height);
        // For part 2: number of visible trees from here
        score *= n_trees;
    }

    (score, visible)
}

// Returns the vector of elements obtained from walking in a direction from a
// given position, until we find one equal or greater than the element in the origin
fn walk((x, y): Pos2D, (dx, dy): (i32, i32), mat: &VecMat<i32>, max_height: i32) -> Vec<i32> {
    let mut vec = vec![];

    let (mut xpos, mut ypos) = (x as i32 + dx, y as i32 + dy);
    let (max_x, max_y) = (mat.width() as i32, mat.height() as i32);

    while xpos >= 0 && xpos < max_x && ypos >= 0 && ypos < max_y {
        let elem = mat[(xpos, ypos)];
        vec.push(elem);

        if elem >= max_height {
            break;
        }

        xpos += dx;
        ypos += dy;
    }

    vec
}
