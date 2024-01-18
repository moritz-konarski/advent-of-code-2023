use crate::modules::{Broadcaster, Message, Module, Modules};
use std::{
    collections::{BTreeMap, VecDeque},
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

    let mut msg_queue = VecDeque::new();
    let start_message = Message::new();
    println!("{start_message:?}");
    broadcaster.receive_message(start_message, &mut msg_queue);

    while let Some(message) = msg_queue.pop_front() {
        println!("{message:?}");
        if let Some(module) = module_map.get_mut(&message.receiver) {
            module.receive_message(message, &mut msg_queue);
        }
    }

    Ok(0)
}

pub fn part2(filename: &str, press_count: u64) -> Result<u64, &'static str> {
    let file = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(_) => return Err("failed to open file"),
    };

    let lines = file.split_terminator('\n').collect::<Vec<_>>();

    Ok(0)
}
