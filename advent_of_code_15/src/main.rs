use anyhow::Result;
use std::fs::{self};

const DEVELOP: bool = false;

enum Operation {
    Equals,
    Dash,
}

#[derive(Debug)]
struct Lense {
    label: String,
    focal_length: usize,
}

#[derive(Debug, Default)]
struct FocalBox {
    lenses: Vec<Lense>,
}

impl FocalBox {
    fn remove_lens(&mut self, lense: Lense) {
        if let Some(index) = self.get_lense_index(&lense) {
            self.lenses.remove(index);
        }
    }

    fn replace_lens(&mut self, lense: Lense) {
        if let Some(index) = self.get_lense_index(&lense) {
            self.lenses.get_mut(index).unwrap().focal_length = lense.focal_length;
        } else {
            self.lenses.push(lense);
        }
    }

    fn get_lense_index(&self, lense: &Lense) -> Option<usize> {
        self.lenses.iter().position(|l| l.label == lense.label)
    }
}

fn main() -> Result<()> {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };

    {
        let hash: usize = fs::read_to_string(filename)?
            .split(',')
            .map(calculate_hash)
            .sum();
        println!("Task 1: {hash}");
    }
    {
        let file_content = fs::read_to_string(filename)?;
        let mut boxes: [FocalBox; 256] = array_init::array_init(|_: usize| FocalBox::default());

        for entry in file_content.split(',') {
            let (lense, op) = parse_string(entry);
            let hash = calculate_hash(&lense.label);
            let current = boxes.get_mut(hash).unwrap();
            match op {
                Operation::Dash => current.remove_lens(lense),
                Operation::Equals => current.replace_lens(lense),
            }
        }

        let focusing_power = boxes
            .iter()
            .enumerate()
            .map(|(i, b)| {
                b.lenses
                    .iter()
                    .enumerate()
                    .map(|(j, l)| l.focal_length * (i + 1) * (j + 1))
                    .sum::<usize>()
            })
            .sum::<usize>();

        println!("Task 2: {focusing_power}");
    }

    Ok(())
}

fn parse_string(string: &str) -> (Lense, Operation) {
    if let Some(index) = string.find('=') {
        let lense = Lense {
            label: string[..index].to_string(),
            focal_length: string[index + 1..].parse::<usize>().unwrap(),
        };
        (lense, Operation::Equals)
    } else if let Some(index) = string.find('-') {
        let lense = Lense {
            label: string[..index].to_string(),
            focal_length: 0,
        };
        (lense, Operation::Dash)
    } else {
        panic!("invalid substring");
    }
}

fn calculate_hash(value: &str) -> usize {
    value
        .as_bytes()
        .iter()
        .fold(0, |acc, curr| ((acc + (*curr as usize)) * 17) % 256)
}
