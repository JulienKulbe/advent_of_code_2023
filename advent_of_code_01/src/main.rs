use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[allow(dead_code)]
fn task1(reader: BufReader<File>) -> u32 {
    reader
        .lines()
        .flatten()
        .map(|line| {
            let digits: Vec<_> = line
                .chars()
                .map(|x| x.to_digit(10))
                .filter(Option::is_some)
                .collect();
            10 * digits.first().unwrap().unwrap() + digits.last().unwrap().unwrap()
        })
        .sum()
}

fn task2(reader: BufReader<File>) -> u32 {
    let mut sum = 0;
    for line in reader.lines().flatten() {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;

        for i in 0..line.len() {
            let sub_string = &line[i..];
            let digit = is_digit(sub_string);

            if first.is_none() {
                first = digit;
            }
            if digit.is_some() {
                last = digit;
            }
        }

        sum += 10 * first.unwrap() + last.unwrap();
    }

    sum
}

fn is_digit(str: &str) -> Option<u32> {
    match str {
        _ if str.starts_with("one") | str.starts_with('1') => Some(1),
        _ if str.starts_with("two") | str.starts_with('2') => Some(2),
        _ if str.starts_with("three") | str.starts_with('3') => Some(3),
        _ if str.starts_with("four") | str.starts_with('4') => Some(4),
        _ if str.starts_with("five") | str.starts_with('5') => Some(5),
        _ if str.starts_with("six") | str.starts_with('6') => Some(6),
        _ if str.starts_with("seven") | str.starts_with('7') => Some(7),
        _ if str.starts_with("eight") | str.starts_with('8') => Some(8),
        _ if str.starts_with("nine") | str.starts_with('9') => Some(9),
        _ => None,
    }
}

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    println!("Sum: {}", task2(reader));

    Ok(())
}
