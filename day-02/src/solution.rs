use crate::game::{Draw, Game};

pub fn part1(file: &'static str) -> Result<u64, &'static str> {
    let counts = Draw::new(12, 13, 14);

    let games = file
        .lines()
        .filter(|l| !l.is_empty())
        .map(Game::from_line)
        .collect::<Result<Vec<_>, _>>()?;

    let valid_ids = games
        .iter()
        .filter_map(|g| g.is_legal(&counts).then_some(g.id));

    Ok(valid_ids.sum::<u32>().into())
}

pub fn part2(file: &'static str) -> Result<u64, &'static str> {
    let games = file
        .lines()
        .filter(|l| !l.is_empty())
        .map(Game::from_line)
        .collect::<Result<Vec<_>, _>>()?;

    let min_draws = games.iter().map(|g| g.min_draw().pow());

    Ok(min_draws.sum::<u32>().into())
}
