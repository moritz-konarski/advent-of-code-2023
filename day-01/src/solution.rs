use crate::number_parser::NumberParser;

pub fn part1(file: &'static str) -> Result<u64, &'static str> {
    let digits = file
        .split_ascii_whitespace()
        .map(|line| {
            let left = line.chars().find_map(|c| c.to_digit(10));
            let right = line.chars().rev().find_map(|c| c.to_digit(10));
            match (left, right) {
                (Some(l), Some(r)) => Ok((l.into(), r.into())),
                (None, Some(_)) => Err("no left digit found"),
                (Some(_), None) => Err("no right digit found"),
                (None, None) => Err("no digits found"),
            }
        })
        .collect::<Result<Vec<(u64, u64)>, &'static str>>()?;

    Ok(digits.iter().fold(0, |sum, (l, r)| sum + 10 * l + r))
}

pub fn part2(file: &'static str) -> Result<u64, &'static str> {
    let parser = NumberParser::new();

    let digits = file
        .split_ascii_whitespace()
        .map(|line| {
            let left = parser.get_left(line);
            let right = parser.get_right(line);
            match (left, right) {
                (Some(l), Some(r)) => Ok((l.into(), r.into())),
                (None, Some(_)) => Err("no left digit found"),
                (Some(_), None) => Err("no right digit found"),
                (None, None) => Err("no digits found"),
            }
        })
        .collect::<Result<Vec<(u64, u64)>, &'static str>>()?;

    Ok(digits.iter().fold(0, |sum, (l, r)| sum + 10 * l + r))
}
