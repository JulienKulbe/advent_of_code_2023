use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

struct Parser;

impl Parser {
    fn parse_file(filename: &str) -> Map {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        Map(reader
            .lines()
            .flatten()
            .map(|line| line.chars().map(Field::from).collect())
            .collect())
    }
}

struct Map(Vec<Vec<Field>>);

impl Map {
    fn count_reachable_tiles(mut self, steps: usize) -> usize {
        let start = self.find_starting_pos();
        self.go_to(start, 0, steps);

        // for y in 0..self.heigth() {
        //     for x in 0..self.width() {
        //         print!("{:?}", self.get(Position::new(x, y)));
        //     }
        //     println!();
        // }

        self.find_even_fields()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn heigth(&self) -> usize {
        self.0.len()
    }

    fn get(&self, pos: Position) -> Field {
        self.0[pos.y][pos.x]
    }

    fn set(&mut self, pos: Position, field: Field) {
        self.0[pos.y][pos.x] = field;
    }

    fn find_starting_pos(&self) -> Position {
        for y in 0..self.heigth() {
            for x in 0..self.width() {
                if self.get(Position::new(x, y)) == Field::Start {
                    return Position::new(x, y);
                }
            }
        }
        panic!("startint pos not found");
    }

    fn find_even_fields(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|&&f| matches!(f, Field::Even(_))).count())
            .sum()
    }

    fn go_to(&mut self, to: Position, step: usize, max_steps: usize) {
        if !self.get(to).can_visit(step) || step > max_steps {
            return;
        }

        self.set(to, Field::from(step));

        for neighbor in to.neighbors(self.width(), self.heigth()) {
            self.go_to(neighbor, step + 1, max_steps);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    fn neighbors(&self, width: usize, heigth: usize) -> Vec<Position> {
        let mut neighbors = Vec::new();

        if self.x > 0 {
            neighbors.push(Self::new(self.x - 1, self.y));
        }
        if self.y > 0 {
            neighbors.push(Self::new(self.x, self.y - 1));
        }
        if self.x < width - 1 {
            neighbors.push(Self::new(self.x + 1, self.y));
        }
        if self.y < heigth - 1 {
            neighbors.push(Self::new(self.x, self.y + 1));
        }

        neighbors
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    Garden,
    Rock,
    Start,
    Odd(usize),
    Even(usize),
}

impl Field {
    fn can_visit(&self, steps: usize) -> bool {
        match *self {
            Field::Even(even) => steps < even,
            Field::Odd(odd) => steps < odd,
            Field::Garden | Field::Start => true,
            Field::Rock => false,
        }
    }
}

impl From<char> for Field {
    fn from(c: char) -> Self {
        match c {
            '.' => Field::Garden,
            '#' => Field::Rock,
            'S' => Field::Start,
            _ => panic!("invalid character"),
        }
    }
}

impl From<usize> for Field {
    fn from(step: usize) -> Self {
        if step & 1 == 0 {
            Field::Even(step)
        } else {
            Field::Odd(step)
        }
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Garden => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Start => write!(f, "S"),
            Self::Odd(_) => write!(f, "-"),
            Self::Even(_) => write!(f, "O"),
        }
    }
}

fn main() {
    let (filename, steps) = if DEVELOP {
        ("input_small.txt", 6)
    } else {
        ("input.txt", 64)
    };

    let map = Parser::parse_file(filename);
    println!("Task 1: {}", map.count_reachable_tiles(steps))
}
