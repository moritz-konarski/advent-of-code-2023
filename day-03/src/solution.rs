use crate::line::Lines;

pub fn part1(file: &'static str) -> Result<u64, &'static str> {
    let mut line_pair = Lines::new();
    let mut sum = 0;

    for line in file.lines() {
        line_pair.parse(line)?;
        sum += line_pair.sum_all();
        line_pair.rotate();
    }
    sum += line_pair.sum_all();

    Ok(sum.into())
}

pub fn part2(file: &'static str) -> Result<u64, &'static str> {
    let mut line_pair = Lines::new();
    let mut sum = 0;

    for line in file.lines() {
        line_pair.parse(line)?;
        sum += line_pair.sum_gears();
        line_pair.rotate();
    }
    sum += line_pair.sum_gears();

    Ok(sum.into())
}
