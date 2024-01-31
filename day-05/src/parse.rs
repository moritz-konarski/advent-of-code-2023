use crate::mapping::{Mapping, MapSet};

pub fn get_seeds(line: &'static str) -> Result<Vec<Mapping>, &'static str> {
    let (_, seeds) = line.split_once(':').ok_or("cannot split by :")?;

    seeds
        .split_whitespace()
        .map(|s| {
            s.parse().map_or_else(
                |_| Err("cannot parse seed"),
                |seed| Ok(Mapping::new_single_seed(seed)),
            )
        })
        .collect()
}

pub fn get_range_seeds(lines: &mut Lines<BufReader<File>>) -> Vec<Mapping> {
    let (_, seeds) = line.split_once(':').ok_or("cannot split by :")?;

    seeds
        .split_whitespace()
        .map(|s| {
            s.parse().map_or_else(
                |_| Err("cannot parse seed"),
                |seed| Mapping::new(seed, seed, 1),
            )
        })
        .collect()

    // TODO: fix stuff below here

    let seeds: Vec<_> = seeds
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    seeds
        .chunks_exact(2)
        .map(|chunk| Mapping::new(chunk[0], chunk[0], chunk[1]))
        .collect()
}

pub fn get_map(lines: &'static str) -> MapSet {
    let mut map = MapSet::new();

    while let Some(Ok(mut line)) = lines.next() {
        // skip lines that indicate a description
        if line.ends_with(':') || line.is_empty() {
            continue;
        }

        // build map from lines
        map.add_map();
        while !line.is_empty() {
            map.parse_line(&line);

            match lines.next() {
                Some(Ok(new_line)) => line = new_line,
                _ => break,
            }
        }
    }

    map
}
