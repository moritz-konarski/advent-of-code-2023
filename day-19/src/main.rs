use solution::{part1, part2};

fn main() {
    let run_type = env::args().nth(1).unwrap_or_default();
    let number = env::args().nth(2).unwrap_or_default();

    let result = match (run_type.as_str(), number.as_str()) {
        ("t", "1") => solution::part1("test1.txt"),
        ("p", "1") => solution::part1("part1.txt"),
        ("t", "2") => solution::part2("test2.txt"),
        ("p", "2") => solution::part2("part2.txt"),
        _ => Err("Incorrect usage!\nUsage: cargo r -- [t|p] [1|2]"),
    };

    match result {
        Ok(r) => println!("Result for {run_type}{number} is {r:?}"),
        Err(e) => eprintln!("An error occurred:\n{e}"),
    }
}

use std::env;

#[test]
fn part1_test() {
    assert_eq!(Ok(19114), solution::part1("test1.txt"));
}

#[test]
fn part1_full() {
    assert_eq!(Ok(449531), solution::part1("part1.txt"));
}

#[test]
fn part2_test() {
    assert_eq!(Ok(167409079868000), solution::part2("test2.txt"));
}

// #[test]
// fn part2_full() {
//     assert_eq!(23, solution::part2("part2.txt"));
// }
