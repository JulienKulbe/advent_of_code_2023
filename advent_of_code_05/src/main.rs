use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DEVELOP: bool = false;

#[derive(Debug)]
struct MappingEntry {
    dst: u64,
    src: u64,
    range: u64,
}

impl MappingEntry {
    fn new(line: String) -> Self {
        let values = line
            .split_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        MappingEntry {
            dst: values[0],
            src: values[1],
            range: values[2],
        }
    }
}

#[derive(Debug, Default)]
struct Map {
    entries: Vec<MappingEntry>,
}

impl Map {
    fn map(&self, value: u64) -> u64 {
        for entry in self.entries.iter() {
            if entry.src <= value && value < entry.src + entry.range {
                return value - entry.src + entry.dst;
            }
        }
        value
    }

    fn map_range(&self, seed: &SeedRange) -> Vec<SeedRange> {
        let mut seeds = vec![*seed; 1];
        for entry in self.entries.iter() {
            let mut next = Vec::new();
            for seed in seeds {
                if seed.start < entry.src && seed.end > entry.src + entry.range {
                    next.push(SeedRange {
                        start: seed.start,
                        end: entry.src - 1,
                    });
                    next.push(SeedRange {
                        start: entry.src,
                        end: entry.src + entry.range - 1,
                    });
                    next.push(SeedRange {
                        start: entry.src + entry.range,
                        end: seed.end,
                    });
                } else if seed.start < entry.src && seed.end > entry.src {
                    next.push(SeedRange {
                        start: seed.start,
                        end: entry.src - 1,
                    });
                    next.push(SeedRange {
                        start: entry.src,
                        end: seed.end,
                    });
                } else if seed.start > entry.src
                    && seed.start < entry.src + entry.range
                    && seed.end > entry.src + entry.range
                {
                    next.push(SeedRange {
                        start: seed.start,
                        end: entry.src + entry.range - 1,
                    });
                    next.push(SeedRange {
                        start: entry.src + entry.range,
                        end: seed.end,
                    });
                } else {
                    next.push(seed);
                }
            }

            seeds = next;
        }

        let mut mapped_seeds = Vec::new();
        for seed in &seeds {
            let mut mapped_seed = *seed;
            for entry in self.entries.iter() {
                if entry.src <= seed.start && seed.end < entry.src + entry.range {
                    mapped_seed = SeedRange {
                        start: seed.start - entry.src + entry.dst,
                        end: seed.end - entry.src + entry.dst,
                    };
                }
            }

            mapped_seeds.push(mapped_seed);
        }

        mapped_seeds
    }
}

#[derive(Debug, Clone, Copy)]
struct SeedRange {
    start: u64,
    end: u64,
}

fn task1(reader: BufReader<File>) -> u64 {
    let mut seeds = Vec::new();
    let mut maps = Vec::new();
    for line in reader.lines().flatten() {
        if line.is_empty() {
            continue;
        }

        if let Some((_, values)) = line.split_once(':') {
            if values.is_empty() {
                maps.push(Map {
                    ..Default::default()
                });
            } else {
                seeds = values
                    .split_whitespace()
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();
            }
        } else {
            let current_map = maps.last_mut().unwrap();
            current_map.entries.push(MappingEntry::new(line));
        }
    }

    seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |s, m| m.map(s)))
        .min()
        .unwrap()
}

fn task2(reader: BufReader<File>) -> u64 {
    let mut seeds = Vec::new();
    let mut maps = Vec::new();
    for line in reader.lines().flatten() {
        if line.is_empty() {
            continue;
        }

        if let Some((_, values)) = line.split_once(':') {
            if values.is_empty() {
                maps.push(Map {
                    ..Default::default()
                });
            } else {
                let seed_range = values
                    .split_whitespace()
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
                for i in (0..seed_range.len()).step_by(2) {
                    seeds.push(SeedRange {
                        start: seed_range[i],
                        end: seed_range[i] + seed_range[i + 1] - 1,
                    });
                }
            }
        } else {
            let current_map = maps.last_mut().unwrap();
            current_map.entries.push(MappingEntry::new(line));
        }
    }

    let mut min = u64::MAX;
    for seed in &seeds {
        let mut mapped_seeds = vec![*seed; 1];

        for map in &maps {
            let mut next = Vec::new();
            for mapped_seed in &mapped_seeds {
                let mut mapped = map.map_range(mapped_seed);
                next.append(&mut mapped);
            }
            mapped_seeds = next;
        }

        let mapped_min = mapped_seeds.iter().map(|r| r.start).min().unwrap();
        min = min.min(mapped_min);
    }

    min
}

fn main() -> Result<()> {
    let filename = if DEVELOP {
        "input_small.txt"
    } else {
        "input.txt"
    };
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    //println!("Task 1: {}", task1(reader));
    println!("Task 2: {}", task2(reader));

    Ok(())
}
