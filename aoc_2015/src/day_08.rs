use common::{Answer, Solution};

use regex::Regex;

pub struct Day08;

impl Solution for Day08 {
    fn name(&self) -> &'static str {
        "Matchsticks"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(input: &str, part2: bool) -> usize {
    let mut code_len = 0;
    let mut render_len = 0;
    let mut encoded_len = 0;

    let re: Regex = Regex::new(r"\\x[a-f0-9A-F]{2}").unwrap();

    for line in input.lines() {
        // println!("{}", line);
        code_len += line.len();
        render_len += re
            .replace_all(line, "x")
            .replace("\\\\", "x")
            .replace("\\\"", "x")
            .len()
            - 2;
        encoded_len += line.len() + line.matches("\\").count() + line.matches("\"").count() + 2;
    }
    if part2 {
        return encoded_len - code_len;
    }
    return code_len - render_len;
}