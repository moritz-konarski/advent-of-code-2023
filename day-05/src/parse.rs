use crate::mapping::{MapSet, Mapping};

pub fn get_seeds(line: &'static str) -> Result<Vec<i64>, &'static str> {
    let (_, seeds) = line.split_once(':').ok_or("Cannot split seed line by :")?;

    seeds
        .split_whitespace()
        .map(|s| s.parse().map_err(|_| "Cannot parse seed"))
        .collect()
}

pub fn get_range_seeds(line: &'static str) -> Result<Vec<Mapping>, &'static str> {
    let (_, seeds) = line.split_once(':').ok_or("cannot split by :")?;

    todo!()

    // seeds
    //     .split_whitespace()
    //     .map(|s| {
    //         s.parse().map_or_else(
    //             |_| Err("cannot parse seed"),
    //             |seed| Mapping::new(seed, seed, 1),
    //         )
    //     })
    //     .collect()

    // // TODO: fix stuff below here

    // let seeds: Vec<_> = seeds
    //     .split_whitespace()
    //     .map(|s| s.parse().unwrap())
    //     .collect();

    // seeds
    //     .chunks_exact(2)
    //     .map(|chunk| Mapping::new(chunk[0], chunk[0], chunk[1]))
    //     .collect()
}

pub fn get_mapset(lines: &'static str) -> Result<MapSet, &'static str> {
    let mappings = lines
        .lines()
        .filter(|line| !line.is_empty() && !line.ends_with(':'))
        .map(|line| Mapping::from_str(line))
        .collect::<Result<_, _>>()?;

    MapSet::from_mappings(mappings)
}
