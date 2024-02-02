use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-06 p<n>";
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

fn get_nums(line: &str) -> Vec<usize> {
    let (_, times) = line.split_once(':').unwrap();
    times
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_nums_merged(line: &str) -> usize {
    let (_, times) = line.split_once(':').unwrap();
    let times: String = times.split_ascii_whitespace().collect();
    times.parse().unwrap()
}

fn part1(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut lines = file.lines();
    let times = get_nums(&lines.next().unwrap().unwrap());
    let distances = get_nums(&lines.next().unwrap().unwrap());

    times
        .iter()
        .zip(distances)
        .fold(1, |product, (time, distance)| {
            let options = (1..*time).into_iter().fold(0, |count, t| {
                if t * (time - t) > distance {
                    count + 1
                } else {
                    count
                }
            });

            product * options
        })
}

fn part2(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut lines = file.lines();
    let time = get_nums_merged(&lines.next().unwrap().unwrap());
    let distance = get_nums_merged(&lines.next().unwrap().unwrap());

    (1..time).into_iter().fold(0, |count, t| {
        if t * (time - t) > distance {
            count + 1
        } else {
            count
        }
    })
}

#[test]
fn part1_example() {
    assert_eq!(288, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(293046, part1(PART1_FILE));
}

#[test]
fn part2_example() {
    assert_eq!(71503, part2("test2.txt"));
}

#[test]
fn part2_puzzle() {
    assert_eq!(35150181, part2(PART2_FILE));
}
