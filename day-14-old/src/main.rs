use core::panic;
use std::env;
use transpose::transpose_inplace;

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

struct Array {
    data: Vec<u8>,
    scratch: Vec<u8>,
    empty_tracker: Vec<Option<usize>>,
    dim: usize,
}

impl std::fmt::Debug for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let array: String = self
            .data
            .chunks_exact(self.dim)
            .map(|chunk| {
                let mut s = String::from_utf8(chunk.to_vec()).unwrap();
                s.push('\n');
                s
            })
            .collect();
        write!(f, "{array}")
    }
}

impl Array {
    fn new(file: &str) -> Self {
        let lines: Vec<_> = file.split_ascii_whitespace().collect();
        let width = lines[0].len();
        let height = lines.len();

        if width != height {
            panic!("input must be square!");
        }

        let data = lines.iter().flat_map(|line| line.bytes()).collect();
        let empty_tracker = (0..width).map(|_| None).collect();

        Self {
            data,
            scratch: vec![0; width],
            empty_tracker,
            dim: width,
        }
    }

    fn tilt_north(&mut self) {
        // reset tracker
        self.empty_tracker.iter_mut().for_each(|el| *el = None);

        // go through all rows top to bottom
        for row in 0..self.dim {
            for (col, empty_index) in self.empty_tracker.iter_mut().enumerate() {
                match self.data[self.dim * row + col] {
                    EMPTY => {
                        if empty_index.is_none() {
                            *empty_index = Some(row);
                        }
                    }
                    CUBE => *empty_index = None,
                    ROUND => {
                        if let Some(row_index) = empty_index {
                            self.data[self.dim * row + col] = EMPTY;
                            self.data[self.dim * *row_index + col] = ROUND;

                            *empty_index = (*row_index + 1..row + 1)
                                .find(|i| self.data[self.dim * *i + col] == EMPTY);
                        }
                    }
                    _ => unreachable!("impossible symbol {:?}", self.data[self.dim * row + col]),
                }
            }
        }
    }

    fn count_rounds(&self) -> usize {
        self.data
            .chunks(self.dim)
            .rev()
            .enumerate()
            .fold(0, |sum, (rank, chunk)| {
                sum + (rank + 1) * chunk.iter().filter(|el| **el == ROUND).count()
            })
    }

    fn rotate_clockwise(&mut self) {
        // transposition
        transpose_inplace(&mut self.data, &mut self.scratch, self.dim, self.dim);

        // reverse each row
        self.data
            .chunks_exact_mut(self.dim)
            .for_each(|chunk| chunk.reverse());
    }

    fn rotate(&mut self) {
        // north
        self.tilt_north();
        // west
        self.rotate_clockwise();
        self.tilt_north();
        // south
        self.rotate_clockwise();
        self.tilt_north();
        // east
        self.rotate_clockwise();
        self.tilt_north();
        // rotate back
        self.rotate_clockwise();
    }

    fn rotate_n(&mut self, count: usize) {
        // NOTE: here one could try to identify recurring patterns and break early
        for _ in 0..count {
            self.rotate();
        }
    }
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();

    let mut a = Array::new(&file);

    a.tilt_north();

    a.count_rounds()
}

fn part2(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    let mut a = Array::new(&file);

    // NOTE: 1000 also happens to work instead of 1 billion
    a.rotate_n(1_000);

    a.count_rounds()
}

#[test]
fn part1_example() {
    assert_eq!(136, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(108813, part1(PART1_FILE));
}

#[test]
fn part2_example_1r() {
    let file = std::fs::read_to_string("test1.txt").unwrap();
    let mut a = Array::new(&file);

    a.tilt_north();

    a.rotate_clockwise();
    a.tilt_north();
    a.rotate_clockwise();
    a.rotate_clockwise();
    a.rotate_clockwise();

    a.rotate_clockwise();
    a.rotate_clockwise();
    a.tilt_north();
    a.rotate_clockwise();
    a.rotate_clockwise();

    a.rotate_clockwise();
    a.rotate_clockwise();
    a.rotate_clockwise();
    a.tilt_north();
    a.rotate_clockwise();

    let result = std::fs::read_to_string("test3.txt").unwrap();
    let r = Array::new(&result);

    assert_eq!(r.data, a.data);
}

#[test]
fn part2_example_2r() {
    let result = std::fs::read_to_string("test4.txt").unwrap();
    let r = Array::new(&result);

    let file = std::fs::read_to_string("test1.txt").unwrap();
    let mut a = Array::new(&file);
    a.rotate_n(2);

    assert_eq!(r.data, a.data);
}

#[test]
fn part2_example_3r() {
    let result = std::fs::read_to_string("test5.txt").unwrap();
    let r = Array::new(&result);

    let file = std::fs::read_to_string("test1.txt").unwrap();
    let mut a = Array::new(&file);
    a.rotate_n(3);

    assert_eq!(r.data, a.data);
}

#[test]
fn part2_example() {
    assert_eq!(64, part2("test2.txt"));
}

#[test]
fn part2_puzzle() {
    assert_eq!(104533, part2(PART2_FILE));
}
