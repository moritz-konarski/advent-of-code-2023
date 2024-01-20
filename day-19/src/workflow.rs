use crate::parts::{AllParts, Field, Part};

#[derive(Debug, Clone)]
pub enum Rule {
    GreaterThan(Field, u64, String),
    LessThan(Field, u64, String),
    Identity(String),
}

#[derive(Debug, Clone)]
pub struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    pub fn new(line: &str) -> Result<Self, &'static str> {
        let rules = line
            .split(',')
            .map(|rule| {
                // filter out identities
                if !rule.contains(':') {
                    return Ok(Rule::Identity(rule.to_string()));
                }

                // safe because of check above
                let (rule, tag) = rule.split_once(':').unwrap();

                // get field name
                let field = match rule.chars().nth(0) {
                    Some('x') => Ok(Field::X),
                    Some('m') => Ok(Field::M),
                    Some('a') => Ok(Field::A),
                    Some('s') => Ok(Field::S),
                    _ => Err("invalid field id in Rule"),
                };

                let number = match rule.get(2..).unwrap_or_default().parse() {
                    Ok(n) => Ok(n),
                    Err(_) => Err("cannot parse number in Rule"),
                };

                match (field, number) {
                    (Ok(f), Ok(n)) => match rule.chars().nth(1) {
                        Some('<') => Ok(Rule::LessThan(f, n, tag.to_string())),
                        Some('>') => Ok(Rule::GreaterThan(f, n, tag.to_string())),
                        _ => Err("invalid operation in Rule"),
                    },
                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => Err(e),
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        if rules.is_empty() || !rules.iter().any(|r| matches!(r, Rule::Identity(_))) {
            Err("Rules are not exhaustive")
        } else {
            Ok(Self { rules })
        }
    }

    pub fn process(&self, part: &Part) -> &String {
        for rule in &self.rules {
            match rule {
                Rule::GreaterThan(field, number, tag) => {
                    if part.get(field) > number {
                        return tag;
                    }
                }
                Rule::LessThan(field, number, tag) => {
                    if part.get(field) < number {
                        return tag;
                    }
                }
                Rule::Identity(tag) => return tag,
            }
        }
        unreachable!("rules are exhaustive")
    }

    pub fn fit_part_through_workflow(&self, mut part: AllParts, next_tag: &String) -> AllParts {
        for rule in &self.rules {
            match rule {
                Rule::GreaterThan(field, number, tag) => {
                    if tag == next_tag {
                        part.get_mut(field).0 = number + 1;
                    } else {
                        part.get_mut(field).1 = *number;
                    }
                }
                Rule::LessThan(field, number, tag) => {
                    if tag == next_tag {
                        part.get_mut(field).1 = number.saturating_sub(1);
                    } else {
                        part.get_mut(field).0 = *number;
                    }
                }
                Rule::Identity(_) => { /* no action required */ }
            }
        }

        part
    }
}
