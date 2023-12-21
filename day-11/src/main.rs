use std::env;

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";
const EMPTY_SPACE: u8 = b'.';
const GALAXY: u8 = b'#';

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-11 p<n>";
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

struct Galaxy {
    row: isize,
    col: isize,
}

impl Galaxy {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row: row as isize,
            col: col as isize,
        }
    }

    fn dist(&self, other: &Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();

    // read image
    let mut image: Vec<Vec<u8>> = file
        .split_ascii_whitespace()
        .map(|row| row.as_bytes().to_vec())
        .collect();

    let (rows, cols) = (image.len(), image[0].len());
    // find empty rows
    let mut empty_rows = vec![];
    for row in 0..rows {
        if image[row].iter().all(|b| *b == EMPTY_SPACE) {
            empty_rows.push(row);
        }
    }
    // find empty cols
    let mut empty_cols = vec![];
    for col in 0..cols {
        if image.iter().map(|row| row[col]).all(|b| b == EMPTY_SPACE) {
            empty_cols.push(col);
        }
    }

    // add empty cols
    for row in 0..rows {
        for col in empty_cols.iter().rev() {
            image[row].insert(*col, EMPTY_SPACE);
        }
    }
    // add empty rows
    for row in empty_rows.iter().rev() {
        image.insert(*row, vec![]);
    }

    let galaxies: Vec<Galaxy> = image
        .iter()
        .enumerate()
        .map(|(row, vec)| {
            vec.iter()
                .enumerate()
                .filter_map(|(col, byte)| {
                    if *byte == GALAXY {
                        Some(Galaxy::new(row, col))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let mut sum = 0;
    for (i1, g1) in galaxies.iter().enumerate() {
        for g2 in &galaxies[i1..] {
            sum += g1.dist(g2);
        }
    }

    sum
}

fn part2(filename: &str) -> usize {
    0
}

#[test]
fn part1_example() {
    assert_eq!(374, part1("test1.txt"));
}

// #[test]
// fn part1_puzzle() {
//     assert_eq!(250058342, part1(PART1_FILE));
// }

// #[test]
// fn part2_example() {
//     assert_eq!(5905, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(250506580, part2(PART2_FILE));
// }
