use crate::parts::{AllParts, Field, Part};
use crate::system::{Outcome, System};
use std::collections::HashSet;
use std::fs;

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
            Outcome::PartAccepted => sum + part.score(),
            Outcome::PartRejected => sum,
        });

    Ok(accepted_part_sum)
}

fn calculate_combinations(passing_parts: &[AllParts]) -> u64 {
    // let mut merged_fields = vec![];

    let mut score = 0;

    for part in passing_parts {
        let mut s = 1;
        for field in [Field::X, Field::M, Field::A, Field::S] {
            let field = part.get(&field);
            s *= field.1 - field.0 + 1;
        }
        score += s;
    }

    // for field in [Field::X, Field::M, Field::A, Field::S] {
    //     let mut set = HashSet::new();
    //     for part in passing_parts {
    //         let field = part.get(&field);
    //         set.extend((field.0..field.1 + 1).collect::<Vec<_>>());
    //     }
    //     merged_fields.push(set);
    // }

    // merged_fields
    //     .iter()
    //     .fold(1, |prod, i| prod * i.len() as u64)
    score
}

fn calculate_acceptable_combinations(lines: &[&str]) -> Result<u64, &'static str> {
    let first_part_line_index = match lines.iter().position(|l| l.starts_with('{')) {
        Some(position) => position,
        None => return Err("could not find `{` in lines"),
    };
    let (workflows, parts) = lines.split_at(first_part_line_index);

    let mut system = System::new(workflows)?;

    let parts = parts.iter().map(Part::new).collect::<Result<Vec<_>, _>>()?;

    system.ingest(&parts);

    let passing_parts = system.get_passing_parts()?;

    passing_parts
        .iter()
        .enumerate()
        .for_each(|(i, x)| println!("{:?} {x:?}", i + 1));

    Ok(calculate_combinations(&passing_parts))
}

pub fn part1(filename: &str) -> Result<u64, &'static str> {
    let file = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(_) => return Err("failed to open file"),
    };

    let lines = file.split_whitespace().collect::<Vec<_>>();

    process_all_parts(&lines)
}

pub fn part2(filename: &str) -> Result<u64, &'static str> {
    let file = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(_) => return Err("failed to open file"),
    };

    let lines = file.split_whitespace().collect::<Vec<_>>();

    calculate_acceptable_combinations(&lines)
}
