#![cfg(test)]

use crate::{
    number_parser::{self, NumberParser},
    solution::{part1, part2},
};

const TEST1: Result<u64, &'static str> = Ok(142);
const PART1: Result<u64, &'static str> = Ok(54561);
const TEST2: Result<u64, &'static str> = Ok(281);
const PART2: Result<u64, &'static str> = Ok(54076);

#[test]
fn part1_example() {
    assert_eq!(TEST1, part1(include_str!("test1.txt")));
}

#[test]
fn part1_full() {
    assert_eq!(PART1, part1(include_str!("part1.txt")));
}

#[test]
fn part2_example() {
    assert_eq!(TEST2, part2(include_str!("test2.txt")));
}

// #[test]
// fn part2_full() {
//     assert_eq!(PART2, part2(include_str!("part1.txt")));
// }

#[test]
fn part2_special() {
    let s = "fjzfb6onefourhtlmvlns";
    let np = NumberParser::new();
    assert_eq!(Some(6), np.get_left(s));
    assert_eq!(Some(4), np.get_right(s));
}
