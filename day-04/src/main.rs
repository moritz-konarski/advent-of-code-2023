use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-04 p<n>";
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

fn get_common_nums(line: &str) -> usize {
    let (_, numbers) = line.split_once(':').unwrap();
    let (winning_nums, our_nums) = numbers.split_once('|').unwrap();

    let winning_set = winning_nums.split_whitespace().collect::<HashSet<&str>>();
    let our_set = our_nums.split_whitespace().collect::<HashSet<&str>>();

    winning_set.intersection(&our_set).count()
}

fn get_score(line: &str) -> u32 {
    let common_num_count = get_common_nums(line) as u32;
    if common_num_count == 0 {
        0
    } else {
        1 << (common_num_count - 1)
    }
}

fn part1(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    file.lines().fold(0, |sum, line| {
        let line = line.unwrap();

        sum + get_score(&line)
    })
}

fn part2(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut card_copies: HashMap<usize, u32> = HashMap::new();

    file.lines().enumerate().fold(0, |sum, (card_num, line)| {
        let line = line.unwrap();

        let copies_of_this_card = *card_copies.get(&card_num).unwrap_or(&1);

        let common_nums = get_common_nums(&line);
        if common_nums > 0 {
            for index in (card_num + 1)..(card_num + common_nums + 1) {
                *card_copies.entry(index).or_insert(1) += copies_of_this_card;
            }
        }

        sum + copies_of_this_card
    })
}

#[test]
fn part1_example() {
    assert_eq!(13, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(25174, part1(PART1_FILE));
}

#[test]
fn part2_example() {
    assert_eq!(30, part2("test2.txt"));
}

#[test]
fn part2_puzzle() {
    assert_eq!(6420979, part2(PART2_FILE));
}
