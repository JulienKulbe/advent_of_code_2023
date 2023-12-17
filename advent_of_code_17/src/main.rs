use anyhow::Result;
use priority_queue::DoublePriorityQueue;
use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = true;

struct Map(Vec<Vec<u8>>);

impl Map {
    fn new(file: BufReader<File>) -> Self {
        let data = file
            .lines()
            .flatten()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        Self(data)
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn heigth(&self) -> usize {
        self.0.len()
    }

    fn get(&self, pos: Position) -> u8 {
        self.0[pos.1][pos.0]
    }
}

type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Eq)]
struct Node {
    position: Position,
    predecessor: Position,
    directions: Vec<Direction>,
}

impl Node {
    fn start() -> Self {
        Self {
            position: (0, 0),
            predecessor: (0, 0),
            directions: Vec::new(),
        }
    }

    fn new(position: Position, predecessor: &Node, direction: Direction) -> Self {
        // copy the directions from the predecessor and ann the current diretion
        let mut directions = predecessor.directions.clone();
        if directions.len() == 3 {
            directions.remove(0);
        }
        directions.push(direction);

        Self {
            position,
            predecessor: predecessor.position,
            directions,
        }
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

struct SearchAStar {
    map: Map,
    open_list: DoublePriorityQueue<Node, u64>,
    closed_list: HashSet<Node>,
}

impl SearchAStar {
    fn new(map: Map) -> Self {
        Self {
            map,
            open_list: DoublePriorityQueue::new(),
            closed_list: HashSet::new(),
        }
    }

    fn search_path(mut self) -> u64 {
        let destination = (self.map.width() - 1, self.map.heigth() - 1);

        // add starting node at positon 0,0
        self.open_list.push(Node::start(), 0);

        while let Some((current_node, score)) = self.open_list.pop_min() {
            if current_node.position == destination {
                return score;
            }

            self.expand_node(&current_node, score);
            self.closed_list.insert(current_node);
        }
        panic!("no path found");
    }

    fn expand_node(&mut self, current: &Node, current_score: u64) {
        for successor in self.get_successors(current) {
            if self.closed_list.contains(&successor) {
                continue;
            }

            let tentative_g = current_score + self.map.get(successor.position) as u64;

            if let Some(successor_score) = self.open_list.get_priority(&successor) {
                if tentative_g >= *successor_score {
                    continue;
                }
                self.open_list.remove(&successor);
            }

            let score = tentative_g + self.get_heuristic_score(&successor);
            self.open_list.push(successor, score);
        }
    }

    fn get_successors(&self, current: &Node) -> Vec<Node> {
        let mut successors = Vec::new();

        if current.position.0 > 0 && self.check_directions(current, Direction::Left) {
            successors.push(Node::new(
                (current.position.0 - 1, current.position.1),
                current,
                Direction::Left,
            ));
        }
        if current.position.0 < self.map.width() - 1
            && self.check_directions(current, Direction::Right)
        {
            successors.push(Node::new(
                (current.position.0 + 1, current.position.1),
                current,
                Direction::Right,
            ));
        }
        if current.position.1 > 0 && self.check_directions(current, Direction::Up) {
            successors.push(Node::new(
                (current.position.0, current.position.1 - 1),
                current,
                Direction::Up,
            ));
        }
        if current.position.1 < self.map.heigth() - 1
            && self.check_directions(current, Direction::Down)
        {
            successors.push(Node::new(
                (current.position.0, current.position.1 + 1),
                current,
                Direction::Down,
            ));
        }

        successors
    }

    fn check_directions(&self, current: &Node, direction: Direction) -> bool {
        current.directions != vec![direction, direction, direction]
    }

    fn get_heuristic_score(&self, current: &Node) -> u64 {
        (self.map.width() + self.map.heigth() - (current.position.0 + current.position.1 + 2))
            as u64
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

    {
        let map = Map::new(reader);
        let search = SearchAStar::new(map);
        println!("Task 1: {}", search.search_path())
    }

    Ok(())
}
