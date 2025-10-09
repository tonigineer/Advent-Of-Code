//! Code Chronicle
//!
//! Summary:

use std::collections::HashSet;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> usize {
    solve(input)
}

pub fn part2(_input: &str) -> &str {
    "🌲☃️🎅"
}

fn solve(input: &str) -> usize {
    let mut keys = HashSet::new();
    let mut locks = HashSet::new();

    for item in input.split("\n\n") {
        let grid = item.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
        if grid[0][0] == b'#' {
            locks.insert(grid);
        } else {
            keys.insert(grid);
        }
    }

    let mut ans = 0;
    for lock in &locks {
        'loop_keys: for key in &keys {
            for r in 0..lock.len() {
                for c in 0..lock[0].len() {
                    if lock[r][c] == b'#' && key[r][c] == b'#' {
                        continue 'loop_keys;
                    }
                }
            }
            ans += 1;
        }
    }

    ans
}
