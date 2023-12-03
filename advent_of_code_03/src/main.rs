use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

#[derive(Debug, Clone, Copy)]
struct Position(usize, usize);

impl Position {
    fn prev_y(mut self) -> Self {
        if self.1 > 0 {
            self.1 -= 1;
        }
        self
    }

    fn prev_x(mut self) -> Self {
        if self.0 > 0 {
            self.0 -= 1;
        }
        self
    }

    fn add_y(mut self, add: usize, max: usize) -> Self {
        self.1 += add;
        if self.1 >= max {
            self.1 = max - 1;
        }
        self
    }
}

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn new(source: impl Iterator<Item = String>) -> Self {
        let grid = source
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();
        Self(grid)
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn is_digit(&self, pos: Position) -> bool {
        self.0[pos.0][pos.1].is_ascii_digit()
    }

    fn is_gear(&self, pos: Position) -> bool {
        self.0[pos.0][pos.1] == '*'
    }

    fn is_symbol(&self, pos: Position) -> bool {
        self.0[pos.0][pos.1] != '.' && !self.is_digit(pos)
    }
}

struct PartNumber<'a> {
    grid: &'a Grid,
    start: Position,
    length: usize,
    number: u32,
}

impl<'a> PartNumber<'a> {
    fn new(grid: &'a Grid, start: Position) -> Self {
        let length = (start.1..grid.width())
            .map(|y| Position(start.0, y))
            .position(|pos| !grid.is_digit(pos))
            .unwrap_or(grid.width() - start.1);

        let mut number = String::with_capacity(length);
        for j in 0..length {
            number.push(grid.0[start.0][start.1 + j]);
        }
        let number = number.parse::<u32>().unwrap();

        Self {
            grid,
            start,
            length,
            number,
        }
    }

    fn end(&self) -> Position {
        Position(self.start.0, self.start.1 + self.length - 1)
    }

    fn is_part(&self) -> bool {
        let mut is_symbol = false;
        // check row above
        if self.start.0 > 0 {
            is_symbol = (self.start.prev_y().1
                ..=self.start.add_y(self.length, self.grid.width()).1)
                .map(|y| Position(self.start.0 - 1, y))
                .any(|pos| self.grid.is_symbol(pos));
        }
        // check row below
        if !is_symbol && self.start.0 + 1 < self.grid.height() {
            is_symbol = (self.start.prev_y().1
                ..=self.start.add_y(self.length, self.grid.width()).1)
                .map(|y| Position(self.start.0 + 1, y))
                .any(|pos| self.grid.is_symbol(pos));
        }
        // check left
        if !is_symbol && self.start.1 > 0 {
            is_symbol = self
                .grid
                .is_symbol(Position(self.start.0, self.start.1 - 1));
        }
        // check right
        if !is_symbol && self.start.1 + self.length < self.grid.width() {
            is_symbol = self
                .grid
                .is_symbol(Position(self.start.0, self.start.1 + self.length));
        }
        is_symbol
    }
}

struct Gear {
    pos: Position,
    parts: Vec<u32>,
}

impl Gear {
    fn new(pos: Position) -> Self {
        Self {
            pos,
            parts: Vec::new(),
        }
    }

    fn next_to(&mut self, part: &PartNumber) {
        if part.start.prev_y().1 <= self.pos.1
            && part.end().1 + 1 >= self.pos.1
            && part.start.prev_x().0 <= self.pos.0
            && part.end().0 + 1 >= self.pos.0
        {
            self.parts.push(part.number);
        }
    }

    fn gear_ratio(&self) -> u32 {
        if self.parts.len() == 2 {
            self.parts.iter().product()
        } else {
            0
        }
    }
}

fn task1(grid: &Grid) -> u32 {
    let mut parts = 0;
    for i in 0..grid.height() {
        let mut j = 0;
        while j < grid.width() {
            j += if grid.is_digit(Position(i, j)) {
                let part = PartNumber::new(grid, Position(i, j));
                if part.is_part() {
                    parts += part.number;
                }
                part.length
            } else {
                1
            }
        }
    }
    parts
}

fn task2(grid: &Grid) -> u32 {
    // iterate over grid and find gears
    let mut gears = Vec::new();
    for i in 0..grid.height() {
        for j in 0..grid.width() {
            if grid.is_gear(Position(i, j)) {
                gears.push(Gear::new(Position(i, j)));
            }
        }
    }

    // iterate over grid and create parts
    let mut parts = Vec::new();
    for i in 0..grid.height() {
        let mut j = 0;
        while j < grid.width() {
            if grid.is_digit(Position(i, j)) {
                let part = PartNumber::new(grid, Position(i, j));
                j += part.length;
                parts.push(part);
            } else {
                j += 1;
            }
        }
    }

    // chech if parts are next to gears
    for gear in &mut gears {
        for part in &parts {
            gear.next_to(part);
        }
    }

    // count gear ratio
    let mut sum = 0;
    for gear in &gears {
        sum += gear.gear_ratio();
    }
    sum
}

fn main() -> Result<()> {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let grid = Grid::new(reader.lines().flatten());

    println!("Task 1 {}", task1(&grid));
    println!("Task 2 {}", task2(&grid));

    Ok(())
}
