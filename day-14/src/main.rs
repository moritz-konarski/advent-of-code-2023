use std::env;

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-14 p<n>";
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

fn tilt_north(lines: &mut [&[u8]]) ->

fn part1(filename: &str) -> usize {
    let lines: Vec<_> = std::fs::read_to_string(filename)
        .unwrap()
        .split_ascii_whitespace()
        .map(|line| line.as_bytes())
        .collect();

    // read in the data
    // tilt all the rocks north
    // O moves, . is empty, # does not move
    // calculate load for O as len() - row for each O

    0
}

fn part2(filename: &str) -> usize {
    // let file = File::open(filename).expect("Should be able to read the file");
    // let file = BufReader::new(file);

    0
}

#[test]
fn part1_example() {
    assert_eq!(136, part1("test1.txt"));
}

// #[test]
// fn part1_puzzle() {
//     assert_eq!(227653707, part1(PART1_FILE));
// }

// #[test]
// fn part2_example() {
//     assert_eq!(46, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(78775051, part2(PART2_FILE));
// }
