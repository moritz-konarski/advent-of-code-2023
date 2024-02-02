use std::collections::BTreeMap;

pub struct Mapping {
    src: std::ops::Range<usize>,
    dst: usize,
}

impl Mapping {
    pub const fn new_single_seed(src: usize) -> Self {
        Self {
            src: (src..=src),
            dst: src,
        }
    }

    const fn new(src: usize, dst: usize, len: usize) -> Self {
        Self {
            src: (src..src + len),
            dst,
        }
    }

    fn map(&self, seed: &usize) -> usize {
        self.dst + seed - self.src.start
    }

    fn contains(&self, seed: &usize) -> bool {
        self.src.contains(seed)
    }
}

pub struct MapSet {
    mappings: Vec<BTreeMap<usize, Mapping>>,
}

impl MapSet {
    fn new() -> Self {
        Self { mappings: vec![] }
    }

    fn add_map(&mut self) {
        self.mappings.push(BTreeMap::new());
    }

    fn parse_line(&mut self, line: &str) {
        let elements: Vec<usize> = line.splitn(3, ' ').map(|s| s.parse().unwrap()).collect();
        let dest_start = elements[0];
        let source_start = elements[1];
        let length = elements[2];

        let len = &self.mappings.len() - 1;
        let map = Mapping::new(source_start, dest_start, length);

        self.mappings[len].insert(source_start, map);
    }

    fn map(&self, seed_mapping: Mapping) -> Vec<usize> {
        let mut seeds: Vec<usize> = seed_mapping.src.collect();

        for map in &self.mappings {
            seeds.par_iter_mut().for_each(|num| {
                if let Some(found_mapping) = map.values().find(|mapping| mapping.contains(num)) {
                    *num = found_mapping.map(num);
                }
            })
        }

        seeds
    }
}
