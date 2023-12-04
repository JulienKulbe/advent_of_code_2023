use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

fn task1(reader: BufReader<File>) -> u32 {
    reader.lines().flatten().fold(0, |acc, line| {
        let count = get_winning_cards(line);
        acc + if count == 0 { 0 } else { 2_u32.pow(count - 1) }
    })
}

fn task2(reader: BufReader<File>) -> u32 {
    let winnings = reader
        .lines()
        .flatten()
        .map(get_winning_cards)
        .collect::<Vec<_>>();

    let mut copies = vec![1; winnings.len()];
    for (i, count) in winnings.iter().enumerate() {
        for j in 0..*count {
            let j = j as usize;
            copies[i + j + 1] += copies[i];
        }
    }
    copies.iter().sum()
}

fn get_winning_cards(line: String) -> u32 {
    let (_, card) = line.split_once(':').unwrap();
    let (winning, current) = card.split_once('|').unwrap();

    let winning = winning.split_whitespace().collect::<Vec<_>>();
    current
        .split_whitespace()
        .filter(|c| winning.contains(c))
        .count() as u32
}

fn main() -> Result<()> {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    //println!("Task 1: {}", task1(reader));

    println!("Task 2: {}", task2(reader));

    Ok(())
}
