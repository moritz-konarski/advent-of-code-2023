use std::{collections::HashMap, env, fs};

const USAGE: &str = "Incorrect usage!\nUsage: cargo r -- [t|p] [1|2]";
const START: &str = "in";
const ACCEPTED: &str = "A";
const REJECTED: &str = "R";

fn main() {
    let run_type = env::args().nth(1).expect(USAGE);
    let number = env::args().nth(2).expect(USAGE);
    let result = match (run_type.as_str(), number.as_str()) {
        ("t", "1") => part1("test1.txt"),
        ("p", "1") => part1("part1.txt"),
        ("t", "2") => part2("test2.txt"),
        ("p", "2") => part2("part2.txt"),
        _ => panic!("{USAGE}"),
    };
    println!("Result for {run_type}{number} is {result:?}");
}

type WorkflowTag = String;
type Rule = Box<dyn Fn(&Part) -> Option<WorkflowTag>>;

fn create_rule(field: &str, operation: &str, number: usize, tag: WorkflowTag) -> Rule {
    match (field, operation) {
        ("x", "<") => Box::new(move |p| (p.x < number).then_some(tag.clone())),
        ("x", ">") => Box::new(move |p| (p.x > number).then_some(tag.clone())),
        ("m", "<") => Box::new(move |p| (p.m < number).then_some(tag.clone())),
        ("m", ">") => Box::new(move |p| (p.m > number).then_some(tag.clone())),
        ("a", "<") => Box::new(move |p| (p.a < number).then_some(tag.clone())),
        ("a", ">") => Box::new(move |p| (p.a > number).then_some(tag.clone())),
        ("s", "<") => Box::new(move |p| (p.s < number).then_some(tag.clone())),
        ("s", ">") => Box::new(move |p| (p.s > number).then_some(tag.clone())),
        _ => unreachable!("illegal {field} and {operation}"),
    }
}

enum WorkflowOutcome {
    Accepted,
    Rejected,
    Switch(WorkflowTag),
}

enum SystemOutcome {
    Accepted,
    Rejected,
}

struct Workflow {
    tag: WorkflowTag,
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(desription: &&str) -> Self {
        let (tag, rules_str) = desription.split_once('{').expect("should find {");
        let mut rules = rules_str[..rules_str.len() - 1]
            .split(',')
            .rev()
            .skip(1)
            .map(|rule| {
                println!("{rule}");
                let (rule, tag) = rule.split_once(':').expect(": not found");
                let field = &rule[0..1];
                let operation = &rule[1..2];
                let number = rule[2..].parse().unwrap();

                println!("{field} {operation} {number}\n");

                create_rule(field, operation, number, tag.to_string())
            })
            .collect::<Vec<_>>();

        let last_tag = rules_str[..rules_str.len() - 1].rsplit_once(',').unwrap().1;

        rules.push(Box::new(move |_| Some(last_tag.to_owned())));

        Self {
            tag: tag.to_string(),
            rules,
        }
    }

    fn process(&self, part: &Part) -> WorkflowOutcome {
        for rule in &self.rules {
            if let Some(result) = rule(part) {
                match result.as_str() {
                    ACCEPTED => return WorkflowOutcome::Accepted,
                    REJECTED => return WorkflowOutcome::Rejected,
                    _ => return WorkflowOutcome::Switch(result),
                }
            }
        }

        unreachable!("there is always a result")
    }
}

struct System {
    workflows: HashMap<WorkflowTag, Workflow>,
}

impl System {
    fn new(workflows: &[&str]) -> Self {
        Self {
            workflows: workflows
                .iter()
                .map(Workflow::new)
                .map(|p| (p.tag.clone(), p))
                .collect(),
        }
    }

    fn process(&self, part: &Part) -> SystemOutcome {
        let mut current_workflow = START.to_string();
        loop {
            match self.workflows.get(&current_workflow).unwrap().process(part) {
                WorkflowOutcome::Accepted => return SystemOutcome::Accepted,
                WorkflowOutcome::Rejected => return SystemOutcome::Rejected,
                WorkflowOutcome::Switch(new_workflow) => current_workflow = new_workflow,
            }
        }
    }
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(description: &&str) -> Self {
        let mut coords = description[1..description.len() - 1]
            .splitn(4, ',')
            .map(|part| part[2..].parse().unwrap());

        Self {
            x: coords.next().unwrap(),
            m: coords.next().unwrap(),
            a: coords.next().unwrap(),
            s: coords.next().unwrap(),
        }
    }

    fn score(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn process_all_parts(lines: &[&str]) -> usize {
    let empty_line = lines.iter().position(|l| l.starts_with('{')).unwrap();
    let (workflows, parts) = (&lines[..empty_line], &lines[empty_line + 1..]);

    // parse all pipelines
    let system = System::new(&workflows);

    // parse and process the parts
    parts
        .iter()
        .map(Part::new)
        .fold(0, |sum, part| match system.process(&part) {
            SystemOutcome::Accepted => sum + part.score(),
            SystemOutcome::Rejected => sum,
        })
}

fn part1(filename: &str) -> usize {
    let file = fs::read_to_string(filename).expect(&format!("the file `{filename}` should exist"));
    let lines = file.split_whitespace().collect::<Vec<_>>();

    process_all_parts(&lines)
}

fn part2(filename: &str) -> usize {
    let file = fs::read_to_string(filename).expect(&format!("the file `{filename}` should exist"));
    0
}

#[test]
fn part1_test() {
    assert_eq!(19114, part1("test1.txt"));
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
