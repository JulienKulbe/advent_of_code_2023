use anyhow::Result;
use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Ranking {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq)]
struct Hand {
    rank: Ranking,
    cards: [u8; 5],
    bid: usize,
}

impl Hand {
    fn new(line: String) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards = cards.chars().map(Hand::get_card_value).collect::<Vec<_>>();
        let cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];

        Hand {
            rank: Hand::get_ranking(&cards),
            cards,
            bid: bid.parse::<usize>().unwrap(),
        }
    }

    fn get_card_value(c: char) -> u8 {
        match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '2'..='9' => c.to_digit(10).unwrap() as u8,
            _ => panic!("Invalid value"),
        }
    }

    fn get_ranking(cards: &[u8; 5]) -> Ranking {
        let mut duplicates = [0; 15];
        for card in cards {
            duplicates[*card as usize] += 1;
        }

        if duplicates.iter().any(|v| *v == 5) {
            return Ranking::FiveOfAKind;
        }
        if duplicates.iter().any(|v| *v == 4) {
            return Ranking::FourOfAKind;
        }
        if duplicates.iter().any(|v| *v == 3) {
            if duplicates.iter().any(|v| *v == 2) {
                return Ranking::FullHouse;
            } else {
                return Ranking::ThreeOfAKind;
            }
        }
        if duplicates.iter().any(|v| *v == 2) {
            if duplicates.iter().filter(|v| **v == 2).count() == 2 {
                return Ranking::TwoPair;
            } else {
                return Ranking::Pair;
            }
        }
        Ranking::HighCard
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.rank, self.cards).cmp(&(other.rank, other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        (self.rank, self.cards) == (other.rank, other.cards)
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

    let mut hands = reader.lines().flatten().map(Hand::new).collect::<Vec<_>>();
    hands.sort();

    let product: usize = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum();

    println!("Task 1: {product}");

    Ok(())
}
