//! Knights of the Dinner Table
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> i32 {
    solve(input, false)
}

pub fn part2(input: &str) -> i32 {
    solve(input, true)
}

fn solve(input: &str, part2: bool) -> i32 {
    let mut h: HashMap<(String, String), i32> = HashMap::new();
    let mut p: HashSet<String> = HashSet::new();

    for line in input.lines() {
        let modded_line = line.trim_end_matches('.').replace("gain ", "").replace("lose ", "-");
        let token: Vec<&str> = modded_line.split(" ").collect();

        let (p1, p2, happiness) =
            (token[0].to_string(), token[9].to_string(), token[2].parse().unwrap());
        h.insert((p1.clone(), p2), happiness);
        p.insert(p1);
    }

    if part2 {
        for p in p.iter() {
            h.insert((p.to_string(), "toni".to_string()), 0);
            h.insert(("toni".to_string(), p.to_string()), 0);
        }
        p.insert("toni".to_string());
    }

    let mut max_happiness = 0;
    for permutation in p.iter().permutations(p.len()) {
        let mut happiness = 0;
        for (p1, p2) in permutation.iter().circular_tuple_windows() {
            happiness += h.get(&(p1.to_string(), p2.to_string())).unwrap();
            happiness += h.get(&(p2.to_string(), p1.to_string())).unwrap();
        }
        if happiness > max_happiness {
            max_happiness = happiness
        }
    }

    return max_happiness;
}
