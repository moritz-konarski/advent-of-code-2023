use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const COUNTS: Counts = Counts {
    red: 12,
    green: 13,
    blue: 14,
};
const GAME_DELIM: &str = ": ";
const DRAW_DELIM: &str = "; ";
const COLOR_DELIM: &str = ", ";
const COUNT_DELIM: &str = " ";
const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";
const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-02 p<n>";
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

fn get_legal_game_id(line: &String) -> Option<u32> {
    let (game, draws) = line.split_once(GAME_DELIM).unwrap();

    for draw in draws.split(DRAW_DELIM) {
        for color_draw in draw.splitn(3, COLOR_DELIM) {
            let (count, color) = color_draw.split_once(COUNT_DELIM).unwrap();

            let count = u32::from_str_radix(count, 10).expect("should work");
            let limit = match color {
                RED => COUNTS.red,
                BLUE => COUNTS.blue,
                GREEN => COUNTS.green,
                _ => unreachable!("there are no other colors"),
            };

            if count > limit {
                return None;
            }
        }
    }

    let (_, id) = game.split_once(COUNT_DELIM).unwrap();
    Some(u32::from_str_radix(id, 10).expect("should work"))
}

fn part1(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the value");
    let file = BufReader::new(file);

    file.lines().fold(0, |sum, line| {
        let l = line.unwrap();

        if let Some(id) = get_legal_game_id(&l) {
            sum + id
        } else {
            sum
        }
    })
}

fn get_required_powers(line: &String) -> u32 {
    let (_, draws) = line.split_once(GAME_DELIM).unwrap();
    let mut counter = Counts::new();

    draws
        .split(DRAW_DELIM)
        .for_each(|draw| counter.update_max(draw));

    counter.power()
}

fn part2(filename: &str) -> u32 {
    let file = File::open(filename).expect("Should be able to read the value");
    let file = BufReader::new(file);

    file.lines().fold(0, |sum, line| {
        let l = line.unwrap();
        sum + get_required_powers(&l)
    })
}

#[test]
fn part1_example() {
    assert_eq!(8, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(2545, part1(PART1_FILE));
}

#[test]
fn part2_example() {
    assert_eq!(2286, part2("test2.txt"));
}

#[test]
fn part2_puzzle() {
    assert_eq!(78111, part2(PART2_FILE));
}

struct Counts {
    red: u32,
    green: u32,
    blue: u32,
}

impl Counts {
    fn new() -> Self {
        Counts {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn power(self) -> u32 {
        self.red * self.green * self.blue
    }

    fn update_max(&mut self, draw: &str) {
        for color_draw in draw.splitn(3, COLOR_DELIM) {
            let (count, color) = color_draw.split_once(COUNT_DELIM).unwrap();

            let count = u32::from_str_radix(count, 10).expect("should work");
            match color {
                RED => self.red = self.red.max(count),
                BLUE => self.blue = self.blue.max(count),
                GREEN => self.green = self.green.max(count),
                _ => unreachable!("there are no other colors"),
            };
        }
    }
}
