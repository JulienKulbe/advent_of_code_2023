use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

const MULTIPLY: usize = 1_000_000;

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
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

    // create vec with all galaxy coordinates
    let mut galaxies = Vec::new();
    let mut width = 0;
    let mut heigth = 0;
    for (y, line) in reader.lines().flatten().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Galaxy::new(x, y));
            }
            width = x;
        }
        heigth = y;
    }

    // found rows and columns without any galaxies
    let rows = (0..=heigth)
        .filter(|&y| galaxies.iter().all(|g| g.y != y))
        .collect::<Vec<_>>();
    let columns = (0..=width)
        .filter(|&x| galaxies.iter().all(|g| g.x != x))
        .collect::<Vec<_>>();

    // correct galaxy coordinates
    for galaxy in &mut galaxies {
        galaxy.y += (MULTIPLY - 1) * rows.iter().filter(|&&r| r < galaxy.y).count();
        galaxy.x += (MULTIPLY - 1) * columns.iter().filter(|&&c| c < galaxy.x).count();
    }

    // calculate manhattan distance of each pair
    let mut distance = 0;
    for (index, galaxy_1) in galaxies.iter().enumerate() {
        for galaxy_2 in galaxies.iter().skip(index) {
            distance += galaxy_1.x.abs_diff(galaxy_2.x) + galaxy_1.y.abs_diff(galaxy_2.y);
        }
    }

    println!("Task: {distance}");

    Ok(())
}
