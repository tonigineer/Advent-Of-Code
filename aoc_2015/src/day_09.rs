use common::{Answer, Solution};

use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

pub struct Day09;

impl Solution for Day09 {
    fn name(&self) -> &'static str {
        "All in a Single Night"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    let mut routes: HashMap<(&str, &str), u32> = HashMap::new();
    let mut cities: HashSet<&str> = HashSet::new();

    for line in input.lines() {
        let [c1, _, c2, _, dist]: [&str; 5] =
            line.split(' ').collect::<Vec<&str>>().try_into().unwrap();

        routes.insert((c1, c2), dist.parse::<u32>().unwrap());
        routes.insert((c2, c1), dist.parse::<u32>().unwrap());
        cities.insert(c1);
        cities.insert(c2);
    }

    let mut target_distance = if part2 { 0 } else { 1_000_000_000 };

    for route in cities.iter().permutations(cities.len()) {
        let mut distance = 0;
        for i in 0..route.len() - 1 {
            distance += routes.get(&(route[i], route[i + 1])).unwrap();
        }
        target_distance = if part2 {
            max(target_distance, distance)
        } else {
            min(target_distance, distance)
        };
    }
    return target_distance;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"
        London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 605);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 982);
    }
}