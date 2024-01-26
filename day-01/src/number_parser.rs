use std::collections::HashMap;

#[derive(Default)]
pub struct NumberParser {
    digit: Option<u32>,
    children: Option<HashMap<&'static u8, Self>>,
}

impl NumberParser {
    pub fn new() -> Self {
        let mut root = Self::default();

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
        line.char_indices()
            .find_map(|(i, c)| self.parse(c, line.get(i..)))
    }

    pub fn get_right(&self, line: &'static str) -> Option<u32> {
        line.char_indices()
            .rev()
            .find_map(|(i, c)| self.parse(c, line.get(i..)))
    }

    fn parse(&self, c: char, line: Option<&'static str>) -> Option<u32> {
        c.to_digit(10).or_else(|| self.parse_str(line))
    }

    fn parse_str(&self, line: Option<&'static str>) -> Option<u32> {
        let line = line?;
        let mut node = self;

        line.as_bytes()
            .iter()
            .take(5)
            .take_while(|c| !c.is_ascii_digit())
            .find_map(|c| {
                node.children
                    .as_ref()
                    .and_then(|map| map.get(&c))
                    .and_then(|child| {
                        node = child;
                        node.digit
                    })
            })
    }

    fn add_word(&mut self, word: &'static str, digit: u32) {
        let mut node = self;

        for byte in word.as_bytes() {
            if node.children.is_none() {
                let mut map = HashMap::with_capacity(4);
                map.insert(byte, Self::default());
                node.children = Some(map);
            } else if let Some(map) = node.children.as_mut() {
                if !map.contains_key(byte) {
                    map.insert(byte, Self::default());
                }
            }

            let next_node = node
                .children
                .as_mut()
                .expect("children cannot be none")
                .get_mut(&byte)
                .expect("entry cannot be none");
            node = next_node;
        }

        node.digit = Some(digit);
    }
}
