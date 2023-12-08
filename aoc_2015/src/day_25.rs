use common::{Answer, Solution};

pub struct Day25;

impl Solution for Day25 {
    fn name(&self) -> &'static str {
        "Let It Snow"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input).into();
    }

    fn part2(&self, _: &str) -> Answer {
        return "NO PUZZLE".into();
    }
}

use regex::Regex;

fn solve(input: &str) -> u64 {
    let re = Regex::new(r"row (\d+), column (\d+)").unwrap();
    let captures = re.captures(input).unwrap();
    let (row, column): (u64, u64) = (captures[1].parse().unwrap(), captures[2].parse().unwrap());

    // Determine min size length (base) of triangle
    // Calculate number of nodes (column of first row)
    // traverse column back till row found for wanted number
    let base = row + column - 1;
    let num_full_triangle = base * (base+1) / 2;
    let number = num_full_triangle - row + 1;

    let mut code: u64 = 20151125;
    let multiplier: u64 = 252533;
    let divider: u64 = 33554393;

    for _ in 2..=number {
        code = code * multiplier % divider;
    }
    return code;
}
