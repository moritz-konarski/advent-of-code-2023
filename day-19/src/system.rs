mod parts;
mod workflow;

pub mod system {
    const START: &str = "in";
    const ACCEPTED: &str = "A";
    const REJECTED: &str = "R";

    use std::collections::{HashMap, HashSet};

    pub enum SystemOutcome {
        Accepted,
        Rejected,
    }

    pub struct System {
        workflows: HashMap<String, workflow::Workflow>,
        accepted_pipelines: HashSet<Vec<String>>,
    }

    impl System {
        pub fn new(workflows: &[&str]) -> Result<Self, &'static str> {
            let workflows = workflows
                .iter()
                .map(|description| match description.split_once('{') {
                    Some((tag, rules)) => match rules.strip_suffix('}') {
                        Some(rules) => match workflow::Workflow::new(rules) {
                            Ok(workflow) => Ok((tag.to_string(), workflow)),
                            Err(e) => Err(e),
                        },
                        None => Err("did not find `}` in workflow::Workflow"),
                    },
                    None => Err("did not find `{` in workflow::Workflow"),
                })
                .collect::<Result<HashMap<_, _>, _>>()?;

            Ok(Self {
                workflows,
                accepted_pipelines: HashSet::new(),
            })
        }

        pub fn process(&mut self, part: &parts::Part) -> SystemOutcome {
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
}
