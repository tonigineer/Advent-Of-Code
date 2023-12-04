use common::{Answer, Solution};

use itertools::Itertools;

pub struct Day10;

impl Solution for Day10 {
    fn name(&self) -> &'static str {
        "Elves Look, Elves Say"
    }

    fn part1(&self, input: &str) -> Answer {
        return lookandsay(input, 40).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return lookandsay(input, 50).into();
    }
}

fn lookandsay(input: &str, repetitions: u8) -> usize {
    let mut process_string: String = String::from(input);

    for _ in 0..repetitions {
        let mut next_string: String = String::new();

        for (digit, group) in &process_string.chars().group_by(|c| *c) {
            let num: Vec<_> = group.collect();
            next_string.push_str(&num.len().to_string());
            next_string.push_str(&digit.to_string());
        }
        process_string = String::from(next_string);
    }
    return process_string.to_string().len();
}