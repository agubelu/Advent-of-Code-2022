use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

const SNAFU_CHARS: [char; 5] = ['0', '1', '2', '=', '-'];

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day25.txt").unwrap();
    let code = input.lines().map(snafu2dec).sum();

    let sol1 = dec2snafu(code);
    let sol2 = "🎄❄️ Merry Christmas! ❄️🎄";

    (Solution::from(sol1), Solution::from(sol2))
}

fn dec2snafu(mut number: i64) -> String {
    let mut res = String::new();

    while number > 0 {
        let rem = number % 5;
        res = format!("{}{}", SNAFU_CHARS[rem as usize], res);
        if rem >= 3 {
            number += 5;
        }
        number /= 5;
    }

    res
}

fn snafu2dec(number: &str) -> i64 {
    number.chars().rev().enumerate()
          .map(|(i, ch)| 5_i64.pow(i as u32) * snafu_digit(ch))
          .sum()
}

fn snafu_digit(ch: char) -> i64 {
    match ch {
        '=' => -2,
        '-' => -1,
         x  => x.to_digit(10).unwrap() as i64,
    }
}
