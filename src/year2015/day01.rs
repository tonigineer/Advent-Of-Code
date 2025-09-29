//! # Not Quite Lisp
//!
//! Nothing fancy here.

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> i32 {
    input.chars().count() as i32 - 2 * input.matches(')').count() as i32
}

pub fn part2(input: &str) -> usize {
    let mut level = 0;
    let mut result: usize = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => level += 1,
            ')' => level -= 1,
            _ => panic!("Char not defined."),
        }
        result = i;
        if level < 0 {
            break;
        }
    }
    return result + 1;
}
