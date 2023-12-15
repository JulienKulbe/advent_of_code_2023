use anyhow::Result;
use std::fs::{self};

const DEVELOP: bool = false;

fn main() -> Result<()> {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };

    let hash: usize = fs::read_to_string(filename)?
        .split(',')
        .map(calculate_hash)
        .sum();

    println!("Task 1: {hash}");

    Ok(())
}

fn calculate_hash(value: &str) -> usize {
    value
        .as_bytes()
        .iter()
        .fold(0, |acc, curr| ((acc + (*curr as usize)) * 17) % 256)
}
