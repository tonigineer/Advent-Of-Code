//! It Hangs in the Balance
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
    solve(input, 3)
}

pub fn part2(input: &str) -> usize {
    solve(input, 4)
}

fn solve(input: &str, compartments: usize) -> usize {
    let packages: Vec<usize> = input.lines().map(|p| p.parse().unwrap()).collect();

    let total_weight: usize = packages.iter().sum::<usize>() / compartments;
    let max_packages_front = packages.len() / compartments;

    let mut entanglement = 0;
    for i in 1..=max_packages_front {
        let c = packages
            .iter()
            .copied()
            .combinations(i)
            .filter(|c| c.iter().sum::<usize>() == total_weight)
            .sorted_by_key(|c| c.iter().product::<usize>())
            .min();

        if let Some(c) = c {
            entanglement = c.iter().product::<usize>();
            break;
        }
    }

    entanglement
}
