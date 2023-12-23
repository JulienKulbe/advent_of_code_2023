use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

const BROADCAST: &str = "BROADCAST";

struct Parser;

impl Parser {
    fn parse_file(filename: &str) -> Modules {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut modules = Modules::new();
        for line in reader.lines().flatten() {
            let (source, destination) = line.split_once(" -> ").unwrap();
            let mod_type = ModuleType::from(&source[0..1]);

            let id = match mod_type {
                ModuleType::Broadcast => BROADCAST,
                _ => &source[1..],
            };
            let destinations = destination
                .split(',')
                .map(|d| String::from(d.trim()))
                .collect::<Vec<_>>();

            modules.add_module(id, Module::new(id, mod_type, destinations));
        }

        modules
    }
}

struct Modules {
    modules: HashMap<String, Module>,
    low_pulses: usize,
    high_pulses: usize,
}

impl Modules {
    fn new() -> Self {
        Self {
            modules: HashMap::new(),
            low_pulses: 0,
            high_pulses: 0,
        }
    }

    fn add_module(&mut self, id: &str, module: Module) {
        self.modules.insert(String::from(id), module);
    }

    fn get_module(&mut self, id: &str) -> &mut Module {
        self.modules.get_mut(id).unwrap()
    }

    fn init(&mut self) {
        let mut pulses = VecDeque::new();
        pulses.push_back(Pulse::init());

        while let Some(pulse) = pulses.pop_front() {
            // it the module not exists yet, create a new defualt module
            if self.modules.get_mut(&pulse.to).is_none() {
                self.add_module(
                    &pulse.to,
                    Module::new(&pulse.to, ModuleType::Broadcast, Vec::new()),
                )
            }

            //println!("{} -{:?}-> {}", pulse.from, pulse.state, pulse.to);

            let module = self.get_module(&pulse.to);
            let mut received = module.init(&pulse);
            pulses.append(&mut received);
        }
    }

    fn push_button(&mut self) {
        let mut pulses = VecDeque::new();
        pulses.push_back(Pulse::init());

        while let Some(pulse) = pulses.pop_front() {
            match &pulse.state {
                State::High => self.high_pulses += 1,
                State::Low => self.low_pulses += 1,
            }
            //println!("{} -{:?}-> {}", pulse.from, pulse.state, pulse.to);

            let module = self.get_module(&pulse.to);
            let mut received = module.send_pulse(&pulse);
            pulses.append(&mut received);
        }
    }
}

struct Module {
    id: String,
    init: bool,
    mod_type: ModuleType,
    destinations: Vec<String>,
}

impl Module {
    fn new(id: &str, mod_type: ModuleType, destinations: Vec<String>) -> Self {
        Self {
            id: id.to_owned(),
            init: false,
            mod_type,
            destinations,
        }
    }

    fn init(&mut self, pulse: &Pulse) -> VecDeque<Pulse> {
        if let ModuleType::Conjunction(con) = &mut self.mod_type {
            con.init(pulse);
        }

        if self.init {
            VecDeque::new()
        } else {
            self.init = true;
            self.create_next_pulse(pulse.state)
        }
    }

    fn send_pulse(&mut self, pulse: &Pulse) -> VecDeque<Pulse> {
        let pulse = match &mut self.mod_type {
            ModuleType::Broadcast => Some(State::Low),
            ModuleType::FlipFlop(ff) => ff.send(pulse),
            ModuleType::Conjunction(con) => con.send(pulse),
        };

        if let Some(state) = pulse {
            self.create_next_pulse(state)
        } else {
            VecDeque::new()
        }
    }

    fn create_next_pulse(&self, state: State) -> VecDeque<Pulse> {
        self.destinations
            .iter()
            .map(|dest| Pulse {
                state,
                from: self.id.clone(),
                to: dest.clone(),
            })
            .collect::<VecDeque<_>>()
    }
}

struct Pulse {
    state: State,
    from: String,
    to: String,
}

impl Pulse {
    fn init() -> Self {
        Pulse {
            state: State::Low,
            from: BROADCAST.to_owned(),
            to: BROADCAST.to_owned(),
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
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

impl From<&str> for ModuleType {
    fn from(value: &str) -> Self {
        match value {
            "%" => ModuleType::FlipFlop(FlipFlop { state: State::Low }),
            "&" => ModuleType::Conjunction(Conjunction {
                connected: HashMap::new(),
            }),
            _ => ModuleType::Broadcast,
        }
    }
}

struct FlipFlop {
    // Off = Low, On = High
    state: State,
}

impl FlipFlop {
    fn send(&mut self, received: &Pulse) -> Option<State> {
        if received.state == State::Low {
            self.state = self.state.toggle();
            Some(self.state)
        } else {
            None
        }
    }
}

struct Conjunction {
    connected: HashMap<String, State>,
}

impl Conjunction {
    fn init(&mut self, pulse: &Pulse) {
        self.connected.insert(pulse.from.clone(), State::Low);
    }

    fn send(&mut self, received: &Pulse) -> Option<State> {
        let state = self.connected.get_mut(&received.from).unwrap();
        *state = received.state;

        if self.connected.values().all(|&state| state == State::High) {
            Some(State::Low)
        } else {
            Some(State::High)
        }
    }
}

fn main() {
    let filename = if DEVELOP {
        "input_small_2.txt"
    } else {
        "input.txt"
    };

    let mut modules = Parser::parse_file(filename);
    modules.init();

    println!("\nPush Button:");
    for _ in 0..1000 {
        modules.push_button();
    }
    println!("Task 1: {}", modules.high_pulses * modules.low_pulses);
}
