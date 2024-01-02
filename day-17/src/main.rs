use std::{
    collections::{BTreeMap, HashSet},
    env,
};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";
const RIGHT: Vector = Vector { row: 0, col: 1 };
const LEFT: Vector = Vector { row: 0, col: -1 };
const UP: Vector = Vector { row: -1, col: 0 };
const DOWN: Vector = Vector { row: 1, col: 0 };

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

enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vector {
    row: isize,
    col: isize,
}

impl Vector {
    fn change_direction(&mut self, new_direction: Direction) {
        *self = match (new_direction, *self) {
            (Direction::Left, LEFT) => DOWN,
            (Direction::Left, RIGHT) => UP,
            (Direction::Left, UP) => LEFT,
            (Direction::Left, DOWN) => RIGHT,
            (Direction::Right, LEFT) => UP,
            (Direction::Right, RIGHT) => DOWN,
            (Direction::Right, UP) => RIGHT,
            (Direction::Right, DOWN) => LEFT,
            _ => *self, // it's not a unit vector, so we ignore it
        }
    }

    fn add(&mut self, other: &Self) {
        self.row += other.row;
        self.col += other.col;
    }
}

#[derive(Clone)]
struct Path {
    direction: Vector,
    position: Vector,
    straight_count: usize,
    heat_loss: usize,
}

impl std::hash::Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.direction.hash(state);
        self.position.hash(state);
        self.straight_count.hash(state);
    }
}

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path {:?}", self.heat_loss)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat_loss.cmp(&other.heat_loss)
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.heat_loss == other.heat_loss
    }
}

impl Eq for Path {}

impl Path {
    fn new(direction: Vector, position: Vector) -> Self {
        Self {
            direction,
            position,
            straight_count: 0,
            heat_loss: 0,
        }
    }

    fn copy_and_turn(&self, direction: Direction) -> Self {
        let new_path = self.clone();
        new_path.turn(direction)
    }

    fn take_step(&mut self) {
        self.position.add(&self.direction);
        self.straight_count += 1;
    }

    fn get_pos(&self) -> Option<(usize, usize)> {
        if self.position.row >= 0 && self.position.col >= 0 {
            Some((self.position.row as usize, self.position.col as usize))
        } else {
            None
        }
    }

    fn turn(mut self, direction: Direction) -> Self {
        self.direction.change_direction(direction);
        self.straight_count = 0;
        self
    }

    fn split(self) -> Vec<Self> {
        if self.straight_count == 3 {
            vec![
                self.copy_and_turn(Direction::Left),
                self.turn(Direction::Right),
            ]
        } else {
            vec![
                self.copy_and_turn(Direction::Left),
                self.copy_and_turn(Direction::Right),
                self,
            ]
        }
    }
}

struct PathMap {
    map: BTreeMap<usize, Vec<Path>>,
}

impl PathMap {
    fn new(start_path: Path) -> Self {
        Self {
            map: BTreeMap::from([(start_path.heat_loss, vec![start_path])]),
        }
    }

    fn insert(&mut self, path: Path) {
        self.map
            .entry(path.heat_loss)
            .or_insert(Vec::with_capacity(1_000))
            .push(path);
    }

    fn pop(&mut self) -> Option<Path> {
        if let Some(mut entry) = self.map.first_entry() {
            if let Some(path) = entry.get_mut().pop() {
                if entry.get().is_empty() {
                    entry.remove_entry();
                }
                Some(path)
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn find_hottest_path(start_path: Path, block_heat_loss: &[Vec<usize>]) -> usize {
    let (rows, cols) = (block_heat_loss.len(), block_heat_loss[0].len());
    let goal = Vector {
        row: rows as isize - 1,
        col: cols as isize - 1,
    };

    let mut seen_paths = HashSet::with_capacity(rows * cols);
    let mut seen_lengths = HashSet::with_capacity(1_000);
    let mut paths = PathMap::new(start_path);

    while let Some(path) = paths.pop() {
        if path.position == goal {
            return path.heat_loss;
        }

        if !seen_paths.insert(path.clone()) {
            continue;
        }

        if seen_lengths.insert(path.heat_loss) {
            println!("{:?}", path.heat_loss);
        }

        for mut new_path in path.split() {
            new_path.take_step();

            if let Some((row, col)) = new_path.get_pos() {
                if row >= rows || col >= cols {
                    continue;
                }

                new_path.heat_loss += block_heat_loss[row][col];

                paths.insert(new_path);
            }
        }
    }

    unreachable!("there is always a path");
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    let heat_loss_grid: Vec<Vec<usize>> = file
        .split_ascii_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let start = Vector { row: 0, col: 0 };
    let start_path = Path::new(RIGHT, start);

    find_hottest_path(start_path, &heat_loss_grid)
}

fn part2(filename: &str) -> usize {
    0
}

#[test]
fn part1_example() {
    assert_eq!(102, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(1260, part1(PART1_FILE));
}

// #[test]
// fn part2_example() {
//     assert_eq!(51, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(7673, part2(PART2_FILE));
// }
