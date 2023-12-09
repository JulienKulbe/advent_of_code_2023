use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = true;

#[derive(Debug, Clone, Copy)]
enum Direction {
    First,
    Last,
}

fn calculate(filename: &str, direction: Direction) -> i32 {
    let file = File::open(filename).unwrap();
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

        let last_row = numbers.last_mut().unwrap();
        match direction {
            Direction::Last => last_row.push(0),
            Direction::First => last_row.insert(0, 0),
        }

        for i in (1..numbers.len()).rev() {
            match direction {
                Direction::Last => {
                    let next = numbers[i].last().unwrap() + numbers[i - 1].last().unwrap();
                    numbers[i - 1].push(next);
                }
                Direction::First => {
                    let next = numbers[i - 1].first().unwrap() - numbers[i].first().unwrap();
                    numbers[i - 1].insert(0, next);
                }
            }
        }

        sum += match direction {
            Direction::Last => numbers[0].last().unwrap(),
            Direction::First => numbers[0].first().unwrap(),
        }
    }

    sum
}

fn main() {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };

    println!("Task 1: {}", calculate(filename, Direction::Last));
    println!("Task 2: {}", calculate(filename, Direction::First));
}
