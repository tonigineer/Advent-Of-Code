use common::{Answer, Solution};

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day13;

impl Solution for Day13 {
    fn name(&self) -> &'static str {
        "Knights of the Dinner Table"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(input: &str, part2: bool) -> i32 {
    let mut h: HashMap<(String, String), i32> = HashMap::new();
    let mut p: HashSet<String> = HashSet::new();

    for line in input.lines() {
        let modded_line = line
            .trim_end_matches('.')
            .replace("gain ", "")
            .replace("lose ", "-");
        let token: Vec<&str> = modded_line.split(" ").collect();

        let (p1, p2, happiness) = (
            token[0].to_string(),
            token[9].to_string(),
            token[2].parse().unwrap(),
        );
        h.insert((p1.clone(), p2), happiness);
        p.insert(p1);
    }

    if part2 {
        for p in p.iter() {
            h.insert((p.to_string(), "toni".to_string()), 0);
            h.insert(("toni".to_string(), p.to_string()), 0);
        }
        p.insert("toni".to_string());
    }

    let mut max_happiness = 0;
    for permutation in p.iter().permutations(p.len()) {
        let mut happiness = 0;
        for (p1, p2) in permutation.iter().circular_tuple_windows() {
            happiness += h.get(&(p1.to_string(), p2.to_string())).unwrap();
            happiness += h.get(&(p2.to_string(), p1.to_string())).unwrap();
        }
        if happiness > max_happiness {
            max_happiness = happiness
        }
    }

    return max_happiness;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"
        Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 330);
    }
}