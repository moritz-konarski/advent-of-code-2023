use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-07 p<n>";
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

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
enum Card {
    _A = 12,
    _K = 11,
    _Q = 10,
    _J = 9,
    _T = 8,
    _9 = 7,
    _8 = 6,
    _7 = 5,
    _6 = 4,
    _5 = 3,
    _4 = 2,
    _3 = 1,
    _2 = 0,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
enum Card2 {
    _A = 12,
    _K = 11,
    _Q = 10,
    _T = 9,
    _9 = 8,
    _8 = 7,
    _7 = 6,
    _6 = 5,
    _5 = 4,
    _4 = 3,
    _3 = 2,
    _2 = 1,
    _J = 0,
}

const CHAR_TO_CARD_LIST: [(char, Card); 13] = [
    ('A', Card::_A),
    ('K', Card::_K),
    ('Q', Card::_Q),
    ('J', Card::_J),
    ('T', Card::_T),
    ('9', Card::_9),
    ('8', Card::_8),
    ('7', Card::_7),
    ('6', Card::_6),
    ('5', Card::_5),
    ('4', Card::_4),
    ('3', Card::_3),
    ('2', Card::_2),
];

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    Quintuplet = 6,
    Quadruplet = 5,
    FullHouse = 4,
    Triplet = 3,
    TwoPair = 2,
    Pair = 1,
    HighCard = 0,
}

struct Hand {
    cards: Vec<Card>,
    poker_type: HandType,
    bid: usize,
}

impl Hand {
    fn new(line: &str, map: &HashMap<char, Card>) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();

        let cards: Vec<Card> = cards.chars().map(|c| *map.get(&c).unwrap()).collect();
        let bid = bid.parse().unwrap();

        Self {
            cards,
            poker_type: HandType::HighCard,
            bid,
        }
    }

    fn parse_hand_type(&mut self) {
        let mut card_to_count = HashMap::with_capacity(self.cards.len());
        self.cards
            .iter()
            .for_each(|card| *card_to_count.entry(card).or_insert(0) += 1);

        self.poker_type = match card_to_count.len() {
            1 => HandType::Quintuplet,
            2 => {
                if *card_to_count.values().max().unwrap() == 4 {
                    HandType::Quadruplet
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if *card_to_count.values().max().unwrap() == 3 {
                    HandType::Triplet
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::Pair,
            5 => HandType::HighCard,
            _ => unreachable!("there are no other options"),
        };
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let rank_ord = self.poker_type.cmp(&other.poker_type);
        if rank_ord != Ordering::Equal {
            return rank_ord;
        }

        if let Some((card, other_card)) = self
            .cards
            .iter()
            .zip(&other.cards)
            .find(|(card, other_card)| card != other_card)
        {
            card.cmp(other_card)
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.poker_type == other.poker_type
    }
}

impl Eq for Hand {}

fn part1(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let char_to_card = HashMap::from(CHAR_TO_CARD_LIST);
    let mut hands = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();
        let mut hand = Hand::new(&line, &char_to_card);
        hand.parse_hand_type();
        hands.push(hand);
    }

    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .fold(0, |sum, (i, hand)| sum + (i + 1) * hand.bid)
}

fn part2(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    let char_to_card = HashMap::from(CHAR_TO_CARD_LIST);
    let mut hands = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();
        hands.push(Hand::new(&line, &char_to_card));
    }

    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .fold(0, |sum, (i, hand)| sum + (i + 1) * hand.bid)
}

#[test]
fn part1_example() {
    assert_eq!(6440, part1("test1.txt"));
}

#[test]
fn part1_puzzle() {
    assert_eq!(250058342, part1(PART1_FILE));
}

#[test]
fn part2_example() {
    assert_eq!(5905, part2("test2.txt"));
}

// #[test]
// fn part2_puzzle() {
//     assert_eq!(35150181, part2(PART2_FILE));
// }
