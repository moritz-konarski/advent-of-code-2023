use crate::mapping::Mapping;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct MapSet {
    mappings: BTreeMap<i64, Mapping>,
}

impl MapSet {
    fn new() -> Self {
        let map = Mapping::default();
        Self {
            mappings: BTreeMap::from([(map.end(), map)]),
        }
    }

    fn find_intersecting_keys(&self, other: &Mapping) -> Vec<i64> {
        let mut keys = vec![];
        for (key, mapping) in self.mappings.iter() {
            if mapping.is_before(&other) {
                continue;
            }
            if mapping.is_after(&other) {
                break;
            }
            keys.push(*key);
        }
        keys
    }

    fn map_i64(&self, seed: &i64) -> Option<i64> {
        self.mappings
            .iter()
            .find(|(k, _)| seed < k)
            .map(|(_, v)| v.map_i64(*seed))
    }

    fn map(&self, map: &Mapping) -> Result<Vec<Mapping>, &'static str> {
        let intersecting_keys = self.find_intersecting_keys(map);
        let mut map = *map;
        let mut resulting_mappings = vec![];

        for key in &intersecting_keys {
            let current_map = self.mappings[key];

            match current_map.map(&map) {
                Some((m, None, None)) => {
                    resulting_mappings.push(m);
                    break;
                }
                Some((m, None, Some(right_remainder))) => {
                    resulting_mappings.push(m);
                    map = right_remainder;
                }
                Some((_, Some(_), Some(_))) | Some((_, Some(_), None)) => {
                    return Err("split map into two even though we're iterating from the left")
                }
                None => return Err("found non-intersecting map with intersecting key"),
            }
        }

        Ok(resulting_mappings)
    }

    fn add_mappings(&mut self, mappings: &[Mapping]) -> Result<(), &'static str> {
        let mappings = mappings
            .iter()
            // .inspect(|m| println!("adding {m:?}"))
            .map(|m| self.map(m))
            .collect::<Result<Vec<_>, _>>()?;

        for m in mappings.iter().flatten() {
            self.add_mapping(m)?;
        }
        Ok(())
    }

    pub fn add_mapping(&mut self, other: &Mapping) -> Result<(), &'static str> {
        let affected_keys = self.find_intersecting_keys(map);

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
}
