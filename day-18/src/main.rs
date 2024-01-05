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

fn dig_trench(commands: &[(&str, usize)]) -> Vec<(usize, usize)> {
    let mut current = (0, 0);
    let mut map = vec![current];

    for command in commands {
        let (direction, distance) = command;

        match *direction {
            LEFT => current.0 -= distance,
            RIGHT => current.0 += distance,
            UP => current.1 -= distance,
            DOWN => current.1 += distance,
            _ => unreachable!("illegal direction {direction}"),
        }
    }

    map
}

fn dig_interior(trench: &[(usize, usize)]) -> usize {
    0
}

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();
    let lines = file.split_ascii_whitespace().collect::<Vec<_>>();
    let commands = lines
        .chunks_exact(3)
        .map(|chunk| {
            let distance = chunk[1].parse::<usize>().unwrap();
            (chunk[0], distance)
        })
        .collect::<Vec<_>>();

    let trench = dig_trench(&commands);

    let volume = dig_interior(&trench);

    volume
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
