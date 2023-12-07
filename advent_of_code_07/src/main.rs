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
    fn new(line: &str, use_joker: bool) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards = cards
            .chars()
            .map(|c| Hand::get_card_value(c, use_joker))
            .collect::<Vec<_>>();
        let cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];

        Hand {
            rank: Hand::get_ranking(cards),
            cards,
            bid: bid.parse::<usize>().unwrap(),
        }
    }

    fn get_card_value(c: char, use_joker: bool) -> u8 {
        let j = if use_joker { 1 } else { 11 };
        match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => j,
            'T' => 10,
            '2'..='9' => c.to_digit(10).unwrap() as u8,
            _ => panic!("Invalid value"),
        }
    }

    fn get_ranking(cards: [u8; 5]) -> Ranking {
        // count all duplicated cards
        let mut duplicates = [0; 15];
        for card in cards {
            duplicates[card as usize] += 1;
        }

        // add the jokers the cards with the most duplicates
        let jokers = duplicates[1];
        duplicates[1] = 0;
        let max_index = duplicates
            .iter()
            .enumerate()
            .max_by(|x, y| x.1.cmp(y.1))
            .unwrap()
            .0;
        duplicates[max_index] += jokers;

        // check the rankings from high to low
        if duplicates.iter().any(|v| *v == 5) {
            return Ranking::FiveOfAKind;
        }
        if duplicates.iter().any(|v| *v == 4) {
            return Ranking::FourOfAKind;
        }
        if duplicates.iter().any(|v| *v == 3) {
            if duplicates.iter().any(|v| *v == 2) {
                return Ranking::FullHouse;
            }
            return Ranking::ThreeOfAKind;
        }
        if duplicates.iter().any(|v| *v == 2) {
            let two_pair = duplicates.iter().filter(|v| **v == 2).count() == 2;
            if two_pair {
                return Ranking::TwoPair;
            }
            return Ranking::Pair;
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

fn task(filename: &str, use_joker: bool) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut hands = reader
        .lines()
        .flatten()
        .map(|line| Hand::new(line.as_str(), use_joker))
        .collect::<Vec<_>>();
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum()
}

fn main() {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };

    println!("Task 1: {}", task(filename, false));
    println!("Task 2: {}", task(filename, true));
}
