use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

struct NumberPosition {
    line: usize,
    start: usize,
    end: usize,
}

fn part1(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut num_positions = Vec::new();

    for (line_number, line) in file.lines().enumerate() {
        println!("{:?}", line.unwrap());
        let line = &line.unwrap().as_bytes();
        if line.is_empty() {
            break;
        }

        let line_len = line.len();

        for mut column in 0..line_len {
            match line[column] {
                0x30..=0x39 => {
                    let start = column;
                    column += 1;
                    while line[column] - 0x30 >= 0 && line[column] - 0x39 <= 0 {
                        column += 1;
                    }
                    let end = column;

                    let num = u32::from_str_radix(
                        std::str::from_utf8(&line[start..end]).expect("should work"),
                        10,
                    )
                    .expect("should work");
                    println!("{line_number}: {num}");

                    num_positions.push(NumberPosition {
                        line: line_number,
                        start,
                        end,
                    });
                }
                0x23..=0x26 | 0x2a | 0x2b | 0x2f | 0x3d | 0x40 => {}
                0x2e => {}
                _ => unreachable!("no other chars allowed"),
            }
        }

        // add all numbers adjacent to a symbol that is not .
        // 1. go through all lines
        // 2. for each line, record all numbers and their index in line and of line
        // 3. record all symbols with their index and line
        // 4. make some type of hash map for quick access
        // hash from number index to symbol
        // map.get(line, range) -> bool ; that then in a filter iterator with .sum(); map_reduce?.sum()
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
