use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const RADIX: u32 = 10;
const TEST1_FILE: &str = "test1.txt";
const TEST2_FILE: &str = "test2.txt";
const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-01 p<n>";
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

#[inline]
fn process_left(line: &String) -> u32 {
    for c in line.chars() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
    }
    unreachable!("there is a digit")
}

#[inline]
fn process_right(line: &String) -> u32 {
    for c in line.chars().rev() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
    }
    unreachable!("there is a digit")
}

fn part1(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the value");
    let file = BufReader::new(file);

    file.lines().fold(0, |sum, line| {
        let l = line.unwrap();
        sum + 10 * process_left(&l) + process_right(&l)
    })
}

fn part2(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the value");
    let file = BufReader::new(file);

    let number_parser = NumberParser::new();

    file.lines().fold(0, |sum, line| {
        let l = line.unwrap();
        sum + 10 * number_parser.get_left(&l) + number_parser.get_right(&l)
    })
}

#[test]
fn part1_example() {
    assert_eq!(142, part1(TEST1_FILE));
}

#[test]
fn part1_puzzle() {
    assert_eq!(54561, part1(PART1_FILE));
}

#[test]
fn part2_example() {
    assert_eq!(281, part2(TEST2_FILE));
}

#[test]
fn part2_puzzle() {
    assert_eq!(54076, part2(PART2_FILE));
}

struct NumberParser {
    letter: char,
    digit: Option<u32>,
    children: Vec<Self>,
}

impl NumberParser {
    fn can_start_digit(&self, letter: char) -> bool {
        for child in &self.children {
            if letter == child.letter {
                return true;
            }
        }
        false
    }

    fn parse(&self, line: &String, start: usize) -> Option<u32> {
        let mut chars = line.chars();
        let index = self.find_child(chars.nth(start).unwrap()).unwrap();
        let mut node = self.children.get(index).unwrap();

        for c in chars {
            if let Some(index) = node.find_child(c) {
                node = node.children.get(index).unwrap();
                if node.digit.is_some() {
                    return node.digit;
                }
            } else {
                return None;
            }
        }
        None
    }

    fn get_left(&self, line: &String) -> u32 {
        for (index, letter) in line.chars().enumerate() {
            if letter.is_digit(RADIX) {
                return letter.to_digit(RADIX).unwrap();
            } else if self.can_start_digit(letter) {
                if let Some(digit) = self.parse(line, index) {
                    return digit;
                }
            }
        }
        unreachable!("there must be a digit")
    }

    fn get_right(&self, line: &String) -> u32 {
        let line_len = line.len() - 1;

        for (index, letter) in line.chars().rev().enumerate() {
            if letter.is_digit(RADIX) {
                return letter.to_digit(RADIX).unwrap();
            } else if index >= 2 && self.can_start_digit(letter) {
                if let Some(digit) = self.parse(line, line_len - index) {
                    return digit;
                }
            }
        }
        unreachable!("there must be a digit")
    }

    fn new() -> Self {
        let mut root = NumberParser::get_new_node('\0');

        root.add_word("zero", 0);
        root.add_word("one", 1);
        root.add_word("two", 2);
        root.add_word("three", 3);
        root.add_word("four", 4);
        root.add_word("five", 5);
        root.add_word("six", 6);
        root.add_word("seven", 7);
        root.add_word("eight", 8);
        root.add_word("nine", 9);

        return root;
    }

    fn get_new_node(letter: char) -> Self {
        NumberParser {
            letter,
            digit: None,
            children: Vec::new(),
        }
    }

    fn find_child(&self, letter: char) -> Option<usize> {
        for (index, child) in self.children.iter().enumerate() {
            if child.letter == letter {
                return Some(index);
            }
        }
        None
    }

    fn add_word(&mut self, word: &str, digit: u32) {
        let mut node = self;

        for letter in word.chars() {
            let new_child = NumberParser::get_new_node(letter);

            let index = if node.children.is_empty() {
                node.children.push(new_child);
                0
            } else {
                if let Some(found_index) = node.find_child(letter) {
                    found_index
                } else {
                    node.children.push(new_child);
                    node.children.len() - 1
                }
            };
            node = node.children.get_mut(index).unwrap();
        }
        node.digit = Some(digit);
    }
}
