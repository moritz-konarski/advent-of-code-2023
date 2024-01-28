#[derive(Debug)]
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
                &parts[t..i + 1]
            })
            .collect();
        sections.push(&parts[last_i..]);

        sections.iter().map(Self::new).collect()
    }

    fn new(parts: &&[(usize, char)]) -> Result<Self, &'static str> {
        let range = match (parts.first(), parts.last()) {
            (Some((f, _)), Some((l, _))) => *f..*l + 1,
            _ => return Err("no first and last in number"),
        };

        match parts.iter().map(|(_, c)| c).collect::<String>().parse() {
            Ok(val) => Ok(Self { val, range }),
            Err(_) => Err("count not parse number from line"),
        }
    }
}

#[derive(Debug)]
struct Symbol {
    val: char,
    index: usize,
    adjacent_nums: Option<Vec<Number>>,
}

impl Symbol {
    fn from_vec(pairs: &[(usize, char)]) -> Result<Vec<Self>, &'static str> {
        pairs.iter().map(Self::new).collect()
    }

    fn new(pair: &(usize, char)) -> Result<Self, &'static str> {
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

    fn sum(&self) -> Option<u32> {
        self.adjacent_nums
            .as_ref()
            .and_then(|v| Some(v.iter().fold(0, |s, n| s + n.val)))
    }

    fn product(&self) -> Option<u32> {
        self.adjacent_nums
            .as_ref()
            .and_then(|v| Some(v.iter().fold(1, |p, n| p * n.val)))
    }

    fn is_adjacent_to(&self, num: &Number) -> bool {
        num.range.contains(&self.index)
            || self.index == num.range.end
            || (num.range.start >= 1 && self.index == num.range.start - 1)
    }
}

#[derive(Debug)]
pub struct Line {
    numbers: Option<Vec<Number>>,
    symbols: Option<Vec<Symbol>>,
}

impl Line {
    pub fn new(line: &'static str) -> Result<Self, &'static str> {
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

        // Ok(Self { numbers, symbols })
        Ok(Self {
            numbers,
            symbols: None,
        })
    }
}

struct Lines {
    previous: Line,
    current: Line,
}
