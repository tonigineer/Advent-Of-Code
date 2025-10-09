//! Matchsticks
//!
//! Summary:

use regex::Regex;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> usize {
    solve(input, false)
}

pub fn part2(input: &str) -> usize {
    solve(input, true)
}

fn solve(input: &str, part2: bool) -> usize {
    let mut code_len = 0;
    let mut render_len = 0;
    let mut encoded_len = 0;

    let re: Regex = Regex::new(r"\\x[a-f0-9A-F]{2}").unwrap();

    for line in input.lines() {
        // println!("{}", line);
        code_len += line.len();
        render_len += re.replace_all(line, "x").replace("\\\\", "x").replace("\\\"", "x").len() - 2;
        encoded_len += line.len() + line.matches("\\").count() + line.matches("\"").count() + 2;
    }
    if part2 {
        return encoded_len - code_len;
    }
    code_len - render_len
}
