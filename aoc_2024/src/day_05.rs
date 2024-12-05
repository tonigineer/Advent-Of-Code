use common::{Answer, Solution};
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Day05;

impl Solution for Day05 {
    fn name(&self) -> &'static str {
        "Print Queue"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, true).into()
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    let mut rules: HashMap<(u32, u32), i32> = HashMap::new();

    let mut ans = 0;

    for rule in rules_str.lines() {
        let (x, y) = rule.split_once('|').unwrap();
        rules.insert((x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()), -1);
        rules.insert((y.parse::<u32>().unwrap(), x.parse::<u32>().unwrap()), 1);
    }

    for line in updates_str.lines() {
        let mut sequence: Vec<u32> = line
            .split(',')
            .map(|num| num.parse::<u32>().unwrap())
            .collect();

        let is_correct = has_correct_order(&sequence, &rules);

        if part2 && !is_correct {
            sequence.sort_by(|x, y| match rules.get(&(*x, *y)).unwrap_or(&0) {
                -1 => Ordering::Less,
                0 => Ordering::Equal,
                1 => Ordering::Greater,
                _ => panic!("What the fuck is going on! Press F for Dillon :D"),
            });

            ans += sequence[&sequence.len() / 2];
            continue;
        }

        if !part2 && is_correct {
            ans += sequence[&sequence.len() / 2];
        }
    }
    ans
}

fn has_correct_order(sequence: &Vec<u32>, rules: &HashMap<(u32, u32), i32>) -> bool {
    for i in 0..sequence.len() {
        for j in i + 1..sequence.len() {
            let key = (sequence[i], sequence[j]);

            if rules.get(&key).unwrap_or(&1) == &1 {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::solve;

    const SAMPLE: &str = indoc::indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 143);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 123);
    }
}
