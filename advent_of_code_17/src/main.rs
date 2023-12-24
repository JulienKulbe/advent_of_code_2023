use anyhow::Result;
use priority_queue::DoublePriorityQueue;
use std::{
    collections::HashSet,
    fmt::Debug,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl Direction {}

#[derive(Eq)]
struct Node {
    position: Position,
    score: u64,
    directions: Vec<Direction>,
}

impl Node {
    fn start() -> Self {
        Self {
            position: (0, 0),
            score: 0,
            directions: Vec::new(),
        }
    }

    fn new(position: Position, predecessor: &Node, score: u64, direction: Direction) -> Self {
        // copy the directions from the predecessor and the current diretion
        let mut directions = predecessor.directions.clone();
        if directions.len() == 3 {
            directions.remove(0);
        }
        directions.push(direction);

        Self {
            position,
            score: predecessor.score + score,
            directions,
        }
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.directions.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        (self.position, &self.directions) == (other.position, &self.directions)
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
            .field("position", &self.position)
            .field("score", &self.score)
            .field("directions", &self.directions)
            .finish()
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

        while let Some((current_node, _)) = self.open_list.pop_min() {
            if current_node.position == destination {
                return current_node.score;
            }

            //println!("\t{current_node:?}");

            self.expand_node(&current_node);
            self.closed_list.insert(current_node);

            // println!("Closed list:");
            // for node in self.closed_list.iter() {
            //     println!("\t{node:?}: {}", node.score);
            // }
            // println!("Open list:");
            // for (node, score) in self.open_list.iter() {
            //     println!("\t{node:?}: {score}");
            // }
            //println!();
        }

        unreachable!()
        // self.closed_list
        //     .iter()
        //     .filter(|node| node.position == destination)
        //     .map(|node| node.score)
        //     .min()
        //     .unwrap()
    }

    fn expand_node(&mut self, current: &Node) {
        for successor in self.get_successors(current) {
            if self.closed_list.contains(&successor) {
                continue;
            }

            let tentative_g = successor.score;

            if let Some((node, _)) = self.open_list.get(&successor) {
                if tentative_g >= node.score {
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

        if current.position.0 > 0 && current.directions.last() != Some(&Direction::Right) {
            successors.push((
                (current.position.0 - 1, current.position.1),
                Direction::Left,
            ));
        }
        if current.position.0 < self.map.width() - 1
            && current.directions.last() != Some(&Direction::Left)
        {
            successors.push((
                (current.position.0 + 1, current.position.1),
                Direction::Right,
            ));
        }
        if current.position.1 > 0 && current.directions.last() != Some(&Direction::Down) {
            successors.push(((current.position.0, current.position.1 - 1), Direction::Up));
        }
        if current.position.1 < self.map.heigth() - 1
            && current.directions.last() != Some(&Direction::Up)
        {
            successors.push((
                (current.position.0, current.position.1 + 1),
                Direction::Down,
            ));
        }

        successors
            .iter()
            .filter(|(_, dir)| self.check_directions(current, *dir))
            .map(|(pos, dir)| Node::new(*pos, current, self.map.get(*pos) as u64, *dir))
            .collect()
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
