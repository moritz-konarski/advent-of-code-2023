use std::collections::HashMap;
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

// TODO: decide on one approach and stick to it

struct ParseLine {
    text: &str,

}

impl ParseLine {
    fn new() -> Self;
}

struct ThreeLines {
    previous_line: ParseLine;
    current_line: ParseLine;
    next_line: ParseLine;
}

impl ThreeLines {
    fn new() -> Self {
        
    }
    
}

fn part1(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut previous_line: (&str, &[bool]) = ("", &vec![]);
    let mut current_line: (&str, &[bool]) = ("", &vec![]);
    let mut next_line: (&str, &[bool]) = ("", &vec![]);

    for (line_number, line) in file.lines().enumerate() {
        let line = line.as_ref().unwrap().as_bytes();

        // TODO: remove
        println!(
            "Line {line_number}: {:?}",
            std::str::from_utf8(&line).expect("should work")
        );
        if line.is_empty() {
            break;
        }

        let line_len = line.len();
        let mut column = 0;
        while column < line_len {
            match line[column] {
                PERIOD_AS_BYTE => { /* we ignore periods */ }
                ZERO_AS_BYTE..=NINE_AS_BYTE => {
                    // all numbers need to be processed

                    let start = column;
                    column += 1;
                    // while we have digits, increment column
                    while column < line_len
                        && line[column] >= ZERO_AS_BYTE
                        && line[column] <= NINE_AS_BYTE
                    {
                        column += 1;
                    }
                    let end = column;

                    // extract the number
                    let mut num = 0;
                    for (count, index) in (start..end).into_iter().rev().enumerate() {
                        num += 10_u32.pow(count as u32) * (line[index] - ZERO_AS_BYTE) as u32;
                    }

                    // TODO: remove
                    println!("{num}");

                    num_positions.push(NumberPosition {
                        num,
                        line: line_number,
                        start,
                        end,
                    });
                }
                _ => {
                    // everything else is treated as a symbol
                    // TODO: remove
                    // each symbol creates 3 three-ranges of legal nums above, below and on the same line
                    println!(
                        "{:?}",
                        char::from_u32(line[column] as u32).expect("cannot fail")
                    );
                    sym_positions.push(SymbolPosition {
                        line: line_number,
                        position: column,
                    });
                }
            }

            column += 1;
        }
    }

    let map = SymbolMap::from_positions(sym_positions);

    num_positions.into_iter().fold(0, |sum, pos| {
        if let Some(num) = map.get(&pos) {
            sum + num
        } else {
            sum
        }
    })
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
