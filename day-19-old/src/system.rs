use crate::parts::{AllParts, Part};
use crate::workflow::Workflow;

use std::collections::{HashMap, HashSet};

const START: &str = "in";
const ACCEPTED: &str = "A";
const REJECTED: &str = "R";

pub enum Outcome {
    PartAccepted,
    PartRejected,
}

pub struct System {
    workflows: HashMap<String, Workflow>,
    accepted_pipelines: HashSet<Vec<String>>,
}

impl System {
    pub fn new(workflows: &[&str]) -> Result<Self, &'static str> {
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

    pub fn ingest(&mut self, parts: &[Part]) {
        for part in parts {
            let _ = self.process(part);
        }
    }

    pub fn process(&mut self, part: &Part) -> Outcome {
        let mut current_workflow = vec![START.to_string()];
        loop {
            let wf = self
                .workflows
                .get(current_workflow.last().unwrap())
                .unwrap();
            match wf.process(part).as_str() {
                ACCEPTED => {
                    current_workflow.push(ACCEPTED.to_string());
                    self.accepted_pipelines.insert(current_workflow);
                    return Outcome::PartAccepted;
                }
                REJECTED => return Outcome::PartRejected,
                new_workflow => current_workflow.push(new_workflow.to_string()),
            }
        }
    }

    fn process_tags(&self, tags: &[String]) -> Result<AllParts, &'static str> {
        let first_tag = match tags.first() {
            Some(f) => f,
            None => return Err("accepted tags list is empty"),
        };
        let mut wf = match self.workflows.get(first_tag) {
            Some(wf) => wf,
            None => return Err("could not find first accepted workflow"),
        };

        tags.iter()
            .skip(1)
            .inspect(|t| println!(" next: {t}"))
            .try_fold(AllParts::new(), |part, next_tag| {
                if next_tag.as_str() == ACCEPTED {
                    println!("  {part:?}");
                    println!("  {wf:?}");
                    let new_part = wf.fit_part_through_workflow(part, next_tag);
                    println!("   {new_part:?}");
                    Ok(new_part)
                } else {
                    match self.workflows.get(next_tag) {
                        Some(new_wf) => {
                            println!("  {part:?}");
                            println!("  {wf:?}");
                            let new_part = wf.fit_part_through_workflow(part, next_tag);
                            wf = new_wf;
                            println!("   {new_part:?}");
                            Ok(new_part)
                        }
                        None => Err("cannot find tag in accepted pipeline"),
                    }
                }
            })
    }

    pub fn get_passing_parts(&self) -> Result<Vec<AllParts>, &'static str> {
        self.accepted_pipelines
            .iter()
            .inspect(|list| println!("PL: {list:?}"))
            .map(
                |list| self.process_tags(list), // match list.split_first() {
                                                // Some((first_tag, remaining_tags)) => match self.workflows.get(first_tag) {
                                                //     Some(wf) => self.process_tags(remaining_tags, wf.clone()),
                                                // Some(mut wf) => remaining_tags
                                                //     .iter()
                                                //     .inspect(|t| println!(" Tag: {t}"))
                                                //     .try_fold(AllParts::new(), |part, next_tag| {
                                                //         match self.workflows.get(next_tag) {
                                                //             Some(new_wf) => {
                                                //                 let old_wf = wf;
                                                //                 println!("  {part:?}");
                                                //                 println!("  {old_wf:?} ->\n   {new_wf:?}");
                                                //                 wf = new_wf;
                                                //                 let new_part = old_wf.fit_part_through_workflow(part, next_tag);
                                                //                 println!("   {new_part:?}");
                                                //                 Ok(new_part)
                                                //             }
                                                //             None => Err("cannot find remaining tag in accepted pipeline"),
                                                //         }
                                                //     }),
                                                //     None => Err("cannot find start workflow"),
                                                // },
                                                // None => Err("no workflows in accepted pipeline"),
            )
            .collect()
    }
}
