//! # Cafeteria
//!
//!

use crate::common::parse::{ParseInteger, ParseUnsigned};

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u32 {
    let (fresh, ingredients) = input.trim().split_once("\n\n").unwrap();

    let mut fresh_ranges = Vec::new();

    for f in fresh.lines() {
        let (start, end) = f.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        fresh_ranges.push((start, end));
    }

    let ingredients = ingredients.parse_uint_iter::<u64>();

    // dbg!(fresh_ranges);
    let mut result = 0;
    for i in ingredients {
        for range in fresh_ranges.iter() {
            if i >= range.0 && i <= range.1 {
                result += 1;
                break;
            }
        }
    }

    result
}

pub fn part2(input: &str) -> u64 {
    let (fresh, ingredients) = input.trim().split_once("\n\n").unwrap();

    let mut fresh_ranges = Vec::new();

    for f in fresh.lines() {
        let (start, end) = f.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        fresh_ranges.push((start, end));
    }

    let ingredients = ingredients.parse_uint_iter::<u64>();

    fresh_ranges.sort_unstable();
    let mut result = 0;
    let mut last = *fresh_ranges.iter().nth(0).unwrap();
    for range in fresh_ranges.iter().skip(1) {
        if last.1 < range.0 {
            result += last.1 - last.0 + 1;
            last = *range;
        } else {
            last.1 = last.1.max(range.1);
        }
    }

    result += last.1 - last.0 + 1;
    result
}
