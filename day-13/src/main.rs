use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-13 p<n>";
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

fn find_vertical_reflection_axis(notes: &[Vec<u8>]) -> Option<usize> {
    let axis_range = 1..notes[0].len();

    for axis in axis_range {
        let left = 0..axis;
        let right = axis..notes[0].len();

        if left
            .rev()
            .zip(right)
            .all(|(i_left, i_right)| notes.iter().all(|line| line[i_left] == line[i_right]))
        {
            return Some(axis);
        }
    }
    None
}

fn find_horizontal_reflection_axis(notes: &[Vec<u8>]) -> Option<usize> {
    let axis_range = 1..notes.len();

    for axis in axis_range {
        let above = &notes[0..axis];
        let below = &notes[axis..notes.len()];

        if above
            .iter()
            .rev()
            .zip(below)
            .all(|(s_above, s_below)| s_above == s_below)
        {
            return Some(axis);
        }
    }
    None
}

fn count_lines_before_reflection(notes: &[Vec<u8>]) -> i64 {
    if let Some(axis) = find_horizontal_reflection_axis(notes) {
        return 100 * axis as i64;
    }

    if let Some(axis) = find_vertical_reflection_axis(notes) {
        return axis as i64;
    }

    unreachable!("there is some reflection");
}

fn part1(filename: &str) -> i64 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut note = vec![];
    let mut sum = 0;

    for line in file.lines() {
        let line = line.unwrap();

        if !line.is_empty() {
            note.push(line.into_bytes());
            continue;
        }

        sum += count_lines_before_reflection(&note);
        note.clear();
    }
    sum += count_lines_before_reflection(&note);

    sum
}

fn find_vertical_smudge(notes: &[Vec<u8>]) -> Option<usize> {
    let axis_range = 1..notes[0].len();

    for axis in axis_range {
        let left = 0..axis;
        let right = axis..notes[0].len();

        if left
            .rev()
            .zip(right)
            .map(|(i_left, i_right)| {
                notes
                    .iter()
                    .filter_map(|line| (line[i_left] != line[i_right]).then_some(1))
                    .sum::<u8>()
            })
            .sum::<u8>()
            == 1
        {
            return Some(axis);
        }
    }
    None
}

fn find_horizontal_smudge(notes: &[Vec<u8>]) -> Option<usize> {
    let axis_range = 1..notes.len();

    for axis in axis_range {
        let above = &notes[0..axis];
        let below = &notes[axis..notes.len()];

        if above
            .iter()
            .rev()
            .zip(below)
            .map(|(s_above, s_below)| {
                s_above
                    .iter()
                    .zip(s_below)
                    .filter_map(|(a, b)| (a != b).then_some(1))
                    .sum::<u8>()
            })
            .sum::<u8>()
            == 1
        {
            return Some(axis);
        }
    }
    None
}

fn count_lines_smudge(notes: &[Vec<u8>]) -> i64 {
    if let Some(axis) = find_horizontal_smudge(notes) {
        return 100 * axis as i64;
    }

    if let Some(axis) = find_vertical_smudge(notes) {
        return axis as i64;
    }

    unreachable!("there is some reflection");
}
fn part2(filename: &str) -> i64 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut note = vec![];
    let mut sum = 0;

    for line in file.lines() {
        let line = line.unwrap();

        if !line.is_empty() {
            note.push(line.into_bytes());
            continue;
        }

        sum += count_lines_smudge(&note);
        note.clear();
    }
    sum += count_lines_smudge(&note);

    sum
}

#[test]
fn part1_example() {
    assert_eq!(405, part1("test1.txt"));
}

#[test]
fn part1_example1() {
    assert_eq!(10, part1("test3.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(29213, part1(PART1_FILE));
}

#[test]
fn part2_example() {
    assert_eq!(400, part2("test2.txt"));
}

#[test]
fn part2_puzzle() {
    assert_eq!(37453, part2(PART2_FILE));
}
