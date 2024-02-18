use std::cmp::Ordering;

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

    pub const fn end(&self) -> i64 {
        self.end
    }

    const fn new(start: i64, end: i64, offset: i64) -> Result<Self, &'static str> {
        if start < end {
            Ok(Self { start, end, offset })
        } else {
            Err("Mapping must have positive length")
        }
    }

    pub const fn is_before(&self, other: &Self) -> bool {
        self.end <= other.start
    }

    pub const fn is_after(&self, other: &Self) -> bool {
        self.start >= other.end
    }

    pub fn map_i64(&self, seed: i64) -> i64 {
        self.offset + seed
    }

    pub fn map(&self, other: &Self) -> Option<(Self, Option<Self>, Option<Self>)> {
        if self.is_before(other) || self.is_after(other) {
            return None;
        }

        use Ordering as O;
        match (self.start.cmp(&other.start), self.end.cmp(&other.end)) {
            (O::Equal, O::Equal)
            | (O::Equal, O::Greater)
            | (O::Less, O::Equal)
            | (O::Less, O::Greater) => {
                // |--self--| |----self----| |-self---| |--self---|
                // |-other--| |--other--|     |-other-|  |-other-|
                let mapped_other = Self {
                    start: self.map_i64(other.start),
                    end: self.map_i64(other.end),
                    offset: other.offset,
                };
                Some((mapped_other, None, None))
            }
            (O::Equal, O::Less) | (O::Less, O::Less) => {
                // |-self--|   |--self--|
                // |--other--|   |--other--|
                let mapped_other = Self {
                    start: self.map_i64(other.start),
                    end: self.map_i64(self.end),
                    offset: other.offset,
                };
                let remaining_other = Self {
                    start: self.end,
                    end: other.end,
                    offset: other.offset,
                };
                Some((mapped_other, None, Some(remaining_other)))
            }
            (O::Greater, O::Equal) | (O::Greater, O::Greater) => {
                //   |-self--|   |--self--|
                // |--other--| |--other-|
                let mapped_other = Self {
                    start: self.map_i64(self.start),
                    end: self.map_i64(other.end),
                    offset: other.offset,
                };
                let remaining_other = Self {
                    start: other.start,
                    end: self.start,
                    offset: other.offset,
                };
                Some((mapped_other, Some(remaining_other), None))
            }
            (O::Greater, O::Less) => {
                //   |-self-|
                // |--other---|
                let remaining_left = Self {
                    start: other.start,
                    end: self.start,
                    offset: other.offset,
                };
                let mapped_other = Self {
                    start: self.map_i64(self.start),
                    end: self.map_i64(self.end),
                    offset: other.offset,
                };
                let remaining_right = Self {
                    start: self.end,
                    end: other.end,
                    offset: other.offset,
                };
                Some((mapped_other, Some(remaining_left), Some(remaining_right)))
            }
        }
    }
}
