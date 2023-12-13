use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PERIOD_AS_BYTE: u8 = 0x2E;
const ZERO_AS_BYTE: u8 = 0x30;
const NINE_AS_BYTE: u8 = 0x39;
const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-03 p<n>";
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

fn process_previous_line(line: &(Vec<u8>, Vec<bool>), line_len: usize) -> u32 {
    let mut num_counts = false;
    let mut sum = 0;
    let mut i = 0;

    while i < line_len {
        if !(ZERO_AS_BYTE..=NINE_AS_BYTE).contains(&line.0[i]) {
            i += 1;
            continue;
        }

        let start = i;
        while i < line_len && line.0[i] >= ZERO_AS_BYTE && line.0[i] <= NINE_AS_BYTE {
            num_counts |= line.1[i];
            i += 1;
        }
        let end = i;

        if num_counts {
            num_counts = false;
            for (count, ii) in (start..end).into_iter().rev().enumerate() {
                sum += 10_u32.pow(count as u32) * (line.0[ii] - ZERO_AS_BYTE) as u32;
            }
        }
    }

    return sum;
}

fn part1(filename: &str) -> u32 {
    // this is really dirty, but it comes in handy
    let file = BufReader::new(File::open(filename).expect("Should be able to read the file"));
    let line_len = file.lines().next().unwrap().unwrap().len();
    let file = BufReader::new(File::open(filename).expect("Should be able to read the file"));

    let mut previous_line = (vec![0; line_len], vec![false; line_len]);
    let mut current_line = (vec![0; line_len], vec![false; line_len]);
    let mut next_line = vec![false; line_len];

    let mut sum = 0;

    for line in file.lines() {
        current_line.0 = line.unwrap().as_bytes().to_vec();

        for (i, c) in current_line.0.iter().enumerate() {
            if *c == PERIOD_AS_BYTE || (ZERO_AS_BYTE..=NINE_AS_BYTE).contains(c) {
                continue;
            }

            for ii in 0.max(i - 1)..line_len.min(i + 2) {
                previous_line.1[ii] |= true;
                current_line.1[ii] |= true;
                next_line[ii] |= true;
            }
        }

        sum += process_previous_line(&previous_line, line_len);

        previous_line = current_line.to_owned();
        current_line.1 = next_line;
        next_line = vec![false; line_len];
    }

    sum + process_previous_line(&previous_line, line_len)
}

fn part2(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    file.lines().fold(0, |sum, line| {
        let l = line.unwrap();
        if l.is_empty() {
            return sum;
        }

        sum + 1
    })
}

#[test]
fn part1_example() {
    assert_eq!(4361, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(550064, part1(PART1_FILE));
}

// #[test]
// fn part2_example() {
//     assert_eq!(2286, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(78111, part2(PART2_FILE));
// }
