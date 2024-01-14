use std::collections::{BTreeMap, VecDeque};

pub enum Pulse {
    High,
    Low,
}

pub enum Modules {
    FlipFlop(FlipFlopStruct),
    Conjunction(ConjunctionStruct),
}

impl Module for Modules {
    fn receive_pulse(&mut self, pulse: Pulse, from: &String) {
        match self {
            FlipFlop(s) => s.receive_pulse(pulse, from),
            Conjunction(s) => s.receive_pulse(pulse, from),
        }
    }
    fn pass_on_pulse(&self, module_map: &mut BTreeMap<String, Modules>) {
        match self {
            FlipFlop(s) => s.pass_on_pulse(module_map),,
            Conjunction(s) => s.pass_on_pulse(module_map),,
        }
    }
}

pub trait Module {
    fn new(line: &str) -> Result<Self, &'static str>;
    fn receive_pulse(&mut self, pulse: Pulse, from: &String);
    fn pass_on_pulse(&self, module_map: &mut BTreeMap<String, Modules>);
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

        let dest_modules = destination_str.split(", ").collect::<VecDeque<_>>();

        Ok(Self { dest_modules })
    }

    fn pass_on_pulse(
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

    fn receive_pulse(&mut self, pulse: Pulse, from: &String) {
        // not required
    }
}

pub struct FlipFlopStruct {
    dest_modules: VecDeque<String>,
}

impl Module for FlipFlopStruct{
    fn receive_pulse(&mut self, pulse: Pulse, from: &String) {}
    fn pass_on_pulse<T: Module>(&self, module_map: &mut BTreeMap<String, T>) {}
}

pub struct ConjunctionStruct {
    dest_modules: Vec<String>,
}

impl Module for ConjunctionStruct {
    fn receive_pulse(&mut self, pulse: Pulse, from: &String) {}
    fn pass_on_pulse<T: Module>(&self, module_map: &mut BTreeMap<String, T>) {}
}
