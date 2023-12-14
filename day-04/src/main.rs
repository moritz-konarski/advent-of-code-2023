use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-04 p<n>";
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

    // Card  n: xx xx xx  | xx xx xx xx xx
    // game  n: winning # | our numbers
    // TODO: build a hash set and then use .intersection().len()
    // use .collect::<HashSet<>>() ...
    // string parse everything just by whitespace
    // score = 1 << (intersect_len - 1)

    let mut sum = 0;

    sum
}

fn part2(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    0
}

#[test]
fn part1_example() {
    assert_eq!(13, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(105, part1(PART1_FILE));
}

// #[test]
// fn part2_example() {
//     assert_eq!(467835, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(85010461, part2(PART2_FILE));
// }
