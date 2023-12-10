use anyhow::Result;
use core::fmt;
use std::fs::{self};

const DEVELOP: bool = false;

struct Literals;

impl Literals {
    const LF: u8 = 10;
    const NORTH_TO_SOUTH: u8 = 124; // |
    const WEST_TO_EAST: u8 = 45; // -
    const NORTH_TO_EAST: u8 = 76; // L
    const NORTH_TO_WEST: u8 = 74; // J
    const WEST_TO_SOUTH: u8 = 55; // 7
    const EAST_TO_SOUTH: u8 = 70; // F
    const DOT: u8 = 46; // .
    const START: u8 = 83; // S
}

type Position = (usize, usize);

struct Maze<'a> {
    data: &'a [u8],
    width: usize,
    heigth: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Closed,
    Path,
    Loop,
}

struct FloodMap {
    map: Vec<Vec<Tile>>,
    width: usize,
    heigth: usize,
}

impl FloodMap {
    fn new(width: usize, heigth: usize) -> Self {
        let mut map = Vec::new();
        for _ in 0..heigth {
            // create a row with: Closed | Path | Closed | Path | ... | Closed
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Tile::Closed);
                row.push(Tile::Path);
            }
            row.pop(); // remove last path
            map.push(row);

            // create a row with: Path | Path | Path | Path | ... | Path
            map.push(vec![Tile::Path; 2 * width - 1]);
        }
        map.pop(); // remove last path row

        Self {
            map,
            width: width * 2 - 1,
            heigth: heigth * 2 - 1,
        }
    }

    fn add_loop(&mut self, from: Position, to: Position) {
        let from = (from.0 * 2, from.1 * 2);
        let to = (to.0 * 2, to.1 * 2);
        let over = ((from.0 + to.0) / 2, (from.1 + to.1) / 2);

        self.map[from.1][from.0] = Tile::Loop;
        self.map[over.1][over.0] = Tile::Loop;
        self.map[to.1][to.0] = Tile::Loop;
    }

    fn flood_fill_map(&mut self) {
        for x in 0..self.width {
            self.flood_fill((x, 0));
            self.flood_fill((x, self.heigth - 1));
        }
        for y in 0..self.heigth {
            self.flood_fill((0, y));
            self.flood_fill((self.width - 1, y));
        }
    }

    fn flood_fill(&mut self, start: Position) {
        if self.map[start.1][start.0] != Tile::Closed && self.map[start.1][start.0] != Tile::Path {
            return;
        }

        self.map[start.1][start.0] = Tile::Open;

        let neighbors = self.get_neighbors(start);
        for neighbor in neighbors {
            self.flood_fill(neighbor);
        }
    }

    fn get_neighbors(&self, current: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        if current.0 > 0 {
            neighbors.push((current.0 - 1, current.1));
        }
        if current.0 < self.width - 1 {
            neighbors.push((current.0 + 1, current.1));
        }
        if current.1 > 0 {
            neighbors.push((current.0, current.1 - 1));
        }
        if current.1 < self.heigth - 1 {
            neighbors.push((current.0, current.1 + 1));
        }
        neighbors
    }

    fn get_closed_tiles(&self) -> usize {
        let mut sum = 0;
        for row in self.map.iter() {
            for tile in row {
                if *tile == Tile::Closed {
                    sum += 1;
                }
            }
        }
        sum
    }
}

