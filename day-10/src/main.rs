use std::env;

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";
const WEST: Point = Point { row: 0, col: -1 };
const EAST: Point = Point { row: 0, col: 1 };
const NORTH: Point = Point { row: -1, col: 0 };
const SOUTH: Point = Point { row: 1, col: 0 };

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

#[derive(Clone, Copy, PartialEq, Debug)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row: row as isize,
            col: col as isize,
        }
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
        let row = self
            .chars
            .iter()
            .position(|row| row.iter().position(|c| *c == 'S').is_some())
            .unwrap();

        let col = self.chars[row].iter().position(|c| *c == 'S').unwrap();

        Point::new(row, col)
    }

    fn get(&self, point: &Point) -> Option<&char> {
        if !point.can_be_index() {
            return None;
        }
        self.chars
            .get(point.row())
            .and_then(|row| row.get(point.col()))
    }

    fn get_pipe(&self, point: &Point) -> Option<&char> {
        self.get(point).and_then(|char| match char {
            '|' | '-' | 'L' | 'J' | '7' | 'F' | 'S' => Some(char),
            _ => None,
        })
    }

    fn get_directions(pipe: &char) -> Vec<Point> {
        match pipe {
            'S' => vec![NORTH, SOUTH, EAST, WEST],
            '|' => vec![NORTH, SOUTH],
            '-' => vec![EAST, WEST],
            'L' => vec![NORTH, EAST],
            'J' => vec![NORTH, WEST],
            '7' => vec![SOUTH, WEST],
            'F' => vec![SOUTH, EAST],
            _ => unreachable!("no other chars allowed"),
        }
    }

    fn is_connected(&self, start_point: &Point, pipe: &char, other_point: &Point) -> Option<Point> {
        if let Some(other_pipe) = self.get_pipe(other_point) {
            let other_directions = CharArray::get_directions(other_pipe);

            None
        } else {
            None
        }
    }

    fn find_connected_points(&self, point: &Point) -> Vec<Point> {
        let mut connected_points = vec![];
        if let Some(pipe) = self.get_pipe(point) {
            for direction in CharArray::get_directions(pipe) {
                let other_point = point.move_by(&direction);
                if let Some(other_pipe) = self.get_pipe(&other_point) {
                    let other_directions = CharArray::get_directions(other_pipe);
                    if other_directions
                        .iter()
                        .find(|dir| other_point.move_by(dir) == *point)
                        .is_some()
                    {
                        connected_points.push(other_point);
                    }
                }
            }
        }
        connected_points
    }

    fn get_next(&self, point: &Point) -> Point {
        *self
            .find_connected_points(point)
            .iter()
            .find(|other| *other != point)
            .unwrap()
    }
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    println!("{file}");

    let char_array = CharArray::new(&file);

    let start_point = char_array.find_start();
    println!("{start_point:?}");
    let connected_points = char_array.find_connected_points(&start_point);
    let mut point_1 = connected_points[0];
    let mut point_2 = connected_points[1];

    println!("{point_1:?} - {point_2:?}");

    point_1 = char_array.get_next(&point_1);
    point_2 = char_array.get_next(&point_2);

    // TODO: implement something to make sure we don't move backwards

    println!("{point_1:?} - {point_2:?}");

    let mut steps = 0;
    while point_1 != point_2 {
        point_1 = char_array.get_next(&point_1);
        point_2 = char_array.get_next(&point_2);

        println!("{point_1:?} - {point_2:?}");
        steps += 1;
        break;
    }

    steps
}

fn part2(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();

    0
}

// #[test]
// fn part1_example() {
//     assert_eq!(8, part1("test1.txt"));
// }

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
