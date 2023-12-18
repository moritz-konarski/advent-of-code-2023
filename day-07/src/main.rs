use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-07 p<n>";
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

const CARDS: [(char, usize); 13] = [
    ('A', 12),
    ('K', 11),
    ('Q', 10),
    ('J', 9),
    ('T', 8),
    ('9', 7),
    ('8', 6),
    ('7', 5),
    ('6', 4),
    ('5', 3),
    ('4', 2),
    ('3', 1),
    ('2', 0),
];

struct Hand {
    cards: [usize; 5],
    bid: usize,
}

impl Hand {
    fn new(line: &str, map: &HashMap<char, usize>) -> Self {
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid = bid;

        Self {
            cards: [0; 5],
            bid: 0,
        }
    }
}

fn part1(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let map = HashMap::from(CARDS);
    let mut hands = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();
        hands.push(Hand::new(&line, &map));
    }

    // poker hands
    // high card: 5 unique cards
    // pair: 4 unique cards
    // two pair: 3 unique cards
    // triplet: 3 unique cards
    // full house: 2 unique cards
    // quadruplet: 2 unique cards
    // quintuples: 1 unique cards

    // then order then by relative card strength

    0
}

fn part2(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    0
}

#[test]
fn part1_example() {
    assert_eq!(6440, part1("test1.txt"));
}

// #[test]
// fn part1_puzzle() {
//     assert_eq!(293046, part1(PART1_FILE));
// }

// #[test]
// fn part2_example() {
//     assert_eq!(71503, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(35150181, part2(PART2_FILE));
// }
