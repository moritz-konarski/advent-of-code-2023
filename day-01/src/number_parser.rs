pub struct NumberParser {
    letter: char,
    digit: Option<u32>,
    children: Vec<Self>,
}

impl NumberParser {
    pub fn new() -> Self {
        let mut root = Self::get_new_node('0');

        root.add_word("one", 1);
        root.add_word("two", 2);
        root.add_word("three", 3);
        root.add_word("four", 4);
        root.add_word("five", 5);
        root.add_word("six", 6);
        root.add_word("seven", 7);
        root.add_word("eight", 8);
        root.add_word("nine", 9);

        root
    }

    pub fn get_left(&self, line: &'static str) -> Option<u32> {
        line.chars()
            .enumerate()
            .inspect(|(i, c)| println!("l {i:?} {c}"))
            .find_map(|(i, c)| c.to_digit(10).or_else(|| self.parse(line.get(i..))))
    }

    pub fn get_right(&self, line: &'static str) -> Option<u32> {
        let line_len = line.len() - 1;
        line.chars()
            .rev()
            .enumerate()
            .inspect(|(i, c)| println!("r {i:?} {c}"))
            .find_map(|(i, c)| {
                c.to_digit(10)
                    .or_else(|| self.parse(line.get(line_len - i..)))
            })
    }

    const fn get_new_node(letter: char) -> Self {
        Self {
            letter,
            digit: None,
            children: Vec::new(),
        }
    }

    fn parse(&self, line: Option<&str>) -> Option<u32> {
        let line = line?;
        println!("parse {line}");
        let mut node = self;

        // TODO: make map while
        line.chars().find_map(|c| {
            node.child_position(c).and_then(|i| {
                node = &node.children[i];
                node.digit
            })
        })
    }

    fn child_position(&self, letter: char) -> Option<usize> {
        self.children.iter().position(|c| c.letter == letter)
    }

    fn add_word(&mut self, word: &'static str, digit: u32) {
        let mut node = self;

        for letter in word.chars() {
            let new_child = Self::get_new_node(letter);

            let index = if let Some(i) = node.child_position(letter) {
                i
            } else {
                node.children.push(new_child);
                node.children.len() - 1
            };

            node = &mut node.children[index];
        }

        node.digit = Some(digit);
    }
}
