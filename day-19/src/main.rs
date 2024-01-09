use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

const START: &str = "in";
const ACCEPTED: &str = "A";
const REJECTED: &str = "R";
const MIN_SCORE: u64 = 1;
const MAX_SCORE: u64 = 4_000;

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
        Ok(r) => println!("Result for {run_type}{number} is {r:?}"),
        Err(e) => eprintln!("An error occurred:\n{e}"),
    }
}

#[derive(Debug)]
enum Field {
    X,
    M,
    A,
    S,
}

struct Part {
    fields: [u64; 4],
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

    fn get(&self, field: &Field) -> &u64 {
        match field {
            Field::X => &self.fields[0],
            Field::M => &self.fields[1],
            Field::A => &self.fields[2],
            Field::S => &self.fields[3],
        }
    }

    fn score(&self) -> u64 {
        self.fields.iter().sum()
    }
}

#[derive(Debug)]
struct AllParts {
    fields: [(u64, u64); 4],
}

impl AllParts {
    fn new() -> Self {
        Self {
            fields: [(MIN_SCORE, MAX_SCORE); 4],
        }
    }

    fn get_mut(&mut self, field: &Field) -> &mut (u64, u64) {
        match field {
            Field::X => &mut self.fields[0],
            Field::M => &mut self.fields[1],
            Field::A => &mut self.fields[2],
            Field::S => &mut self.fields[3],
        }
    }

    fn possibilities(&self) -> u64 {
        self.fields.iter().map(|(min, max)| max - min + 1).product()
    }
}

type WorkflowTag = String;

#[derive(Debug)]
enum Rule {
    GreaterThan(Field, u64, WorkflowTag),
    LessThan(Field, u64, WorkflowTag),
    Identity(WorkflowTag),
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(line: &str) -> Result<Self, &'static str> {
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

    fn count_ways_to_tag(&self, mut part: AllParts, next_tag: &String) -> AllParts {
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

enum SystemOutcome {
    Accepted,
    Rejected,
}

struct System {
    workflows: HashMap<WorkflowTag, Workflow>,
    accepted_pipelines: HashSet<Vec<WorkflowTag>>,
}

impl System {
    fn new(workflows: &[&str]) -> Result<Self, &'static str> {
        let workflows = workflows
            .iter()
            .map(|description| match description.split_once('{') {
                Some((tag, rules)) => match rules.strip_suffix('}') {
                    Some(rules) => match Workflow::new(rules) {
                        Ok(workflow) => Ok((tag.to_string(), workflow)),
                        Err(e) => Err(e),
                    },
                    None => Err("did not find `}` in Workflow"),
                },
                None => Err("did not find `{` in Workflow"),
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Self {
            workflows,
            accepted_pipelines: HashSet::new(),
        })
    }

    fn process(&mut self, part: &Part) -> SystemOutcome {
        let mut current_workflow = vec![START.to_string()];
        loop {
            let wf = self
                .workflows
                .get(current_workflow.last().unwrap())
                .unwrap();
            match wf.process(part).as_str() {
                ACCEPTED => {
                    self.accepted_pipelines.insert(current_workflow);
                    return SystemOutcome::Accepted;
                }
                REJECTED => return SystemOutcome::Rejected,
                new_workflow => current_workflow.push(new_workflow.to_string()),
            }
        }
    }
}

fn process_all_parts(lines: &[&str]) -> Result<u64, &'static str> {
    let first_part_line_index = match lines.iter().position(|l| l.starts_with('{')) {
        Some(position) => position,
        None => return Err("could not find `{` in lines"),
    };
    let (workflows, parts) = lines.split_at(first_part_line_index);

    let mut system = System::new(workflows)?;

    let parts = parts.iter().map(Part::new).collect::<Result<Vec<_>, _>>()?;

    let accepted_part_sum = parts
        .iter()
        .fold(0, |sum, part| match system.process(part) {
            SystemOutcome::Accepted => sum + part.score(),
            SystemOutcome::Rejected => sum,
        });

    Ok(accepted_part_sum)
}

fn part1(filename: &str) -> Result<u64, &'static str> {
    let file = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(_) => return Err("failed to open file"),
    };

    let lines = file.split_whitespace().collect::<Vec<_>>();

    process_all_parts(&lines)
}

fn calculate_acceptable_combinations(lines: &[&str]) -> Result<u64, &'static str> {
    let first_part_line_index = match lines.iter().position(|l| l.starts_with('{')) {
        Some(position) => position,
        None => return Err("could not find `{` in lines"),
    };
    let (workflows, parts) = lines.split_at(first_part_line_index);

    let mut system = System::new(workflows)?;

    let parts = parts.iter().map(Part::new).collect::<Result<Vec<_>, _>>()?;

    parts.iter().for_each(|part| _ = system.process(part));

    let mut possible_combinations = vec![];

    system.accepted_pipelines.iter().for_each(|list| {
        println!("{list:?}");
        let mut part = AllParts::new();
        println!("  {:?}", part);
        let mut w = system.workflows.get(&list[0]).unwrap();
        for tag in &list[1..] {
            part = w.count_ways_to(part, tag);
            println!("  {tag} -> {w:?}");
            println!("  {:?}", part);
            possible_combinations.push(part);

            w = system.workflows.get(tag).unwrap();
        }
        println!("---------");
    });

    // TODO:
    merge_possible_parts(...)
    // Ok(parts
    //     .iter()
    //     .fold(0, |sum, part| match system.process(part) {
    //         SystemOutcome::Accepted => sum + part.score(),
    //         SystemOutcome::Rejected => sum,
    //     }))

    Ok(possible_combinations)
}

fn part2(filename: &str) -> Result<u64, &'static str> {
    let file = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(_) => return Err("failed to open file"),
    };

    let lines = file.split_whitespace().collect::<Vec<_>>();

    calculate_acceptable_combinations(&lines)
}

#[test]
fn part1_test() {
    assert_eq!(Ok(19114), part1("test1.txt"));
}

#[test]
fn part1_full() {
    assert_eq!(Ok(449531), part1("part1.txt"));
}

#[test]
fn part2_test() {
    assert_eq!(Ok(167409079868000), part2("test2.txt"));
}

// #[test]
// fn part2_full() {
//     assert_eq!(23, part2("part2.txt"));
// }
