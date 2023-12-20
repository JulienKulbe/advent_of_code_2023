use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = true;

const BROADCAST: &str = "BROASCAST";

struct Parser;

impl Parser {
    fn parse_file(filename: &str) -> Modules {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut modules = Modules(HashMap::new());
        for line in reader.lines().flatten() {
            let (source, destination) = line.split_once(" -> ").unwrap();
            let mod_type = ModuleType::from(&source[0..1]);

            let label = match mod_type {
                ModuleType::Broadcast => BROADCAST,
                _ => &source[1..],
            };
            let destinations = destination
                .split(',')
                .map(|d| String::from(d.trim()))
                .collect::<Vec<_>>();

            modules.add_module(label, Module::new(mod_type, destinations));
        }

        modules
    }
}

struct Modules(HashMap<String, Module>);

impl Modules {
    fn add_module(&mut self, id: &str, module: Module) {
        self.0.insert(String::from(id), module);
    }

    fn push_button(&mut self) {
        let mut pulses = VecDeque::new();
        pulses.push_back((String::from(BROADCAST), State::Low));

        while let Some((id, pulse)) = pulses.pop_front() {
            let module = self.get(&id);
            let received = module.send_pulse(pulse);

            for next in received {
                pulses.push_back(next);
            }
        }
    }

    fn get(&mut self, id: &str) -> &mut Module {
        self.0.get_mut(id).unwrap()
    }
}

struct Module {
    mod_type: ModuleType,
    destinations: Vec<String>,
    state: State,
}

impl Module {
    fn new(mod_type: ModuleType, destinations: Vec<String>) -> Self {
        Self {
            mod_type,
            destinations,
            state: State::High,
        }
    }

    fn send_pulse(&mut self, from: &str, received: State) -> Vec<(String, State)> {
        let pulse = match self.mod_type {
            ModuleType::Broadcast => Some(State::Low),
            ModuleType::FlipFlop => {
                if received == State::Low {
                    // Off = High, On = Low
                    let state = self.state;
                    self.state = self.state.toggle();
                    Some(state)
                } else {
                    None
                }
            }
            ModuleType::Conjunction => {
                todo!()
            }
        };

        if let Some(pulse) = pulse {
            self.destinations
                .iter()
                .map(|label| (label.clone(), pulse))
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    High,
    Low,
}

impl State {
    fn toggle(self) -> State {
        match self {
            Self::High => Self::Low,
            Self::Low => Self::High,
        }
    }
}

enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
}

impl From<&str> for ModuleType {
    fn from(value: &str) -> Self {
        match value {
            "%" => ModuleType::FlipFlop,
            "&" => ModuleType::Conjunction,
            _ => ModuleType::Broadcast,
        }
    }
}

fn main() {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };

    let mut modules = Parser::parse_file(filename);
    modules.push_button();
}
