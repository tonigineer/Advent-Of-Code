//! Probably a Fire Hazard
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use lazy_static::lazy_static;
use regex::Regex;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u32 {
    solve(input, false)
}

pub fn part2(input: &str) -> u32 {
    solve(input, true)
}

struct Instruction {
    command: String,
    start: (usize, usize),
    end: (usize, usize),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Instruction {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?P<command>turn on|turn off|toggle) (?P<start>\d+,\d+) through (?P<end>\d+,\d+)"
            )
            .unwrap();
        };

        let capture = RE.captures(s).unwrap();

        let command = capture.name("command").unwrap().as_str();
        let start: Vec<usize> = capture
            .name("start")
            .unwrap()
            .as_str()
            .split(',')
            .map(|v| v.parse().expect("Could not parse"))
            .collect();
        let end: Vec<usize> = capture
            .name("end")
            .unwrap()
            .as_str()
            .split(',')
            .map(|v| v.parse().expect("Could not parse"))
            .collect();

        return Instruction {
            command: command.to_string(),
            start: (start[0], start[1]),
            end: (end[0], end[1]),
        };
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u64>>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            grid: vec![vec![0u64; 1000]; 1000],
        }
    }

    fn apply_instruction(&mut self, inst: Instruction, part2: bool) {
        for r in inst.start.1..=inst.end.1 {
            for c in inst.start.0..=inst.end.0 {
                if part2 {
                    match inst.command.as_str() {
                        "turn off" => {
                            if self.grid[r][c] > 0 {
                                self.grid[r][c] -= 1
                            }
                        }
                        "turn on" => self.grid[r][c] += 1,
                        "toggle" => self.grid[r][c] += 2,
                        _ => panic!("Command not implemented."),
                    }
                } else {
                    match inst.command.as_str() {
                        "turn off" => self.grid[r][c] = 0,
                        "turn on" => self.grid[r][c] = 1,
                        "toggle" => self.grid[r][c] = (self.grid[r][c] + 1) % 2,
                        _ => panic!("Command not implemented."),
                    }
                }
            }
        }
    }

    fn num_lights_on(&self) -> u64 {
        let mut lights: u64 = 0;
        for r in self.grid.iter() {
            lights = lights + r.iter().sum::<u64>();
        }
        return lights;
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    let mut g: Grid = Grid::new();

    for line in input.lines() {
        g.apply_instruction(line.into(), part2);
    }

    g.num_lights_on() as u32
}
