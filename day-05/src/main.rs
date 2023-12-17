use rayon::prelude::*;
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

struct Mapping {
    src: std::ops::Range<usize>,
    dst: usize,
}

impl Mapping {
    fn new(src: usize, dst: usize, len: usize) -> Self {
        Self {
            src: (src..src + len),
            dst,
        }
    }

    fn map(&self, seed: &usize) -> usize {
        self.dst + seed - self.src.start
    }

    fn contains(&self, seed: &usize) -> bool {
        self.src.contains(seed)
    }
}

struct MapSet {
    mappings: Vec<BTreeMap<usize, Mapping>>,
}

impl MapSet {
    fn new() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }

    fn add_map(&mut self) {
        self.mappings.push(BTreeMap::new());
    }

    fn parse_line(&mut self, line: &str) {
        let elements: Vec<usize> = line.splitn(3, ' ').map(|s| s.parse().unwrap()).collect();
        let dest_start = elements[0];
        let source_start = elements[1];
        let length = elements[2];

        let len = &self.mappings.len() - 1;
        let map = Mapping::new(source_start, dest_start, length);

        self.mappings[len].insert(source_start, map);
    }

    fn map(&self, seed_mapping: Mapping) -> Vec<usize> {
        let mut seeds: Vec<usize> = seed_mapping.src.collect();

        for map in &self.mappings {
            seeds.par_iter_mut().for_each(|num| {
                if let Some(found_mapping) = map.values().find(|mapping| mapping.contains(num)) {
                    *num = found_mapping.map(num);
                }
            })
        }

        seeds
    }
}

fn get_seeds(lines: &mut Lines<BufReader<File>>) -> Vec<Mapping> {
    let line = lines.next().unwrap().unwrap();
    let (_, seeds) = line.split_once(':').unwrap();

    seeds
        .split_whitespace()
        .map(|s| {
            let seed = s.parse().unwrap();
            Mapping::new(seed, seed, 1)
        })
        .collect()
}

fn get_range_seeds(lines: &mut Lines<BufReader<File>>) -> Vec<Mapping> {
    let line = lines.next().unwrap().unwrap();
    let (_, seeds) = line.split_once(':').unwrap();

    let seeds: Vec<_> = seeds
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    seeds
        .chunks_exact(2)
        .map(|chunk| Mapping::new(chunk[0], chunk[0], chunk[1]))
        .collect()
}

fn get_map(lines: &mut Lines<BufReader<File>>) -> MapSet {
    let mut map = MapSet::new();

    while let Some(Ok(mut line)) = lines.next() {
        // skip lines that indicate a description
        if line.ends_with(':') || line.is_empty() {
            continue;
        }

        // build map from lines
        map.add_map();
        while !line.is_empty() {
            map.parse_line(&line);

            match lines.next() {
                Some(Ok(new_line)) => line = new_line,
                _ => break,
            }
        }
    }

    map
}

fn part1(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut lines = file.lines();

    let seed_list = get_seeds(&mut lines);

    let map = get_map(&mut lines);

    let mut minima = Vec::new();
    for seed in seed_list {
        let mapped_seeds = map.map(seed);
        minima.push(*mapped_seeds.par_iter().min().unwrap());
    }

    *minima.par_iter().min().unwrap()
}

fn part2(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut lines = file.lines();

    let seed_list = get_range_seeds(&mut lines);

    let map = get_map(&mut lines);

    let mut minima = Vec::new();
    for seed in seed_list {
        let mapped_seeds = map.map(seed);
        minima.push(*mapped_seeds.par_iter().min().unwrap());
    }

    *minima.par_iter().min().unwrap()
}

#[test]
fn part1_example() {
    assert_eq!(35, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(227653707, part1(PART1_FILE));
}

#[test]
fn part2_example() {
    assert_eq!(46, part2("test2.txt"));
}

// #[test]
// fn part2_puzzle() {
//     assert_eq!(78775051, part2(PART2_FILE));
// }
