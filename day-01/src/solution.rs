use crate::number_parser::NumberParser;

fn get_digits<F, G>(
    file: &'static str,
    get_left: F,
    get_right: G,
) -> Result<Vec<(u32, u32)>, &'static str>
where
    F: Fn(&'static str) -> Option<u32>,
    G: Fn(&'static str) -> Option<u32>,
{
    file.lines()
        .filter(|line| !line.is_empty())
        .map(|line| match (get_left(line), get_right(line)) {
            (Some(l), Some(r)) => Ok((l, r)),
            (None, Some(_)) => Err("no left digit found"),
            (Some(_), None) => Err("no right digit found"),
            (None, None) => Err("no digits found"),
        })
        .collect()
}

pub fn part1(file: &'static str) -> Result<u64, &'static str> {
    let digits = get_digits(
        file,
        |l| l.chars().find_map(|c| c.to_digit(10)),
        |l| l.chars().rev().find_map(|c| c.to_digit(10)),
    )?;

    Ok(digits
        .iter()
        .fold(0, |sum, (l, r)| sum + 10 * *l + *r)
        .into())
}

pub fn part2(file: &'static str) -> Result<u64, &'static str> {
    let parser = NumberParser::new();

    let digits = get_digits(file, |l| parser.get_left(l), |l| parser.get_right(l))?;

    Ok(digits
        .iter()
        .fold(0, |sum, (l, r)| sum + 10 * *l + *r)
        .into())
}
