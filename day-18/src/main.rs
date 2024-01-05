use std::env;

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";
const UP: &str = "U";
const DOWN: &str = "D";
const LEFT: &str = "L";
const RIGHT: &str = "R";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-18 p<n>";
    if let Some(part) = env::args().nth(1) {
        match part.as_str() {
            "t1" => {
                println!("Reading `test1.txt`");
                println!("Sum is {}", part1("test1.txt"));
            }
            "p1" => {
                println!("Reading `{PART1_FILE}`");
                println!("Sum is {}", part1(PART1_FILE));
            }
            "t2" => {
                println!("Reading `test2.txt`");
                println!("Sum is {}", part1("test2.txt"));
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

fn parse_corners(commands: &[(&str, isize)]) -> Vec<(usize, usize)> {
    let mut position = (0_isize, 0_isize);

    let mut corners = vec![position];
    for command in commands {
        let (direction, distance) = command;

        match *direction {
            LEFT => position.1 -= distance,
            RIGHT => position.1 += distance,
            UP => position.0 -= distance,
            DOWN => position.0 += distance,
            _ => unreachable!("illegal direction {direction}"),
        }

        corners.push(position);
    }

    let row_min = corners.iter().min_by_key(|e| e.0).unwrap().0;
    let col_min = corners.iter().min_by_key(|e| e.1).unwrap().1;

    corners
        .iter()
        .map(|(row, col)| ((row - row_min) as usize, (col - col_min) as usize))
        .collect()
}

#[derive(Clone, Copy, PartialEq)]
enum Earth {
    Normal,
    Hole,
    AnyCorner,
    LUCorner,
    RUCorner,
    LDCorner,
    RDCorner,
}

#[derive(Clone, Copy, PartialEq)]
enum LagoonState {
    Outside,
    FirstTrench,
    Inside,
    SecondTrench,
    // Outside again
}

fn dig_trench(corners: &[(usize, usize)]) -> Vec<Vec<Earth>> {
    use Earth::*;
    use LagoonState::*;

    let row_max = corners.iter().max_by_key(|e| e.0).unwrap().0 + 1;
    let col_max = corners.iter().max_by_key(|e| e.1).unwrap().1 + 1;

    let mut map = vec![vec![Normal; col_max]; row_max];
    for (row, col) in corners {
        map[*row][*col] = AnyCorner;
    }

    // add horizontal trenches
    map.iter_mut().for_each(|row| {
        let mut status = Outside;
        row.iter_mut().for_each(|e| {
            match (status, *e) {
                (Outside, AnyCorner) => status = Inside,
                (Inside, AnyCorner) => status = Outside,
                (Inside, Normal) => *e = Hole,
                _ => { /* no change */ }
            }
        });
    });

    // add vertical trenches
    for col in 0..col_max {
        let mut status = Outside;
        map.iter_mut().for_each(|row| {
            match (status, row[col]) {
                (Outside, AnyCorner) => status = Inside,
                (Inside, AnyCorner) => status = Outside,
                (Inside, Normal) => row[col] = Hole,
                _ => { /* no change */ }
            }
        });
    }

    // TODO: parse corner type in index-based loop

    map.iter_mut().for_each(|row| {
        let mut status = Outside;
        row.iter_mut().for_each(|e| {
            match (status, *e) {
                (Outside, AnyCorner) => status = Inside,
                (Inside, AnyCorner) => status = Outside,
                (Inside, Normal) => *e = Hole,
                _ => { /* no change */ }
            }
        });
    });

    map
}

fn dig_interior(trench: &[Vec<Earth>]) -> Vec<Vec<Earth>> {
    use Earth::*;
    use LagoonState::*;

    let mut lagoon = trench.to_owned();

    for row in lagoon.iter_mut() {
        let mut status = Outside;
        let mut crossed_trenches = 0;

        for e in row.iter_mut() {
            match (status, *e) {
                (Outside, Hole) | (Outside, AnyCorner) => status = FirstTrench,
                (FirstTrench, Normal) => {
                    crossed_trenches += 1;
                    status = Inside;
                    *e = Hole;
                }
                (FirstTrench, AnyCorner) => {
                    crossed_trenches += 1;
                    status = Inside;
                    *e = Hole;
                }
                (Inside, Hole) => status = SecondTrench,
                (Inside, Normal) => *e = Hole,
                (SecondTrench, Normal) => status = Outside,
                _ => { /* we can ignore these */ }
            }
        }
    }

    println!();
    lagoon.iter().for_each(|row| {
        row.iter().for_each(|e| {
            print!(
                "{}",
                match *e {
                    Hole => '#',
                    AnyCorner => 'x',
                    Normal => '.',
                }
            )
        });
        println!();
    });

    lagoon
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    let lines = file.split_ascii_whitespace().collect::<Vec<_>>();
    let commands = lines
        .chunks_exact(3)
        .map(|chunk| {
            let distance = chunk[1].parse().unwrap();
            (chunk[0], distance)
        })
        .collect::<Vec<_>>();

    let corners = parse_corners(&commands);
    let trench = dig_trench(&corners);
    let lagoon = dig_interior(&trench);

    lagoon
        .iter()
        .map(|row| row.iter().filter(|e| **e == Earth::Hole).count())
        .sum()
}

fn part2(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    0
}

#[test]
fn part1_example() {
    assert_eq!(62, part1("test1.txt"));
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
