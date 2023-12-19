use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-09 p<n>";
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

fn add_last(line: &str) -> i64 {
    let mut last_nums = Vec::new();
    let mut nums: Vec<i64> = line
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    while !nums.iter().all(|n| *n == 0) {
        last_nums.push(*nums.last().unwrap());
        nums = nums.windows(2).map(|w| w[1] - w[0]).collect();
    }

    last_nums.iter().sum()
}

fn part1(filename: &str) -> i64 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    file.lines()
        .fold(0, |sum, line| sum + add_last(&line.unwrap()))
}

fn add_first(line: &str) -> i64 {
    let mut first_nums = Vec::new();
    let mut nums: Vec<i64> = line
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    while !nums.iter().all(|n| *n == 0) {
        first_nums.push(*nums.first().unwrap());
        nums = nums.windows(2).map(|w| w[1] - w[0]).collect();
    }

    first_nums.iter().rev().fold(0, |diff, num| num - diff)
}

fn part2(filename: &str) -> i64 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    file.lines()
        .fold(0, |sum, line| sum + add_first(&line.unwrap()))
}

#[test]
fn part1_example() {
    assert_eq!(114, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(1882395907, part1(PART1_FILE));
}

#[test]
fn part2_example() {
    assert_eq!(2, part2("test2.txt"));
}

#[test]
fn part2_puzzle() {
    assert_eq!(1005, part2(PART2_FILE));
}
