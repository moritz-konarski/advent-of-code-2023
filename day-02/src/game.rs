pub struct Game {
    pub id: u64,
    draws: Vec<Draw>,
}

impl Game {
    pub fn from_line(line: &'static str) -> Result<Self, &'static str> {
        let Some((game, draws)) = line.split_once(": ") else {
            return Err("invalid line, no : found");
        };
        let Some(id) = game.get(5..).and_then(|n| n.parse().ok()) else {
            return Err("cannot parse game id");
        };
        let draws = draws
            .split("; ")
            .map(Draw::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Self { id, draws })
    }

    pub fn is_legal(&self, other: &Draw) -> bool {
        self.draws.iter().all(|d| d.is_legal(other))
    }

    pub fn min_draw(&self) -> Draw {
        let initial = Draw {
            red: None,
            green: None,
            blue: None,
        };

        self.draws.iter().fold(initial, |min, d| min.min(d))
    }
}

pub struct Draw {
    pub red: Option<u64>,
    pub green: Option<u64>,
    pub blue: Option<u64>,
}

impl Draw {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        let mut red = None;
        let mut green = None;
        let mut blue = None;

        for v in s.split(", ").map(Self::parse_part) {
            match v {
                Ok((n, c)) => match c {
                    "red" => red = n,
                    "green" => green = n,
                    "blue" => blue = n,
                    _ => return Err("illegal color value for Draw"),
                },
                Err(e) => return Err(e),
            }
        }

        Ok(Self { red, green, blue })
    }

    fn parse_part(part: &str) -> Result<(Option<u64>, &str), &'static str> {
        part.split_once(' ').map_or_else(
            || Err("no space found in Daw"),
            |(n, c)| Ok((n.parse().ok(), c)),
        )
    }

    pub fn min(&self, other: &Self) -> Self {
        let red = match (self.red, other.red) {
            (Some(r), Some(o)) => Some(r.max(o)),
            (Some(r), None) => Some(r),
            (None, Some(o)) => Some(o),
            _ => None,
        };

        let green = match (self.green, other.green) {
            (Some(g), Some(o)) => Some(g.max(o)),
            (Some(g), None) => Some(g),
            (None, Some(o)) => Some(o),
            _ => None,
        };

        let blue = match (self.blue, other.blue) {
            (Some(b), Some(o)) => Some(b.max(o)),
            (Some(b), None) => Some(b),
            (None, Some(o)) => Some(o),
            _ => None,
        };

        Self { red, green, blue }
    }

    pub const fn pow(&self) -> Option<u64> {
        match (self.red, self.green, self.blue) {
            (Some(r), Some(g), Some(b)) => Some(r * g * b),
            (None, Some(g), Some(b)) => Some(g * b),
            (Some(r), None, Some(b)) => Some(r * b),
            (Some(r), Some(g), None) => Some(r * g),
            (Some(r), _, _) => Some(r),
            (_, Some(g), _) => Some(g),
            (_, _, Some(b)) => Some(b),
            _ => None,
        }
    }

    const fn is_legal(&self, other: &Self) -> bool {
        match (self.red, other.red) {
            (Some(r), Some(o)) if r > o => return false,
            _ => { /* not a false result */ }
        }

        match (self.green, other.green) {
            (Some(g), Some(o)) if g > o => return false,
            _ => { /* not a false result */ }
        }

        match (self.blue, other.blue) {
            (Some(b), Some(o)) if b > o => return false,
            _ => { /* not a false result */ }
        }
        true
    }
}
