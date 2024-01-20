use std::{collections::BTreeMap, env};

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

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    let elements: Vec<_> = file
        .split_ascii_whitespace()
        .flat_map(|part| part.split(','))
        .collect();

    elements.iter().fold(0, |sum, ele| sum + hash(ele) as usize)
}

fn part2(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();

    let elements: Vec<_> = file
        .split_ascii_whitespace()
        .flat_map(|part| part.split(','))
        .map(|part| {
            let parts: Vec<_> = part.split_inclusive(&['=', '-']).collect();
            let (label, operation) = parts[0].split_at(parts[0].len() - 1);
            (operation, label, parts.get(1).copied())
        })
        .collect();

    let mut boxes: BTreeMap<usize, Vec<(&str, usize)>> = BTreeMap::new();

    for ele in elements {
        let (operation, label, focal_length) = ele;
        let h = &(hash(label) as usize);
        match operation {
            "-" => {
                if let Some(e) = boxes.get_mut(h) {
                    if let Some(pos) = e.iter().position(|(l, _)| *l == label) {
                        e.remove(pos);
                    }
                }
            }
            "=" => {
                let focal_length = focal_length.unwrap().parse().unwrap();
                boxes
                    .entry(*h)
                    .and_modify(|e| {
                        if let Some(pos) = e.iter().position(|(l, _)| *l == label) {
                            e[pos].1 = focal_length;
                        } else {
                            e.push((label, focal_length));
                        }
                    })
                    .or_insert(vec![(label, focal_length)]);
            }
            _ => unreachable!("illegal operation {operation:?}"),
        }
    }

    boxes.iter().fold(0, |sum, (key, value)| {
        sum + (key + 1)
            * value
                .iter()
                .enumerate()
                .fold(0, |s, (i, (_, focal_length))| s + (i + 1) * *focal_length)
    })
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

#[test]
fn part2_example() {
    assert_eq!(145, part2("test2.txt"));
}

#[test]
fn part2_puzzle() {
    assert_eq!(295719, part2(PART2_FILE));
}
