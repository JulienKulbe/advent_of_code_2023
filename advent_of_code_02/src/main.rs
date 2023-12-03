use anyhow::Result;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

fn task1(reader: BufReader<File>) -> Result<u32> {
    let colors = HashMap::from([("red", 12_u32), ("blue", 14), ("green", 13)]);

    let mut sum = 0;
    for line in reader.lines().flatten() {
        let split_line = line.split(':').collect::<Vec<&str>>();

        // check if game is valid
        let mut is_valid = true;
        'game: for reveal in split_line[1].split(';') {
            for cube in reveal.split(',') {
                for color in colors.iter() {
                    if let Some(count) = cube.strip_suffix(color.0) {
                        let count = count.trim().parse::<u32>()?;
                        if count > *color.1 {
                            is_valid = false;
                            break 'game;
                        }
                    }
                }
            }
        }

        // get game id
        if is_valid {
            let game_id = split_line[0].strip_prefix("Game").unwrap();
            let game_id = game_id.trim().parse::<u32>()?;
            sum += game_id;
        }
    }

    Ok(sum)
}

struct Game {
    colors: [u32; 3],
}

impl Game {
    fn new() -> Self {
        Self { colors: [0, 0, 0] }
    }

    fn add(&mut self, entry: &str) {
        for (i, color) in ["red", "blue", "green"].iter().enumerate() {
            if let Some(count) = entry.strip_suffix(color) {
                let count = count.trim().parse::<u32>().unwrap();
                self.colors[i] = self.colors[i].max(count);
            }
        }
    }

    fn power(&self) -> u32 {
        self.colors.iter().product()
    }
}

fn task2(reader: BufReader<File>) -> u32 {
    let mut sum = 0;
    for line in reader.lines().flatten() {
        let mut game = Game::new();
        let split_line = line.split(':').collect::<Vec<&str>>();
        for reveal in split_line[1].split(';') {
            for cube in reveal.split(',') {
                game.add(cube);
            }
        }
        sum += game.power();
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

    //println!("Sum {}", task1(reader));
    println!("Sum {}", task2(reader));

    Ok(())
}
