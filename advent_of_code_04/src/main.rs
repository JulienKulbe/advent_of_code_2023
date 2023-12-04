use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DEBUG: bool = true;
const DEVELOP: bool = false;

fn main() -> Result<()> {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let sum = reader.lines().flatten().fold(0, |acc, line| {
        let (_, card) = line.split_once(':').unwrap();
        let (winning, current) = card.split_once('|').unwrap();

        let winning = winning.split_whitespace().collect::<Vec<_>>();
        let count = current
            .split_whitespace()
            .filter(|curr| winning.contains(curr))
            .count() as u32;

        acc + if count == 0 { 0 } else { 2_u32.pow(count - 1) }
    });

    println!("Task 1: {sum}");

    Ok(())
}
