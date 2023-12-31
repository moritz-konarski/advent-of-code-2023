use std::{
    collections::{HashMap, HashSet},
    env,
};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";
const RIGHT: Vector = Vector { dr: 0, dc: 1 };
const LEFT: Vector = Vector { dr: 0, dc: -1 };
const UP: Vector = Vector { dr: -1, dc: 0 };
const DOWN: Vector = Vector { dr: 1, dc: 0 };
const PATHS_PER_POINT: usize = 5;

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-17 p<n>";
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

#[derive(Clone, PartialEq)]
struct Path {
    direction: Vector,
    position: Vector,
    visited_positions: HashSet<Vector>,
    has_cycle: bool,
    straight_count: usize,
    length: usize,
}

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

    fn advance(&mut self) {
        let new_pos = self.position + self.direction;
        self.has_cycle |= !self.visited_positions.insert(new_pos);
        self.position = new_pos;
        self.straight_count += 1;
    }

    #[inline]
    fn in_bounds(&self) -> bool {
        self.position.dr >= 0 && self.position.dc >= 0
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
    let goal = Vector {
        dr: rows as isize - 1,
        dc: cols as isize - 1,
    };

    let mut min_path_length: Option<usize> = None;
    let mut paths = Vec::with_capacity(rows * cols / 2);
    let mut new_paths = Vec::with_capacity(rows * cols / 2);
    let mut best_paths_per_point: HashMap<_, Vec<Path>> = HashMap::with_capacity(rows * cols / 2);

    // first basic iteration
    paths.push(first_path.clone());
    paths.extend(first_path.split());

    while !paths.is_empty() {
        best_paths_per_point.clear();

        paths.retain_mut(|p| {
            p.advance();

            // check if we retain
            if !p.has_cycle && p.row() < rows && p.col() < cols && p.in_bounds() {
                // we retain, meaning we can also update be path
                p.length += city_grid[p.row()][p.col()];

                if let Some(entry) = best_paths_per_point.get_mut(&(p.row(), p.col())) {
                    if let Some(pos) = entry.iter().position(|i| i.length > p.length) {
                        entry.insert(pos, p.clone());
                        if entry.len() >= PATHS_PER_POINT {
                            entry.pop();
                        }
                    } else if entry.len() < PATHS_PER_POINT {
                        entry.push(p.clone());
                    }
                } else {
                    best_paths_per_point.insert((p.row(), p.col()), vec![p.clone()]);
                }

                true
            } else {
                false
            }
        });

        new_paths.clear();
        paths.retain_mut(|p| {
            let entry = best_paths_per_point.get(&(p.row(), p.col())).unwrap();

            if entry.contains(p) {
                if p.position == goal {
                    if let Some(min) = min_path_length {
                        min_path_length = Some(min.min(p.length));
                    } else {
                        min_path_length = Some(p.length);
                    }

                    false
                } else {
                    new_paths.extend(p.split());
                    true
                }
            } else {
                false
            }
        });

        // TODO: remove
        min_path_length
            .is_some()
            .then(|| println!("{:?}", min_path_length.unwrap()));

        // combine new beams with our list
        paths.extend_from_slice(&new_paths);
    }

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
