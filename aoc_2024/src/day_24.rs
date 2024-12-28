use common::{Answer, Solution};
use std::collections::HashMap;

pub struct Day24;

impl Solution for Day24 {
    fn name(&self) -> &'static str {
        ""
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, true).into()
    }
}

fn solve(input: &str, part2: bool) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 0);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 0);
    }
}
