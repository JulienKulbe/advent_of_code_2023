use anyhow::Result;
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
        // find starting position
        let start = self.find_starting_position();

        // find all fields next to start
        let neighbors = self.get_neighbors(start);

        // calculate the route from all fields next to start to start again
        neighbors
            .iter()
            .filter_map(|&node| self.get_path_length(start, node, start))
            .max()
            .unwrap()
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

    fn get_path_length(
        &self,
        mut previous: Position,
        mut current: Position,
        end: Position,
    ) -> Option<usize> {
        //println!("{previous:?} -> {current:?}");

        for i in 0.. {
            if current == end {
                return Some(i);
            }
            match self.get_next_field(previous, current) {
                Some(next) => {
                    previous = current;
                    current = next;
                    //println!("-> {next:?}");
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
        "input_small.txt"
    } else {
        "input.txt"
    };

    let contents = fs::read_to_string(filename)?;
    let maze = Maze::new(contents.as_bytes());

    println!("Task 1: {}", (maze.get_length() + 1) / 2);

    Ok(())
}
