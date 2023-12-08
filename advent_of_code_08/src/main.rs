use anyhow::Result;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

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
}

fn parse_directions(line: String) -> Vec<Direction> {
    line.chars().map(Direction::new).collect()
}

fn parse_network(lines: impl Iterator<Item = String>) -> HashMap<String, (String, String)> {
    lines
        .map(|line| {
            let (source, destination) = line.split_once('=').unwrap();

            let destination = destination.trim().strip_prefix('(').unwrap();
            let destination = destination.strip_suffix(')').unwrap();
            let (left, right) = destination.split_once(',').unwrap();

            (
                source.trim().to_owned(),
                (left.trim().to_owned(), right.trim().to_owned()),
            )
        })
        .collect()
}

fn calculate_steps(
    start: String,
    is_end: impl Fn(&String) -> bool,
    directions: &Vec<Direction>,
    network: &HashMap<String, (String, String)>,
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

fn task1(directions: &Vec<Direction>, network: &HashMap<String, (String, String)>) -> u64 {
    calculate_steps(String::from("AAA"), |n| n == "ZZZ", directions, network)
}

fn task2(directions: &Vec<Direction>, network: &HashMap<String, (String, String)>) -> u64 {
    let steps = network
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
    let directions = parse_directions(lines.next().unwrap());
    lines.next(); // skip empty line
    let network = parse_network(lines);

    println!("Task 1: {}", task1(&directions, &network));
    println!("Task 2: {}", task2(&directions, &network));

    Ok(())
}
