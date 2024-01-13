#[derive(Debug, Clone)]
pub enum Field {
    X,
    M,
    A,
    S,
}

pub struct Part {
    fields: [u64; 4],
}

impl Part {
    pub fn new(line: &&str) -> Result<Self, &'static str> {
        // remove { and } from start and end
        let line = match line.strip_prefix('{') {
            Some(leading_stripped) => match leading_stripped.strip_suffix('}') {
                Some(stripped) => stripped,
                None => return Err("no trailing `}` found in Part"),
            },
            None => return Err("no leading `{` found in Part"),
        };

        // try to parse all 4 fields
        let fields = line
            .splitn(4, ',')
            .map(|part| match part.split_once('=') {
                Some((_, num_str)) => match num_str.parse() {
                    Ok(num) => Ok(num),
                    Err(_) => Err("couldn't parse number in Part"),
                },
                None => Err("no `=` found in Part"),
            })
            .collect::<Result<Vec<_>, _>>()?;

        let fields: [u64; 4] = match fields.try_into() {
            Ok(f) => f,
            Err(_) => return Err("Could not parse 4 fields"),
        };

        Ok(Self { fields })
    }

    pub fn get(&self, field: &Field) -> &u64 {
        match field {
            Field::X => &self.fields[0],
            Field::M => &self.fields[1],
            Field::A => &self.fields[2],
            Field::S => &self.fields[3],
        }
    }

    pub fn score(&self) -> u64 {
        self.fields.iter().sum()
    }
}

#[derive(Debug)]
pub struct AllParts {
    fields: [(u64, u64); 4],
}

impl AllParts {
    pub fn new() -> Self {
        Self {
            fields: [(1, 4_000); 4],
        }
    }

    pub fn get_mut(&mut self, field: &Field) -> &mut (u64, u64) {
        match field {
            Field::X => &mut self.fields[0],
            Field::M => &mut self.fields[1],
            Field::A => &mut self.fields[2],
            Field::S => &mut self.fields[3],
        }
    }

    pub fn get(&self, field: &Field) -> &(u64, u64) {
        match field {
            Field::X => &self.fields[0],
            Field::M => &self.fields[1],
            Field::A => &self.fields[2],
            Field::S => &self.fields[3],
        }
    }
}
