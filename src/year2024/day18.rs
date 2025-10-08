//! RAM Run
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> String {
    solve(input, false)
}

pub fn part2(input: &str) -> String {
    solve(input, true)
}

fn solve(inp: &str, part2: bool) -> String {
    // Differentiate between tests and puzzle
    let dim = if inp.lines().count() == 25 { 6 } else { 70 };
    let n = if inp.lines().count() == 25 { 12 } else { 1024 };

    let mut corrupt: VecDeque<(i64, i64)> = VecDeque::new();

    for l in inp.lines() {
        let (c, r) = l.split(',').map(|v| v.parse().unwrap()).collect_tuple().unwrap();
        corrupt.push_back((r, c));
    }

    let mut lo = n;
    let mut hi = corrupt.len() - 1;

    if !part2 {
        lo = n - 2;
        hi = n;
    }

    while lo < hi {
        let mi = (hi + lo) / 2 + 1;

        let mut q = VecDeque::new();
        let mut visited = HashSet::new();

        q.push_back((0, 0, 0));

        let mut connected = false;

        while !q.is_empty() {
            let (r, c, dist) = q.pop_front().unwrap();

            if (r, c) == (dim, dim) {
                if !part2 {
                    return dist.to_string();
                }

                connected = true;
                break;
            }

            if !visited.insert((r, c)) {
                continue;
            }

            for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (nr, nc) = (r + dr, c + dc);

                if corrupt.iter().take(mi).contains(&&(nr, nc))
                    || nr < 0
                    || nr > dim
                    || nc < 0
                    || nc > dim
                {
                    continue;
                }

                q.push_back((nr, nc, dist + 1));
            }
        }

        if connected {
            lo = mi;
        } else {
            hi = mi - 1;
        }
    }

    let point = corrupt.get(lo).unwrap();
    format!("{},{}", point.1, point.0)
}
