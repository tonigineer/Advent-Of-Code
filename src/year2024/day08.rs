//! Resonant Collinearity
//!
//! Summary:

use crate::common::grid_legacy::*;
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
    let grid: Grid<char> = input.into();
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if grid.data[r][c] == '.' {
                continue;
            }

            antennas
                .entry(grid.data[r][c])
                .and_modify(|v| v.push((c as isize, r as isize)))
                .or_insert(vec![(c as isize, r as isize)]);
        }
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    if part2 {
        for locations in antennas.values() {
            for (c1, r1) in locations.iter() {
                for (c2, r2) in locations.iter() {
                    if (c1, r1) == (c2, r2) {
                        continue;
                    }

                    // NOTE: Since each pair is looped twice, only one direction
                    // is checked here.
                    let mut c = *c1;
                    let mut r = *r1;
                    let dr = r2 - r1;
                    let dc = c2 - c1;

                    while grid.in_bounds(c, r) {
                        antinodes.insert((c, r));
                        c += dc;
                        r += dr;
                    }
                }
            }
        }

        return antinodes.len() as u32;
    }

    for locations in antennas.values() {
        for (i, (c1, r1)) in locations.iter().enumerate() {
            for (c2, r2) in locations.iter().skip(i + 1) {
                antinodes.insert((2 * c1 - c2, 2 * r1 - r2));
                antinodes.insert((2 * c2 - c1, 2 * r2 - r1));
            }
        }
    }

    antinodes.iter().filter(|(c, r)| grid.in_bounds(*c, *r)).count() as u32
}
