use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";
const START_LABEL: &str = "AAA";
const END_LABEL: &str = "ZZZ";
const START_LABEL2: &str = "A";
const END_LABEL2: &str = "Z";

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

#[derive(Debug)]
struct Node {
    label: String,
    left: Option<usize>,
    right: Option<usize>,
    is_end: bool,
}

impl Node {
    fn new(label: &str, end_str: &str) -> Self {
        let label = label.to_string();
        let is_end = label.ends_with(end_str);

        Self {
            label,
            left: None,
            right: None,
            is_end,
        }
    }

    fn add_left(&mut self, other: usize) {
        self.left = Some(other);
    }

    fn add_right(&mut self, other: usize) {
        self.right = Some(other);
    }
}

fn part1(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut lines = file.lines();
    let command_line = lines.next().unwrap().unwrap();

    let mut node_list = Vec::new();
    let mut conn_list = Vec::new();

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let (label, connections) = line.split_once(" = ").unwrap();
        let node = Node::new(&label, END_LABEL);

        node_list.push(node);
        conn_list.push(connections.to_owned());
    }

    println!("{node_list:?}");

    for (i, conn) in conn_list.iter().enumerate() {
        let (left, right) = conn.split_once(", ").unwrap();

        let left = left.get(1..).unwrap();
        let left = node_list
            .iter()
            .position(|node| node.label == left)
            .unwrap();

        let right = right.get(..right.len() - 1).unwrap();
        println!("{right:?}");
        let right = node_list
            .iter()
            .position(|node| node.label == right)
            .unwrap();
        println!("{right:?}");

        node_list[i].add_left(left);
        node_list[i].add_right(right);
    }

    println!("{node_list:?}");

    let mut current_node = node_list
        .iter()
        .position(|node| node.label.ends_with(START_LABEL))
        .unwrap();

    for (index, command) in command_line.chars().cycle().enumerate() {
        match command {
            'R' => current_node = node_list[current_node].right.unwrap(),
            'L' => current_node = node_list[current_node].left.unwrap(),
            _ => unreachable!("there are no other commands"),
        }

        if node_list[current_node].is_end {
            return index + 1;
        }
    }
    unreachable!("there must be an end");
}

fn part2(filename: &str) -> usize {
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
//     assert_eq!(6, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(250506580, part2(PART2_FILE));
// }
