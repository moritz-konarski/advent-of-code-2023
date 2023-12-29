use std::env;

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-16 p<n>";
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

fn part1(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();

    0
}

fn part2(filename: &str) -> usize {
    let file = std::fs::read_to_string(filename).unwrap();

    0
}

#[test]
fn part1_example() {
    assert_eq!(1320, part1("test1.txt"));
}

// #[test]
// fn part1_puzzle() {
//     assert_eq!(504036, part1(PART1_FILE));
// }

// #[test]
// fn part2_example() {
//     assert_eq!(145, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(295719, part2(PART2_FILE));
// }
