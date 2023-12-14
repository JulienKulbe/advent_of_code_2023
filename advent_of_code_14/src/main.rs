use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = true;

fn main() -> Result<()> {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = reader
        .lines()
        .flatten()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = lines[0].len();
    let heigth = lines.len();

    // for line in lines.iter() {
    //     println!("{line:?}");
    // }

    let mut sum = 0;
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

        for y in 0..heigth {
            if lines[y][x] == 'O' {
                sum += heigth - y;
            }
        }
    }

    // println!();
    // for line in lines.iter() {
    //     println!("{line:?}");
    // }

    println!("Task 1: {sum}");

    Ok(())
}
