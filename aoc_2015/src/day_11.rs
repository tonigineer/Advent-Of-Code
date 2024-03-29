use common::{Answer, Solution};

use fancy_regex::Regex;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashSet;


pub struct Day11;

impl Solution for Day11 {
    fn name(&self) -> &'static str {
        "Corporate Policy"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(&solve(input)).into();
    }
}

fn is_valid(password: &str) -> bool {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"(.)\1").unwrap();
        static ref RE2: Regex = Regex::new(r"[ilo]").unwrap();
    };

    // Second criteria
    if RE2.is_match(password).unwrap() {
        return false;
    };

    // Third criteria
    let matches = RE1
        .find_iter(password)
        .map(|m| m.unwrap().as_str())
        .collect::<HashSet<&str>>();
    if matches.len() < 2 {
        return false;
    }

    // First criteria
    for (a, b, c) in password.chars().tuple_windows() {
        if (c as u8 - 2 == a as u8) && (b as u8 - 1 == a as u8) {
            return true;
        }
    }

    return false;
}

fn solve(input: &str) -> String {
    let mut flip_next = true;
    let mut new_password: String = String::new();
    let mut password: String = String::from(input);

    loop {
        for c in password.chars().rev() {
            if flip_next {
                if c == 'z' {
                    flip_next = true;
                    new_password.push_str("a")
                } else {
                    flip_next = false;
                    new_password.push_str(&((c as u8 + 1) as char).to_string());
                }
                continue;
            }
            new_password.push_str(&c.to_string());
        }
        password = new_password.chars().rev().collect::<String>();
        if is_valid(&password) {
            break;
        }

        new_password = String::new();
        flip_next = true;
    }
    return password;
}
