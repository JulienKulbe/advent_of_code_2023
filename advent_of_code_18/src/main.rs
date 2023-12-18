use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = true;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(c: &str) -> Self {
        match c {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("invalid direction"),
        }
    }
}

impl From<u32> for Direction {
    fn from(c: u32) -> Self {
        match c {
            0 => Direction::Right,
            2 => Direction::Left,
            3 => Direction::Up,
            1 => Direction::Down,
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self::new(0, 0)
    }

    fn go_to(&self, direction: Direction, width: i64) -> Self {
        match direction {
            Direction::Up => Self::new(self.x, self.y - width),
            Direction::Down => Self::new(self.x, self.y + width),
            Direction::Left => Self::new(self.x - width, self.y),
            Direction::Right => Self::new(self.x + width, self.y),
        }
    }
}

struct Line {
    start: i64,
    end: i64,
}

impl Line {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
}

struct Lines(HashMap<i64, Vec<Line>>);

impl Lines {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn push(&mut self, id: i64, mut start: i64, mut end: i64) {
        (start, end) = (min(start, end), max(start, end));

        let line = Line::new(start, end);
        if let Some(lines) = self.0.get_mut(&id) {
            lines.push(line);
        } else {
            self.0.insert(id, vec![line]);
        }
    }
}

struct Map {
    horizontal: Lines,
    vertical: Lines,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl Map {
    fn new_task1(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut horizontal = Lines::new();
        let mut vertical = Lines::new();
        let mut x_min = 0;
        let mut x_max = 0;
        let mut y_min = 0;
        let mut y_max = 0;
        let mut current = Position::zero();

        for line in reader.lines().flatten() {
            let (direction, count) = Self::parse_line(line.as_str());
            let next = current.go_to(direction, count);

            match direction {
                Direction::Left | Direction::Right => horizontal.push(current.y, current.x, next.x),
                Direction::Down | Direction::Up => vertical.push(current.x, current.y, next.y),
            }

            match direction {
                Direction::Left => x_min = x_min.min(next.x),
                Direction::Right => x_max = x_max.max(next.x),
                Direction::Down => y_max = y_max.max(next.y),
                Direction::Up => y_min = y_min.min(next.y),
            }

            current = next;
        }

        Self {
            horizontal,
            vertical,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    fn new_task2(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut horizontal = Lines::new();
        let mut vertical = Lines::new();
        let mut x_min = 0;
        let mut x_max = 0;
        let mut y_min = 0;
        let mut y_max = 0;
        let mut current = Position::zero();

        for line in reader.lines().flatten() {
            let (direction, count) = Self::parse_color(line.as_str());
            let next = current.go_to(direction, count);

            match direction {
                Direction::Left | Direction::Right => horizontal.push(current.y, current.x, next.x),
                Direction::Down | Direction::Up => vertical.push(current.x, current.y, next.y),
            }

            match direction {
                Direction::Left => x_min = x_min.min(next.x),
                Direction::Right => x_max = x_max.max(next.x),
                Direction::Down => y_max = y_max.max(next.y),
                Direction::Up => y_min = y_min.min(next.y),
            }

            current = next;
        }

        Self {
            horizontal,
            vertical,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    fn parse_line(line: &str) -> (Direction, i64) {
        let mut items = line.split_whitespace();
        let direction = Direction::from(items.next().unwrap());
        let count = items.next().unwrap().parse::<i64>().unwrap();
        (direction, count)
    }

    fn parse_color(line: &str) -> (Direction, i64) {
        let start = line.find('#').unwrap();
        let count = i64::from_str_radix(&line[start + 1..start + 6], 16).unwrap();
        let direction = Direction::from(line[start + 6..start + 7].parse::<u32>().unwrap());
        (direction, count)
    }

    fn contains(&self, position: Position) -> bool {
        if let Some(horizontal) = self.horizontal.0.get(&position.y) {
            if horizontal
                .iter()
                .any(|line| line.start <= position.x && position.x <= line.end)
            {
                return true;
            }
        }

        if let Some(vertical) = self.vertical.0.get(&position.x) {
            if vertical
                .iter()
                .any(|line| line.start <= position.y && position.y <= line.end)
            {
                return true;
            }
        }

        false
    }

    fn is_valid(&self, position: Position) -> bool {
        self.x_min <= position.x
            && position.x <= self.x_max
            && self.y_min <= position.y
            && position.y <= self.y_max
    }
}

struct DigPlan {
    map: Map,
}

impl DigPlan {
    fn new(map: Map) -> Self {
        Self { map }
    }

    fn area(&self) -> usize {
        let mut flood_map = HashSet::new();
        for x in self.map.x_min..self.map.x_max {
            self.flood(Position::new(x, self.map.y_max), &mut flood_map);
            self.flood(Position::new(x, self.map.y_min), &mut flood_map);
        }
        for y in self.map.y_min..self.map.y_max {
            self.flood(Position::new(self.map.x_min, y), &mut flood_map);
            self.flood(Position::new(self.map.x_max, y), &mut flood_map);
        }

        let full_area =
            (self.map.x_max - self.map.x_min + 1) * (self.map.y_max - self.map.y_min + 1);
        (full_area as usize) - flood_map.len()
    }

    fn flood(&self, current: Position, flood_map: &mut HashSet<Position>) {
        // check if:
        //  - position is not valid (outside the map) or
        //  - position if already flooded or
        //  - reached a wall
        if flood_map.contains(&current) || !self.map.is_valid(current) || self.map.contains(current)
        {
            return;
        }

        flood_map.insert(current);

        self.flood(current.go_to(Direction::Right, 1), flood_map);
        self.flood(current.go_to(Direction::Left, 1), flood_map);
        self.flood(current.go_to(Direction::Up, 1), flood_map);
        self.flood(current.go_to(Direction::Down, 1), flood_map);
    }
}

fn main() {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };

    {
        let map = Map::new_task1(filename);
        let plan = DigPlan::new(map);
        println!("Task 1: {}", plan.area());
    }
    {
        let map = Map::new_task2(filename);
        let plan = DigPlan::new(map);
        println!("Task 2: {}", plan.area());
    }
}
