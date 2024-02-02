mod number_parser;
mod solution;
mod tests;

fn main() {
    let run_type = std::env::args().nth(1).unwrap_or_default();
    let number = std::env::args().nth(2).unwrap_or_default();
    let result = match (run_type.as_str(), number.as_str()) {
        ("t", "1") => solution::part1(include_str!("test1.txt")),
        ("p", "1") => solution::part1(include_str!("part1.txt")),
        ("t", "2") => solution::part2(include_str!("test2.txt")),
        ("p", "2") => solution::part2(include_str!("part1.txt")),
        _ => Err("Incorrect usage!\nUsage: cargo r -- [t|p] [1|2]"),
    };

    match result {
        Ok(r) => println!("Result for {run_type}{number} is {r:?}"),
        Err(e) => eprintln!("An error occurred:\n{e}"),
    }
}
