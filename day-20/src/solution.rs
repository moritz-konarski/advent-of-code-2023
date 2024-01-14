use crate::modules::{Broadcaster, Module};
use std::{collections::BTreeMap, fs};

fn parse_modules(lines: &[&str]) -> (Broadcaster, BTreeMap<String, Module>) {
    todo!();
}

pub fn part1(filename: &str, press_count: u64) -> Result<u64, &'static str> {
    let file = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(_) => return Err("failed to open file"),
    };

    let lines = file.split_whitespace().collect::<Vec<_>>();

    let (broadcaster, modules) = parse_modules(&lines);

    Ok(0)
}

pub fn part2(filename: &str, press_count: u64) -> Result<u64, &'static str> {
    let file = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(_) => return Err("failed to open file"),
    };

    let lines = file.split_whitespace().collect::<Vec<_>>();

    Ok(0)
}
