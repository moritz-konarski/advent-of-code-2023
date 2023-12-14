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

#[derive(Clone, Debug)]
struct Number {
    val: u32,
    start: usize,
    end: usize,
}

impl Number {
    fn parse(line: &String) -> Vec<Self> {
        let mut nums = vec![];
        let mut iter = line.chars().enumerate();

        while let Some((mut index, mut letter)) = iter.next() {
            if !letter.is_digit(10) {
                continue;
            }

            let start = index;
            let mut digits = vec![];
            while letter.is_digit(10) {
                digits.push(letter);

                match iter.next() {
                    Some((x, y)) => {
                        index = x;
                        letter = y;
                    }
                    None => break,
                }
            }

            nums.push(Number {
                val: digits.iter().collect::<String>().parse::<u32>().unwrap(),
                start,
                end: index,
            });
        }

        return nums;
    }
}

#[derive(Clone, Debug)]
struct Symbol {
    sym: char,
    index: usize,
    numbers: Vec<u32>,
}

impl Symbol {
    fn parse(line: &String) -> Vec<Self> {
        let mut syms = vec![];

        for (index, sym) in line.chars().enumerate() {
            match sym {
                '.' | '0'..='9' => { /* not symbols */ }
                _ => syms.push(Self {
                    sym,
                    index,
                    numbers: vec![],
                }),
            }
        }

        return syms;
    }

    fn sum(&self) -> u32 {
        self.numbers.iter().sum()
    }

    fn product(&self) -> u32 {
        self.numbers.iter().product()
    }

    fn is_adjacent_to(&self, num: &Number) -> bool {
        if self.index >= num.start && self.index <= num.end {
            return true;
        }

        if num.start > 0 && self.index == num.start - 1 {
            return true;
        }

        return false;
    }

    fn add_num(&mut self, num: &Number) {
        self.numbers.push(num.val);
    }
}

struct SymbolData {
    prev_sym: Vec<Symbol>,
    prev_num: Vec<Number>,
    curr_sym: Vec<Symbol>,
    curr_num: Vec<Number>,
}

impl SymbolData {
    fn new() -> Self {
        Self {
            prev_sym: vec![],
            prev_num: vec![],
            curr_sym: vec![],
            curr_num: vec![],
        }
    }

    fn parse(&mut self, line: String) {
        // parse numbers from current line
        self.curr_num = Number::parse(&line);

        // add current nums to previous symbols
        for sym in self.prev_sym.iter_mut() {
            for num in &self.curr_num {
                if sym.is_adjacent_to(&num) {
                    sym.add_num(num);
                } else if num.start > sym.index {
                    break;
                }
            }
        }

        // parse symbols from current line
        self.curr_sym = Symbol::parse(&line);

        // add previous and current nums to current symbols
        for sym in self.curr_sym.iter_mut() {
            for num in &self.prev_num {
                if sym.is_adjacent_to(&num) {
                    sym.add_num(&num);
                } else if num.start > sym.index {
                    break;
                }
            }

            for num in &self.curr_num {
                if sym.is_adjacent_to(&num) {
                    sym.add_num(&num);
                } else if num.start > sym.index {
                    break;
                }
            }
        }
    }

    fn shift_left(&mut self) {
        self.prev_sym = self.curr_sym.clone();
        self.prev_num = self.curr_num.clone();
        self.curr_sym = vec![];
        self.curr_num = vec![];
    }

    fn sum_all(&self) -> u32 {
        self.prev_sym.iter().map(|s| s.sum()).sum()
    }

    fn sum_gears(&self) -> u32 {
        self.prev_sym
            .iter()
            .filter_map(|s| {
                if s.sym == '*' && s.numbers.len() == 2 {
                    Some(s.product())
                } else {
                    None
                }
            })
            .sum()
    }
}

fn part1(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut sum = 0;
    let mut symbol_data = SymbolData::new();

    for line in file.lines() {
        symbol_data.parse(line.unwrap());
        sum += symbol_data.sum_all();
        symbol_data.shift_left();
    }

    sum + symbol_data.sum_all()
}

fn part2(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let mut sum = 0;
    let mut symbol_data = SymbolData::new();

    for line in file.lines() {
        symbol_data.parse(line.unwrap());
        sum += symbol_data.sum_gears();
        symbol_data.shift_left();
    }

    sum + symbol_data.sum_all()
}

#[test]
fn part1_example() {
    assert_eq!(4361, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(550064, part1(PART1_FILE));
}

#[test]
fn part2_example() {
    assert_eq!(467835, part2("test2.txt"));
}

#[test]
fn part2_puzzle() {
    assert_eq!(85010461, part2(PART2_FILE));
}
