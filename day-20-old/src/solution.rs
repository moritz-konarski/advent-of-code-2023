use crate::modules::{Broadcaster, Message, Module, Modules, Pulse};
use std::{
    collections::{BTreeMap, HashSet, VecDeque},
    fs,
};

fn parse_modules(lines: &[&str]) -> Result<(Broadcaster, BTreeMap<String, Modules>), &'static str> {
    let mut modules = Vec::with_capacity(lines.len() - 1);
    let mut broadcast_index = None;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with('b') {
            broadcast_index = Some(i);
            continue;
        }
        modules.push(Modules::new(line)?);
    }

    let broadcaster = match broadcast_index {
        Some(i) => Broadcaster::new(lines[i])?,
        None => return Err("could not find broadcaster in lines"),
    };

    let map = modules.iter().map(|m| (m.label(), m.clone())).collect();

    Ok((broadcaster, map))
}

pub fn part1(filename: &str, press_count: u64) -> Result<u64, &'static str> {
    let file = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(_) => return Err("failed to open file"),
    };

    let lines = file.split_terminator('\n').collect::<Vec<_>>();

    let (mut broadcaster, mut module_map) = parse_modules(&lines)?;
    let mut module_set = HashSet::new();
    module_set.insert(module_map.clone());

    let mut msg_queue = VecDeque::new();

    broadcaster.receive_message(Message::new(), &mut msg_queue);

    let mut low_count = 1;
    let mut high_count = 0;

    let periodicity = loop {
        let message = match msg_queue.pop_front() {
            Some(m) => m,
            None => {
                println!("started over!");
                broadcaster.receive_message(Message::new(), &mut msg_queue);
                msg_queue.pop_front().unwrap()
            }
        };

        println!("{message:?}");

        match message.pulse {
            Pulse::High => high_count += 1,
            Pulse::Low => low_count += 1,
        }

        if let Some(module) = module_map.get_mut(&message.receiver) {
            module.receive_message(message, &mut msg_queue);
        }

        if msg_queue.len() == initial_length {
            if !module_set.insert(msg_queue.clone()) {
                break module_set.len();
            }
        }
    };

    println!("period: {periodicity:?}");
    println!("High: {high_count:?}, Low {low_count:?}");

    let repeats = press_count / (periodicity as u64 + 1);
    let score = repeats * high_count * repeats * low_count;
    Ok(score)
}

pub fn part2(filename: &str, press_count: u64) -> Result<u64, &'static str> {
    let file = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(_) => return Err("failed to open file"),
    };

    let lines = file.split_terminator('\n').collect::<Vec<_>>();

    Ok(0)
}
