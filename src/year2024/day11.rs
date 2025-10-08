//! Plutonian Pebbles
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use cached::proc_macro::cached;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u64 {
    input
        .trim()
        .split(" ")
        .map(|stone| solve(stone.parse::<u64>().unwrap(), 25))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .trim()
        .split(" ")
        .map(|stone| solve(stone.parse::<u64>().unwrap(), 75))
        .sum()
}

#[cached]
fn solve(stone: u64, steps: u8) -> u64 {
    if steps == 0 {
        return 1;
    }

    if stone == 0 {
        return solve(1, steps - 1);
    }

    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let (left, right) = stone_str.split_at(stone_str.len() / 2);

        return solve(left.parse::<u64>().unwrap(), steps - 1)
            + solve(right.parse::<u64>().unwrap(), steps - 1);
    }

    solve(stone * 2024, steps - 1)
}
