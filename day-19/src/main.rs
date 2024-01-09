use std::{collections::HashMap, env, fs, any::Any};

const START: &str = "in";
const ACCEPTED: &str = "A";
const REJECTED: &str = "R";

fn main() {
    let run_type = env::args().nth(1).unwrap_or_default();
    let number = env::args().nth(2).unwrap_or_default();

    let result = match (run_type.as_str(), number.as_str()) {
        ("t", "1") => part1("test1.txt"),
        ("p", "1") => part1("part1.txt"),
        ("t", "2") => part2("test2.txt"),
        ("p", "2") => part2("part2.txt"),
        _ => Err("Incorrect usage!\nUsage: cargo r -- [t|p] [1|2]"),
    };

    match result {
        Ok(number) => println!("Result for {run_type}{number} is {number:?}"),
        Err(e) => eprintln!("An error occurred:\n{e}"),
    }
}

enum Field {
    X,
    M,
    A,
    S,
}

struct Part {
    fields: [usize; 4],
}

impl Part {
    fn new(line: &&str) -> Result<Self, &'static str> {
        // remove { and } from start and end
        let line = match line.strip_prefix('{') {
            Some(leading_stripped) => match leading_stripped.strip_suffix('}') {
                Some(stripped) => stripped,
                None => return Err("no trailing `}` found in Part"),
            },
            None => return Err("no leading `{` found in Part"),
        };

        // try to parse all 4 fields
        let fields = line.splitn(4, ',').filter_map(|part| {
            let num_str = match part.split_once('=') {
                Some((_, num_str)) => num_str,
                None => return None,
            };

            match num_str.parse() {
                Ok(num) => Some(num),
                Err(_) => None,
            }
        }).collect::<Vec<usize>>();

        // try and convert the vec into an array of length 4
        let fields: [usize; 4] = match fields.try_into() {
            Ok(fields) => fields,
            Err(_) => return Err("Could not parse 4 fields"),
        };

        Ok(Self { fields })
    }

    fn get(&self, field: &Field) -> &usize {
        match field {
            Field::X => &self.fields[0],
            Field::M => &self.fields[1],
            Field::A => &self.fields[2],
            Field::S => &self.fields[3],
        }
    }

    fn score(&self) -> usize {
        self.fields.iter().sum()
    }
}

type WorkflowTag = String;

enum Rule {
    GreaterThan(Field, usize, WorkflowTag),
    LessThan(Field, usize, WorkflowTag),
    Identity(WorkflowTag),
}

struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(line: &str) -> Result<Self, &'static str> {
        let rules = line
            .split(',')
            .filter_map(|rule| {
                // filter out identities
                if !rule.contains(':') {
                    return Some(Rule::Identity(rule.to_string()));
                }
                
                // safe because of check above
                let (rule, tag) = rule.split_once(':').unwrap();

                // get field name
                let field = match rule.chars().nth(0) {
                    Some('x') => Field::X,
                    Some('m') => Field::M,
                    Some('a') => Field::A,
                    Some('s') => Field::S,
                    _ => return None,
                };

                let number = match rule.get(2..).unwrap_or_default().parse() {
                    Ok(n) => n,
                    Err(_)=> return None,
                };

                match rule.chars().nth(1) {
                    Some('<') => Some(Rule::LessThan(field, number, tag.to_string())),
                    Some('>') => Some(Rule::GreaterThan(field, number, tag.to_string())),
                    _ => None
                }
            })
            .collect::<Vec<_>>();

        if rules.is_empty() || !rules.iter().any(|r| match r {
            Rule::Identity(_) => true,
            _ => false,
        }) {
            Err("Rules are not exhaustive")
        } else {
           Ok(Self { rules })
        }
    }

    fn process(&self, part: &Part) -> &WorkflowTag {
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
}

enum SystemOutcome {
    Accepted,
    Rejected,
}

struct System {
    workflows: HashMap<WorkflowTag, Workflow>,
}

impl System {
    fn new(workflows: &[&str]) -> Result<Self, &'static str> {
        let workflows = workflows
            .iter()
            .map(|description| {
                let (tag, rules) = description.split_once('{').expect("should find {");
                let workflow = Workflow::new(rules.strip_suffix('}').expect("should find }"));
                (tag.to_string(), workflow)
            })
            .collect();

        Self { workflows }
    }

    fn process(&self, part: &Part) -> SystemOutcome {
        let mut current_workflow = START.to_string();
        loop {
            let wf = self.workflows.get(&current_workflow).unwrap();
            match wf.process(part).as_str() {
                ACCEPTED => return SystemOutcome::Accepted,
                REJECTED => return SystemOutcome::Rejected,
                new_workflow => current_workflow = new_workflow.to_string(),
            }
        }
    }
}

fn process_all_parts(lines: &[&str]) -> Result<usize, &'static str> {
    let first_part_line_index = match lines.iter().position(|l| l.starts_with('{')) {
        Some(position) => position,
        None => return Err("could not find `{` in lines"),
    };
    let (workflows, parts) = lines.split_at(first_part_line_index);

    let system = System::new(workflows);

    let parts = parts .iter() .map(Part::new);

        .fold(0, |sum, part| match system.process(&part) {
            SystemOutcome::Accepted => sum + part.score(),
            SystemOutcome::Rejected => sum,
        })
}

fn part1(filename: &str) -> Result<usize, &'static str> {
    let file = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(_) => return Err("failed to open file"),
    };

    let lines = file.split_whitespace().collect::<Vec<_>>();

    process_all_parts(&lines)
}

fn part2(filename: &str) -> Result<usize, &'static str> {
    let file = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(_) => return Err("failed to open file"),
    };

    Ok(0)
}

#[test]
fn part1_test() {
    assert_eq!(Ok(19114), part1("test1.txt"));
}

// #[test]
// fn part1_test() {
//     assert_eq!(23, part1("part1.txt"));
// }

// #[test]
// fn part2_test() {
//     assert_eq!(23, part2("test2.txt"));
// }

// #[test]
// fn part2_test() {
//     assert_eq!(23, part2("part2.txt"));
// }
