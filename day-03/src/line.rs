#[derive(Debug)]
struct Number {
    val: u32,
    range: std::ops::Range<usize>,
}

impl Number {
    fn new(parts: &&[(usize, char)]) -> Result<Self, &'static str> {
        let range = match (parts.first(), parts.last()) {
            (Some((f, _)), Some((l, _))) => *f..*l + 1,
            _ => return Err("no first and last in number"),
        };

        match parts
            .iter()
            .map(|(_, c)| c)
            .collect::<String>()
            .parse()
            .ok()
        {
            Some(val) => Ok(Self { val, range }),
            None => Err("count not parse number from line"),
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
    fn new(pair: &(usize, char)) -> Result<Self, &'static str> {
        let (i, c) = pair;
        match c {
            '.' => Err("period found at this index"),
            val if val.is_ascii_punctuation() => Ok(Self {
                val: *val,
                index: *i,
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
        let enumerate_chars = line.char_indices().collect::<Vec<_>>();
        // TODO: map out nums and syms not separated by .
        let (numbers, symbols): (Vec<_>, Vec<_>) = enumerate_chars
            .split(|(_, c)| *c == '.')
            .filter(|v| !v.is_empty())
            .partition(|v| v[0].1.is_ascii_digit());

        let numbers = if numbers.is_empty() {
            None
        } else {
            Some(
                numbers
                    .iter()
                    .inspect(|v| println!("{v:?}"))
                    .map(Number::new)
                    .collect::<Result<_, _>>()?,
            )
        };

        let symbols = if symbols.is_empty() {
            None
        } else {
            Some(
                symbols
                    .iter()
                    .map_while(|v| v.last())
                    .map(Symbol::new)
                    .collect::<Result<_, _>>()?,
            )
        };

        Ok(Self { numbers, symbols })
    }
}

struct Lines {
    previous: Line,
    current: Line,
}
