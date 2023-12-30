use core::panic;
use std::{collections::BTreeSet, env};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

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

// TODO: rewrite this using some type of path like on day 16

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn left(&self) -> Option<Self> {
        if let Some(prev_row) = self.row.checked_sub(1) {
            Some(Self {
                row: prev_row,
                col: self.col,
            })
        } else {
            None
        }
    }

    fn right(&self) -> Self {
        Self {
            row: self.row,
            col: self.col + 1,
        }
    }

    fn above(&self) -> Option<Self> {
        if let Some(prev_col) = self.col.checked_sub(1) {
            Some(Self {
                row: self.row,
                col: prev_col,
            })
        } else {
            None
        }
    }

    fn below(&self) -> Self {
        Self {
            row: self.row + 1,
            col: self.col,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash)]
struct Node {
    point: Point,
    heat_loss: usize,
    dist: usize,
    was_visited: bool,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Node {
    fn new(c: char, row: usize, col: usize) -> Self {
        Self {
            point: Point::new(row, col),
            heat_loss: c.to_digit(10).expect("this is not a number") as usize,
            dist: usize::MAX,
            was_visited: false,
        }
    }

    fn neighbors(&self) -> Vec<Point> {
        let mut vec = vec![self.point.right(), self.point.below()];

        if let Some(left) = self.point.left() {
            vec.push(left);
        }

        if let Some(above) = self.point.above() {
            vec.push(above);
        }

        vec
    }
}

struct Array {
    ranking: BTreeSet<Node>,
}

impl Array {
    fn new(file: &str) -> Self {
        let mut data: BTreeSet<Node> = file
            .split_ascii_whitespace()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| Node::new(c, row, col)).collect()
            })
            .collect();

        let start = data.iter().find(|n| n.point == Point::new(0, 0));
        data.get()
        
        let mut ranking = BTreeSet::new();
        for e in data.iter().flatten() {
            ranking.insert(*e);
        }

        Self { ranking }
    }

    fn get_mut(&mut self, point: Point) -> &mut Node {
        if let Some(row) = self.data.get_mut(point.row) {
            row.get_mut(point.col).unwrap()
        } else {
            panic!();
        }
    }

    fn get(&self, point: Point) -> &Node {
        if let Some(row) = self.data.get(point.row) {
            row.get(point.col).unwrap()
        } else {
            panic!();
        }
    }

    fn get_neighbors(&self, point: Point) -> Vec<Point> {
        self.get(point)
            .neighbors()
            .iter()
            .filter_map(|n| {
                if n.row < self.rows && n.col < self.cols {
                    Some(*n)
                } else {
                    None
                }
            })
            .collect()
    }
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();

    let mut a = Array::new(&file);

    let start_point = Point::new(0, 0);

    let mut current_point = start_point;

    while !a.data[a.rows - 1][a.cols - 1].was_visited {
        let current_dist = a.get(current_point).dist;
        for neighbor in a.get_neighbors(current_point) {
            if a.get(neighbor).was_visited {
                continue;
            }

            let hl = a.get(neighbor).heat_loss;
            let d = a.get(neighbor).dist;
            a.get_mut(neighbor).dist = d.min(current_dist + hl);
        }

        a.get_mut(current_point).was_visited = true;
        // find closest point in
        // current_point =
    }

    // 1. run dijkstra's algorithm
    // 2. go back and enforce our constraints, i.e. only 3 straights after another, only straight-ahead, left, or right

    0
}

fn part2(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    let map: Vec<Vec<_>> = file
        .split_ascii_whitespace()
        .map(|line| line.chars().map(|c| c.to_digit(10)).collect())
        .collect();
    let (rows, cols) = (map.len(), map[0].len());

    println!("{rows:?} by {cols:?}");

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
