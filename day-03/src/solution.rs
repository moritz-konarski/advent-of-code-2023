use crate::line::Line;

pub fn part1(file: &'static str) -> Result<u64, &'static str> {
    file.lines()
        .inspect(|l| println!("{l}"))
        .map(Line::new)
        .for_each(|l| println!("{l:?}"));
    Ok(0)
}

pub fn part2(file: &'static str) -> Result<u64, &'static str> {
    Ok(0)
}
