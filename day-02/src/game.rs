pub struct Game {
    pub id: u32,
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
        self.draws.iter().fold(Draw::default(), |min, d| min.min(d))
    }
}

#[derive(Default)]
pub struct Draw {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Draw {
    pub const fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn from_str(s: &str) -> Result<Self, &'static str> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for v in s.split(", ").map(Self::parse_part) {
            match v {
                Some((n, "red")) => red = n,
                Some((n, "green")) => green = n,
                Some((n, "blue")) => blue = n,
                Some(_) => return Err("cannot parse color part"),
                _ => return Err("cannot parse number part"),
            }
        }

        Ok(Self { red, green, blue })
    }

    fn parse_part(part: &str) -> Option<(u32, &str)> {
        part.split_once(' ')
            .and_then(|(num, col)| num.parse().ok().map(|n| (n, col)))
    }

    pub fn min(&self, other: &Self) -> Self {
        let red = self.red.max(other.red);
        let green = self.green.max(other.green);
        let blue = self.blue.max(other.blue);
        Self { red, green, blue }
    }

    pub fn pow(&self) -> u32 {
        self.red.max(1) * self.green.max(1) * self.blue.max(1)
    }

    const fn is_legal(&self, o: &Self) -> bool {
        self.red <= o.red && self.green <= o.green && self.blue <= o.blue
    }
}
