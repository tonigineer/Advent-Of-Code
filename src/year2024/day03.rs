//! Mull It Over
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use regex::Regex;

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
    let mut ans = 0;
    let mut toggle = true;

    let re_mul = Regex::new(r"mul\((\d+),\s*(\d+)\)|do\(\)|don't\(\)").unwrap();

    for cap in re_mul.captures_iter(input) {
        if &cap[0] == "do()" {
            toggle = true;
        } else if &cap[0] == "don't()" {
            toggle = false;
        } else if toggle || !part2 {
            let x1 = &cap[1].parse::<u32>().unwrap();
            let x2 = &cap[2].parse::<u32>().unwrap();
            ans += x1 * x2;
        }
    }

    ans
}
