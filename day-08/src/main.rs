use num::integer;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

const START_LABEL: &str = "AAA";
const END_LABEL: &str = "ZZZ";

const START_LABEL2: &str = "A";
const END_LABEL2: &str = "Z";

const RIGHT: u32 = 0;
const LEFT: u32 = 1;

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

    fn with_left_right(
        label: String,
        left: Option<usize>,
        right: Option<usize>,
        is_end: bool,
    ) -> Self {
        Self {
            label,
            left,
            right,
            is_end,
        }
    }
}

fn get_commands(lines: &mut Lines<BufReader<File>>) -> Vec<u32> {
    let command_line = lines.next().unwrap().unwrap();
    command_line
        .chars()
        .map(|c| match c {
            'R' => RIGHT,
            'L' => LEFT,
            _ => unreachable!("no other commands"),
        })
        .collect()
}

fn get_nodes_conns(
    lines: &mut Lines<BufReader<File>>,
    end_label: &str,
) -> (Vec<Node>, Vec<String>) {
    let mut node_list = Vec::new();
    let mut conn_list = Vec::new();

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let (label, connections) = line.split_once(" = ").unwrap();
        let node = Node::new(label, end_label);

        node_list.push(node);
        conn_list.push(connections.to_string());
    }

    (node_list, conn_list)
}

fn update_nodes(node_list: Vec<Node>, conn_list: Vec<String>) -> Vec<Node> {
    conn_list
        .iter()
        .enumerate()
        .map(|(i, conn)| {
            let (left, right) = conn.split_once(", ").unwrap();

            let left = left.get(1..).unwrap();
            let left = node_list
                .iter()
                .position(|node| node.label == left)
                .unwrap();

            let right = right.get(..right.len() - 1).unwrap();
            let right = node_list
                .iter()
                .position(|node| node.label == right)
                .unwrap();

            Node::with_left_right(
                node_list[i].label.clone(),
                Some(left),
                Some(right),
                node_list[i].is_end,
            )
        })
        .collect()
}

fn get_steps_to_end(node_list: &[Node], command_line: &[u32], current_node: usize) -> usize {
    let mut current_node = current_node;
    for (index, command) in command_line.iter().cycle().enumerate() {
        match *command {
            RIGHT => current_node = node_list[current_node].right.unwrap(),
            LEFT => current_node = node_list[current_node].left.unwrap(),
            _ => unreachable!("there are no other commands"),
        }

        if node_list[current_node].is_end {
            return index + 1;
        }
    }
    unreachable!("there is always an end");
}

fn part1(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut lines = file.lines();
    let command_line = get_commands(&mut lines);

    let (node_list, conn_list) = get_nodes_conns(&mut lines, END_LABEL);

    let node_list = update_nodes(node_list, conn_list);

    let current_node = node_list
        .iter()
        .position(|node| node.label.ends_with(START_LABEL))
        .unwrap();

    get_steps_to_end(&node_list, &command_line, current_node)
}

fn part2(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut lines = file.lines();
    let command_line = get_commands(&mut lines);

    let (node_list, conn_list) = get_nodes_conns(&mut lines, END_LABEL2);

    let node_list = update_nodes(node_list, conn_list);

    let current_nodes: Vec<_> = node_list
        .iter()
        .enumerate()
        .filter_map(|(i, node)| {
            if node.label.ends_with(START_LABEL2) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    current_nodes
        .iter()
        .map(|n| get_steps_to_end(&node_list, &command_line, *n))
        .fold(1, |lcm, s| integer::lcm(lcm, s))
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

#[test]
fn part2_example() {
    assert_eq!(6, part2("test2.txt"));
}

#[test]
fn part2_puzzle() {
    assert_eq!(10371555451871, part2(PART2_FILE));
}
