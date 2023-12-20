use std::{collections::HashSet, env};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";
const WEST: (isize, isize) = (0, -1);
const EAST: (isize, isize) = (0, 1);
const NORTH: (isize, isize) = (-1, 0);
const SOUTH: (isize, isize) = (1, 0);

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

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct CharPoint {
    sym: char,
    row: usize,
    col: usize,
    dirs: Vec<(isize, isize)>,
}

impl CharPoint {
    fn get_adj(&self, dir: (isize, isize)) -> (usize, usize) {
        let r = (self.row as isize + dir.0) as usize;
        let c = (self.col as isize + dir.1) as usize;
        (r, c)
    }
}

#[derive(Clone)]
struct Trace<'a> {
    curr: &'a CharPoint,
    prev: Option<&'a CharPoint>,
}

struct PipeMap<'a> {
    points: Vec<Vec<CharPoint>>,
    trace_a: Trace<'a>,
    trace_b: Trace<'a>,
    point_set: HashSet<&'a CharPoint>,
    step_count: usize,
}

impl PipeMap<'_> {
    fn new(file: &str) -> Self {
        let points: Vec<Vec<_>> = file
            .split_ascii_whitespace()
            .enumerate()
            .map(|(row, part)| {
                part.chars()
                    .enumerate()
                    .map(|(col, sym)| CharPoint {
                        sym,
                        row,
                        col,
                        dirs: Self::get_dirs(sym),
                    })
                    .collect()
            })
            .collect();

        let start = *points.iter().flatten().find(|cp| cp.sym == 'S').unwrap();

        let mut point_set = HashSet::new();
        point_set.insert(start);

        let trace_a = Trace {
            prev: None,
            curr: start,
        };
        let trace_b = trace_a.clone();

        Self {
            points,
            trace_a,
            trace_b,
            step_count: 0,
            point_set,
        }
    }

    fn get_dirs(c: char) -> Vec<(isize, isize)> {
        match c {
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
        let even_loop_match = self.trace_a.curr == self.trace_b.curr;
        let odd_loop_match = self.trace_a.curr == self.trace_b.prev.unwrap()
            && self.trace_b.curr == self.trace_a.prev.unwrap();

        even_loop_match || odd_loop_match
    }

    fn get_next_point(&self, trace: &Trace) -> &CharPoint {
        let (r, c) = trace.curr.get_adj(trace.curr.dirs[0]);
        let p1 = &self.points[r][c];

        if p1 == trace.prev.unwrap() {
            return p1;
        } else {
            let (r, c) = trace.curr.get_adj(trace.curr.dirs[1]);
            return &self.points[r][c];
        }
    }

    fn advance_points(&mut self) {
        self.step_count += 1;

        let new_a = self.get_next_point(&self.trace_a);
        self.point_set.insert(&new_a.clone());
        self.trace_a.prev = Some(&mut self.trace_a.curr);
        self.trace_a.curr = new_a;

        let new_b = self.get_next_point(&self.trace_b);
        self.point_set.insert(&new_b.clone());
        self.trace_b.prev = Some(self.trace_b.curr);
        self.trace_b.curr = new_b;
    }

    fn advance_from_start(&mut self) {
        self.step_count += 1;

        let mut indices = vec![];

        for direction in PipeMap::get_dirs(self.trace_a.curr.sym) {
            let (r, c) = self.trace_a.curr.get_adj(direction);
            let next = &self.points[r][c];

            PipeMap::get_dirs(next.sym).iter().for_each(|dir| {
                let (r, c) = next.get_adj(*dir);
                if &self.points[r][c] == self.trace_a.curr {
                    indices.push((next.row, next.col))
                }
            });
        }

        let next = &self.points[indices[0].0][indices[0].1];
        self.point_set.insert(next);
        self.trace_a.prev = Some(self.trace_a.curr);
        self.trace_a.curr = next;

        let next = &self.points[indices[1].0][indices[1].1];
        self.point_set.insert(next);
        self.trace_b.prev = Some(self.trace_b.curr);
        self.trace_b.curr = next;
    }

    fn count_points_in_loop(&self) -> usize {
        // self.chars
        //     .iter()
        //     .enumerate()
        //     .fold(0, |outer_sum, (row, vec)| {
        //         outer_sum + vec.iter().enumerate().fold(0, |sum, (col, item)| {
        //            sum + vec[..col].
        //         })
        //     })
        0
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
    assert_eq!(8, part2("test2.txt"));
}

#[test]
fn part2_example1() {
    assert_eq!(8, part2("test4.txt"));
}

// #[test]
// fn part2_puzzle() {
//     assert_eq!(250506580, part2(PART2_FILE));
// }
