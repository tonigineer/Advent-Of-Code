use cached::proc_macro::cached;
use common::{Answer, Solution};

pub struct Day11;

impl Solution for Day11 {
    fn name(&self) -> &'static str {
        "Plutonian Pebbles"
    }

    fn part1(&self, input: &str) -> Answer {
        common::Answer::Number(
            input
                .trim()
                .split(" ")
                .map(|stone| solve(stone.parse::<u64>().unwrap(), 25))
                .sum(),
        )
    }

    fn part2(&self, input: &str) -> Answer {
        common::Answer::Number(
            input
                .trim()
                .split(" ")
                .map(|stone| solve(stone.parse::<u64>().unwrap(), 75))
                .sum(),
        )
    }
}

#[cached]
fn solve(stone: u64, steps: u8) -> u64 {
    if steps == 0 {
        return 1;
    }

    if stone == 0 {
        return solve(1, steps - 1);
    }

    if stone.clone().to_string().len() % 2 == 0 {
        let i = stone.to_string().len() / 2;
        let stone_str = stone.to_string();
        let (left, right) = stone_str.split_at(i);

        return solve(left.parse::<u64>().unwrap(), steps - 1)
            + solve(right.parse::<u64>().unwrap(), steps - 1);
    }

    solve(stone * 2024, steps - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        125 17
    "};

    #[test]
    fn example_part() {
        assert_eq!(Day11::part1(&Day11, SAMPLE), common::Answer::Number(55312));
    }
}
