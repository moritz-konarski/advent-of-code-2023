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
    row: i64,
    col: i64,
}

impl Galaxy {
    fn dist(&self, other: &Self) -> u64 {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

fn find_empty_rows_cols(image: &Vec<Vec<u8>>) -> (Vec<usize>, Vec<usize>) {
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

    (empty_rows, empty_cols)
}

fn parse_galaxies(image: &Vec<Vec<u8>>, distance_factor: i64) -> Vec<Galaxy> {
    let rows: Vec<i64> = (0..image.len())
        .into_iter()
        .scan(0, |offset, row| {
            if image[row].iter().all(|b| *b == EMPTY_SPACE) {
                *offset += distance_factor;
            }
            Some(*offset + row as i64)
        })
        .collect();

    let cols: Vec<i64> = (0..image[0].len())
        .into_iter()
        .scan(0, |offset, col| {
            if image.iter().map(|row| row[col]).all(|b| b == EMPTY_SPACE) {
                *offset += distance_factor;
            }
            Some(*offset + col as i64)
        })
        .collect();

    image
        .iter()
        .zip(&rows)
        .map(|(vec, row)| {
            vec.iter()
                .zip(&cols)
                .filter_map(|(character, col)| {
                    if *character == GALAXY {
                        Some(Galaxy {
                            row: *row,
                            col: *col,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<Galaxy>>()
        })
        .flatten()
        .collect()
}

fn sum_galaxy_distances(galaxies: &Vec<Galaxy>) -> u64 {
    let mut sum = 0;
    for (i1, g1) in galaxies.iter().enumerate() {
        for g2 in &galaxies[i1..] {
            sum += g1.dist(g2);
        }
    }
    sum
}

fn part1(filename: &str) -> u64 {
    let file = std::fs::read_to_string(filename).unwrap();

    // read image
    let image: Vec<Vec<u8>> = file
        .split_ascii_whitespace()
        .map(|row| row.as_bytes().to_vec())
        .collect();

    let galaxies = parse_galaxies(&image, 1);

    sum_galaxy_distances(&galaxies)
}

fn part2(filename: &str) -> u64 {
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
                        Some(Galaxy {
                            row: row as i64,
                            col: col as i64,
                        })
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

#[test]
fn part1_example() {
    assert_eq!(374, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(9312968, part1(PART1_FILE));
}

#[test]
fn part2_example() {
    assert_eq!(5905, part2("test2.txt"));
}

// #[test]
// fn part2_puzzle() {
//     assert_eq!(250506580, part2(PART2_FILE));
// }
