use std::{
    collections::{btree_map::Entry, HashMap, HashSet},
    env,
};

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
    let usage = "Incorrect arguements!\nUsage: day-17 p<n>";
    if let Some(part) = env::args().nth(1) {
        match part.as_str() {
            "p1" => {
                println!("Reading `{PART1_FILE}`");
                println!("Sum is {}", part1("test1.txt"));
                // println!("Sum is {}", part1(PART1_FILE));
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

#[derive(Clone, PartialEq)]
struct Path {
    direction: Vector,
    position: Vector,
    visited_positions: Vec<Vector>,
    has_cycle: bool,
    straight_count: usize,
    length: usize,
}

impl Path {
    fn new(start_pos: Vector, start_dir: Vector) -> Self {
        Self {
            direction: start_dir,
            position: start_pos,
            visited_positions: vec![start_pos],
            has_cycle: false,
            straight_count: 0,
            length: 0,
        }
    }

    fn advance(&mut self) {
        let new_pos = self.position + self.direction;
        self.has_cycle |= self.visited_positions.contains(&new_pos);
        self.visited_positions.push(new_pos);
        self.position = new_pos;
        self.straight_count += 1;
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
        self.straight_count = 0;
    }

    fn split(&mut self) -> Vec<Self> {
        let left = Self {
            direction: self.direction.turn(Turn::Left),
            position: self.position,
            visited_positions: self.visited_positions.clone(),
            has_cycle: self.has_cycle,
            straight_count: 0,
            length: self.length,
        };

        if self.straight_count == 2 {
            self.turn(Turn::Right);
            return vec![left];
        } else {
            let right = Self {
                direction: self.direction.turn(Turn::Right),
                position: self.position,
                visited_positions: self.visited_positions.clone(),
                has_cycle: self.has_cycle,
                straight_count: 0,
                length: self.length,
            };
            return vec![left, right];
        }
    }
}

fn find_hottest_path(first_path: Path, city_grid: &[Vec<u8>], rows: usize, cols: usize) -> usize {
    let mut paths = vec![first_path];
    let other_paths = paths[0].split();
    paths.extend(other_paths);

    let mut best_points: HashMap<(usize, usize), Path> = HashMap::new();

    let mut min_path_length = usize::MAX;
    let mut count = 0;

    while !paths.is_empty() {
        println!("{:?}\n  {:?}", count, paths.len());

        // advance beams
        paths.iter_mut().for_each(|b| b.advance());

        // filter out beams that are out of bounds
        paths.retain(|b| b.in_bounds() && b.row() < rows && b.col() < cols && !b.has_cycle);

        let mut new_beams = vec![];
        // interact with environment
        best_points.clear();
        paths.iter_mut().for_each(|b| {
            b.length += city_grid[b.row()][b.col()] as usize;

            if let Some(entry) = best_points.get_mut(&(b.row(), b.col())) {
                if b.length < entry.length {
                    *entry = b.clone();
                }
            } else {
                best_points.insert((b.row(), b.col()), b.clone());
            }
        });

        paths.retain(|b| best_points.values().any(|v| *v == *b));

        // for r in 0..rows {
        //     for c in 0..cols {
        //         if best_points.contains_key(&(r, c)) {
        //             print!("*");
        //         } else {
        //             print!("_");
        //         }
        //     }
        //     println!();
        // }

        paths.iter_mut().for_each(|b| {
            if b.row() == rows - 1 && b.col() == cols - 1 {
                if b.length < min_path_length {
                    // TODO: print min path to end for checking
                }
                min_path_length = min_path_length.min(b.length);

                for r in 0..rows {
                    for c in 0..cols {
                        if best_points.contains_key(&(r, c)) {
                            print!("*");
                        } else {
                            print!("_");
                        }
                    }
                    println!();
                }
                path_lengths.push(b.length);
            } else {
                new_beams.extend(b.split());
            }
        });

        if let Some(min) = path_lengths.iter().min() {
            println!("  {min:?}");
        }
        // combine new beams with our list
        paths.extend(new_beams);

        // retain only the shortest path to each point with a path on it
        count += 1;
    }

    *path_lengths.iter().min().unwrap()
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    let city_grid: Vec<Vec<u8>> = file
        .split_ascii_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    let (rows, cols) = (city_grid.len(), city_grid[0].len());

    let first_beam = Path::new(Vector { d_row: 0, d_col: 0 }, RIGHT);

    city_grid.iter().for_each(|l| {
        l.iter().for_each(|n| print!("{n:?}"));
        println!();
    });

    find_hottest_path(first_beam, &city_grid, rows, cols)
}

fn part2(filename: &str) -> usize {
    0
}

#[test]
fn part1_example() {
    assert_eq!(102, part1("test1.txt"));
}

// #[test]
// fn part1_puzzle() {
//     assert_eq!(7210, part1(PART1_FILE));
// }

// #[test]
// fn part2_example() {
//     assert_eq!(51, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(7673, part2(PART2_FILE));
// }
