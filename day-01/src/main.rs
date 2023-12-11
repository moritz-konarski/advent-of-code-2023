use std::env;

fn trebuchet_calibration(strs: Vec<String>) -> usize {
    0
}

fn main() {
    if let Some(filename) = env::args().nth(1) {
        println!("Reading `{filename}`");

        let sum = trebuchet_calibration(Vec::new());

        println!("The sum is {sum}.");
    } else {
        eprintln!("Missing filename!\nUsage: day-01 <filename>")
    }
}