impl fmt::Display for FloodMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.map.iter() {
            for tail in row {
                let c = match tail {
                    Tile::Closed => 'I',
                    Tile::Open => 'O',
                    Tile::Loop => '=',
                    Tile::Path => ' ',
                };
                write!(f, "{c}")?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'a> Maze<'a> {
    fn new(data: &'a [u8]) -> Self {
        let width = data.iter().position(|&x| x == Literals::LF).unwrap();
        let heigth = data.len() / (width + 1);

        Self {
            data,
            width,
            heigth,
        }
    }

    fn get_length(&self) -> usize {
        let start = self.find_starting_position();

        if DEVELOP {
            // find all fields next to start
            let neighbors = self.get_neighbors(start);

            // calculate the route from all fields next to start to start again
            neighbors
                .iter()
                .filter_map(|&node| self.get_path_length(start, node, start))
                .max()
                .unwrap()
        } else {
            let next = (start.0, start.1 + 1);
            self.get_path_length(start, next, start).unwrap()
        }
    }

    fn get_enclosed_tiles(&self) -> usize {
        // create open/ enclosed map
        let mut map = FloodMap::new(self.width, self.heigth);
        //println!("Created: \n{map}");

        // creat path and mark tiles
        let start = self.find_starting_position();
        let next = (start.0, start.1 + 1);
        self.mark_path(&mut map, start, next, start);
        //println!("Mark Path:\n{map}");

        // flood fill from edge of grid
        map.flood_fill_map();
        //println!("Flood fill:\n{map}");

        // search all enclosed tiles
        map.get_closed_tiles()
    }

    fn get_neighbors(&self, current: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        if current.0 > 0 {
            neighbors.push((current.0 - 1, current.1));
        }
        if current.0 < self.width - 1 {
            neighbors.push((current.0 + 1, current.1));
        }
        if current.1 > 0 {
            neighbors.push((current.0, current.1 - 1));
        }
        if current.1 < self.heigth - 1 {
            neighbors.push((current.0, current.1 + 1));
        }
        neighbors
    }

    fn find_starting_position(&self) -> Position {
        let index = self
            .data
            .iter()
            .position(|&x| x == Literals::START)
            .unwrap();

        let x = index % (self.width + 1);
        let y = index / (self.width + 1);
        (x, y)
    }

    fn get_next_field(&self, previous: Position, current: Position) -> Option<Position> {
        let pipe = self.data[current.0 + current.1 * (self.width + 1)];

        match pipe {
            Literals::DOT => None,
            Literals::NORTH_TO_SOUTH => Some(if previous.1 < current.1 {
                (current.0, current.1 + 1)
            } else {
                (current.0, current.1 - 1)
            }),
            Literals::WEST_TO_EAST => Some(if previous.0 < current.0 {
                (current.0 + 1, current.1)
            } else {
                (current.0 - 1, current.1)
            }),
            Literals::NORTH_TO_EAST => Some(if previous.0 == current.0 {
                (current.0 + 1, current.1)
            } else {
                (current.0, current.1 - 1)
            }),
            Literals::NORTH_TO_WEST => Some(if previous.0 == current.0 {
                (current.0 - 1, current.1)
            } else {
                (current.0, current.1 - 1)
            }),
            Literals::WEST_TO_SOUTH => Some(if previous.0 == current.0 {
                (current.0 - 1, current.1)
            } else {
                (current.0, current.1 + 1)
            }),
            Literals::EAST_TO_SOUTH => Some(if previous.0 == current.0 {
                (current.0 + 1, current.1)
            } else {
                (current.0, current.1 + 1)
            }),
            _ => panic!("Invalid character"),
        }
    }

    fn mark_path(
        &self,
        map: &mut FloodMap,
        mut previous: Position,
        mut current: Position,
        end: Position,
    ) {
        map.add_loop(previous, current);
        while current != end {
            match self.get_next_field(previous, current) {
                Some(next) => {
                    previous = current;
                    current = next;
                    map.add_loop(previous, current);
                }
                None => panic!("Invalid path"),
            }
        }
    }

    fn get_path_length(
        &self,
        mut previous: Position,
        mut current: Position,
        end: Position,
    ) -> Option<usize> {
        for i in 0.. {
            if current == end {
                return Some(i);
            }
            match self.get_next_field(previous, current) {
                Some(next) => {
                    previous = current;
                    current = next;
                }
                None => {
                    return None;
                }
            }
        }
        unreachable!()
    }
}

fn main() -> Result<()> {
    let filename = if DEVELOP {
        "input_small4.txt"
    } else {
        "input.txt"
    };

    let contents = fs::read_to_string(filename)?;
    let maze = Maze::new(contents.as_bytes());

    println!("Task 1: {}", (maze.get_length() + 1) / 2);
    println!("Task 2: {}", maze.get_enclosed_tiles());

    Ok(())
}
