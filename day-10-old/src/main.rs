use std::{collections::HashSet, env};

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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

fn get_pipe_deltas(pipe: char) -> (Point, Point) {
    match pipe {
        '|' => (NORTH, SOUTH),
        '-' => (EAST, WEST),
        'L' => (NORTH, EAST),
        'J' => (NORTH, WEST),
        '7' => (SOUTH, WEST),
        'F' => (SOUTH, EAST),
        _ => unreachable!("impossible pipe: {}", pipe),
    }
}

fn is_pipe(pipe: char) -> bool {
    match pipe {
        '|' | 'L' | 'J' | '7' | 'F' | 'S' | '-' => true,
        _ => false,
    }
}

struct PipeMap {
    points: Vec<Vec<char>>,
    trace_a: Vec<Point>,
    trace_b: Vec<Point>,
    point_set: HashSet<Point>,
    step_count: usize,
}

impl PipeMap {
    fn new(file: &str) -> Self {
        println!("{file}");
        let points: Vec<Vec<_>> = file
            .split_ascii_whitespace()
            .map(|part| part.chars().collect())
            .collect();

        let start = points.iter().flatten().position(|c| *c == 'S').unwrap();
        let row = start / points.first().unwrap().len();
        let col = start % points.first().unwrap().len();
        let start = Point::new(row, col);

        let mut point_set = HashSet::new();
        point_set.insert(start);

        let trace_a = vec![start];
        let trace_b = vec![start];

        Self {
            points,
            trace_a,
            trace_b,
            point_set,
            step_count: 0,
        }
    }

    fn points_match(&self) -> bool {
        let curr_a = self.trace_a.last().unwrap();
        let curr_b = self.trace_b.last().unwrap();
        if curr_a == curr_b {
            return true;
        }

        let prev_a = self.trace_a.get(self.trace_a.len() - 2).unwrap();
        let prev_b = self.trace_b.get(self.trace_b.len() - 2).unwrap();

        curr_a == prev_b && curr_b == prev_a
    }

    fn get_next_point(&self, trace: &[Point]) -> Point {
        let curr_p = trace.last().unwrap();
        let curr_sym = self.get_pipe(curr_p).unwrap();
        let (d1, d2) = get_pipe_deltas(curr_sym);

        if *curr_p + d1 != *trace.get(trace.len() - 2).unwrap() {
            *curr_p + d1
        } else {
            *curr_p + d2
        }
    }

    fn advance_points(&mut self) {
        self.step_count += 1;

        let new_a = self.get_next_point(&self.trace_a);
        self.point_set.insert(new_a);
        self.trace_a.push(new_a);

        let new_b = self.get_next_point(&self.trace_b);
        self.point_set.insert(new_b);
        self.trace_b.push(new_b);
    }

    fn get_pipe(&self, point: &Point) -> Option<char> {
        if point.row < 0 || point.col < 0 {
            None
        } else {
            let sym = self.points[point.row as usize][point.col as usize];
            match sym {
                '|' | '-' | 'L' | 'J' | '7' | 'F' => Some(sym),
                _ => None,
            }
        }
    }

    fn advance_from_start(&mut self) {
        self.step_count += 1;

        let start = *self.trace_a.last().unwrap();

        let next_points: Vec<Point> = [NORTH, SOUTH, EAST, WEST]
            .iter()
            .filter_map(|delta| {
                let next_p = start + *delta;
                if let Some(next_sym) = self.get_pipe(&next_p) {
                    let (d1, d2) = get_pipe_deltas(next_sym);
                    (next_p + d1 == start || next_p + d2 == start).then_some(next_p)
                } else {
                    None
                }
            })
            .collect();

        let (new_a, new_b) = (next_points[0], next_points[1]);

        self.point_set.insert(new_a);
        self.trace_a.push(new_a);

        self.point_set.insert(new_b);
        self.trace_b.push(new_b);
    }

    fn count_points_in_loop(&self) -> usize {
        self.points.iter().enumerate().fold(0, |sum, (row, vec)| {
            let s = vec[1..vec.len() - 1]
                .iter()
                .enumerate()
                .scan(
                    (false, 0),
                    |(last_was_vertical, crossed_pipes), (col, character)| {
                        if last_was_vertical {
                            print!("|");
                            *crossed_pipes += 1;
                            Some(0)
                        } else {
                            match character {
                                '-' => { /* don't increment */ }
                                _ => {
                                    print!("|");
                                    *crossed_pipes += 1;
                                    Some(0)
                                }
                            }
                        }
                        let p = Point::new(row, col);
                        if self.point_set.contains(&p) {
                            // if is_vertical_pipe(*character) {
                            print!("|");
                            *crossed_pipes += 1;
                            Some(0)
                            // } else {
                            //     print!("-");
                            //     Some(0)
                            // }
                        } else if *crossed_pipes % 2 != 0 {
                            print!("+");
                            Some(1)
                        } else {
                            print!(".");
                            Some(0)
                        }
                    },
                )
                .sum::<usize>();
            print!("\n");
            sum + s
        })
    }
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();

    let mut char_array = PipeMap::new(&file);
    char_array.advance_from_start();

    while !char_array.points_match() {
        char_array.advance_points();
    }

    char_array.step_count
}

fn part2(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();

    let mut char_array = PipeMap::new(&file);
    char_array.advance_from_start();

    while !char_array.points_match() {
        char_array.advance_points();
    }

    char_array.count_points_in_loop()
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

#[test]
fn part2_example() {
    assert_eq!(4, part2("test2.txt"));
}

#[test]
fn part2_example1() {
    assert_eq!(8, part2("test4.txt"));
}

// #[test]
// fn part2_puzzle() {
//     assert_eq!(250506580, part2(PART2_FILE));
// }
