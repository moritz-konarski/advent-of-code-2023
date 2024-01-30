pub fn part1(file: &'static str) -> Result<u64, &'static str> {
    let mut lines = file.lines();

    let seed_list = get_seeds(&mut lines)?;
    let map = get_map(&mut lines)?;

    let mut minima = Vec::new();
    for seed in seed_list {
        let mapped_seeds = map.map(seed);
        minima.push(*mapped_seeds.par_iter().min().unwrap());
    }

    *minima.par_iter().min().unwrap()
}

pub fn part2(file: &'static str) -> Result<u64, &'static str> {
    Ok(0)
}
