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

#[derive(Debug)]
struct SeedRange {
    dst_start: usize,
    src_start: usize,
    len: usize,
}

impl SeedRange {
    fn new(dst_start: usize, src_start: usize, len: usize) -> Self {
        Self {
            dst_start,
            src_start,
            len,
        }
    }

    fn get(&self, index: usize) -> usize {
        self.dst_start + index - self.src_start
    }

    fn contains(&self, other: &Self) -> bool {
        self.src_start <= other.src_start
            && other.src_start + other.len <= self.src_start + self.len
    }

    fn contains_point(&self, point: usize) -> bool {
        self.src_start <= point && self.src_start + self.len > point
    }

    fn is_contained_in(&self, other: &Self) -> bool {
        other.contains(self)
    }

    fn intersects(&self, other: &Self) -> bool {
        let self_then_other = other.contains_point(self.src_start + self.len)
            && !other.contains_point(self.src_start);
        let other_then_self = self.contains_point(other.src_start + other.len)
            && !other.contains_point(other.src_start);
        self_then_other || other_then_self
    }

    fn partition(self, pivots: Vec) -> Vec<Self> {
        let mut ranges = Vec::new();

        let mut src_start = self.src_start;
        let mut dst_start = self.dst_start;
        let end = src_start + self.len;

        for pivot in pivots {
            ranges.push(Self {
                dst_start,
                src_start,
                len: pivot - src_start,
            });
            src_start = pivot;
            dst_start = dst_start + pivot - src_start
        }

        ranges.push(Self {
            dst_start,
            src_start,
            len: end - src_start,
        });

        ranges
    }

    fn get_seeds(lines: &mut Lines<BufReader<File>>) -> BTreeMap<usize, Self> {
        let line = lines.next().unwrap().unwrap();
        let (_, seeds) = line.split_once(':').unwrap();

        let seeds: Vec<usize> = seeds
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut map = BTreeMap::new();
        for seed in seeds {
            map.insert(seed, Self::new(seed, seed, 1));
        }

        map
    }

    fn get_range_seeds(lines: &mut Lines<BufReader<File>>) -> BTreeMap<usize, Self> {
        let line = lines.next().unwrap().unwrap();
        let (_, seeds) = line.split_once(':').unwrap();

        let seeds: Vec<usize> = seeds
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let seeds: Vec<(usize, usize)> = seeds
            .chunks_exact(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();

        let mut map = BTreeMap::new();
        for (seed, len) in seeds {
            map.insert(seed, Self::new(seed, seed, len));
        }

        map
    }
}

#[derive(Debug)]
struct RangeMap {
    ranges: BTreeMap<usize, SeedRange>,
}

impl RangeMap {
    fn new() -> Self {
        Self {
            ranges: BTreeMap::new(),
        }
    }

    fn parse_line(&mut self, line: &str) {
        let elements: Vec<usize> = line.splitn(3, ' ').map(|s| s.parse().unwrap()).collect();
        let dst_start = elements[0];
        let src_start = elements[1];
        let len = elements[2];

        self.ranges
            .insert(src_start, SeedRange::new(dst_start, src_start, len));
    }

    fn get(&self, seed: SeedRange) -> Vec<SeedRange> {
        let candidates = self
            .ranges
            .iter()
            .filter(|(_, range)| {
                seed.intersects(range) || seed.contains(range) || seed.is_contained_in(range)
            })
            .collect::<Vec<_>>();

        println!("{seed:?}: {candidates:?}");

        let ranges = Vec::new();
        for (index, range) in candidates {
            if seed.intersects(range) {
                if seed.src_start < range.src_start
                /* end of seed in range */
                {
                } else {
                }
            } else if seed.contains(range) {
            } else
            /* seed is_contained_in range */
            {
            }
        }

        ranges
    }
}

fn iterate_seed_list(
    lines: &mut Lines<BufReader<File>>,
    seed_list: BTreeMap<usize, SeedRange>,
) -> BTreeMap<usize, SeedRange> {
    let mut seed_list = seed_list;

    while let Some(Ok(mut line)) = lines.next() {
        // skip lines that indicate a description
        if line.ends_with(':') || line.is_empty() {
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
        println!("{line}");
        let mut list = BTreeMap::new();
        for (_, seed) in seed_list {
            let seeds = map.get(seed);

            for s in seeds {
                list.insert(s.src_start, s);
            }
        }
        seed_list = list;
    }

    seed_list
}

fn part1(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut lines = file.lines();

    // extract seeds and consume that line
    let seed_list = SeedRange::get_seeds(&mut lines);
    println!("SeedList {seed_list:?}");

    // get final seed list
    let final_seeds = iterate_seed_list(&mut lines, seed_list);

    // find smallest value
    // *final_seeds.keys().next().unwrap() as u32
    0
}

fn part2(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut lines = file.lines();

    // extract seeds and consume that line
    let seed_list = SeedRange::get_range_seeds(&mut lines);

    // get final seed list
    let final_seeds = iterate_seed_list(&mut lines, seed_list);

    // find smallest value
    *final_seeds.keys().next().unwrap() as u32
}

#[test]
fn part1_example() {
    assert_eq!(35, part1("test1.txt"));
}

// #[test]
// fn part1_puzzle() {
//     assert_eq!(227653707, part1(PART1_FILE));
// }

// #[test]
// fn part2_example() {
//     assert_eq!(46, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(6420979, part2(PART2_FILE));
// }
