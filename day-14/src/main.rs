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
    let mut empty_index: Option<usize>;

    for col in 0..lines[0].len() {
        empty_index = None;

        for row in 0..lines.len() {
            // TODO: make 2nd match where empty_index is incremented separately

            match lines[row][col] {
                EMPTY => {
                    if empty_index.is_none() {
                        empty_index = Some(row);
                    }
                }
                CUBE => {
                    empty_index = None;
                }
                ROUND => {
                    if let Some(index) = empty_index {
                        lines[row][col] = EMPTY;
                        lines[index][col] = ROUND;

                        empty_index = Some(row);
                    }
                }
                _ => unreachable!("no other shapes"),
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
    // O moves, . is empty, # does not move
    // calculate load for O as len() - row for each O

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
