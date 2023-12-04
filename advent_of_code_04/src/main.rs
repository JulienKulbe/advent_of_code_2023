use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

fn task1(reader: BufReader<File>) -> u32 {
    reader.lines().flatten().fold(0, |acc, line| {
        let (_, card) = line.split_once(':').unwrap();
        let (winning, current) = card.split_once('|').unwrap();

        let winning = winning.split_whitespace().collect::<Vec<_>>();
        let count = current
            .split_whitespace()
            .filter(|curr| winning.contains(curr))
            .count() as u32;

        acc + if count == 0 { 0 } else { 2_u32.pow(count - 1) }
    })
}

struct Card {
    winning: Vec<u32>,
    current: Vec<u32>,
    copies: u32,
}

impl Card {
    fn new(content: String) -> Card {
        let (_, card) = content.split_once(':').unwrap();
        let (winning, current) = card.split_once('|').unwrap();

        let winning = winning
            .split_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let current = current
            .split_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        Card {
            winning,
            current,
            copies: 1,
        }
    }

    fn get_matching_cards(&self) -> usize {
        self.current
            .iter()
            .filter(|c| self.winning.contains(c))
            .count()
    }

    fn increase(&mut self, v: u32) {
        self.copies += v;
    }
}

fn task2(reader: BufReader<File>) -> u32 {
    let mut cards = reader.lines().flatten().map(Card::new).collect::<Vec<_>>();

    let winnings = cards
        .iter()
        .map(|c| c.get_matching_cards())
        .collect::<Vec<_>>();

    for (i, count) in winnings.iter().enumerate() {
        let copies = cards[i].copies;
        cards
            .iter_mut()
            .skip(i + 1)
            .take(*count)
            .for_each(|c| c.increase(copies));
    }

    cards.iter().map(|c| c.copies).sum()
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
