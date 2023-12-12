use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const test1_file: &str = "test1.txt";
const test2_file: &str = "test2.txt";
const part1_file: &str = "part1.txt";
const part2_file: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-01 p<n>";
    if let Some(part) = env::args().nth(1) {
        match part.as_str() {
            "p1" => {
                println!("Reading `{part1_file}`");
                println!("Sum is {}", part1(part1_file));
            }
            "p2" => {
                println!("Reading `{part2_file}`");
                println!("Sum is {}", part2(part2_file));
            }
            _ => eprintln!("{usage}"),
        }
    } else {
        eprintln!("{usage}");
    }
}

fn part1(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the value");
    let file = BufReader::new(file);

    file.lines().fold(0, |sum, line| {
        let line = line.unwrap();
        let mut left = 0;
        let mut right = 0;

        for c in line.chars() {
            if c.is_digit(10) {
                left = c.to_digit(10).unwrap();
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_digit(10) {
                right = c.to_digit(10).unwrap();
                break;
            }
        }

        sum + 10 * left + right
    })
}

fn part2(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the value");
    let file = BufReader::new(file);

    file.lines().fold(0, |sum, line| {
        let line = line.unwrap();
        let mut left = 0;
        let mut right = 0;

        println!("{line}");

        for c in line.chars() {
            if c.is_digit(10) {
                left = c.to_digit(10).unwrap();
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_digit(10) {
                right = c.to_digit(10).unwrap();
                break;
            }
        }
        println!("left: {left}, right: {right}");

        sum + 10 * left + right
    })
}

#[test]
fn part1_example() {
    assert_eq!(142, part1(test1_file));
}

#[test]
fn part2_example() {
    assert_eq!(281, part1(test2_file));
}

#[test]
fn ce() {
    let mut root = CharElement::new();
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
    for i in root.next {
        println!("{:?}", i.c)
    }
    assert!(false);
}

#[derive(Debug, Clone)]
struct CharElement {
    c: Option<char>,
    num: Option<u32>,
    next: Vec<Self>,
}

impl CharElement {
    fn new() -> Self {
        CharElement {
            c: None,
            num: None,
            next: Vec::new(),
        }
    }

    fn get_next(&self, c: char) -> Option<usize> {
        for (i, element) in self.next.iter().enumerate() {
            if let Some(ch) = element.c {
                if ch == c {
                    return Some(i);
                }
            }
        }

        None
    }

    fn add_word(&mut self, word: &str, value: u32) {
        let mut node = self;
        for c in word.chars() {
            let new_node = CharElement {
                c: Some(c),
                num: None,
                next: Vec::new(),
            };

            let index = if node.next.is_empty() {
                node.next.push(new_node);
                0
            } else {
                if let Some(ii) = node.get_next(c) {
                    ii
                } else {
                    node.next.push(new_node);
                    node.next.len() - 1
                }
            };
            node = node.next.get_mut(index).unwrap();
        }
        node.num = Some(value);
    }

    fn parse_num(&mut self, word: &str) -> Option<u32> {
        None
    }
}
