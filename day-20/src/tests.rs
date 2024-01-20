#![cfg(test)]

use crate::solution::{part1, part2};

// #[test]
// fn part1_example_1_small() {
//     assert_eq!(32, part1("test1.txt", 1).unwrap());
// }

// #[test]
// fn part1_example_1() {
//     assert_eq!(32000000, part1("test1.txt", 1_000).unwrap());
// }

#[test]
fn part1_example_2() {
    assert_eq!(11687500, part1("test2.txt", 1_000).unwrap());
}

// #[test]
// fn part1_full() {
//     assert_eq!(Ok(449531), part1("part1.txt", 1_000));
// }

// #[test]
// fn part2_example() {
//     assert_eq!(Ok(167409079868000), part2("test2.txt"));
// }

// #[test]
// fn part2_full() {
//     assert_eq!(23, solution::part2("part2.txt"));
// }
