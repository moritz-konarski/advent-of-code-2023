struct Number {
    val: u32,
    range: std::ops::Range<usize>,
}

impl Number {
    fn new(line: &str, start: usize) -> Result<Self, &'static str> {
        let Some((end, _)) = line
            .chars()
            .enumerate()
            .skip(start)
            .take_while(|(_, c)| c.is_ascii_digit())
            .last()
        else {
            return Err("could not find end of number");
        };

        let range = start..end + 1;

        match line.get(range.clone()).and_then(|s| s.parse().ok()) {
            Some(val) => Ok(Self { val, range }),
            None => Err("count not parse number from line"),
        }
    }
}

struct Symbol {
    val: char,
    index: usize,
    adjacent_nums: Option<Vec<Number>>,
}

impl Symbol {
    fn new(line: &str, index: usize) -> Result<Self, &'static str> {
        match line.get(index..index).and_then(|s| s.chars().next()) {
            Some('.') => Err("period found at this index"),
            Some(val) if val.is_ascii_punctuation() => Ok(Symbol {
                val,
                index,
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

struct Line {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

struct Lines {
    previous: Line,
    current: Line,
}
