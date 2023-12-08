use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::alphanumeric1,
    character::complete::char,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = true;

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn new(c: char) -> Self {
        match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Invalid direction"),
        }
    }

    fn parse_directions(line: String) -> Vec<Direction> {
        line.chars().map(Direction::new).collect()
    }
}

struct Network(HashMap<String, (String, String)>);

impl Network {
    fn parse_network(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| Network::parse_line(line.as_str()))
                .collect(),
        )
    }

    fn parse_line(input: &str) -> (String, (String, String)) {
        let (destination, source) = Network::split_source_from_destination(input).unwrap();
        let (_, (first, second)) = Network::parse_route(destination).unwrap();
        (source.to_owned(), (first.to_owned(), second.to_owned()))
    }

    fn split_source_from_destination(input: &str) -> IResult<&str, &str> {
        terminated(alphanumeric1, tag(" = "))(input)
    }

    fn parse_route(input: &str) -> IResult<&str, (&str, &str)> {
        delimited(
            char('('),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            char(')'),
        )(input)
    }

    fn get(&self, node: &str) -> Option<&(String, String)> {
        self.0.get(node)
    }
}

fn calculate_steps(
    start: String,
    is_end: impl Fn(&String) -> bool,
    directions: &Vec<Direction>,
    network: &Network,
) -> u64 {
    let mut current = start;
    for i in 1.. {
        current = directions.iter().fold(current.clone(), |node, direction| {
            let node = network.get(&node).unwrap();
            match direction {
                Direction::Left => node.0.clone(),
                Direction::Right => node.1.clone(),
            }
        });

        if is_end(&current) {
            return i * directions.len() as u64;
        }
    }
    unreachable!()
}

fn task1(directions: &Vec<Direction>, network: &Network) -> u64 {
    calculate_steps(String::from("AAA"), |n| n == "ZZZ", directions, network)
}

fn task2(directions: &Vec<Direction>, network: &Network) -> u64 {
    let steps = network
        .0
        .keys()
        .filter(|n| n.ends_with('A'))
        .cloned()
        .map(|node| calculate_steps(node, |n| n.ends_with('Z'), directions, network))
        .collect::<Vec<_>>();

    lcmx::lcmx(&steps).unwrap()
}

fn main() -> Result<()> {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().flatten();
    let directions = Direction::parse_directions(lines.next().unwrap());
    lines.next(); // skip empty line
    let network = Network::parse_network(lines);

    //println!("Task 1: {}", task1(&directions, &network));
    println!("Task 2: {}", task2(&directions, &network));

    Ok(())
}
