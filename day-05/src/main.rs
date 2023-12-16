use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

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

struct RangeMap {
    range_to_dest: BTreeMap<usize, (usize, usize)>,
}

impl RangeMap {
    fn new() -> Self {
        Self {
            range_to_dest: BTreeMap::new(),
        }
    }

    fn parse_line(&mut self, line: &String) {
        let elements: Vec<usize> = line.splitn(3, ' ').map(|s| s.parse().unwrap()).collect();
        let dest_start = elements[0];
        let source_start = elements[1];
        let length = elements[2];

        self.range_to_dest
            .insert(source_start, (dest_start, length));
    }

    fn get(&self, seed: usize) -> usize {
        let mapping = self
            .range_to_dest
            .iter()
            .find(|(source_start, (_, length))| {
                **source_start <= seed && seed < **source_start + length
            });

        match mapping {
            Some((source_start, (dest_start, _))) => dest_start + seed - source_start,
            None => seed,
        }
    }
}

fn get_seeds(lines: &mut Lines<BufReader<File>>) -> Vec<usize> {
    let line = lines.next().unwrap().unwrap();
    let (_, seeds) = line.split_once(':').unwrap();

    seeds
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut lines = file.lines();

    // extract seeds and consume that line
    let mut seed_list = get_seeds(&mut lines);
    // skip the following empty line
    lines.next();

    while let Some(Ok(mut line)) = lines.next() {
        // skip lines that indicate a description
        if line.ends_with(':') {
            continue;
        }

        // build map from lines
        let mut map = RangeMap::new();
        while !line.is_empty() {
            map.parse_line(&line);

            match lines.next() {
                Some(Ok(new_line)) => line = new_line,
                _ => break,
            }
        }

        // use map to process seeds stuff
        seed_list = seed_list.iter().map(|item| map.get(*item)).collect();
    }

    // find smallest value
    *seed_list.iter().min().unwrap() as u32
}

fn part2(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    file.lines().fold(0, |sum, line| sum)
}

#[test]
fn part1_example() {
    assert_eq!(35, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(227653707, part1(PART1_FILE));
}

// #[test]
// fn part2_example() {
//     assert_eq!(30, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(6420979, part2(PART2_FILE));
// }
