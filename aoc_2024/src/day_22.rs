use common::{Answer, Solution};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day22;

impl Solution for Day22 {
    fn name(&self) -> &'static str {
        "Monkey Market"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, true).into()
    }
}

fn solve(input: &str, part2: bool) -> i64 {
    let mut ans = 0;
    let mut prices = HashMap::new();

    let mut cache = HashSet::new();

    // let mut cache2 = HashMap::new();

    for line in input.lines() {
        let mut num = line.parse::<i64>().unwrap();
        let mut nums = [0; 2000];

        for i in 0..2000 {
            num = (num ^ (num * 64)) % 16777216;
            num = (num ^ (num / 32)) % 16777216;
            num = (num ^ (num * 2048)) % 16777216;
            nums[i] = num % 10;
        }

        ans += num;

        cache.clear();
        for (a, b, c, d, e) in nums.iter().tuple_windows() {
            let k = (b - a) + (c - b) * 20 + (d - c) * 400 + (e - d) * 8000;
            if cache.insert(k) {
                *prices.entry(k).or_default() += *e;
            }
        }
        // cache2.clear();

        // for (a, b, c, d, e) in nums.into_iter().tuple_windows() {
        //     // let k = (b - a) + (c - b) * 100 + (d - c) * 10000 + (e - d) * 1000000;
        //     if cache.insert((b - a, c - b, d - c, e - d)) {
        //         cache2.insert((b - a, c - b, d - c, e - d), e);
        //         *prices.entry((b - a, c - b, d - c, e - d)).or_insert(0) += e;
        //     } else {
        //         if let Some(v) = cache2.get(&(b - a, c - b, d - c, e - d)) {
        //             if v < &e {
        //                 *prices.entry((b - a, c - b, d - c, e - d)).or_insert(0) += e - v;
        //                 cache2
        //                     .entry((b - a, c - b, d - c, e - d))
        //                     .and_modify(|val| *val = e);
        //             }
        //         }
        //     }
        // }
    }

    if part2 {
        *prices.values().max().unwrap()
    } else {
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        10
        1
        100
        2024
    "};
    const SAMPLE2: &str = indoc::indoc! {"
        1
        2
        3
        2024
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 37327623);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE2, true), 23);
    }
}
