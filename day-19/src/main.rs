use std::{env, fs};

const USAGE: &str = "Incorrect usage!\nUsage: cargo r -- [t|p] [1|2]";

fn main() {
    let run_type = env::args().nth(1).expect(USAGE);
    let number = env::args().nth(2).expect(USAGE);
    let result = match (run_type.as_str(), number.as_str()) {
        ("t", "1") => part1("test1.txt"),
        ("p", "1") => part1("part1.txt"),
        ("t", "2") => part2("test2.txt"),
        ("p", "2") => part2("part2.txt"),
        _ => panic!("{USAGE}"),
    };
    println!("Result for {run_type}{number} is {result:?}");
}

fn part1(filename: &str) -> usize {
    let file = fs::read_to_string(filename).expect(&format!("the file `{filename}` should exist"));
    0
}

fn part2(filename: &str) -> usize {
    let file = fs::read_to_string(filename).expect(&format!("the file `{filename}` should exist"));
    0
}

#[test]
fn part1_test() {
    assert_eq!(19114, part1("test1.txt"));
}

// #[test]
// fn part1_test() {
//     assert_eq!(23, part1("part1.txt"));
// }

// #[test]
// fn part2_test() {
//     assert_eq!(23, part2("test2.txt"));
// }

// #[test]
// fn part2_test() {
//     assert_eq!(23, part2("part2.txt"));
// }
