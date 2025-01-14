use common::{Answer, Solution};

pub struct Day21;

impl Solution for Day21 {
    fn name(&self) -> &'static str {
        "Keypad Conundrum"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, true).into()
    }
}

fn solve(_input: &str, _part2: bool) -> usize {
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
