//! Restroom Redoubt
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use itertools::Itertools;
use std::collections::HashMap;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> usize {
    solve(input, 101, 103, false)
}

pub fn part2(input: &str) -> usize {
    solve(input, 101, 103, false)
}

fn solve(input: &str, width: isize, height: isize, part2: bool) -> usize {
    let robots = input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|w| !w.is_empty())
        .map(|w| w.parse::<isize>().unwrap())
        .tuples()
        .collect::<Vec<_>>();

    let mut tiles: HashMap<(isize, isize), usize> = HashMap::new();

    if !part2 {
        for (x, y, vx, vy) in robots.iter() {
            tiles
                .entry((
                    (x + vx * 100).rem_euclid(width),
                    (y + vy * 100).rem_euclid(height),
                ))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        let mut quadrants: Vec<usize> = vec![0, 0, 0, 0];

        for (tile, num_robots) in tiles.into_iter() {
            if tile.0 == width / 2 || tile.1 == height / 2 {
                continue;
            }

            let x = (tile.0 < width / 2) as usize;
            let y = (tile.1 < height / 2) as usize;

            quadrants[x * 2 + y] += num_robots;
        }

        return quadrants.iter().product();
    }

    for i in 1..100_000 {
        tiles.clear();

        for (x, y, vx, vy) in robots.iter() {
            tiles
                .entry((
                    (x + vx * i).rem_euclid(width),
                    (y + vy * i).rem_euclid(height),
                ))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        if tiles.values().all(|&v| v == 1) {
            return i as usize;
        }
    }

    panic!("Something went wrong.");
}
