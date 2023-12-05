use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

#[derive(Debug)]
struct MappingEntry {
    dst: u64,
    src: u64,
    range: u64,
}

impl MappingEntry {
    fn new(line: String) -> Self {
        let values = line
            .split_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        MappingEntry {
            dst: values[0],
            src: values[1],
            range: values[2],
        }
    }
}

#[derive(Debug, Default)]
struct Map {
    entries: Vec<MappingEntry>,
}

impl Map {
    fn map(&self, value: u64) -> u64 {
        for entry in self.entries.iter() {
            if entry.src <= value && value < entry.src + entry.range {
                return value - entry.src + entry.dst;
            }
        }
        value
    }
}

fn main() -> Result<()> {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut seeds = Vec::new();
    let mut maps = Vec::new();
    for line in reader.lines().flatten() {
        if line.is_empty() {
            continue;
        }

        if let Some((_, values)) = line.split_once(':') {
            if values.is_empty() {
                maps.push(Map {
                    ..Default::default()
                });
            } else {
                seeds = values
                    .split_whitespace()
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();
            }
        } else {
            let current_map = maps.last_mut().unwrap();
            current_map.entries.push(MappingEntry::new(line));
        }
    }

    let min = seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |s, m| m.map(s)))
        .min()
        .unwrap();

    println!("Task 1: {min}");

    Ok(())
}
