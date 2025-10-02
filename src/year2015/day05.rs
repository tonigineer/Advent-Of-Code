//! Doesn't He Have Intern-Elves For This?
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use fancy_regex::Regex;
use lazy_static::lazy_static;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> usize {
    return input.lines().filter(|l| nice_part1(l)).count();
}

pub fn part2(input: &str) -> usize {
    return input.lines().filter(|l| nice_part2(l)).count();
}

fn nice_part1(line: &str) -> bool {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"([aeiou].*){3}").unwrap();
        static ref RE2: Regex = Regex::new(r"(.)\1").unwrap();
        static ref RE3: Regex = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    }

    return RE1.is_match(line).unwrap()
        && RE2.is_match(line).unwrap()
        && !RE3.is_match(line).unwrap();
}

fn nice_part2(line: &str) -> bool {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"(..).*(\1)").unwrap();
        static ref RE2: Regex = Regex::new(r"(.).(\1)").unwrap();
    }

    return RE1.is_match(line).unwrap() && RE2.is_match(line).unwrap();
}
