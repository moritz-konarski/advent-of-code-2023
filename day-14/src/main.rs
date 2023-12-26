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
                println!("Sum is {}", part2(PART2_FILE));
            }
            _ => eprintln!("{usage}"),
        }
    } else {
        eprintln!("{usage}");
    }
}

fn tilt_north(lines: &mut [Vec<u8>]) {
    let mut empty_tracker: Vec<Option<usize>> = (0..lines[0].len()).map(|_| None).collect();

    for row in 0..lines.len() {
        for col in 0..lines[0].len() {
            match lines[row][col] {
                EMPTY => {
                    if empty_tracker[col].is_none() {
                        empty_tracker[col] = Some(row);
                    }
                }
                CUBE => empty_tracker[col] = None,
                ROUND => {
                    if let Some(row_index) = empty_tracker[col] {
                        lines[row][col] = EMPTY;
                        lines[row_index][col] = ROUND;

                        empty_tracker[col] =
                            (row_index + 1..row + 1).find(|i| lines[*i][col] == EMPTY);
                    }
                }
                _ => unreachable!("impossible symbol {:?}", lines[row][col]),
            }
        }
    }
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    let mut lines: Vec<_> = file
        .split_ascii_whitespace()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    lines
        .iter()
        .for_each(|l| println!("{}", String::from_utf8(l.clone()).unwrap()));
    println!();

    tilt_north(&mut lines);

    lines
        .iter()
        .for_each(|l| println!("{}", String::from_utf8(l.clone()).unwrap()));

    0
}

fn part2(filename: &str) -> usize {
    // let file = File::open(filename).expect("Should be able to read the file");
    // let file = BufReader::new(file);

    0
}

#[test]
fn part1_example() {
    assert_eq!(136, part1("test1.txt"));
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
//     assert_eq!(78775051, part2(PART2_FILE));
// }
