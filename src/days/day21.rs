use std::fs::read_to_string;

use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::etc::maybe_val::MaybeVal;
use crate::{Solution, SolutionPair};

////////////////////////////////////////////////////////////////////////////////

type MonkeyMap<'a> = FxHashMap<&'a str, MonkeyData<'a>>;

enum MonkeyData<'a> {
    Value(MaybeVal),
    Operation{left: &'a str, right: &'a str, op: Operator}
}

#[derive(Copy, Clone)]
enum Operator { Add, Sub, Mul, Div, Eq }

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day21.txt").unwrap();
    let mut map: MonkeyMap = input.lines().map(MonkeyData::from_line).collect();

    // Part 1: just calculate the value recursively
    let sol1 = map["root"].resolve(&map).unwrap();

    // Part 2: replace and find out the unknown value
    map.insert("humn", MonkeyData::Value(MaybeVal::Unknown));
    map.get_mut("root").unwrap().replace_op(Operator::Eq);
    let sol2 = map["root"].resolve_uncertainty(&map, 0);

    (Solution::from(sol1), Solution::from(sol2))
}

////////////////////////////////////////////////////////////////////////////////

impl<'a> MonkeyData<'a> {
    pub fn from_line(line: &'a str) -> (&str, Self) {
        let (id, info) = line.split_once(": ").unwrap();
        let data = if info.chars().next().unwrap().is_ascii_digit() {
            Self::Value(MaybeVal::Known(info.parse().unwrap()))
        } else {
            let (left, operator, right) = info.split(' ').collect_tuple().unwrap();
            let op = match operator {
                "+" => Operator::Add,
                "-" => Operator::Sub,
                "*" => Operator::Mul,
                "/" => Operator::Div,
                 _  => unreachable!()
            };
            Self::Operation { left, right, op }
        };

        (id, data)
    }

    pub fn resolve(&self, map: &MonkeyMap) -> MaybeVal {
        match self {
            Self::Value(x) => *x,
            Self::Operation { left, right, op } => {
                let left_val = map[left].resolve(map);
                let right_val = map[right].resolve(map);
                match op {
                    Operator::Add => left_val + right_val,
                    Operator::Sub => left_val - right_val,
                    Operator::Mul => left_val * right_val,
                    Operator::Div => left_val / right_val,
                    _  => unreachable!()
                }
            }
        }
    }

    pub fn resolve_uncertainty(&self, map: &MonkeyMap, must_be: i64) -> i64 {
        let (left, right, op) = self.unpack_operation();
        let left_val = map[left].resolve(map);
        let right_val = map[right].resolve(map);

        let left_unknown = left_val.is_unknown();
        let unknown_label = if left_unknown { left } else { right };
        let known_value = left_val.as_option().unwrap_or_else(|| right_val.unwrap());

        // Resolve the equation depending on the operation type and on which side it is
        let solved_val = match op {
            Operator::Add                 => must_be - known_value,
            Operator::Sub if left_unknown => known_value + must_be,
            Operator::Sub                 => known_value - must_be,
            Operator::Mul                 => must_be / known_value,
            Operator::Div if left_unknown => known_value * must_be,
            Operator::Div                 => known_value / must_be,
            Operator::Eq                  => known_value 
        };

        // If this is the human's value, return it right away,
        // otherwise keep recursively solving until we reach it
        if unknown_label == "humn" {
            return solved_val;
        } else {
            return map[unknown_label].resolve_uncertainty(map, solved_val);
        }
    }

    pub fn replace_op(&mut self, new_op: Operator) {
        if let MonkeyData::Operation { op, .. } = self {
            *op = new_op
        }
    }

    fn unpack_operation(&self) -> (&str, &str, Operator) {
        match self {
            MonkeyData::Value(_) => panic!("Tried to unpack operation on a value"),
            MonkeyData::Operation { left, right, op } => (left, right, *op),
        }
    }
}
