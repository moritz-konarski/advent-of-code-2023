#[derive(Debug, Clone)]
struct Number {
    val: u32,
    range: std::ops::Range<usize>,
}

impl Number {
    fn from_vec(parts: &[(usize, char)]) -> Result<Vec<Self>, &'static str> {
        let mut last_i = 0;
        let mut sections: Vec<_> = parts
            .windows(2)
            .enumerate()
            .filter_map(|(i, w)| (w[1].0 - w[0].0 != 1).then_some(i))
            .map(|i| {
                let t = last_i;
                last_i = i + 1;
                &parts[t..=i]
            })
            .collect();
        sections.push(&parts[last_i..]);

        sections.iter().map(Self::new).collect()
    }

    fn new(parts: &&[(usize, char)]) -> Result<Self, &'static str> {
        let range = match (parts.first(), parts.last()) {
            #[allow(clippy::range_plus_one)]
            (Some((f, _)), Some((l, _))) => (*f)..(*l + 1),
            _ => return Err("no first and last in number"),
        };

        parts
            .iter()
            .map(|(_, c)| c)
            .collect::<String>()
            .parse()
            .map_or(Err("count not parse number from line"), |val| {
                Ok(Self { val, range })
            })
    }
}

#[derive(Debug, Clone)]
struct Symbol {
    val: char,
    index: usize,
    adjacent_nums: Option<Vec<Number>>,
}

impl Symbol {
    fn from_vec(pairs: &[(usize, char)]) -> Result<Vec<Self>, &'static str> {
        pairs.iter().map(Self::new).collect()
    }

    const fn new(pair: &(usize, char)) -> Result<Self, &'static str> {
        match pair.1 {
            '.' => Err("period found at this index"),
            val if val.is_ascii_punctuation() => Ok(Self {
                val,
                index: pair.0,
                adjacent_nums: None,
            }),
            _ => Err("no symbol found at this index"),
        }
    }

    fn add_num(&mut self, num: &Number) {
        if let Some(adj) = &mut self.adjacent_nums {
            adj.push(num.clone());
        } else {
            self.adjacent_nums = Some(vec![num.clone()]);
        }
    }

    fn sum(&self) -> Option<u32> {
        self.adjacent_nums
            .as_ref()
            .map(|v| v.iter().fold(0, |s, n| s + n.val))
    }

    fn is_gear(&self) -> bool {
        self.val == '*' && self.adjacent_nums.as_ref().is_some_and(|n| n.len() == 2)
    }

    fn product(&self) -> Option<u32> {
        self.adjacent_nums
            .as_ref()
            .map(|v| v.iter().fold(1, |p, n| p * n.val))
    }

    fn is_adjacent_to(&self, num: &Number) -> bool {
        num.range.contains(&self.index)
            || self.index == num.range.end
            || (num.range.start >= 1 && self.index == num.range.start - 1)
    }
}

#[derive(Debug, Clone)]
struct Line {
    numbers: Option<Vec<Number>>,
    symbols: Option<Vec<Symbol>>,
}

impl Line {
    const fn default() -> Self {
        Self {
            numbers: None,
            symbols: None,
        }
    }

    fn new(line: &'static str) -> Result<Self, &'static str> {
        let (numbers, symbols): (Vec<_>, Vec<_>) = line
            .char_indices()
            .filter(|(_, c)| *c != '.')
            .partition(|(_, c)| c.is_ascii_digit());

        let numbers = if numbers.is_empty() {
            None
        } else {
            Some(Number::from_vec(&numbers)?)
        };

        let symbols = if symbols.is_empty() {
            None
        } else {
            Some(Symbol::from_vec(&symbols)?)
        };

        Ok(Self { numbers, symbols })
    }

    fn add_numbers(&mut self, other: &Self) {
        if let Some(other_nums) = &other.numbers {
            if let Some(symbols) = &mut self.symbols {
                for sym in symbols {
                    for num in other_nums {
                        if sym.is_adjacent_to(num) {
                            sym.add_num(num);
                        } else if num.range.start > sym.index {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn sum(&self) -> u32 {
        self.symbols.as_ref().map_or_else(
            || 0,
            |sym| sym.iter().fold(0, |sum, s| sum + s.sum().map_or(0, |x| x)),
        )
    }

    fn sum_gears(&self) -> u32 {
        self.symbols.as_ref().map_or_else(
            || 0,
            |sym| {
                sym.iter()
                    .filter(|s| s.is_gear())
                    .fold(0, |sum, s| sum + s.product().map_or(0, |x| x))
            },
        )
    }
}

#[derive(Debug)]
pub struct Lines {
    previous: Line,
    current: Line,
}

impl Lines {
    pub fn new() -> Self {
        Self {
            previous: Line::default(),
            current: Line::default(),
        }
    }

    pub fn parse(&mut self, line: &'static str) -> Result<(), &'static str> {
        let new_line = Line::new(line)?;

        self.current = new_line.clone();
        self.previous.add_numbers(&self.current);
        self.current.add_numbers(&self.previous);
        self.current.add_numbers(&new_line);

        Ok(())
    }

    pub fn rotate(&mut self) {
        self.previous = self.current.clone();
        self.current = Line::default();
    }

    pub fn sum_all(&self) -> u32 {
        self.previous.sum()
    }

    pub fn sum_gears(&self) -> u32 {
        self.previous.sum_gears()
    }
}
