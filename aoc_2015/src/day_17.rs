use common::{Answer, Solution};

use itertools::Itertools;

pub struct Day17;

impl Solution for Day17 {
    fn name(&self) -> &'static str {
        "No Such Thing as Too Much"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(input: &str, part2: bool) -> usize {
    let container: Vec<u32> = input.lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect();

    let mut num_solutions = 0;
    for n in 1..container.len() {
        let num = container
            .iter()
            .combinations(n)
            .filter(|c| c.into_iter()
                .copied()
                .sum::<u32>() == 150
            )
            .count();
        num_solutions += num;
        if part2 && num > 0 { return num }
    }
    return num_solutions;
}
