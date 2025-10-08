//! Let It Snow
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u64 {
    solve(input)
}

pub fn part2(_input: &str) -> &str {
    "🌲☃️🎅"
}

use regex::Regex;

fn solve(input: &str) -> u64 {
    let re = Regex::new(r"row (\d+), column (\d+)").unwrap();
    let captures = re.captures(input).unwrap();
    let (row, column): (u64, u64) = (captures[1].parse().unwrap(), captures[2].parse().unwrap());

    // Determine min size length (base) of triangle
    // Calculate number of nodes (column of first row)
    // traverse column back till row found for wanted number
    let base = row + column - 1;
    let num_full_triangle = base * (base + 1) / 2;
    let number = num_full_triangle - row + 1;

    let mut code: u64 = 20151125;
    let multiplier: u64 = 252533;
    let divider: u64 = 33554393;

    for _ in 2..=number {
        code = code * multiplier % divider;
    }

    code
}
