use crate::game::{Draw, Game};

const COUNTS: Draw = Draw {
    red: Some(12),
    green: Some(13),
    blue: Some(14),
};

pub fn part1(file: &'static str) -> Result<u64, &'static str> {
    let games = file
        .split_terminator('\n')
        .filter(|l| !l.is_empty())
        .map(Game::from_line)
        .collect::<Result<Vec<_>, _>>()?;

    let valid_ids = games
        .iter()
        .filter_map(|g| g.is_legal(&COUNTS).then_some(g.id));

    Ok(valid_ids.sum())
}

pub fn part2(file: &'static str) -> Result<u64, &'static str> {
    let games = file
        .split_terminator('\n')
        .filter(|l| !l.is_empty())
        .map(Game::from_line)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(games.iter().filter_map(|g| g.min_draw().pow()).sum())
}
