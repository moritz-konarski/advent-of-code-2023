use crate::{
    mapping::Mapping,
    parse::{get_mapset, get_seeds},
};

pub fn part1(file: &'static str) -> Result<u64, &'static str> {
    let (seeds, mappings) = file.split_once('\n').ok_or("Cannot find seed line")?;

    let seed_list = get_seeds(seeds)?;
    let map = get_mapset(mappings)?;

    for s in &seed_list {
        println!("{s:?} -> {:?}", map.map_seed(s));
    }

    let m = Mapping::new(110, 50, 200)?;
    println!("{m:?}");
    println!("{map:?}");
    println!("{:?}", map.map_mapping(m)?);
    println!("{:?}", map.map_mapping(m)?);

    // *minima.par_iter().min().unwrap()
    Err("not impl")
}

pub fn part2(file: &'static str) -> Result<u64, &'static str> {
    Ok(0)
}
