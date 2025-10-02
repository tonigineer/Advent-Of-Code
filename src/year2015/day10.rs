//! Elves Look, Elves Say
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use itertools::Itertools;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> usize {
    solve(input, 40)
}

pub fn part2(input: &str) -> usize {
    solve(input, 50)
}

fn solve(input: &str, repetitions: u8) -> usize {
    let mut process_string: String = String::from(input);

    for _ in 0..repetitions {
        let mut next_string: String = String::new();

        for (digit, group) in &process_string.chars().chunk_by(|c| *c) {
            let num: Vec<_> = group.collect();
            next_string.push_str(&num.len().to_string());
            next_string.push_str(&digit.to_string());
        }
        process_string = String::from(next_string);
    }
    return process_string.to_string().len();
}
