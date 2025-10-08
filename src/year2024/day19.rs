//! Linen Layout
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use std::collections::HashMap;

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> usize {
    solve(input, false)
}

pub fn part2(input: &str) -> usize {
    solve(input, true)
}

fn solve(input: &str, part2: bool) -> usize {
    let (item_list, design_list) = input.split_once("\n\n").unwrap();

    let items = item_list.split(", ").collect::<Vec<_>>();
    let designs = design_list.trim().split("\n").collect::<Vec<_>>();

    let mut seen: HashMap<String, usize> = HashMap::new();

    let (ans1, ans2) = designs.iter().fold((0, 0), |acc, design| {
        let sol = check(design.to_string(), &items, &mut seen);
        (acc.0 + (sol > 0) as usize, acc.1 + sol)
    });

    if part2 {
        return ans2;
    }

    ans1
}

fn check(receipt: String, items: &Vec<&str>, seen: &mut HashMap<String, usize>) -> usize {
    if receipt.is_empty() {
        return 1;
    };

    if seen.contains_key(&receipt) {
        return *seen.get(&receipt).unwrap();
    }

    let solutions = items
        .iter()
        .filter(|v| receipt.starts_with(**v))
        .map(|v| check(receipt.clone().split_off(v.len()), items, seen))
        .sum();

    seen.insert(receipt, solutions);

    solutions
}
