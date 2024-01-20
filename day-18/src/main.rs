use std::env;

const USAGE: &str = "Incorrect usage!\nUsage: cargo r -- [t|p] [1|2]";
const UP: &str = "U";
const DOWN: &str = "D";
const LEFT: &str = "L";
const RIGHT: &str = "R";

fn main() {
    let run_type = env::args().nth(1).expect(USAGE);
    let number = env::args().nth(2).expect(USAGE);
    let result = match (run_type.as_str(), number.as_str()) {
        ("t", "1") => part1("test1.txt"),
        ("p", "1") => part1("part1.txt"),
        ("t", "2") => part2("test2.txt"),
        ("p", "2") => part2("part2.txt"),
        _ => panic!("{USAGE}"),
    };
    println!("Result for {run_type}{number} is {result:?}");
}

fn parse_corners(commands: &[(&str, isize)]) -> Vec<(usize, usize)> {
    let mut position = (0_isize, 0_isize);
    let mut corners = vec![position];

    for command in commands {
        match command {
            (LEFT, distance) => position.1 -= distance,
            (RIGHT, distance) => position.1 += distance,
            (UP, distance) => position.0 -= distance,
            (DOWN, distance) => position.0 += distance,
            _ => unreachable!("illegal command {command:?}"),
        }

        corners.push(position);
    }

    let row_min = corners
        .iter()
        .min_by_key(|e| e.0)
        .expect("there should be at least one corner")
        .0;
    let col_min = corners
        .iter()
        .min_by_key(|e| e.1)
        .expect("there should be at least one corner")
        .1;

    corners
        .iter()
        .map(|(row, col)| ((row - row_min) as usize, (col - col_min) as usize))
        .collect()
}

#[derive(Clone, Copy, PartialEq)]
enum Earth {
    Normal,
    Trench,
    Hole,
    AnyCorner,
    TLCorner,
    TRCorner,
    BLCorner,
    BRCorner,
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

    let row_max = corners
        .iter()
        .max_by_key(|e| e.0)
        .expect("there should be at least one corner")
        .0
        + 1;
    let col_max = corners
        .iter()
        .max_by_key(|e| e.1)
        .expect("there should be at least one corner")
        .1
        + 1;

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
                (Inside, Normal) => *e = Trench,
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
                (Inside, Normal) => row[col] = Trench,
                _ => { /* no change */ }
            }
        });
    }

    // parse the corner type
    for row in 0..row_max {
        for col in 0..col_max {
            if map[row][col] == AnyCorner {
                let mut is_up = false;
                if let Some(above_row) = map.get(row.saturating_sub(1)) {
                    if above_row[col] == Trench {
                        is_up = true;
                    }
                }

                let mut is_left = false;
                if let Some(left_col) = map[row].get(col.saturating_sub(1)) {
                    if *left_col == Trench {
                        is_left = true;
                    }
                }

                map[row][col] = match (is_up, is_left) {
                    (true, true) => BRCorner,
                    (true, false) => BLCorner,
                    (false, true) => TRCorner,
                    (false, false) => TLCorner,
                };
            }
        }
    }

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
                (Outside, Trench) | (Outside, AnyCorner) => status = FirstTrench,
                (FirstTrench, Normal) => {
                    crossed_trenches += 1;
                    status = Inside;
                    *e = Trench;
                }
                (FirstTrench, AnyCorner) => {
                    crossed_trenches += 1;
                    status = Inside;
                    *e = Trench;
                }
                (Inside, Trench) => status = SecondTrench,
                (Inside, Normal) => *e = Trench,
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
                    Trench => '#',
                    AnyCorner => 'x',
                    Normal => '.',
                    _ => ' ',
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
        .map(|row| row.iter().filter(|e| **e == Earth::Trench).count())
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
