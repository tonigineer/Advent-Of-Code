//! Corporate Policy
//!
//! Summary:

use itertools::Itertools;

pub fn parse(input: &str) -> String {
    solve(input.trim())
}

pub fn part1(input: &str) -> String {
    input.to_string()
}

pub fn part2(input: &str) -> String {
    solve(input)
}

fn solve(input: &str) -> String {
    let mut password = input.as_bytes().to_vec();

    loop {
        let mut i = password.len();
        while i > 0 {
            i -= 1;
            if password[i] == b'z' {
                password[i] = b'a';
            } else {
                password[i] += 1;
                break;
            }
        }

        let s_password = std::str::from_utf8(&password).unwrap();
        if is_valid(s_password) {
            return s_password.to_string();
        }
    }
}

fn is_valid(password: &str) -> bool {
    let mut first = false;
    let mut pairs = 0;

    let mut it = password.as_bytes().iter().tuple_windows().peekable();

    while let Some((&a, &b, &c)) = it.next() {
        // First criterion
        if !first && a + 2 == c && b + 1 == c {
            first = true;
        }

        // Second criterion
        if matches!(b, b'i' | b'l' | b'o') {
            return false;
        }

        // Third criterion
        if a == b && b != c {
            pairs += 1;
            continue;
        }

        if it.peek().is_none() && b == c {
            pairs += 1;
        }
    }

    first && pairs >= 2
}
