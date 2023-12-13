use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

struct Map {
    data: Vec<Vec<char>>,
    width: usize,
    heigth: usize,
}

impl Map {
    fn new() -> Self {
        Map {
            data: Vec::new(),
            width: 0,
            heigth: 0,
        }
    }

    fn add_data(&mut self, line: &str) {
        self.data.push(line.chars().collect());
        self.width = self.data.iter().last().unwrap().len();
        self.heigth += 1;
    }

    fn get_symmetry(&self) -> usize {
        for column in 0..self.width - 1 {
            if self.check_vertical_symmetry(column) {
                return column + 1;
            }
        }

        for row in 0..self.heigth - 1 {
            if self.check_horizontal_symmetry(row) {
                return (row + 1) * 100;
            }
        }

        panic!("no symmetry found");
    }

    fn check_vertical_symmetry(&self, column: usize) -> bool {
        for i in 0.. {
            let left = column - i;
            let right = column + 1 + i;

            for y in 0..self.heigth {
                if self.data[y][left] != self.data[y][right] {
                    return false;
                }
            }

            if left == 0 || right == self.width - 1 {
                return true;
            }
        }
        unreachable!()
    }

    fn check_horizontal_symmetry(&self, row: usize) -> bool {
        for i in 0.. {
            let top = row - i;
            let bottom = row + 1 + i;

            for x in 0..self.width {
                if self.data[top][x] != self.data[bottom][x] {
                    return false;
                }
            }

            if top == 0 || bottom == self.heigth - 1 {
                return true;
            }
        }
        unreachable!()
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

    let mut maps = Vec::new();
    let mut current_map = Map::new();
    for line in reader.lines().flatten() {
        if line.is_empty() {
            maps.push(current_map);
            current_map = Map::new();
        } else {
            current_map.add_data(line.as_str());
        }
    }
    maps.push(current_map);

    let sum: usize = maps.iter().map(|m| m.get_symmetry()).sum();
    println!("Task 1: {sum}");

    Ok(())
}
