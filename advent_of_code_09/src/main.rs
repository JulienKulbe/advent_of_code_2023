use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

fn main() -> Result<()> {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines().flatten() {
        let mut numbers = Vec::new();
        numbers.push(
            line.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>(),
        );

        for i in 0.. {
            let mut next = Vec::new();
            for j in 1..numbers[i].len() {
                next.push(numbers[i][j] - numbers[i][j - 1]);
            }
            let all_zeros = next.iter().all(|v| *v == 0);
            numbers.push(next);

            if all_zeros {
                break;
            }
        }

        numbers.last_mut().unwrap().push(0);
        for i in (1..numbers.len()).rev() {
            let next = numbers[i].last().unwrap() + numbers[i - 1].last().unwrap();
            numbers[i - 1].push(next);
        }

        sum += numbers[0].last().unwrap();
    }

    println!("Task 1: {sum}\n");

    Ok(())
}
