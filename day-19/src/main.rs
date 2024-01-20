mod parts;
mod solution;
mod system;
mod tests;
mod workflow;

use solution::{part1, part2};

fn main() {
    let run_type = std::env::args().nth(1).unwrap_or_default();
    let number = std::env::args().nth(2).unwrap_or_default();

    let result = match (run_type.as_str(), number.as_str()) {
        ("t", "1") => part1("test1.txt"),
        ("p", "1") => part1("part1.txt"),
        ("t", "2") => part2("test2.txt"),
        ("p", "2") => part2("part2.txt"),
        _ => Err("Incorrect usage!\nUsage: cargo r -- [t|p] [1|2]"),
    };

    match result {
        Ok(r) => println!("Result for {run_type}{number} is {r:?}"),
        Err(e) => eprintln!("An error occurred:\n{e}"),
    }
}
