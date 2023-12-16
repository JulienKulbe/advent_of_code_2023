use bitflags::bitflags;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Empty,
    MirrorSlash,
    MirrorBackslash,
    MirrorHorizontal,
    MirrorVertical,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            '.' => Field::Empty,
            '/' => Field::MirrorSlash,
            '\\' => Field::MirrorBackslash,
            '-' => Field::MirrorHorizontal,
            '|' => Field::MirrorVertical,
            _ => panic!("Invalid character"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Down,
    Up,
    Right,
    Left,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VisitedDirection: u8 {
        const None =  0b00000000;
        const Down =  0b00000001;
        const Up =    0b00000010;
        const Right = 0b00000100;
        const Left =  0b00001000;
    }
}

impl From<Direction> for VisitedDirection {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Down => VisitedDirection::Down,
            Direction::Up => VisitedDirection::Up,
            Direction::Left => VisitedDirection::Left,
            Direction::Right => VisitedDirection::Right,
        }
    }
}

struct Map(Vec<Vec<Field>>);

impl Map {
    fn new(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let data = reader
            .lines()
            .flatten()
            .map(|line| line.chars().map(Field::from).collect())
            .collect();
        Self(data)
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn heigth(&self) -> usize {
        self.0.len()
    }

    fn get(&self, beam: &Beam) -> Field {
        self.0[beam.position.1][beam.position.0]
    }

    fn next(&self, current: &Beam, direction: Direction) -> Option<Beam> {
        match direction {
            Direction::Right => {
                if current.position.0 < self.width() - 1 {
                    Some(Beam::new(
                        current.position.0 + 1,
                        current.position.1,
                        direction,
                    ))
                } else {
                    None
                }
            }
            Direction::Left => {
                if current.position.0 > 0 {
                    Some(Beam::new(
                        current.position.0 - 1,
                        current.position.1,
                        direction,
                    ))
                } else {
                    None
                }
            }
            Direction::Down => {
                if current.position.1 < self.heigth() - 1 {
                    Some(Beam::new(
                        current.position.0,
                        current.position.1 + 1,
                        direction,
                    ))
                } else {
                    None
                }
            }
            Direction::Up => {
                if current.position.1 > 0 {
                    Some(Beam::new(
                        current.position.0,
                        current.position.1 - 1,
                        direction,
                    ))
                } else {
                    None
                }
            }
        }
    }
}

struct Visited(Vec<Vec<VisitedDirection>>);

impl Visited {
    fn new(width: usize, heigth: usize) -> Self {
        let data = (0..heigth)
            .map(|_| vec![VisitedDirection::None; width])
            .collect();
        Self(data)
    }

    fn has_visited(&self, beam: &Beam) -> bool {
        self.0[beam.position.1][beam.position.0] == VisitedDirection::from(beam.direction)
    }

    fn visit(&mut self, beam: &Beam) {
        self.0[beam.position.1][beam.position.0].insert(VisitedDirection::from(beam.direction));
    }

    fn count(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|d| !d.is_empty()).count())
            .sum()
    }
}

#[derive(Debug, Clone, Copy)]
struct Beam {
    position: (usize, usize),
    direction: Direction,
}

impl Beam {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self {
            position: (x, y),
            direction,
        }
    }
}

fn main() {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };

    // read file content into a map
    let map = Map::new(filename);

    // creat a visited map (enum flags)
    let mut visited = Visited::new(map.width(), map.heigth());

    // visit map with light beam
    visit_field(&map, Beam::new(0, 0, Direction::Right), &mut visited);

    // count visited fields
    println!("Task 1: {}", visited.count());
}

fn visit_field(map: &Map, current: Beam, visited: &mut Visited) {
    if visited.has_visited(&current) {
        return;
    }

    // println!(
    //     "add visited: {:?} {:?}",
    //     current.position, current.direction
    // );

    visited.visit(&current);

    match map.get(&current) {
        Field::Empty => {
            go_next(map, current, visited);
        }
        Field::MirrorHorizontal => match current.direction {
            Direction::Right | Direction::Left => go_next(map, current, visited),
            Direction::Down | Direction::Up => {
                go_to(map, current, Direction::Left, visited);
                go_to(map, current, Direction::Right, visited);
            }
        },
        Field::MirrorVertical => match current.direction {
            Direction::Down | Direction::Up => go_next(map, current, visited),
            Direction::Left | Direction::Right => {
                go_to(map, current, Direction::Up, visited);
                go_to(map, current, Direction::Down, visited);
            }
        },
        Field::MirrorSlash => match current.direction {
            Direction::Down => go_to(map, current, Direction::Left, visited),
            Direction::Up => go_to(map, current, Direction::Right, visited),
            Direction::Right => go_to(map, current, Direction::Up, visited),
            Direction::Left => go_to(map, current, Direction::Down, visited),
        },
        Field::MirrorBackslash => match current.direction {
            Direction::Down => go_to(map, current, Direction::Right, visited),
            Direction::Up => go_to(map, current, Direction::Left, visited),
            Direction::Right => go_to(map, current, Direction::Down, visited),
            Direction::Left => go_to(map, current, Direction::Up, visited),
        },
    }
}

fn go_next(map: &Map, current: Beam, visited: &mut Visited) {
    if let Some(next) = map.next(&current, current.direction) {
        visit_field(map, next, visited);
    }
}

fn go_to(map: &Map, current: Beam, direction: Direction, visited: &mut Visited) {
    if let Some(next) = map.next(&current, direction) {
        visit_field(map, next, visited);
    }
}
