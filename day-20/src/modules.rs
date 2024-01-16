use std::collections::{BTreeMap, HashMap, VecDeque};

enum Pulse {
    High,
    Low,
}

enum State {
    On,
    Off,
}

pub trait Module {
    fn new(line: &str) -> Result<Self, &'static str>;
    fn receive_pulse(&mut self, pulse: Pulse, from: &String);
    fn pass_on_pulse(
        &self,
        pulse_queue: &mut VecDeque<String>,
        module_map: &mut BTreeMap<String, Modules>,
    ) -> Result<(), &'static str>;
}

pub enum Modules {
    FlipFlop(FlipFlopStruct),
    Conjunction(ConjunctionStruct),
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

    fn receive_pulse(&mut self, pulse: Pulse, from: &String) {
        match self {
            Modules::FlipFlop(s) => s.receive_pulse(pulse, from),
            Modules::Conjunction(s) => s.receive_pulse(pulse, from),
        }
    }

    fn pass_on_pulse(
        &self,
        pulse_queue: &mut VecDeque<String>,
        module_map: &mut BTreeMap<String, Modules>,
    ) {
        match self {
            Modules::FlipFlop(s) => s.pass_on_pulse(pulse_queue, module_map),
            Modules::Conjunction(s) => s.pass_on_pulse(pulse_queue, module_map),
        }
    }
}

pub struct Broadcaster {
    dest_modules: VecDeque<String>,
}

impl Broadcaster {
    fn new(line: &str) -> Result<Self, &'static str> {
        let destination_str = match line.split_once(" -> ") {
            Some(("broadcaster", dest)) => dest,
            _ => return Err("cannot find broadcaster in line"),
        };

        let dest_modules = destination_str.split(", ").collect::<VecDeque<_>>();

        Ok(Self { dest_modules })
    }

    fn send_low_pulse(
        &self,
        module_map: &mut BTreeMap<String, Modules>,
    ) -> Result<VecDeque<String>, &'static str> {
        for dest in &self.dest_modules {
            let mut module = match module_map.get_mut(dest) {
                Some(m) => m,
                None => return Err("cannot find destination in module map"),
            };
            module.receive_pulse(Pulse::Low, "broadcaster");
        }

        Ok(self.dest_modules)
    }
}

struct FlipFlopStruct {
    label: String,
    state: State,
    dest_modules: VecDeque<String>,
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

        let dest_modules = destination_str.split(", ").collect::<VecDeque<_>>();

        Ok(Self {
            label: label.to_string(),
            state: State::Off,
            dest_modules,
        })
    }

    fn receive_pulse(&mut self, pulse: Pulse, _from: &String) {
        match pulse {
            Pulse::Low => {
                self.state = match self.state {
                    State::On => State::Off,
                    State::Off => State::On,
                }
            }
            Pulse::High => { /* ignore this pulse */ }
        }
    }

    fn pass_on_pulse(
        &self,
        pulse_queue: &mut VecDeque<String>,
        module_map: &mut BTreeMap<String, Modules>,
    ) -> Result<(), &'static str> {
        let pulse = match self.state {
            State::On => Pulse::High,
            State::Off => Pulse::Low,
        };

        for dest in &self.dest_modules {
            let mut module = match module_map.get_mut(dest) {
                Some(m) => m,
                None => return Err("cannot find destination in module map"),
            };
            module.receive_pulse(pulse, &self.label);
            pulse_queue.push_back(dest);
        }

        Ok(())
    }
}

struct ConjunctionStruct {
    label: String,
    input_to_state: HashMap<String, State>,
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

        let dest_modules = destination_str.split(", ").collect::<VecDeque<_>>();

        Ok(Self {
            label: label.to_string(),
            input_to_state: HashMap::new(),
            dest_modules,
        })
    }

    fn receive_pulse(&mut self, pulse: Pulse, from: &String) {
        self.input_to_state
            .entry(from)
            .and_modify(|e| e = pulse)
            .or_insert(pulse);
    }

    fn pass_on_pulse(
        &self,
        pulse_queue: &mut VecDeque<String>,
        module_map: &mut BTreeMap<String, Modules>,
    ) -> Result<(), &'static str> {
        let pulse = if self.input_to_state.values().any(|v| v == Pulse::Low) {
            Pulse::High
        } else {
            Pulse::Low
        };

        for dest in &self.dest_modules {
            let mut module = match module_map.get_mut(dest) {
                Some(m) => m,
                None => return Err("cannot find destination in module map"),
            };
            module.receive_pulse(pulse, &self.label);
            pulse_queue.push_back(dest);
        }

        Ok(())
    }
}
