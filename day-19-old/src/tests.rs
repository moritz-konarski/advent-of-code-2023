#![cfg(test)]

use crate::solution::{part1, part2};

#[test]
fn part1_example() {
    assert_eq!(Ok(19114), part1("test1.txt"));
}

#[test]
fn part1_full() {
    assert_eq!(Ok(449531), part1("part1.txt"));
}

#[test]
fn part2_example() {
    assert_eq!(Ok(167409079868000), part2("test2.txt"));
}

// #[test]
// fn part2_full() {
//     assert_eq!(23, solution::part2("part2.txt"));
// }
