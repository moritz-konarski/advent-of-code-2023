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
    fn new() -> Self {
        Self {
            direction: RIGHT,
            position: Vector {
                d_row: 0,
                d_col: -1,
            },
        }
    }

    fn advance(&mut self) {
        self.position = self.position + self.direction;
    }

    fn is_negative(&self) -> bool {
        self.position.d_row < 0 || self.position.d_col < 0
    }

    fn row(&self) -> usize {
        self.position.d_row as usize
    }

    fn col(&self) -> usize {
        self.position.d_col as usize
    }

    fn turn(self, new_dir: Turn) -> Option<Vec<Self>> {
        let new_self = Self {
            direction: self.direction.turn(new_dir),
            position: self.position,
        };

        Some(vec![new_self])
    }

    fn split(self) -> Option<Vec<Self>> {
        let left = Beam {
            direction: self.direction.turn(Turn::Left),
            position: self.position,
        };
        let right = Beam {
            direction: self.direction.turn(Turn::Right),
            position: self.position,
        };

        Some(vec![left, right])
    }
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    let contraption: Vec<Vec<_>> = file
        .split_ascii_whitespace()
        .map(|line| line.chars().collect())
        .collect();
    let (row_count, col_count) = (contraption.len(), contraption[0].len());

    let mut beams = vec![Beam::new()];
    let mut visited_tiles = HashSet::new();

    let mut last_visited_counts = vec![0; 10];

    while !beams.is_empty() {
        beams = beams
            .iter_mut()
            .filter_map(|beam| {
                beam.advance();

                // remove a beam if it is out of bounds
                if beam.is_negative() || beam.row() >= row_count || beam.col() >= col_count {
                    return None;
                }

                // track the tile as visited
                visited_tiles.insert(beam.position);

                // interact with environment
                match (contraption[beam.row()][beam.col()], beam.direction) {
                    ('|', LEFT) | ('|', RIGHT) => beam.split(),
                    ('-', UP) | ('-', DOWN) => beam.split(),
                    ('/', LEFT) | ('/', RIGHT) => beam.turn(Turn::Left),
                    ('/', UP) | ('/', DOWN) => beam.turn(Turn::Right),
                    ('\\', LEFT) | ('\\', RIGHT) => beam.turn(Turn::Right),
                    ('\\', UP) | ('\\', DOWN) => beam.turn(Turn::Left),
                    (_, _) => Some(vec![*beam]),
                }
            })
            .flatten()
            .collect();

        let visited_count = visited_tiles.len();
        if last_visited_counts.iter().all(|e| *e == visited_count) {
            break;
        }
        last_visited_counts.remove(0);
        last_visited_counts.push(visited_count);
    }

    visited_tiles.len()
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
