use common::{Answer, Solution};

use itertools::Itertools;

pub struct Day24;

impl Solution for Day24 {
    fn name(&self) -> &'static str {
        "It Hangs in the Balance"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, 3).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, 4).into();
    }
}

fn solve(input: &str, compartments: usize) -> usize {
    let packages: Vec<usize> = input.lines().map(|p| p.parse().unwrap()).collect();

    let total_weight: usize = packages.iter().sum::<usize>() / compartments;
    let max_packages_front = packages.len()/compartments;

    let mut entanglement = 0;
    for i in 1..=max_packages_front {
        let c = packages.iter().copied()
            .combinations(i)
            .filter(|c| c.iter().sum::<usize>() == total_weight)
            .sorted_by_key(|c| c.iter().fold(1, |a, b| a * *b as i64))
            .min();

        if ! c.is_none() {
            entanglement = c.unwrap().iter().fold(1, |a, b| a * b);
            break
        }
    }
    return entanglement;
}
