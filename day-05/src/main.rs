use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-05 p<n>";
    if let Some(part) = env::args().nth(1) {
        match part.as_str() {
            "p1" => {
                println!("Reading `{PART1_FILE}`");
                println!("Sum is {}", part1(PART1_FILE));
            }
            "p2" => {
                println!("Reading `{PART2_FILE}`");
                println!("Sum is {}", part2(PART2_FILE));
            }
            _ => eprintln!("{usage}"),
        }
    } else {
        eprintln!("{usage}");
    }
}

fn part1(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    file.lines().fold(0, |sum, line| sum)
}

fn part2(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    file.lines().fold(0, |sum, line| sum)
}

#[test]
fn part1_example() {
    assert_eq!(13, part1("test1.txt"));
}

// #[test]
// fn part1_puzzle() {
//     assert_eq!(25174, part1(PART1_FILE));
// }

// #[test]
// fn part2_example() {
//     assert_eq!(30, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(6420979, part2(PART2_FILE));
// }
