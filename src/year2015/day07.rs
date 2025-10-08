//! Some Assembly Required
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use std::collections::HashMap;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u16 {
    solve(input, false)
}

pub fn part2(input: &str) -> u16 {
    solve(input, true)
}
#[derive(Debug)]
struct Gate {
    op: String,
    in1: String,
    in2: String,
}

impl From<&str> for Gate {
    fn from(s: &str) -> Gate {
        let vec: Vec<&str> = s.split(' ').collect();

        if vec.len() == 1 {
            Gate { op: "NONE".to_string(), in1: vec[0].to_string(), in2: "NONE".to_string() }
        } else if vec[0] == "NOT" {
            Gate { op: vec[0].to_string(), in1: vec[1].to_string(), in2: "NONE".to_string() }
        } else {
            Gate { op: vec[1].to_string(), in1: vec[0].to_string(), in2: vec[2].to_string() }
        }
    }
}

struct Solve {
    gates: HashMap<String, Gate>,
    found: HashMap<String, u16>,
}

impl Solve {
    fn new() -> Self {
        Self { gates: HashMap::new(), found: HashMap::new() }
    }

    fn get_value(&mut self, output: String) -> u16 {
        if self.found.contains_key(&output) {
            return *self.found.get(&output).unwrap();
        }

        if output.chars().all(|c| c.is_ascii_digit()) {
            return output.parse::<u16>().unwrap();
        }

        let v1 = self.get_value(self.gates.get(&output).unwrap().in1.to_string());
        let result = match self.gates.get(&output).unwrap().op.as_str() {
            "NONE" => v1,
            "NOT" => !v1,
            _ => {
                let v2 = self.get_value(self.gates.get(&output).unwrap().in2.to_string());
                match self.gates.get(&output).unwrap().op.as_str() {
                    "AND" => v1 & v2,
                    "OR" => v1 | v2,
                    "LSHIFT" => v1 << v2,
                    "RSHIFT" => v1 >> v2,
                    _ => panic!("Not implemented."),
                }
            }
        };

        self.found.insert(output.to_string(), result);
        result
    }
}

fn solve(input: &str, part2: bool) -> u16 {
    let mut s = Solve::new();

    for line in input.lines() {
        let (gate_string, output) = line.split_once(" -> ").unwrap();
        s.gates.insert(output.to_string(), gate_string.into());
    }

    let mut result = s.get_value("a".to_string());

    if part2 {
        s.found = HashMap::new();
        s.gates.entry("b".to_string()).and_modify(|g| {
            g.in1 = result.to_string();
            g.in2 = "NONE".to_string();
            g.op = "NONE".to_string()
        });

        result = s.get_value("a".to_string());
    }
    result
}
