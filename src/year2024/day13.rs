//! Garden Groups
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use itertools::Itertools;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u64 {
    solve(input, false)
}

pub fn part2(input: &str) -> u64 {
    solve(input, true)
}

fn solve(input: &str, part2: bool) -> u64 {
    let mut token = 0;
    for game in input.split("\n\n") {
        let (ax, ay, bx, by, mut rx, mut ry) = game
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();

        if part2 {
            rx += 10_000_000_000_000;
            ry += 10_000_000_000_000;
        }
        let (a, b) = solve_lineq(ax, bx, ay, by, rx, ry);
        token += 3 * a + b;
    }

    token as u64
}

fn solve_lineq(x1: i64, y1: i64, x2: i64, y2: i64, z1: i64, z2: i64) -> (i64, i64) {
    // Cramer's Rule
    //  Multiply:
    //      x1 * a + y1 * b = z1;  * x2
    //      x2 * a + y2 * b = z2;  * x1
    //  Substract:
    //      x2 * x1 * a + x2 * y1 * b = x2 * z1  -  x1 * x2 * a + x1 * y2 * b = x1 * z2
    //      (x1 * y2 - x2 * y1) * b = x1 * z2 - x2 * z1

    let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
    let a = (z1 - b * y1) / x1;

    if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
        return (0, 0);
    }

    (a, b)
}
