use std::{collections::BTreeMap, hash::Hash};

#[derive(Debug, Hash, Clone, Copy)]
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

    pub const fn new(start: i64, len: i64, destination: i64) -> Result<Self, &'static str> {
        let end = start + len;
        let offset = destination - start;
        Self::init(start, end, offset)
    }

    const fn init(start: i64, end: i64, offset: i64) -> Result<Self, &'static str> {
        if end <= start {
            Err("Mapping must have positive length")
        } else {
            Ok(Self { start, end, offset })
        }
    }

    fn split_at(self, point: i64) -> Result<(Self, Self), &'static str> {
        if point < self.start || point >= self.end {
            println!("trying to split {self:?} at {point:?}");
            Err("point does not split self")
        } else {
            let left_part = Self::init(self.start, point, self.offset)?;
            let right_part = Self::init(point, self.end, self.offset)?;

            Ok((left_part, right_part))
        }
    }

    pub fn from_str(line: &'static str) -> Result<Self, &'static str> {
        let mut parts = line.splitn(3, ' ').map_while(|e| e.parse().ok());

        let dest = parts.next().ok_or("Failed map destination parse")?;
        let start = parts.next().ok_or("Failed map start parse")?;
        let len = parts.next().ok_or("Failed map length parse")?;

        Self::new(start, len, dest)
    }

    pub fn map(&self, seed: &i64) -> i64 {
        self.offset + seed
    }

    pub const fn is_before(&self, other: &Self) -> bool {
        self.end <= other.start
    }

    pub const fn is_after(&self, other: &Self) -> bool {
        self.start >= other.end
    }

    pub const fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn split_around(self, other: &Self) -> Result<(Self, Self), &'static str> {
        let left_part = Self::init(self.start, other.start, self.offset)?;
        let right_part = Self::init(other.end, self.end, self.offset)?;

        Ok((left_part, right_part))
    }

    // TODO
    pub fn shrink_around(self, other: &Self) -> Result<Self, &'static str> {
        if other.start <= self.start {
            //     |--self--|
            // |--other--|
            Self::init(other.end, self.end, self.offset)
        } else {
            // |--self--|
            //     |--other--|
            Self::init(self.start, other.start, self.offset)
        }
    }
}

#[derive(Debug)]
pub struct MapSet {
    mappings: BTreeMap<i64, Mapping>,
}

impl MapSet {
    pub fn new() -> Self {
        let map = Mapping::default();
        Self {
            mappings: BTreeMap::from([(map.end, map)]),
        }
    }

    pub fn add_mappings(&mut self, mappings: &[Mapping]) -> Result<(), &'static str> {
        let mappings = mappings
            .iter()
            // .inspect(|m| println!("adding {m:?}"))
            .map(|m| self.map_mapping(m))
            .collect::<Result<Vec<_>, _>>()?;

        for m in mappings.iter().flatten() {
            self.add_mapping(m)?;
        }
        Ok(())
    }

    fn find_overlapping_keys(&self, other: &Mapping) -> Vec<i64> {
        let mut affected_keys = vec![];

        for (key, mapping) in self.mappings.iter() {
            // println!("checking k:{key:?} v:{mapping:?}");
            if mapping.is_before(&other) {
                // println!("mapping before new");
                continue;
            }
            if mapping.is_after(&other) {
                // println!("mapping after new");
                break;
            }
            // println!("mapping affected");
            affected_keys.push(*key);
        }

        affected_keys
    }

    pub fn add_mapping(&mut self, other: &Mapping) -> Result<(), &'static str> {
        let affected_keys = self.find_overlapping_keys(other);

        for key in &affected_keys {
            // remove old map for processing
            let old_map = self
                .mappings
                .remove(key)
                .ok_or("Could not find key in map")?;

            // handle case where new map is inside one current map
            if old_map.contains(other) {
                println!("containment:\n {other:?}\n in {old_map:?}");
                let (left, right) = old_map.split_around(other)?;
                self.mappings.insert(left.end, left);
                self.mappings.insert(right.end, right);
                break;
            }

            // handle case where new key usurps part of current key
            let shrunk_map = old_map.shrink_around(other)?;
            // println!("shrinkage:\n old: {old_map:?}\n new: {other:?}\n shrunk: {shrunk_map:?}");
            self.mappings.insert(shrunk_map.end, shrunk_map);
        }

        // insert new key
        self.mappings.insert(other.end, *other);

        Ok(())
    }

    pub fn map_seed(&self, seed: &i64) -> Option<i64> {
        self.mappings
            .iter()
            .find(|(k, _)| seed < k)
            .map(|(_, v)| v.map(seed))
    }

    pub fn map_mapping(&self, seed_range: &Mapping) -> Result<Vec<Mapping>, &'static str> {
        let mut seed_range = *seed_range;
        let affected_keys = self.find_overlapping_keys(&seed_range);
        let mut resulting_mappings = vec![];

        for key in &affected_keys {
            let current_map = self.mappings.get(key).ok_or("Could not find key in map")?;

            if current_map.contains(&seed_range) {
                println!("containment:\n {seed_range:?}\n in {current_map:?}");
                let new_start = current_map.map(&seed_range.start);
                let new_end = current_map.map(&seed_range.end);
                println!("mapped to: {new_start:?} - {new_end:?}");
                resulting_mappings.push(Mapping::init(new_start, new_end, seed_range.offset)?);
                break;
            }

            println!("we are partially contained");
            if current_map.start <= seed_range.start {
                let (left, right) = seed_range.split_at(current_map.end)?;
                seed_range = right;
                let new_start = current_map.map(&left.start);
                let new_end = current_map.map(&left.end);
                resulting_mappings.push(Mapping::init(new_start, new_end, left.offset)?);
            } else {
                return Err("reached seed range starting before current map");
            }
        }

        Ok(resulting_mappings)
    }
}
