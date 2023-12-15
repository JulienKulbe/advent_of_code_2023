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

    {
        let mut lines = create_map(filename);
        tilt_north(&mut lines);
        println!("Task 1: {}", calculate_load(&lines));
    }
    {
        let mut lines = create_map(filename);

        let mut history = Vec::new();
        for i in 1..200 {
            tilt_north(&mut lines);
            tilt_west(&mut lines);
            tilt_south(&mut lines);
            tilt_east(&mut lines);

            if i > 161 && i < 180 {
                println!("{i}: {}", calculate_load(&lines));
            }
            // if i < 10 {
            //     //print_map(&lines);
            //     println!("{i}: {}", calculate_load(&lines));
            // }

            let current = lines.clone();
            for (j, previous) in history.iter().enumerate() {
                if *previous == current {
                    println!("found duplicate {j} --> {i} ");
                    break;
                }
            }
            history.push(current);
        }

        // (i-10) % 7 + 2
        // (1000000000 - 10) % 7 + 2 = 5

        // (i - 180) % 18 + 162
        // (1000000000 - 180) % 18 + 162 = 172

        println!("Task 2: {}", calculate_load(&lines));
    }

    Ok(())
}

fn print_map(lines: &Vec<Vec<char>>) {
    println!();
    for line in lines {
        println!("{line:?}");
    }
}

fn create_map(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .flatten()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn tilt_north(lines: &mut Vec<Vec<char>>) {
    let width = lines[0].len();
    let heigth = lines.len();

    for x in 0..width {
        let mut next_pos: Option<usize> = None;
        let mut y = 0;

        while y < heigth {
            match lines[y][x] {
                'O' => {
                    if let Some(next) = next_pos {
                        lines[next][x] = 'O';
                        lines[y][x] = '.';
                        next_pos = None;
                        y = next;
                    }
                }
                '#' => {
                    next_pos = None;
                }
                '.' => {
                    if next_pos.is_none() {
                        next_pos = Some(y);
                    }
                }
                _ => panic!("invalid token"),
            }
            y += 1;
        }
    }
}

fn tilt_south(lines: &mut Vec<Vec<char>>) {
    let width = lines[0].len();
    let heigth = lines.len();

    for x in 0..width {
        let mut next_pos: Option<usize> = None;
        let mut y = heigth - 1;

        loop {
            match lines[y][x] {
                'O' => {
                    if let Some(next) = next_pos {
                        lines[next][x] = 'O';
                        lines[y][x] = '.';
                        next_pos = None;
                        y = next;
                    }
                }
                '#' => {
                    next_pos = None;
                }
                '.' => {
                    if next_pos.is_none() {
                        next_pos = Some(y);
                    }
                }
                _ => panic!("invalid token"),
            }

            if y == 0 {
                break;
            } else {
                y -= 1;
            }
        }
    }
}

fn tilt_west(lines: &mut Vec<Vec<char>>) {
    let width = lines[0].len();
    let heigth = lines.len();

    for y in 0..heigth {
        let mut next_pos: Option<usize> = None;
        let mut x = 0;

        while x < width {
            match lines[y][x] {
                'O' => {
                    if let Some(next) = next_pos {
                        lines[y][next] = 'O';
                        lines[y][x] = '.';
                        next_pos = None;
                        x = next;
                    }
                }
                '#' => {
                    next_pos = None;
                }
                '.' => {
                    if next_pos.is_none() {
                        next_pos = Some(x);
                    }
                }
                _ => panic!("invalid token"),
            }
            x += 1;
        }
    }
}

fn tilt_east(lines: &mut Vec<Vec<char>>) {
    let width = lines[0].len();
    let heigth = lines.len();

    for y in 0..heigth {
        let mut next_pos: Option<usize> = None;
        let mut x = width - 1;

        loop {
            match lines[y][x] {
                'O' => {
                    if let Some(next) = next_pos {
                        lines[y][next] = 'O';
                        lines[y][x] = '.';
                        next_pos = None;
                        x = next;
                    }
                }
                '#' => {
                    next_pos = None;
                }
                '.' => {
                    if next_pos.is_none() {
                        next_pos = Some(x);
                    }
                }
                _ => panic!("invalid token"),
            }

            if x == 0 {
                break;
            } else {
                x -= 1;
            }
        }
    }
}

fn calculate_load(lines: &Vec<Vec<char>>) -> usize {
    let width = lines[0].len();
    let heigth = lines.len();

    let mut sum = 0;
    for x in 0..width {
        for y in 0..heigth {
            if lines[y][x] == 'O' {
                sum += heigth - y;
            }
        }
    }
    sum
}
