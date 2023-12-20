use std::env;

use alloc::collections;

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";
const START_CHAR: char = 'S';
const WEST: Point = Point::new(0, -1);
const EAST: Point = Point::new(0, 1);
const NORTH: Point = Point::new(-1, 0);
const SOUTH: Point = Point::new(1, 0);
const DX_DY_VEC: [Point; 4] = [WEST, EAST, NORTH, SOUTH];

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-10 p<n>";
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

#[derive(Clone)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    fn row(&self) -> usize {
        self.row as usize
    }

    fn col(&self) -> usize {
        self.col as usize
    }

    fn move_by(&self, other: &Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }

    fn delta(&self, other: &Self) -> Self {
        Self {
            row: other.row - self.row,
            col: other.col - self.col,
        }
    }

    fn can_be_index(&self) -> bool {
        self.row >= 0 && self.col >= 0
    }
}

struct CharArray {
    chars: Vec<Vec<char>>,
}

impl CharArray {
    fn new(file: &str) -> Self {
        let chars = file
            .split_ascii_whitespace()
            .map(|part| part.chars().collect())
            .collect();

        Self { chars }
    }

    fn find_start(&self) -> Point {
        let start_row = self
            .chars
            .iter()
            .position(|row| row.iter().position(|c| *c == START_CHAR).is_some())
            .unwrap();

        let start_col = self.chars[start_row]
            .iter()
            .position(|c| *c == START_CHAR)
            .unwrap();

        Point::new(start_row as isize, start_col as isize)
    }

    fn get(&self, point: &Point) -> Option<&char> {
        if !point.can_be_index() {
            return None;
        }

        if let Some(row) = self.chars.get(point.row()) {
            row.get(point.col())
        } else {
            None
        }
    }

    fn get_pipe(&self, point: &Point) -> Option<&char> {
        if let Some(char) = self.get(point) {
            match char {
                '|' | '-' | 'L' | 'J' | '7' | 'F' => Some(char),
                _ => None,
            }
        } else {
            None
        }
    }

    fn follow_connection(&self, point: &Point, offset: &Point, pipe: &char) -> Option<Point> {
        match (*pipe, *offset) {
            ('|', NORTH)
            | ('|', SOUTH)
            | ('-', WEST)
            | ('-', EAST)
            | ('L', SOUTH)
            | ('L', WEST)
            | ('J', EAST)
            | ('J', SOUTH)
            | ('7', NORTH)
            | ('7', EAST)
            | ('F', NORTH)
            | ('F', WEST) => Some(point.move_by(offset)),
            _ => None,
        }
    }

    fn find_connected_points(&self, point: &Point) -> Option<Vec<Point>> {
        let adjacent_pipes: Vec<_> = DX_DY_VEC
            .iter()
            .filter_map(|offset| {
                if let Some(pipe) = self.get_pipe(&point.move_by(&offset)) {
                    self.follow_connection(point, offset, pipe)
                } else {
                    None
                }
            })
            .collect();

        if adjacent_pipes.is_empty() {
            return None;
        } else {
            Some(adjacent_pipes)
        }
    }
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    println!("{file}");

    let char_array = CharArray::new(&file);

    let start_point = char_array.find_start();

    let a = 

    // load in complete file
    // find S
    // from S, use two-pointer method to traverse the loop
    // count the steps for each pointer
    // when they meet, return that number

    0
}

fn part2(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();

    0
}

#[test]
fn part1_example() {
    assert_eq!(8, part1("test1.txt"));
}

#[test]
fn part1_example1() {
    assert_eq!(4, part1("test3.txt"));
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
