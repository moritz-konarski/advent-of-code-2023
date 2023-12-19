use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";
const START_LABEL: &str = "AAA";
const END_LABEL: &str = "ZZZ";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-08 p<n>";
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

struct Node {
    label: String,
    left: String,
    right: String,
}

impl Node {
    fn new(line: &str) -> Self {
        let (label, connections) = line.split_once(" = ").unwrap();
        let label = label.to_string();

        let (left, right) = connections.split_once(", ").unwrap();
        let left = left.get(1..).unwrap().to_string();
        let right = right.get(..right.len() - 1).unwrap().to_string();

        Self { label, left, right }
    }

    fn is_end(&self) -> bool {
        self.label == END_LABEL
    }
}

fn part1(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut lines = file.lines();

    let command_line = lines.next().unwrap().unwrap();

    let mut nodes = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let node = Node::new(&line);
        nodes.insert(node.label.clone(), node);
    }

    let mut node = nodes.get(START_LABEL).unwrap();
    for (index, command) in command_line.chars().cycle().enumerate() {
        match command {
            'R' => node = nodes.get(&node.right).unwrap(),
            'L' => node = nodes.get(&node.left).unwrap(),
            _ => unreachable!("there are no other commands"),
        }

        if node.is_end() {
            return index + 1;
        }
    }
    unreachable!("there must be an end");
}

fn part2(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    0
}

#[test]
fn part1_example() {
    assert_eq!(2, part1("test1.txt"));
}

#[test]
fn part1_example2() {
    assert_eq!(6, part1("test3.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(22357, part1(PART1_FILE));
}

// #[test]
// fn part2_example() {
//     assert_eq!(5905, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(250506580, part2(PART2_FILE));
// }
