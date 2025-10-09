//! All in a Single Night
//!
//! Summary:

use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

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
    let mut routes: HashMap<(&str, &str), u32> = HashMap::new();
    let mut cities: HashSet<&str> = HashSet::new();

    for line in input.lines() {
        let [c1, _, c2, _, dist]: [&str; 5] =
            line.split(' ').collect::<Vec<&str>>().try_into().unwrap();

        routes.insert((c1, c2), dist.parse::<u32>().unwrap());
        routes.insert((c2, c1), dist.parse::<u32>().unwrap());
        cities.insert(c1);
        cities.insert(c2);
    }

    let mut target_distance = if part2 { 0 } else { 1_000_000_000 };

    for route in cities.iter().permutations(cities.len()) {
        let mut distance = 0;
        for i in 0..route.len() - 1 {
            distance += routes.get(&(route[i], route[i + 1])).unwrap();
        }
        target_distance =
            if part2 { max(target_distance, distance) } else { min(target_distance, distance) };
    }
    target_distance
}
