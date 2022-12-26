use std::fs::read_to_string;

use rayon::prelude::*;
use scanf::sscanf;

use crate::{Solution, SolutionPair};
use crate::etc::coords::Coords;
use BoundType::*;

////////////////////////////////////////////////////////////////////////////////

type Pos = Coords<i64>;
const COUNT_ROW: i64 = 2_000_000;
const PART2_BOUND: i64 = 4_000_000;

struct SensorInfo {
    position: Pos,
    range: i64,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RangeBound {
    pos: i64,
    kind: BoundType,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum BoundType { Start, End }

pub fn solve() -> SolutionPair {
    let sensors: Vec<SensorInfo> = read_to_string("input/day15.txt").unwrap()
        .lines()
        .map(load_line_info)
        .collect();

    // Part 1: count the spaces in the provided row
    let sol1 = count_spaces_row(&sensors, COUNT_ROW, None).0;

    // Part 2: find out which row contains a single gap
    let sol2 = (0..=PART2_BOUND).into_par_iter()
        .find_map_any(|y| {
            let found = count_spaces_row(&sensors, y, Some((0, PART2_BOUND))).1;
            found.map(|x| x * 4_000_000 + y)
        }).unwrap();

    (Solution::from(sol1), Solution::from(sol2))
}

// Fins out how many guaranteed sensor-free spaces there are in a row,
// optionally bounded to a range of x-coordinates, and also returns the
// only space where a sensor could be for part 2
fn count_spaces_row(sensors: &[SensorInfo], row: i64, bounds: Option<(i64, i64)>) -> (i64, Option<i64>) {
    let mut ranges = vec![];

    for &SensorInfo { position, range } in sensors {
        let y_diff = (position.y - row).abs();

        // If this sensor is out of range for the provided row, skip it
        if y_diff > range {
            continue;
        }

        let x_margin = range - y_diff;
        let start = position.x - x_margin;
        let end = position.x + x_margin;

        // Only add this range if it's within bounds, if they are provided
        if let Some((min, max)) = bounds {
            if end < min || start > max {
                continue;
            }
        }
        
        ranges.push(RangeBound { pos: position.x - x_margin, kind: Start });
        ranges.push(RangeBound { pos: position.x + x_margin, kind: End });
    }

    count_range_sizes(&mut ranges)
}

// Algorithm that counts the number of distinct elements in overlapping number
// ranges in O(n), from https://stackoverflow.com/questions/20553345/length-of-union-of-ranges
// Slightly modified to find length 1 gaps for part 2
fn count_range_sizes(ranges: &mut[RangeBound]) -> (i64, Option<i64>) {
    ranges.sort();

    let (mut start, mut end) = (ranges[0].pos, i64::MAX);
    let (mut counter, mut res) = (0, 0);
    let mut part2 = None;

    for &RangeBound { pos, kind } in ranges.iter() {
        if counter == 0 {
            start = pos;
            if pos > 0 && pos <= PART2_BOUND && start == end + 2 {
                part2 = Some(pos - 1);
            }
        }

        counter += if kind == Start { 1 } else { -1 };

        if counter == 0 {
            end = pos;
            res += end - start;
        }
    }

    (res, part2)
}


fn load_line_info(line: &str) -> SensorInfo {
    let (mut x_sensor, mut y_sensor) = (0, 0);
    let (mut x_beacon, mut y_beacon) = (0, 0);

    sscanf!(line, 
        "Sensor at x={}, y={}: closest beacon is at x={}, y={}", 
        x_sensor, y_sensor, x_beacon, y_beacon
    ).unwrap();

    let position = Pos::new(x_sensor, y_sensor);
    let range = (x_beacon - x_sensor).abs() + (y_beacon - y_sensor).abs();
    SensorInfo { position, range }
}
