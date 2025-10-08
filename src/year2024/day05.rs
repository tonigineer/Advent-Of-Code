//! Print Queue
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use std::cmp::Ordering;
use std::collections::HashMap;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u32 {
    solve(input, false)
}

pub fn part2(input: &str) -> u32 {
    solve(input, true)
}

fn solve(input: &str, part2: bool) -> u32 {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    let mut rules: HashMap<(u32, u32), i32> = HashMap::new();

    let mut ans = 0;

    for rule in rules_str.lines() {
        let (x, y) = rule.split_once('|').unwrap();
        rules.insert((x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()), -1);
        rules.insert((y.parse::<u32>().unwrap(), x.parse::<u32>().unwrap()), 1);
    }

    for line in updates_str.lines() {
        let mut sequence: Vec<u32> = line
            .split(',')
            .map(|num| num.parse::<u32>().unwrap())
            .collect();

        let is_correct = has_correct_order(&sequence, &rules);

        if part2 && !is_correct {
            sequence.sort_by(|x, y| match rules.get(&(*x, *y)).unwrap_or(&0) {
                -1 => Ordering::Less,
                0 => Ordering::Equal,
                1 => Ordering::Greater,
                _ => panic!("What the fuck is going on! Press F for Dillon :D"),
            });

            ans += sequence[&sequence.len() / 2];
            continue;
        }

        if !part2 && is_correct {
            ans += sequence[&sequence.len() / 2];
        }
    }
    ans
}

fn has_correct_order(sequence: &Vec<u32>, rules: &HashMap<(u32, u32), i32>) -> bool {
    for i in 0..sequence.len() {
        for j in i + 1..sequence.len() {
            let key = (sequence[i], sequence[j]);

            if rules.get(&key).unwrap_or(&1) == &1 {
                return false;
            }
        }
    }
    true
}
