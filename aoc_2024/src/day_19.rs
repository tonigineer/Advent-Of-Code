use common::{Answer, Solution};
use std::collections::HashMap;

pub struct Day19;

impl Solution for Day19 {
    fn name(&self) -> &'static str {
        "Linen Layout"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, true).into()
    }
}

fn solve(input: &str, part2: bool) -> usize {
    let (item_list, design_list) = input.split_once("\n\n").unwrap();

    let items = item_list.split(", ").collect::<Vec<_>>();
    let designs = design_list.trim().split("\n").collect::<Vec<_>>();

    let mut seen: HashMap<String, usize> = HashMap::new();

    let (ans1, ans2) = designs.iter().fold((0, 0), |acc, design| {
        let sol = check(design.to_string(), &items, &mut seen);
        (acc.0 + (sol > 0) as usize, acc.1 + sol)
    });

    if part2 {
        return ans2;
    }

    ans1
}

fn check(receipt: String, items: &Vec<&str>, seen: &mut HashMap<String, usize>) -> usize {
    if receipt == "" {
        return 1;
    };

    if seen.contains_key(&receipt) {
        return *seen.get(&receipt).unwrap();
    }

    let solutions = items
        .iter()
        .filter(|v| receipt.starts_with(**v))
        .map(|v| check(receipt.clone().split_off(v.len()), items, seen))
        .sum();

    seen.insert(receipt, solutions);

    solutions
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 6);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 16);
    }
}
