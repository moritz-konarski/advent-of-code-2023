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

    fn move_by(&self, other: &Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

struct PipeMap {
    chars: Vec<Vec<char>>,
    left_hist: (Option<Point>, Point),
    right_hist: (Option<Point>, Point),
    steps: usize,
}

impl PipeMap {
    fn new(file: &str) -> Self {
        let chars: Vec<Vec<char>> = file
            .split_ascii_whitespace()
            .map(|part| part.chars().collect())
            .collect();

        let start_row = chars
            .iter()
            .position(|row| row.iter().any(|c| *c == 'S'))
            .unwrap();
        let start_col = chars[start_row].iter().position(|c| *c == 'S').unwrap();
        let start = Point::new(start_row, start_col);

        Self {
            chars,
            left_hist: (None, start),
            right_hist: (None, start),
            steps: 0,
        }
    }

    fn get(&self, point: &Point) -> Option<&char> {
        if point.row < 0 || point.col < 0 {
            return None;
        }

        self.chars
            .get(point.row as usize)
            .and_then(|row| row.get(point.col as usize))
    }

    #[inline]
    fn get_connections(pipe: &char) -> Vec<Point> {
        match pipe {
            'S' => vec![NORTH, SOUTH, EAST, WEST],
            '|' => vec![NORTH, SOUTH],
            '-' => vec![EAST, WEST],
            'L' => vec![NORTH, EAST],
            'J' => vec![NORTH, WEST],
            '7' => vec![SOUTH, WEST],
            'F' => vec![SOUTH, EAST],
            _ => vec![],
        }
    }

    fn points_match(&self) -> bool {
        self.left_hist.1 == self.right_hist.1
            || (self.left_hist.1 == self.right_hist.0.unwrap()
                && self.right_hist.1 == self.left_hist.0.unwrap())
    }

    fn get_next_point(&self, hist: (Option<Point>, Point)) -> Point {
        let pipe = self.get(&hist.1).unwrap();
        let dirs = PipeMap::get_connections(pipe);

        let (p1, p2) = (hist.1.move_by(&dirs[0]), hist.1.move_by(&dirs[1]));

        if p1 != hist.0.unwrap() {
            p1
        } else {
            p2
        }
    }

    fn advance_points(&mut self) {
        self.steps += 1;

        let new_left = self.get_next_point(self.left_hist);
        let new_right = self.get_next_point(self.right_hist);

        self.left_hist = (Some(self.left_hist.1), new_left);
        self.right_hist = (Some(self.right_hist.1), new_right);
    }

    fn advance_from_start(&mut self) {
        self.steps += 1;

        let curr = self.left_hist.1;
        let mut points = vec![];

        for direction in PipeMap::get_connections(&'S') {
            let next = curr.move_by(&direction);

            if let Some(next_pipe) = self.get(&next) {
                PipeMap::get_connections(next_pipe).iter().for_each(|dir| {
                    if next.move_by(dir) == curr {
                        points.push(next)
                    }
                });
            }
        }

        self.left_hist = (Some(curr), points[0]);
        self.right_hist = (Some(curr), points[1]);
    }
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();

    let mut char_array = PipeMap::new(&file);
    char_array.advance_from_start();

    while !char_array.points_match() {
        char_array.advance_points();
    }

    char_array.steps
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

#[test]
fn part1_puzzle() {
    assert_eq!(6979, part1(PART1_FILE));
}

// #[test]
// fn part2_example() {
//     assert_eq!(5905, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(250506580, part2(PART2_FILE));
// }
