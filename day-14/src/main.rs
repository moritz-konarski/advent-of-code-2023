use std::env;

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";
const ROUND: u8 = b'O';
const CUBE: u8 = b'#';
const EMPTY: u8 = b'.';

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-14 p<n>";
    if let Some(part) = env::args().nth(1) {
        match part.as_str() {
            "p1" => {
                println!("Reading `{PART1_FILE}`");
                println!("Sum is {}", part1(PART1_FILE));
            }
            "p2" => {
                println!("Reading `{PART2_FILE}`");
                // println!("Sum is {}", part2(PART2_FILE));
                println!("Sum is {}", part2("test2.txt"));
            }
            _ => eprintln!("{usage}"),
        }
    } else {
        eprintln!("{usage}");
    }
}

fn tilt_north(lines: &mut [Vec<u8>]) {
    let mut empty_tracker: Vec<_> = (0..lines[0].len()).map(|_| None).collect();

    for row in 0..lines.len() {
        for (col, empty_index) in empty_tracker.iter_mut().enumerate() {
            match lines[row][col] {
                EMPTY => {
                    if empty_index.is_none() {
                        *empty_index = Some(row);
                    }
                }
                CUBE => *empty_index = None,
                ROUND => {
                    if let Some(row_index) = empty_index {
                        lines[row][col] = EMPTY;
                        lines[*row_index][col] = ROUND;

                        *empty_index = (*row_index + 1..row + 1).find(|i| lines[*i][col] == EMPTY);
                    }
                }
                _ => unreachable!("impossible symbol {:?}", lines[row][col]),
            }
        }
    }
}

fn tilt_south(lines: &mut [Vec<u8>]) {
    let mut empty_tracker: Vec<_> = (0..lines[0].len()).map(|_| None).collect();

    for row in (0..lines.len()).rev() {
        for (col, empty_index) in empty_tracker.iter_mut().enumerate() {
            match lines[row][col] {
                EMPTY => {
                    if empty_index.is_none() {
                        *empty_index = Some(row);
                    }
                }
                CUBE => *empty_index = None,
                ROUND => {
                    if let Some(row_index) = empty_index {
                        lines[row][col] = EMPTY;
                        lines[*row_index][col] = ROUND;

                        *empty_index = (row..*row_index + 1)
                            .rev()
                            .find(|i| lines[*i][col] == EMPTY);
                    }
                }
                _ => unreachable!("impossible symbol {:?}", lines[row][col]),
            }
        }
    }
}

fn tilt_west(lines: &mut [Vec<u8>]) {
    let mut empty_tracker: Vec<_> = (0..lines.len()).map(|_| None).collect();

    for col in 0..lines[0].len() {
        for (row, empty_index) in empty_tracker.iter_mut().enumerate() {
            match lines[row][col] {
                EMPTY => {
                    if empty_index.is_none() {
                        *empty_index = Some(col);
                    }
                }
                CUBE => *empty_index = None,
                ROUND => {
                    if let Some(col_index) = empty_index {
                        lines[row][col] = EMPTY;
                        lines[row][*col_index] = ROUND;

                        *empty_index = (*col_index + 1..col + 1).find(|i| lines[row][*i] == EMPTY);
                    }
                }
                _ => unreachable!("impossible symbol {:?}", lines[row][col]),
            }
        }
    }
}

fn tilt_east(lines: &mut [Vec<u8>]) {
    let mut empty_tracker: Vec<_> = (0..lines.len()).rev().map(|_| None).collect();

    for col in (0..lines[0].len()).rev() {
        for (row, empty_index) in empty_tracker.iter_mut().enumerate() {
            match lines[row][col] {
                EMPTY => {
                    if empty_index.is_none() {
                        *empty_index = Some(col);
                    }
                }
                CUBE => *empty_index = None,
                ROUND => {
                    if let Some(col_index) = empty_index {
                        lines[row][col] = EMPTY;
                        lines[row][*col_index] = ROUND;

                        *empty_index = (col..*col_index + 1)
                            .rev()
                            .find(|i| lines[row][*i] == EMPTY);
                    }
                }
                _ => unreachable!("impossible symbol {:?}", lines[row][col]),
            }
        }
    }
}

fn count_rounds(lines: &[Vec<u8>]) -> usize {
    lines
        .iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (weight, line)| {
            sum + (weight + 1) * line.iter().filter(|sym| **sym == ROUND).count()
        })
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    let mut lines: Vec<_> = file
        .split_ascii_whitespace()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    tilt_north(&mut lines);

    count_rounds(&lines)
}

fn part2(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    let mut lines: Vec<_> = file
        .split_ascii_whitespace()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    for i in 1..1_000_000_000 {
        // for i in 0..3 {
        tilt_north(&mut lines);
        tilt_west(&mut lines);
        tilt_south(&mut lines);
        tilt_east(&mut lines);

        // lines
        //     .iter()
        //     .for_each(|line| println!("{}", String::from_utf8(line.clone()).unwrap()));
        // println!();

        if i % 100 == 0 {
            println!("{:?}", count_rounds(&lines));
        }
    }

    count_rounds(&lines)
}

#[test]
fn part1_example() {
    assert_eq!(136, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(108813, part1(PART1_FILE));
}

#[test]
fn part2_example() {
    assert_eq!(64, part2("test2.txt"));
}

// #[test]
// fn part2_puzzle() {
//     assert_eq!(78775051, part2(PART2_FILE));
// }
