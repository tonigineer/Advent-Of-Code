//! Monkey Market
//!
//! Summary: Calculate all secrets once along with their corresponding prices. Each
//! part then solves its respective puzzle independently. The second part could be
//! made more efficient by converting the sequence into an index, allowing the use
//! of direct lookup instead of a hashmap.

use crate::common::parse::*;

use ahash::{AHashMap, AHashSet};
use itertools::Itertools;

type InputParsed = Vec<(i64, [i64; 2000])>;

pub fn parse(input: &str) -> InputParsed {
    input
        .trim()
        .parse_int_iter::<i64>()
        .map(|mut num| {
            let mut prices = [0; 2000];
            for price in &mut prices {
                // Intructions converted to bitwise operations
                num = (num ^ (num << 6)) & 0xffffff;
                num = (num ^ (num >> 5)) & 0xffffff;
                num = (num ^ (num << 11)) & 0xffffff;

                *price = num % 10;
            }
            (num, prices)
        })
        .collect()
}

pub fn part1(input: &InputParsed) -> i64 {
    input.iter().map(|s| s.0).sum()
}

pub fn part2(input: &InputParsed) -> i64 {
    let mut sequences = AHashMap::new();
    let mut seen = AHashSet::new();

    for (_, prices) in input.iter() {
        seen.clear();
        for (a, b, c, d, e) in prices.iter().tuple_windows() {
            let key = (b - a, c - b, d - c, e - d);
            if seen.insert(key) {
                *sequences.entry(key).or_insert(0) += *e;
            }
        }
    }

    *sequences.values().max().unwrap()
}
