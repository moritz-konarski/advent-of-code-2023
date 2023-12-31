// use rayon::prelude::*;
use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    env,
};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";
const RIGHT: Vector = Vector { dr: 0, dc: 1 };
const LEFT: Vector = Vector { dr: 0, dc: -1 };
const UP: Vector = Vector { dr: -1, dc: 0 };
const DOWN: Vector = Vector { dr: 1, dc: 0 };
const PATHS_PER_POINT: usize = 2;

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
    dr: isize,
    dc: isize,
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
            dr: self.dr + other.dr,
            dc: self.dc + other.dc,
        }
    }
}

#[derive(Clone)]
struct Path {
    direction: Vector,
    position: Vector,
    visited_positions: HashSet<Vector>,
    has_cycle: bool,
    straight_count: usize,
    length: usize,
}

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path {:?}", self.length)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.length.cmp(&other.length)
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length
    }
}

impl Eq for Path {}

impl Path {
    fn new(direction: Vector, position: Vector) -> Self {
        Self {
            direction,
            position,
            visited_positions: HashSet::from([position]),
            has_cycle: false,
            straight_count: 0,
            length: 0,
        }
    }

    #[inline]
    fn copy(&self, turn: Turn) -> Self {
        Self {
            direction: self.direction.turn(turn),
            position: self.position,
            visited_positions: self.visited_positions.clone(),
            has_cycle: self.has_cycle,
            straight_count: 0,
            length: self.length,
        }
    }

    fn advance(&mut self) -> (usize, usize) {
        let new_pos = self.position + self.direction;
        self.has_cycle |= !self.visited_positions.insert(new_pos);
        self.position = new_pos;
        self.straight_count += 1;

        (self.row(), self.col())
    }

    #[inline]
    fn in_bounds(&self, row_max: isize, col_max: isize) -> bool {
        self.position.dr >= 0
            && self.position.dr < row_max
            && self.position.dc >= 0
            && self.position.dc < col_max
    }

    #[inline]
    fn row(&self) -> usize {
        self.position.dr as usize
    }

    #[inline]
    fn col(&self) -> usize {
        self.position.dc as usize
    }

    #[inline]
    fn turn(&mut self, new_dir: Turn) {
        self.direction = self.direction.turn(new_dir);
        self.straight_count = 0;
    }

    #[inline]
    fn split(&mut self) -> Vec<Self> {
        let left = self.copy(Turn::Left);

        if self.straight_count == 3 {
            self.turn(Turn::Right);
            vec![left]
        } else {
            vec![left, self.copy(Turn::Right)]
        }
    }
}

fn find_hottest_path(mut first_path: Path, city_grid: &[Vec<usize>]) -> usize {
    let (rows, cols) = (city_grid.len(), city_grid[0].len());
    let (r_goal, c_goal) = (rows - 1, cols - 1);

    let mut path_grid = vec![vec![VecDeque::new(); cols]; rows];

    let mut min_path_length: Option<usize> = None;

    path_grid[0][0].push_front(first_path.clone());
    path_grid[0][0].extend(first_path.split());

    let mut is_empty = false;
    while !is_empty {
        is_empty = true;
        for row in 0..rows {
            for col in 0..cols {
                // if row == r_goal && col == c_goal {
                //     println!("{:?}", path_grid[row][col]);
                //     // continue;
                // }

                while let Some(mut path) = path_grid[row][col].pop_front() {
                    is_empty = false;

                    let (new_row, new_col) = path.advance();

                    if !path.has_cycle && path.in_bounds(rows as isize, cols as isize) {
                        path.length += city_grid[new_row][new_col];

                        for next_path in path.split() {
                            if let Some(found_position) = path_grid[new_row][new_col]
                                .iter()
                                .position(|saved_path| next_path < *saved_path)
                            {
                                path_grid[new_row][new_col].insert(found_position, next_path);
                                if path_grid[new_row][new_col].len() >= PATHS_PER_POINT {
                                    path_grid[new_row][new_col].pop_back();
                                }
                            } else if path_grid[new_row][new_col].len() < PATHS_PER_POINT {
                                path_grid[new_row][new_col].push_back(next_path);
                            }
                        }

                        if let Some(pos) = path_grid[new_row][new_col]
                            .iter()
                            .position(|next_path| path < *next_path)
                        {
                            path_grid[new_row][new_col].insert(pos, path);
                            if path_grid[new_row][new_col].len() >= PATHS_PER_POINT {
                                path_grid[new_row][new_col].pop_back();
                            }
                        } else if path_grid[new_row][new_col].len() < PATHS_PER_POINT {
                            path_grid[new_row][new_col].push_back(path);
                        }
                    }
                }
            }
        }

        while let Some(finished_path) = path_grid
            .last_mut()
            .unwrap()
            .last_mut()
            .unwrap()
            .pop_front()
        {
            println!("finishers {:?}", finished_path.length);
            if let Some(minimal_length) = min_path_length {
                min_path_length = Some(minimal_length.min(finished_path.length));
            } else {
                min_path_length = Some(finished_path.length);
            }
        }

        min_path_length
            .is_some()
            .then(|| println!("{:?}", min_path_length.unwrap(),));
    }

    // while !paths.is_empty() {
    //     best_paths_per_point.clear();
    //     let mut new_paths = Vec::with_capacity(rows * cols / 2);

    //     // advance all
    //     paths.par_iter_mut().for_each(|p| p.advance());

    //     // filter out stuff
    //     paths = paths
    //         .par_iter()
    //         .filter_map(|p| {
    //             if !p.has_cycle && p.row() < rows && p.col() < cols && p.in_bounds() {
    //                 Some(p.clone())
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect();

    //     // would be hard to parallelize
    //     paths.iter().for_each(|p| {
    //         if let Some(entry) = best_paths_per_point.get_mut(&(p.row(), p.col())) {
    //             if entry.len() < PATHS_PER_POINT {
    //                 entry.push(p.clone());
    //             } else if let Some(pos) = entry.iter().position(|i| i.length > p.length) {
    //                 entry.insert(pos, p.clone());
    //                 if entry.len() >= PATHS_PER_POINT {
    //                     entry.pop();
    //                 }
    //             }
    //         } else {
    //             best_paths_per_point.insert((p.row(), p.col()), vec![p.clone()]);
    //         }
    //     });

    //     paths.retain_mut(|p| {
    //         let entry = best_paths_per_point.get(&(p.row(), p.col())).unwrap();

    //         if p.position == goal {
    //             if let Some(min) = min_path_length {
    //                 min_path_length = Some(min.min(p.length));
    //             } else {
    //                 min_path_length = Some(p.length);
    //             }
    //             return false;
    //         }

    //         if entry.par_iter().any(|e| p.length <= e.length) {
    //             new_paths.extend(p.split());
    //             true
    //         } else {
    //             false
    //         }
    //     });

    //     // TODO: remove

    // combine new beams with our list
    // paths.par_extend(new_paths);
    // }

    min_path_length.unwrap_or(0)
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    let city_grid: Vec<Vec<usize>> = file
        .split_ascii_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let first_path = Path::new(RIGHT, Vector { dr: 0, dc: 0 });

    // city_grid.iter().for_each(|l| {
    //     l.iter().for_each(|n| print!("{n:?}"));
    //     println!();
    // });

    find_hottest_path(first_path, &city_grid)
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
