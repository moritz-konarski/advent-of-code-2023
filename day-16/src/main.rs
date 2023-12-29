use rayon::prelude::*;
use std::{collections::HashSet, env};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";
const RIGHT: Vector = Vector { d_row: 0, d_col: 1 };
const LEFT: Vector = Vector {
    d_row: 0,
    d_col: -1,
};
const UP: Vector = Vector {
    d_row: -1,
    d_col: 0,
};
const DOWN: Vector = Vector { d_row: 1, d_col: 0 };

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-16 p<n>";
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

enum Turn {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vector {
    d_row: isize,
    d_col: isize,
}

impl Vector {
    fn turn(&self, new_dir: Turn) -> Self {
        match new_dir {
            Turn::Left => match *self {
                LEFT => DOWN,
                RIGHT => UP,
                UP => LEFT,
                DOWN => RIGHT,
                _ => unreachable!("illegal vector value {:?}", self),
            },
            Turn::Right => match *self {
                LEFT => UP,
                RIGHT => DOWN,
                UP => RIGHT,
                DOWN => LEFT,
                _ => unreachable!("illegal vector value {:?}", self),
            },
        }
    }
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            d_row: self.d_row + other.d_row,
            d_col: self.d_col + other.d_col,
        }
    }
}

#[derive(Clone, Copy)]
struct Beam {
    direction: Vector,
    position: Vector,
}

impl Beam {
    fn new(start_pos: Vector, start_dir: Vector) -> Self {
        Self {
            direction: start_dir,
            position: start_pos,
        }
    }

    fn advance(&mut self) {
        self.position = self.position + self.direction;
    }

    fn in_bounds(&self) -> bool {
        self.position.d_row >= 0 && self.position.d_col >= 0
    }

    fn row(&self) -> usize {
        self.position.d_row as usize
    }

    fn col(&self) -> usize {
        self.position.d_col as usize
    }

    fn turn(&mut self, new_dir: Turn) {
        self.direction = self.direction.turn(new_dir);
    }

    fn split(&mut self) -> Self {
        let new_beam = Self {
            direction: self.direction.turn(Turn::Right),
            position: self.position,
        };

        self.turn(Turn::Left);

        new_beam
    }
}

fn count_tiles(first_beam: Beam, contraption: &[Vec<char>], rows: usize, cols: usize) -> usize {
    let mut beams = vec![first_beam];
    let mut visited_tiles = HashSet::new();
    let mut visited_tiles_counts = vec![0; 5];

    while !beams.is_empty() {
        // advance beams
        beams.iter_mut().for_each(|b| b.advance());

        // filter out beams that are out of bounds
        beams.retain(|b| b.in_bounds() && b.row() < rows && b.col() < cols);

        // collect visited positions
        beams.iter().for_each(|b| {
            visited_tiles.insert(b.position);
        });

        let mut new_beams = vec![];
        // interact with environment
        beams
            .iter_mut()
            .for_each(|b| match (contraption[b.row()][b.col()], b.direction) {
                ('|', LEFT) | ('|', RIGHT) | ('-', UP) | ('-', DOWN) => {
                    let new_beam = b.split();
                    new_beams.push(new_beam);
                }
                ('/', LEFT) | ('/', RIGHT) | ('\\', UP) | ('\\', DOWN) => b.turn(Turn::Left),
                ('/', UP) | ('/', DOWN) | ('\\', LEFT) | ('\\', RIGHT) => b.turn(Turn::Right),
                (_, _) => (),
            });

        // combine new beams with our list
        beams.extend(new_beams);

        // ensure that we don't get stuck in loops
        let visited_count = visited_tiles.len();
        if visited_tiles_counts.iter().all(|e| *e == visited_count) {
            println!("break because equal");
            break;
        }
        visited_tiles_counts.remove(0);
        visited_tiles_counts.push(visited_count);
    }
    visited_tiles.len()
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    let contraption: Vec<Vec<_>> = file
        .split_ascii_whitespace()
        .map(|line| line.chars().collect())
        .collect();
    let (rows, cols) = (contraption.len(), contraption[0].len());

    let first_beam = Beam::new(
        Vector {
            d_row: 0,
            d_col: -1,
        },
        RIGHT,
    );

    count_tiles(first_beam, &contraption, rows, cols)
}

fn part2(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();

    // beam enters (0, 0) coming from (-1, 0)
    // then bounces around the chamber
    // ignores .
    // / and \ are mirrors that reflect the beam 90 degrees
    // flat end of | or - -> two beams that move to where the ends of the splitters are pointing
    // energized tile: at least one beam on top of it

    0
}

#[test]
fn part1_example() {
    assert_eq!(46, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(7210, part1(PART1_FILE));
}

// #[test]
// fn part2_example() {
//     assert_eq!(145, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(295719, part2(PART2_FILE));
// }
