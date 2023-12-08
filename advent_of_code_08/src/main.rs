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

fn task1(directions: &Vec<Direction>, network: &HashMap<String, (String, String)>) -> usize {
    let mut current = String::from("AAA");
    for i in 1.. {
        for direction in directions {
            let node = network.get(&current).unwrap();
            current = match direction {
                Direction::Left => node.0.to_owned(),
                Direction::Right => node.1.to_owned(),
            }
        }

        if current == "ZZZ" {
            return i * directions.len();
        }
    }
    unreachable!()
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
    lines.next();
    let network = parse_network(lines);

    println!("Task 1: {}", task1(&directions, &network));

    Ok(())
}
