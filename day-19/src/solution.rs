mod solution {
    use std::fs;

    fn process_all_parts(lines: &[&str]) -> Result<u64, &'static str> {
        let first_part_line_index = match lines.iter().position(|l| l.starts_with('{')) {
            Some(position) => position,
            None => return Err("could not find `{` in lines"),
        };
        let (workflows, parts) = lines.split_at(first_part_line_index);

        let mut system = system::System::new(workflows)?;

        let parts = parts
            .iter()
            .map(parts::Part::new)
            .collect::<Result<Vec<_>, _>>()?;

        let accepted_part_sum = parts
            .iter()
            .fold(0, |sum, part| match system.process(part) {
                system::SystemOutcome::Accepted => sum + part.score(),
                system::SystemOutcome::Rejected => sum,
            });

        Ok(accepted_part_sum)
    }

    fn merge_passing_pipelines(passing_pargs: &[parts::AllParts]) -> [Vec<(u64, u64)>; 4] {
        [vec![(0, 0)], vec![(0, 0)], vec![(0, 0)], vec![(0, 0)]]
    }

    fn calculate_combinations(merged_part: [Vec<(u64, u64)>; 4]) -> u64 {
        0
    }

    fn calculate_acceptable_combinations(lines: &[&str]) -> Result<u64, &'static str> {
        let first_part_line_index = match lines.iter().position(|l| l.starts_with('{')) {
            Some(position) => position,
            None => return Err("could not find `{` in lines"),
        };
        let (workflows, parts) = lines.split_at(first_part_line_index);

        let mut system = parts::System::new(workflows)?;

        let parts = parts
            .iter()
            .map(parts::Part::new)
            .collect::<Result<Vec<_>, _>>()?;

        parts.iter().for_each(|part| _ = system.process(part));

        let passing_parts_per_pipeline = system
            .accepted_pipelines
            .iter()
            .inspect(|list| println!("PL: {list:?}"))
            .map(|list| match list.split_first() {
                Some((first_tag, remaining_tags)) => match system.workflows.get(first_tag) {
                    Some(mut wf) => remaining_tags
                        .iter()
                        .inspect(|t| println!(" Tag: {t}"))
                        .fold(Ok(parts::AllParts::new()), |part, next_tag| {
                            match system.workflows.get(next_tag) {
                                Some(new_wf) => match part {
                                    Ok(p) => {
                                        let old_wf = wf;
                                        println!("  {p:?}");
                                        println!("  {old_wf:?} -> {new_wf:?}");
                                        wf = new_wf;
                                        Ok(old_wf.fit_part_through_workflow(p, next_tag))
                                    }
                                    Err(e) => Err(e),
                                },
                                None => Err("cannot find remaining tag in accepted pipeline"),
                            }
                        }),
                    None => Err("cannot find start workflow"),
                },
                None => Err("no workflows in accepted pipeline"),
            })
            .collect::<Result<Vec<_>, _>>()?;

        println!("{passing_parts_per_pipeline:?}");

        let merged_part = merge_passing_pipelines(&passing_parts_per_pipeline);

        Ok(calculate_combinations(merged_part))
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
}
