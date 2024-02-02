use std::collections::{HashMap, HashSet};

fn get_common_nums(line: &'static str) -> Result<usize, &'static str> {
    let (winning_nums, our_nums) = line
        .split_once(':')
        .map_or(Err("could not find : in line"), |(_, nums)| {
            nums.split_once('|').ok_or("could not find | in line")
        })?;

    let winning_set = winning_nums.split_whitespace().collect::<HashSet<_>>();
    let our_set = our_nums.split_whitespace().collect::<HashSet<_>>();

    Ok(winning_set.intersection(&our_set).count())
}

fn get_score(line: &'static str) -> Result<usize, &'static str> {
    match get_common_nums(line)? {
        0 => Ok(0),
        count => Ok(1 << (count - 1)),
    }
}

pub fn part1(file: &'static str) -> Result<u64, &'static str> {
    let mut sum = 0;
    for line in file.lines() {
        sum += get_score(line)?;
    }
    Ok(sum as u64)
}

pub fn part2(file: &'static str) -> Result<u64, &'static str> {
    let mut sum = 0;
    let mut card_copies: HashMap<usize, u64> = HashMap::new();

    for (card_num, line) in file.lines().enumerate() {
        let copies_of_this_card = *card_copies.get(&card_num).unwrap_or(&1);

        match get_common_nums(line)? {
            0 => (),
            count => {
                for index in (card_num + 1)..=(card_num + count) {
                    *card_copies.entry(index).or_insert(1) += copies_of_this_card;
                }
            }
        }

        sum += copies_of_this_card;
    }

    Ok(sum)
}
