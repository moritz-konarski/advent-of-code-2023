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

fn part1(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    // get tuples for lines
    let mut previous_line: (String, Vec<bool>) = (String::new(), Vec::new());
    let mut current_line: (String, Vec<bool>) = (String::new(), Vec::new());
    let mut next_line: Vec<bool> = Vec::new();

    // iter over all lines
    for line in file.lines() {
        current_line.0 = line.unwrap();

        // TODO: remove
        println!("{:?}", current_line.0);

        let mut index = 0;
        while index < current_line.0.len() {
            match current_line.0.as_bytes()[index] {
                PERIOD_AS_BYTE => { /* we ignore periods */ }
                ZERO_AS_BYTE..=NINE_AS_BYTE => {
                    let start = index;
                    index += 1;
                    // while we have digits, increment column
                    while index < current_line.0.len()
                        && current_line.0.as_bytes()[index] >= ZERO_AS_BYTE
                        && current_line.0.as_bytes()[index] <= NINE_AS_BYTE
                    {
                        index += 1;
                    }
                    let end = index;

                    // extract the number
                    let mut num = 0;
                    for (count, index) in (start..end).into_iter().rev().enumerate() {
                        num += 10_u32.pow(count as u32)
                            * (current_line.0.as_bytes()[index] - ZERO_AS_BYTE) as u32;
                    }

                    // TODO: remove
                    println!("{num}");
                }
                _ => {
                    // everything else is treated as a symbol
                    // TODO: remove
                    // each symbol creates 3 three-ranges of legal nums above, below and on the same line
                    println!(
                        "{:?}",
                        char::from_u32(current_line.0.as_bytes()[index] as u32)
                            .expect("cannot fail")
                    );
                }
            }

            index += 1;
            previous_line = current_line.clone();
            current_line.1 = next_line.clone();
            next_line = Vec::with_capacity(next_line.len());
        }
    }
    0
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

// #[test]
// fn part1_puzzle() {
//     assert_eq!(2545, part1(PART1_FILE));
// }

// #[test]
// fn part2_example() {
//     assert_eq!(2286, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(78111, part2(PART2_FILE));
// }
