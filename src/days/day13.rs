use itertools::Itertools;
use crate::{Solution, SolutionPair};
use std::cmp::{Ordering, PartialOrd, Ord};
use std::fs::read_to_string;
use PacketElem::*;

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
enum PacketElem {
    Int(u32),
    List(Vec<PacketElem>),
}

pub fn solve() -> SolutionPair {
    let mut packets = read_to_string("input/day13.txt").unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_packet)
        .collect_vec();

    let sol1 = packets.iter()
        .tuples()
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    let divider1 = parse_packet("[[2]]");
    let divider2 = parse_packet("[[6]]");

    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort();
    
    let sol2: usize = packets.iter()
        .enumerate()
        .filter(|(_, pkt)| **pkt == divider1 || **pkt == divider2)
        .map(|(i, _)| i + 1)
        .product();

    (Solution::from(sol1), Solution::from(sol2))
}

// Parses a whole line containing a packet
fn parse_packet(line: &str) -> PacketElem {
    let chars = line.chars().collect_vec();
    parse_list_token(&chars).0
}

// Parses a list token and returns the element and its total length
fn parse_list_token(chars: &[char]) -> (PacketElem, usize) {
    let mut pos = 1; // skip the opening bracket
    let mut ls = vec![];

    while chars[pos] != ']' {
        let (element, len) = match chars[pos] {
            '[' => parse_list_token(&chars[pos..]),
             _  => parse_int_token(&chars[pos..]),
        };

        pos += len;
        ls.push(element);
    }

    // Increment position by 1 to account for the closing bracket
    pos += 1;

    // If we're not at the end of the string and the following character
    // is a comma, add it to the length too
    if pos < chars.len() - 1 && chars[pos] == ',' {
        pos += 1;
    }

    (PacketElem::List(ls), pos)
}

// Parses an integer token in a packet, returns the integer
// and its length (including the trailing comma if present)
fn parse_int_token(chars: &[char]) -> (PacketElem, usize) {
    let mut num = 0;
    let mut pos = 0;

    while let Some(digit) = chars[pos].to_digit(10) {
        num = num * 10 + digit;
        pos += 1;
    }

    if chars[pos] == ',' {
        pos += 1;
    }

    (PacketElem::Int(num), pos)
}

impl PartialOrd<PacketElem> for PacketElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketElem {
    fn cmp(&self, other: &PacketElem) -> Ordering {
        match (self, other) {
            (Int(left), Int(right)) => left.cmp(right),
            (List(_), Int(right)) => self.cmp(&List(vec![Int(*right)])),
            (Int(left), List(_)) => List(vec![Int(*left)]).cmp(other),
            (List(left_list), List(right_list)) => {
                let mut ord = Ordering::Equal;

                for (left, right) in left_list.iter().zip(right_list.iter()) {
                    ord = left.cmp(right);
                    if ord != Ordering::Equal {
                        break;
                    }
                }

                // If we're still tied, sort by list order
                if ord == Ordering::Equal {
                    ord = left_list.len().cmp(&right_list.len());
                }

                ord
            },
        }
    }
}
