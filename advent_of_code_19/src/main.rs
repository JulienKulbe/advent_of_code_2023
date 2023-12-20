use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

struct Parser {}

impl Parser {
    fn parse(filename: &str) -> (Workflows, MachineParts) {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut workflows = Workflows::new();
        let mut machine_parts = MachineParts::new();
        let mut found_empty = false;
        for line in reader.lines().flatten() {
            if line.is_empty() {
                found_empty = true;
            } else if found_empty {
                machine_parts.0.push(Self::parse_part(line));
            } else {
                let (id, workflow) = Self::parse_workflow(line.as_str());
                workflows.0.insert(id, workflow);
            }
        }
        (workflows, machine_parts)
    }

    fn parse_workflow(line: &str) -> (String, Workflow) {
        let id_end = line.find('{').unwrap();
        let rule_end = line.find('}').unwrap();

        let id = line[0..id_end].to_owned();

        let mut rules = Vec::new();
        for rule in line[id_end + 1..rule_end].split(',') {
            if let Some(rule) = Self::parse_rule(rule) {
                rules.push(rule);
            } else {
                let result = RuleResult::from(rule);
                rules.push(Rule::Result(result));
            }
        }

        (id, Workflow(rules))
    }

    fn parse_rule(line: &str) -> Option<Rule> {
        if line.len() > 1 {
            if let Ok(operator) = Operator::try_from(&line[1..2]) {
                let colon = line.find(':').unwrap();
                return Some(Rule::Decision(Decision {
                    category: Category::from(&line[0..1]),
                    operator,
                    value: line[2..colon].parse::<i32>().unwrap(),
                    result: RuleResult::from(&line[colon + 1..]),
                }));
            }
        }
        None
    }

    fn parse_part(line: String) -> Part {
        let mut machine_part = Part::new();
        for part in line[1..line.len() - 1].split(',') {
            let category = Category::from(&part[0..1]);
            let value = part[2..].parse::<i32>().unwrap();
            machine_part.set(category, value)
        }
        machine_part
    }
}

struct Workflows(HashMap<String, Workflow>);

impl Workflows {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn apply(&self, part: &Part) -> RuleResult {
        let mut result = RuleResult::GoTo(String::from("in"));
        while let RuleResult::GoTo(label) = result {
            let workflow = self.0.get(&label).unwrap();

            result = RuleResult::Rejected;
            for rule in workflow.0.iter() {
                if let Some(rule_result) = rule.apply(part) {
                    result = rule_result;
                    break;
                }
            }
        }

        result
    }

    fn apply_range(&self, parts: &PartRange) -> PartRange {
        let workflow = self.0.get(&String::from("in")).unwrap();
        workflow.apply_range(parts)
    }
}

struct Workflow(Vec<Rule>);

impl Workflow {
    fn apply_range(&self, parts: &PartRange) -> PartRange {
        // for rule in self.0.iter() {

        // }
        todo!()
    }
}

enum Rule {
    Decision(Decision),
    Result(RuleResult),
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<RuleResult> {
        match &self {
            Rule::Decision(decision) => decision.decide(part),
            Rule::Result(result) => Some(result.clone()),
        }
    }

    fn apply_range(&self, part: &mut PartRange) -> (PartRange, RuleResult) {
        todo!()
    }
}

struct Decision {
    category: Category,
    operator: Operator,
    value: i32,
    result: RuleResult,
}

impl Decision {
    fn decide(&self, part: &Part) -> Option<RuleResult> {
        let part_value = part.get(self.category);

        let is_true = match self.operator {
            Operator::Less => part_value < self.value,
            Operator::More => part_value > self.value,
        };

        if is_true {
            Some(self.result.clone())
        } else {
            None
        }
    }
}

enum Operator {
    Less,
    More,
}

impl TryFrom<&str> for Operator {
    type Error = ();

    fn try_from(op: &str) -> Result<Self, Self::Error> {
        match op {
            "<" => Ok(Operator::Less),
            ">" => Ok(Operator::More),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RuleResult {
    GoTo(String),
    Accepted,
    Rejected,
}

impl From<&str> for RuleResult {
    fn from(value: &str) -> Self {
        match value {
            "A" => RuleResult::Accepted,
            "R" => RuleResult::Rejected,
            _ => Self::GoTo(String::from(value)),
        }
    }
}

struct MachineParts(Vec<Part>);

impl MachineParts {
    fn new() -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug, Clone, Copy)]
enum Category {
    CoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl From<&str> for Category {
    fn from(c: &str) -> Self {
        match c {
            "x" => Category::CoolLooking,
            "m" => Category::Musical,
            "a" => Category::Aerodynamic,
            "s" => Category::Shiny,
            _ => panic!("invalid string"),
        }
    }
}

struct Part {
    cool_looking: i32,
    musical: i32,
    aerodynamic: i32,
    shiny: i32,
}

impl Part {
    fn new() -> Self {
        Self {
            cool_looking: 0,
            musical: 0,
            aerodynamic: 0,
            shiny: 0,
        }
    }

    fn set(&mut self, category: Category, value: i32) {
        match category {
            Category::CoolLooking => self.cool_looking = value,
            Category::Musical => self.musical = value,
            Category::Aerodynamic => self.aerodynamic = value,
            Category::Shiny => self.shiny = value,
        }
    }

    fn get(&self, category: Category) -> i32 {
        match category {
            Category::CoolLooking => self.cool_looking,
            Category::Musical => self.musical,
            Category::Aerodynamic => self.aerodynamic,
            Category::Shiny => self.shiny,
        }
    }

    fn rating(&self) -> i32 {
        self.cool_looking + self.musical + self.aerodynamic + self.shiny
    }
}

struct PartRange {
    cool_looking: Vec<i32>,
    musical: Vec<i32>,
    aerodynamic: Vec<i32>,
    shiny: Vec<i32>,
}

impl PartRange {
    fn new() -> Self {
        let values: Vec<i32> = (1..4000).collect();
        PartRange {
            cool_looking: values.clone(),
            musical: values.clone(),
            aerodynamic: values.clone(),
            shiny: values,
        }
    }

    fn combinations(&self) -> usize {
        self.cool_looking.len() * self.musical.len() * self.aerodynamic.len() * self.shiny.len()
    }
}

fn main() {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };

    let (workflows, machine_parts) = Parser::parse(filename);

    {
        let ratings: i32 = machine_parts
            .0
            .iter()
            .filter(|part| workflows.apply(part) == RuleResult::Accepted)
            .map(|part| part.rating())
            .sum();
        println!("Task 1: {ratings}");
    }
    {
        let parts = workflows.apply_range(&PartRange::new());
        println!("Task 2: {}", parts.combinations());
    }

    println!();
}
