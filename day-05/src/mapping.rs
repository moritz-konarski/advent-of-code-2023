use std::{collections::BTreeMap};

#[derive(Hash, Clone, Copy)]
pub struct Mapping {
    start: i64,
    end: i64,
    offset: i64,
}

impl Mapping {
    pub const fn default() -> Self {
        Self {
            start: 0,
            end: i64::MAX,
            offset: 0,
        }
    }

    pub const fn new(start: i64, len: i64, destination: i64) -> Self {
        Self {
            start,
            end: start + len,
            offset: destination - start,
        }
    }

    pub const fn map(&self, seed: &i64) -> i64{
        self.offset + seed
    }

    pub const fn overlaps(&self, other: &Self) -> bool {
        self.start >= other.start && self.start < other.end || self.end <= other.end && self.end > other.start
    }

    pub const fn is_contained_in(&self, other: &Self) -> bool {
        other.start <= self.start && other.end >= self.end
    }

    pub fn split(&self, other: &Self) -> (Self,Self) {
        let new_mapping = Self::new(other.end, self.end - other.end, self.offset);
        let mut new_self = self.clone();
        new_self.end = other.start - 1;

        (new_self, new_mapping)
    }

    pub fn shrink(&self, other: &Self) -> Self {
        let mut new_self = self.clone();

        if other.start <= self.start {
            new_self.start = other.end;
        } else {
            new_self.end = other.start - 1;            
        }

        new_self
    }
}

pub struct MapSet {
    mappings: BTreeMap<i64, Mapping>,
}

impl MapSet {
    pub fn new() -> Self {
        let mut mappings = BTreeMap::new();
        let map = Mapping::default();
        mappings.insert(map.end, map);

        Self { mappings  }
    }

    pub fn map(&self, seed: &i64) ->Option< i64> {
        self.mappings.iter().find(|(k, _)| seed < k).map(|(_, v)| v.map(seed))
    }

    pub fn add_map(&mut self, other: Mapping) {
        let mut affected_maps = vec![];

        for (key, mapping) in self.mappings.iter() {
            if other.end < mapping.start {
                continue; // no contact
            }
            if mapping.start >= other.end {
                break; // no contact
            } 
            
            if other.overlaps(mapping) {
                affected_maps.push((key, mapping));
            }
        }

        for (key, mapping) in affected_maps {
            if other.is_contained_in(&mapping) {
                self.mappings.remove(key);
                let (left, right) = mapping.split(&other);
                self.mappings.insert(left.end, left);
                self.mappings.insert(right.end, right);
                break;
            }

            if mapping.is_contained_in(&other) {
                self.mappings.remove(key);
            } else {
                let shrunk = mapping.shrink(&other);
                if shrunk.end == mapping.end {
                    self.mappings.remove(key);
                }
                self.mappings.insert(shrunk.end, shrunk);
            }
        }
        self.mappings.insert(other.end, other);
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

    fn map_individual(&self, seed: &usize) -> usize {
        let new_seed
        for map in &self.mappings {
            seeds.par_iter_mut().for_each(|num| {
                if let Some(found_mapping) = map.values().find(|mapping| mapping.contains(num)) {
                    *num = found_mapping.map(num);
                }
            })
        }

        seeds
    }

fn map_ranges(&self, seed_range: Mapping) -> Vec<usize> {
        let mut seeds: Vec<usize> = seed_mapping.src.collect();

        for map in &self.mappings {
            seeds.par_iter_mut().for_each(|num| {
                if let Some(found_mapping) = map.values().find(|mapping| mapping.contains(num)) {
                    *num = found_mapping.map(num);
                }
            })
        }

        seeds
    }}
