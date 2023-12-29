use std::env;

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";
const RIGHT: Vector = Vector { dx: 1, dy: 0 };
const LEFT: Vector = Vector { dx: -1, dy: 0 };
const UP: Vector = Vector { dx: 0, dy: -1 };
const DOWN: Vector = Vector { dx: 0, dy: 1 };

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

struct Vector {
    dx: isize,
    dy: isize,
}

impl Vector {
    fn x(&self) -> usize {
        self.dx.max(0) as usize
    }

    fn y(&self) -> usize {
        self.dy.max(0) as usize
    }

    // TODO: reflections
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            dx: self.dx + other.dx,
            dy: self.dy + other.dy,
        }
    }
}

struct Beam {
    direction: Vector,
}

impl Beam {
    fn split(self) -> (Self, Self) {}
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();

    0
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

// #[test]
// fn part1_puzzle() {
//     assert_eq!(504036, part1(PART1_FILE));
// }

// #[test]
// fn part2_example() {
//     assert_eq!(145, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(295719, part2(PART2_FILE));
// }
