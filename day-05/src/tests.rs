#![cfg(test)]

use crate::solution::{part1, part2};

const TEST1: Result<u64, &'static str> = Ok(35);
const PART1: Result<u64, &'static str> = Ok(227653707);
const TEST2: Result<u64, &'static str> = Ok(46);
const PART2: Result<u64, &'static str> = Ok(78775051);

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
    assert_eq!(TEST2, part2(include_str!("test1.txt")));
}

// #[test]
// fn part2_full() {
//     assert_eq!(PART2, part2(include_str!("part1.txt")));
// }
