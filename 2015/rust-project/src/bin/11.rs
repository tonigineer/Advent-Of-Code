use fancy_regex::Regex;
use std::collections::HashSet;
use itertools::Itertools;
use lazy_static::lazy_static;


fn is_valid(password: &str) -> bool {
    lazy_static! {
        static ref re1: Regex = Regex::new(r"(.)\1").unwrap();
        static ref re2: Regex = Regex::new(r"[ilo]").unwrap();
    };

    // Second criteria
    if re2.is_match(password).unwrap() {return false};

    // Third criteria
    let matches = re1.find_iter(password)
        .map(|m| m.unwrap().as_str())
        .collect::<HashSet<&str>>();
    if matches.len() < 2 { return false }

    // First criteria
    for (a, b, c) in password.chars().tuple_windows() {
        if (c as u8 -2 == a as u8) && (b as u8 - 1 == a as u8) {
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
        if is_valid(&password) { break }

        new_password = String::new();
        flip_next = true;
    }
    return password;
}

fn main() {
    let input = include_str!("../../inputs/11.in").trim();

    print!("󰎤 {} ", solve(&input));
    print!("󰎧 {} ", solve(&solve(&input)));  // whatever ;)
}
