use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-03 p<n>";
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

#[derive(Clone)]
struct Symbol {
    sym: char,
    index: usize,
    numbers: Vec<u32>,
}

impl Symbol {
    fn new(sym: char, index: usize) -> Self {
        Self {
            sym,
            index,
            numbers: vec![],
        }
    }

    fn sum_symbol_adjacent(&self) -> u32 {
        let mut sum = 0;
        let mut iter = self
            .chars
            .iter()
            .zip(&self.is_digit)
            .zip(&self.adjacent_sym);

        while let Some(((mut letter, mut is_digit), mut symbol_count)) = iter.next() {
            if !is_digit {
                continue;
            }

            let mut num_counts = 0;
            let mut nums: Vec<char> = vec![];
            while *is_digit {
                nums.push(*letter);
                num_counts += *symbol_count;

                match iter.next() {
                    Some(((x, y), z)) => {
                        letter = x;
                        is_digit = y;
                        symbol_count = z;
                    }
                    None => break,
                }
            }

            if num_counts > 0 {
                sum += nums.iter().collect::<String>().parse::<u32>().unwrap();
            }
        }

        return sum;
    }

    fn sum_gear_adjacent(&self) -> u32 {
        let mut prod = 1;
        let mut iter = self
            .chars
            .iter()
            .zip(&self.is_digit)
            .zip(&self.adjacent_gear);

        while let Some(((mut letter, mut is_digit), mut gear_count)) = iter.next() {
            if !is_digit {
                continue;
            }

            let mut num_counts = 0;
            let mut nums: Vec<char> = vec![];
            while *is_digit {
                nums.push(*letter);
                num_counts += *gear_count;

                match iter.next() {
                    Some(((x, y), z)) => {
                        letter = x;
                        is_digit = y;
                        gear_count = z;
                    }
                    None => break,
                }
            }

            if num_counts > 0 {
                prod *= nums.iter().collect::<String>().parse::<u32>().unwrap();
            }
        }

        return prod;
    }
}

struct SymbolData {
    prev: Vec<Symbol>,
    curr: Vec<Symbol>,
    next: Vec<Symbol>,
}

impl SymbolData {
    fn new() -> Self {
        Self {
            prev: vec![],
            curr: vec![],
            next: vec![],
        }
    }

    fn parse(&mut self, line: String) {
        self.curr.parse_line(line);
    }

    fn shift_left(&mut self) {
        self.prev = self.curr.clone();
        self.curr = self.next.clone();
        self.next = vec![];
    }

    fn update_adj_sym(&mut self, indices: Vec<usize>) {
        let len = self.curr.adjacent_sym.len() - 1;
        indices
            .iter()
            .map(|i| {
                let i = *i;
                if i == 0 {
                    i..i + 2
                } else if i == len {
                    i - 1..i + 1
                } else {
                    i - 1..i + 2
                }
            })
            .for_each(|r| {
                for i in r {
                    self.prev.adjacent_sym[i] += 1;
                    self.curr.adjacent_sym[i] += 1;
                    self.next.adjacent_sym[i] += 1;
                }
            });
    }

    fn update_adj_gear(&mut self, indices: Vec<usize>) {
        let len = self.curr.adjacent_gear.len() - 1;
        indices
            .iter()
            .map(|i| {
                let i = *i;
                if i == 0 {
                    i..i + 2
                } else if i == len {
                    i - 1..i + 1
                } else {
                    i - 1..i + 2
                }
            })
            .for_each(|r| {
                for i in r {
                    self.prev.adjacent_gear[i] += 1;
                    self.curr.adjacent_gear[i] += 1;
                    self.next.adjacent_gear[i] += 1;
                }
            });
    }

    fn process_all(&mut self) {
        let symbols_indices = self
            .curr
            .chars
            .iter()
            .zip(&self.curr.is_digit)
            .enumerate()
            .filter_map(
                |(i, (c, is_d))| {
                    if *is_d || *c == '.' {
                        None
                    } else {
                        Some(i)
                    }
                },
            )
            .collect();
        self.update_adj_sym(symbols_indices);
    }

    fn process_numbers(&mut self) {
        let number_indices = self
            .curr
            .is_digit
            .iter()
            .enumerate()
            .filter_map(|(i, is_d)| if *is_d { Some(i) } else { None })
            .collect();
        self.update_adj_sym(number_indices);
    }

    fn process_gears(&mut self) {
        let number_indices = self
            .prev
            .chars
            .iter()
            .zip(&self.prev.adjacent_sym)
            .enumerate()
            .filter_map(|(i, (c, sym_count))| {
                if *c == '*' && *sym_count == 2 {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        self.update_adj_gear(number_indices);
    }

    fn sum(&self) -> u32 {
        self.prev.sum_symbol_adjacent()
    }

    fn sum_gears(&self) -> u32 {
        self.prev.sum_gear_adjacent()
    }
}

fn get_line_len(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);
    file.lines().next().unwrap().unwrap().len()
}

fn part1(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut sum = 0;
    let mut symbol_data = SymbolData::new();

    for line in file.lines() {
        symbol_data.parse(line.unwrap());
        symbol_data.process_all();
        sum += symbol_data.sum();
        symbol_data.shift_left();
    }

    sum + symbol_data.sum()
}

// fn part2(filename: &str) -> u32 {
//     let line_len = get_line_len(filename);
//     let file = File::open(filename).expect("Should be able to read the file");
//     let file = BufReader::new(file);

//     let mut sum = 0;
//     let mut line_data = LineData::new(line_len);

//     for line in file.lines() {
//         line_data.parse(line.unwrap());
//         line_data.process_numbers();
//         line_data.process_gears();
//         sum += line_data.sum_gears();
//         line_data.shift_left();
//     }

//     sum + line_data.sum_numbers()
// }

#[test]
fn part1_example() {
    assert_eq!(4361, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(550064, part1(PART1_FILE));
}

// #[test]
// fn part2_example() {
//     assert_eq!(467835, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(78111, part2(PART2_FILE));
// }
