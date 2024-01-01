use std::env;

// NOTE: 1272 is not the answer

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";
const RIGHT: Vector = Vector { dr: 0, dc: 1 };
const LEFT: Vector = Vector { dr: 0, dc: -1 };
const UP: Vector = Vector { dr: -1, dc: 0 };
const DOWN: Vector = Vector { dr: 1, dc: 0 };
const PATHS_PER_POINT: usize = 300;

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

#[derive(Clone)]
struct Path {
    direction: Vector,
    position: Vector,
    straight_count: usize,
    heat_loss: usize,
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

    #[inline]
    fn copy(&self, turn: Turn) -> Self {
        Self {
            direction: self.direction.turn(turn),
            position: self.position,
            straight_count: 0,
            heat_loss: self.heat_loss,
        }
    }

    fn advance(&mut self) -> (usize, usize) {
        let new_pos = self.position + self.direction;
        self.position = new_pos;
        self.straight_count += 1;

        (self.row(), self.col())
    }

    #[inline]
    fn is_in_bounds(&self, row_max: usize, col_max: usize) -> bool {
        self.position.dr >= 0
            && self.position.dr < row_max as isize
            && self.position.dc >= 0
            && self.position.dc < col_max as isize
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
    fn turn(mut self, new_dir: Turn) -> Self {
        self.direction = self.direction.turn(new_dir);
        self.straight_count = 0;
        self
    }

    #[inline]
    fn split(self) -> Vec<Self> {
        if self.straight_count == 3 {
            vec![self.copy(Turn::Left), self.turn(Turn::Right)]
        } else {
            vec![self.copy(Turn::Left), self.copy(Turn::Right), self]
        }
    }
}

fn find_hottest_path(first_path: Path, block_heat_loss: &[Vec<usize>]) -> usize {
    // some variables that crop up
    let (rows, cols) = (block_heat_loss.len(), block_heat_loss[0].len());
    let (r_goal, c_goal) = (rows - 1, cols - 1);

    let mut read_grid = vec![vec![vec![]; cols]; rows];
    let mut write_grid = vec![vec![vec![]; cols]; rows];
    // populate the starting position
    read_grid[0][0].push(first_path);

    // the minimum observed length of a path that reached the end
    let mut min_heat_loss = usize::MAX;
    let mut last_min = 0;

    let mut cull_count = 0;
    let mut iter_count = 0;

    let mut step_count = 1;
    let mut is_empty = true;

    for _ in 0..(rows * cols) {
        is_empty = true;
        step_count += 1;
        // iterate over the whole grid
        cull_count = 0;
        iter_count = 0;
        for row in 0..rows {
            for col in 0..cols {
                // process finished paths
                if row == r_goal && col == c_goal {
                    // read_grid[r_goal][c_goal].iter_mut().min_by(|p| )
                    while let Some(best_path) = read_grid[r_goal][c_goal].pop() {
                        is_empty = false;
                        min_heat_loss = min_heat_loss.min(best_path.heat_loss);
                        if last_min != min_heat_loss {
                            last_min = min_heat_loss;
                            dbg!(min_heat_loss);
                            dbg!(cull_count);
                            dbg!(iter_count);
                        }
                    }
                    continue;
                }

                // process other paths
                while let Some(path) = read_grid[row][col].pop() {
                    is_empty = false;
                    for mut new_path in path.split() {
                        let (new_row, new_col) = new_path.advance();

                        if new_path.is_in_bounds(rows, cols) {
                            // if !new_path.has_cycle && new_path.is_in_bounds(rows, cols) {
                            // get appropriate heat loss
                            new_path.heat_loss += block_heat_loss[new_row][new_col];

                            // aggressively cull the number of paths that can never be optimal
                            if new_path.heat_loss >= min_heat_loss {
                                cull_count += 1;
                                // dbg!(cull_count);
                                // dbg!(iter_count);
                                continue;
                            } else {
                                // dbg!(new_path.heat_loss);
                                // dbg!(min_heat_loss);
                                // dbg!(new_path.visited_positions.len());
                                iter_count += 1;
                            }

                            // write_grid[new_row][new_col].push(new_path);
                            // // write_grid[new_row][new_col].sort_unstable();

                            // see where our path fits in the new vec
                            let pos = write_grid[new_row][new_col]
                                .iter()
                                .position(|p| new_path < *p)
                                .unwrap_or(write_grid[new_row][new_col].len());
                            write_grid[new_row][new_col].insert(pos, new_path);

                            if write_grid[new_row][new_col].len() >= PATHS_PER_POINT {
                                write_grid[new_row][new_col].pop();
                            }

                            //     // if the vec is too long, remove the worst path
                            //     if write_grid[new_row][new_col].len() >= PATHS_PER_POINT {
                            //         write_grid[new_row][new_col].pop_back();
                            //     }
                            // }
                            // } else if new_vec.len() < PATHS_PER_POINT {
                            //     // if there is no spot but there are not enough paths, we add this one
                        }
                    }
                }
            }
        }

        // println!(
        //     "all {:?}? {:?}",
        //     step_count,
        //     write_grid.iter().all(|r| r
        //         .iter()
        //         .all(|v| v.iter().all(|p| p.visited_positions.len() == step_count)))
        // );

        if is_empty {
            break;
        }

        read_grid = write_grid.to_owned();
        write_grid
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|col| col.clear()));
        // write_grid
        //     .par_iter_mut()
        //     .for_each(|row| row.iter_mut().for_each(|col| *col = VecDeque::new()));

        // read_grid.par_iter_mut().enumerate().for_each(|(r, row)| {
        //     row.iter_mut()
        //         .enumerate()
        //         .for_each(|(c, col)| *col = write_grid[r][c].clone());
        // });
        // for row in 0..rows {
        //     (0..cols).into_par_iter().for_each(|col| {
        //         read_grid[row][col] = write_grid[row][col].clone();
        //     });
        // }
        // write_grid = vec![vec![vec![]; cols]; rows];
    }

    min_heat_loss
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

    let first_path = Path::new(RIGHT, Vector { dr: 0, dc: 0 });

    find_hottest_path(first_path, &heat_loss_grid)
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
