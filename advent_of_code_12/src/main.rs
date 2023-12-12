use anyhow::{bail, Result};
use core::fmt;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Token {
    Unknown,
    Operational,
    Damaged,
}

impl Token {
    fn new(token: char) -> Self {
        match token {
            '?' => Token::Unknown,
            '.' => Token::Operational,
            '#' => Token::Damaged,
            _ => panic!("invalid token"),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token = match *self {
            Token::Unknown => '?',
            Token::Operational => '.',
            Token::Damaged => '#',
        };
        write!(f, "{token}")
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

    let sum: usize = reader.lines().flatten().map(calculate_arrangements).sum();

    println!("Task 1: {sum}");

    Ok(())
}

fn calculate_arrangements(line: String) -> usize {
    let (tokens, groups) = line.split_once(' ').unwrap();
    let tokens = tokens.chars().map(Token::new).collect::<Vec<_>>();
    let groups = groups
        .split(',')
        .map(|g| g.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let count = get_different_arrangements(tokens, &groups).unwrap();

    //println!("{line}: {count}\n");
    count
}

fn get_different_arrangements(mut tokens: Vec<Token>, groups: &Vec<u32>) -> Result<usize> {
    // check if tokens are valid to group
    if !are_tokens_valid(&tokens, groups) {
        bail!("invalid arrangement");
    };

    // add new token and call fn recursive
    let mut sum = 0;
    if let Some(next_pos) = tokens.iter().position(|&t| t == Token::Unknown) {
        let mut tokens_clone = tokens.clone();
        tokens_clone[next_pos] = Token::Operational;
        if let Ok(count) = get_different_arrangements(tokens_clone, groups) {
            sum += count;
        }

        tokens[next_pos] = Token::Damaged;
        if let Ok(count) = get_different_arrangements(tokens, groups) {
            sum += count;
        }
    } else {
        //println!("{tokens:?}");
        sum = 1;
    }

    Ok(sum)
}

fn are_tokens_valid(tokens: &Vec<Token>, groups: &Vec<u32>) -> bool {
    // create list of groups from tokens
    let mut token_groups = Vec::new();
    let mut current_group = None;

    for token in tokens {
        current_group = match token {
            Token::Damaged => match current_group {
                Some(group) => Some(group + 1),
                None => Some(1),
            },
            Token::Operational => {
                if let Some(group) = current_group {
                    token_groups.push(group);
                }
                None
            }
            Token::Unknown => {
                break;
            }
        }
    }

    if tokens.iter().all(|&t| t != Token::Unknown) {
        if let Some(group) = current_group {
            token_groups.push(group);
        }

        if token_groups.len() != groups.len() {
            return false;
        }
    }

    // compare token groups against the reference group
    token_groups
        .iter()
        .zip(groups)
        .all(|(curr, reference)| curr == reference)
}
