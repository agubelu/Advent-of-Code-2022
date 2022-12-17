// TO-DO: i'm catching up after being away and this code is
// a bit of a mess as i was going through trial and error,
// a nice refactor will happen soon

use crate::{Solution, SolutionPair};
use crate::etc::coords::Coords;
use rustc_hash::FxHashMap;
use itertools::Itertools;
use scanf::sscanf;
use std::cmp::Ordering;
use std::fs::read_to_string;
use BoundType::*;

///////////////////////////////////////////////////////////////////////////////

type Pos = Coords<i64>;
const COUNT_ROW: i64 = 2_000_000;
const PART2_BOUND: i64 = 4_000_000;

struct SensorInfo {
    position: Pos,
    nearest_beacon: Pos,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct RangeBound {
    pos: i64,
    kind: BoundType,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum BoundType { Start, End }

pub fn solve() -> SolutionPair {
    let data = read_to_string("input/day15.txt").unwrap()
        .lines()
        .map(load_line_info)
        .collect_vec();

    // Store all positions where we are sure that no beacon exists, that is,
    // those that are at the same distance or closer to the sensor than the
    // closest beacon
    let mut no_beacons = FxHashMap::default();
    data.iter().for_each(|x| update_no_beacons(x, &mut no_beacons));

    let sol1 = count_range_sizes(no_beacons.get_mut(&COUNT_ROW).unwrap()).0;
    let beacon_pos = (0..=PART2_BOUND).map(|y| (y, count_range_sizes(no_beacons.get_mut(&y).unwrap()).1))
        .find(|(_, opt)| opt.is_some())
        .unwrap();

    let sol2 = beacon_pos.1.unwrap() * 4000000 + beacon_pos.0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn count_range_sizes(ranges: &mut[RangeBound]) -> (usize, Option<i64>) {
    ranges.sort();

    let mut start = ranges[0].pos;
    let mut end = i64::MAX;
    let mut counter = 0;
    let mut res = 0;
    let mut part2 = None;

    for &RangeBound { pos, kind } in ranges.iter() {
        if kind == Start {
            if counter == 0 {
                start = pos;
                if pos > 0 && pos <= PART2_BOUND && start == end + 2 {
                    part2 = Some(pos - 1);
                }
            }
            counter += 1;
        } else {
            counter -= 1;
            if counter == 0 {
                end = pos;
                res += end - start;
            }
        }
    }

    (res as usize, part2)
}

fn update_no_beacons(data: &SensorInfo, no_beacons: &mut FxHashMap<i64, Vec<RangeBound>>) {
    let origin = data.position;
    let dist = (origin.x - data.nearest_beacon.x).abs() + (origin.y - data.nearest_beacon.y).abs();

    for y_diff in 0..=dist {
        let x_diff = dist - y_diff;

        for sign in [1, -1] {
            let y_pos = origin.y + y_diff * sign;
            
            if !(0..=PART2_BOUND).contains(&y_pos) {
                continue;
            }
            let x_range_start = RangeBound{pos: origin.x - x_diff, kind: Start};
            let x_range_end = RangeBound{pos: origin.x + x_diff, kind: End};

            let entry = no_beacons.entry(y_pos).or_insert(vec![]);
            entry.push(x_range_start);
            entry.push(x_range_end);
        }
    }
}

fn load_line_info(line: &str) -> SensorInfo {
    let (mut x_sensor, mut y_sensor) = (0, 0);
    let (mut x_beacon, mut y_beacon) = (0, 0);

    sscanf!(line, 
        "Sensor at x={}, y={}: closest beacon is at x={}, y={}", 
        x_sensor, y_sensor, x_beacon, y_beacon
    ).unwrap();

    let position = Pos::new(x_sensor, y_sensor);
    let nearest_beacon = Pos::new(x_beacon, y_beacon);
    SensorInfo { position, nearest_beacon }
}

impl Ord for RangeBound {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.pos.cmp(&other.pos) {
            Ordering::Equal => self.kind.cmp(&other.kind),
            x => x,
        }
    }
}

impl PartialOrd for RangeBound {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}