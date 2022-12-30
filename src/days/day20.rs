use std::fs::read_to_string;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let numbers: Vec<i64> = read_to_string("input/day20.txt").unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    let sol1 = decrypt(&numbers, 1, 1);
    let sol2 = decrypt(&numbers, 811_589_153, 10);

    (Solution::from(sol1), Solution::from(sol2))
}

fn decrypt(numbers: &[i64], key: i64, rounds: usize) -> i64 {
    // Build the list that we will shuffle around.
    let mut decoded: Vec<(usize, i64)> = numbers.iter()
        .map(|x| x * key)
        .enumerate()
        .collect();

    let modulo = numbers.len() as i64 - 1;

    for _ in 0..rounds {
        for orig_order in 0..numbers.len() {
            // Find the number's value and its current position in the array
            let (old_i, (_, value)) = decoded.iter().enumerate().find(|(_, t)| t.0 == orig_order).unwrap();

            // Determine the target position after moving it
            let new_i = (old_i as i64 + value).rem_euclid(modulo) as usize;

            // Rotate the elements between the old and new indices as needed. 
            // This is more efficient than just popping it and inserting it back, 
            // since it only disrupts the minimum required amount of elements.
            if new_i > old_i {
                decoded[old_i..=new_i].rotate_left(1);
            } else {
                decoded[new_i..=old_i].rotate_right(1);
            }
        }
    }

    // Compute the final value
    let zero_pos = decoded.iter().position(|(_, num)| *num == 0).unwrap();
    [1000, 2000, 3000].into_iter()
        .map(|offset| decoded[(zero_pos + offset) % numbers.len()].1)
        .sum()
}
