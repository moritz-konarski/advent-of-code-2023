use std::env;

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-15 p<n>";
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

fn hash(string: &str) -> u8 {
    string
        .bytes()
        .fold(0, |hash, c| hash.wrapping_add(c).wrapping_mul(17))
}

fn part1(filename: &str) -> u64 {
    let file = std::fs::read_to_string(filename).unwrap();
    let elements: Vec<_> = file
        .split_ascii_whitespace()
        .flat_map(|part| part.split(','))
        .collect();

    elements.iter().fold(0, |sum, ele| sum + hash(ele) as u64)
}

fn part2(filename: &str) -> u64 {
    let file = std::fs::read_to_string(filename).unwrap();

    0
}

#[test]
fn part1_example() {
    assert_eq!(1320, part1("test1.txt"));
}

#[test]
fn part1_example_hash() {
    assert_eq!(52, hash("HASH"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(504036, part1(PART1_FILE));
}

// #[test]
// fn part2_example() {
//     assert_eq!(1030, part2("test2.txt", 10));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(597714117556, part2(PART2_FILE, 1000000));
// }
