use std::{fs::File, io::BufRead, io::BufReader};

fn split_line(line: &str) -> Vec<u64> {
    line.split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().flatten().collect::<Vec<_>>();
    let times = split_line(&lines[0]);
    let distances = split_line(&lines[1]);

    let product = times
        .iter()
        .zip(distances.iter())
        .map(|(max, distance)| {
            (1..*max)
                .map(move |t| (max - t) * t)
                .filter(|d| d > distance)
                .count()
        })
        .product::<usize>();

    println!("Task 1: {product}");
}
