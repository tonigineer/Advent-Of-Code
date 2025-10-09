//! No Such Thing as Too Much
//!
//! Summary:

use itertools::Itertools;

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
    let container: Vec<u32> = input.lines().map(|l| l.parse::<u32>().unwrap()).collect();

    let mut num_solutions = 0;
    for n in 1..container.len() {
        let num = container
            .iter()
            .combinations(n)
            .filter(|c| c.iter().copied().sum::<u32>() == 150)
            .count();
        num_solutions += num;
        if part2 && num > 0 {
            return num;
        }
    }
    num_solutions
}
