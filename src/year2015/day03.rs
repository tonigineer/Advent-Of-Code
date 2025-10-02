//! Perfectly Spherical Houses in a Vacuum
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

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
    let mut pos = (0, 0);
    let mut visited: HashMap<(i32, i32), u32> = HashMap::new();
    visited.insert(pos, 1);

    let mut pos_santa = (0, 0);
    let mut pos_robo = (0, 0);
    let mut current: &mut (i32, i32);
    let mut visited_part2: HashMap<(i32, i32), u32> = HashMap::new();
    visited_part2.insert(pos, 1);

    for (i, c) in input.chars().enumerate() {
        current = if i % 2 == 0 {
            &mut pos_santa
        } else {
            &mut pos_robo
        };

        match c {
            '^' => {
                pos.1 += 1;
                current.1 += 1;
            }
            'v' => {
                pos.1 -= 1;
                current.1 -= 1;
            }
            '>' => {
                pos.0 += 1;
                current.0 += 1;
            }
            '<' => {
                pos.0 -= 1;
                current.0 -= 1;
            }
            _ => panic!("Char {c} not implemented."),
        }

        visited.entry(pos).and_modify(|n| *n += 1).or_insert(1);
        visited_part2
            .entry(*current)
            .and_modify(|n| *n += 1)
            .or_insert(1);
    }

    if part2 {
        return visited_part2.len() as u32;
    };
    return visited.len() as u32;
}
