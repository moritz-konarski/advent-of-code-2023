use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct Message {
    sender: String,
    pub pulse: Pulse,
    pub receiver: String,
}

impl Message {
    pub fn new() -> Self {
        Self {
            sender: "".to_string(),
            pulse: Pulse::Low,
            receiver: "broadcaster".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub enum Pulse {
    High,
    Low,
}

pub trait Module {
    fn new(line: &str) -> Result<impl Module, &'static str>;
    fn receive_message(&mut self, msg: Message, msg_queue: &mut VecDeque<Message>);
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub enum Modules {
    FlipFlop(FlipFlopStruct),
    Conjunction(ConjunctionStruct),
}

impl Modules {
    pub fn label(&self) -> String {
        match self {
            Modules::FlipFlop(s) => s.label.clone(),
            Modules::Conjunction(s) => s.label.clone(),
        }
    }
}

impl Module for Modules {
    fn new(line: &str) -> Result<Self, &'static str> {
        match line.chars().next() {
            Some('%') => {
                let s = FlipFlopStruct::new(&line)?;
                Ok(Modules::FlipFlop(s))
            }
            Some('&') => {
                let s = ConjunctionStruct::new(&line)?;
                Ok(Modules::Conjunction(s))
            }
            _ => Err("illegal module identifyer"),
        }
    }

    fn receive_message(&mut self, msg: Message, msg_queue: &mut VecDeque<Message>) {
        match self {
            Modules::FlipFlop(s) => s.receive_message(msg, msg_queue),
            Modules::Conjunction(s) => s.receive_message(msg, msg_queue),
        }
    }
}

pub struct Broadcaster {
    dest_modules: VecDeque<String>,
}

impl Module for Broadcaster {
    fn new(line: &str) -> Result<Self, &'static str> {
        let destination_str = match line.split_once(" -> ") {
            Some(("broadcaster", dest)) => dest,
            _ => return Err("cannot find broadcaster in line"),
        };

        let dest_modules = destination_str
            .split(", ")
            .map(|s| s.to_string())
            .collect::<VecDeque<_>>();

        Ok(Self { dest_modules })
    }

    fn receive_message(&mut self, msg: Message, msg_queue: &mut VecDeque<Message>) {
        for dest in &self.dest_modules {
            msg_queue.push_back(Message {
                sender: "broadcaster".to_string(),
                pulse: msg.pulse,
                receiver: dest.clone(),
            })
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct FlipFlopStruct {
    label: String,
    state: Pulse,
    dest_modules: Vec<String>,
}

impl Module for FlipFlopStruct {
    fn new(line: &str) -> Result<Self, &'static str> {
        let rest = match line.strip_prefix('%') {
            Some(rest) => rest,
            None => return Err("% at start of FlipFlop not found!"),
        };
        let (label, destination_str) = match rest.split_once(" -> ") {
            Some(t) => t,
            _ => return Err("cannot find -> in flip flop line"),
        };

        let dest_modules = destination_str
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Ok(Self {
            label: label.to_string(),
            state: Pulse::Low,
            dest_modules,
        })
    }

    fn receive_message(&mut self, msg: Message, msg_queue: &mut VecDeque<Message>) {
        let new_pulse = match msg.pulse {
            Pulse::Low => {
                self.state = match self.state {
                    Pulse::High => Pulse::Low,
                    Pulse::Low => Pulse::High,
                };
                self.state
            }
            Pulse::High => self.state,
        };

        for dest in &self.dest_modules {
            msg_queue.push_back(Message {
                sender: self.label.clone(),
                pulse: new_pulse,
                receiver: dest.clone(),
            });
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct ConjunctionStruct {
    label: String,
    input_to_state: HashMap<String, Pulse>,
    dest_modules: Vec<String>,
}

impl Module for ConjunctionStruct {
    fn new(line: &str) -> Result<Self, &'static str> {
        let rest = match line.strip_prefix('&') {
            Some(rest) => rest,
            None => return Err("% at start of Conjunction not found!"),
        };
        let (label, destination_str) = match rest.split_once(" -> ") {
            Some(t) => t,
            _ => return Err("cannot find -> in conjunction line"),
        };

        let dest_modules = destination_str
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Ok(Self {
            label: label.to_string(),
            input_to_state: HashMap::new(),
            dest_modules,
        })
    }

    fn receive_message(&mut self, msg: Message, msg_queue: &mut VecDeque<Message>) {
        self.input_to_state
            .entry(msg.sender)
            .and_modify(|e| *e = msg.pulse)
            .or_insert(msg.pulse);

        let new_pulse = if self
            .input_to_state
            .values()
            .any(|v| matches!(v, Pulse::Low))
        {
            Pulse::High
        } else {
            Pulse::Low
        };

        for dest in &self.dest_modules {
            msg_queue.push_back(Message {
                sender: self.label.clone(),
                pulse: new_pulse,
                receiver: dest.clone(),
            });
        }
    }
}
