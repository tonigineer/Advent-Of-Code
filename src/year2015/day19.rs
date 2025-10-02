//! Medicine for Rudolph
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use std::collections::{HashMap, HashSet};

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> usize {
    solve(input)
}

pub fn part2(input: &str) -> usize {
    solve2(input)
}

fn solve(input: &str) -> usize {
    let (replacements, molecule) = input.split_once("\n\n").unwrap();
    let mut distinct_molecules: HashSet<String> = HashSet::new();

    for line in replacements.lines() {
        let (pattern, substitute) = line.split_once(" => ").unwrap();

        for i in 0..=molecule.len() - pattern.len() {
            if molecule[i..i + pattern.len()].to_string() == pattern {
                distinct_molecules.insert(format!(
                    "{}{}{}",
                    molecule[0..i].to_string(),
                    substitute,
                    molecule[i + pattern.len()..molecule.len()].to_string()
                ));
            }
        }
    }
    return distinct_molecules.len();
}

fn solve2(input: &str) -> usize {
    let (replacements, molecule) = input.split_once("\n\n").unwrap();
    let mut rm: HashMap<&str, &str> = HashMap::new();

    for line in replacements.lines() {
        let (pattern, substitute) = line.split_once(" => ").unwrap();
        rm.insert(substitute, pattern);
    }

    let mut sorted_keys: Vec<&&str> = rm.keys().into_iter().collect::<Vec<_>>();
    sorted_keys.sort_by(|a, b| b.len().cmp(&a.len()));

    let mut current_molecule: String = molecule.to_string();
    let mut count = 0;
    loop {
        for substitute in sorted_keys.iter_mut() {
            if current_molecule.contains(*substitute) {
                current_molecule =
                    current_molecule.replacen(*substitute, rm.get(*substitute).unwrap(), 1);
                count += 1;
                break;
            }
        }

        if current_molecule == "e" {
            break;
        }
    }
    return count;
}
