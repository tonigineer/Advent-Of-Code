use common::{Answer, Solution};

use fancy_regex::Regex;
use lazy_static::lazy_static;

pub struct Day05;

impl Solution for Day05 {
    fn name(&self) -> &'static str {
        "Doesn't He Have Intern-Elves For This?"
    }

    fn part1(&self, input: &str) -> Answer {
        return input.lines().filter(|l| nice_part1(l)).count().into();
    }

    fn part2(&self, input: &str) -> Answer {
        return input.lines().filter(|l| nice_part2(l)).count().into();
    }
}

fn nice_part1(line: &str) -> bool {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"([aeiou].*){3}").unwrap();
        static ref RE2: Regex = Regex::new(r"(.)\1").unwrap();
        static ref RE3: Regex = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    }

    return RE1.is_match(line).unwrap()
        && RE2.is_match(line).unwrap()
        && !RE3.is_match(line).unwrap();
}

fn nice_part2(line: &str) -> bool {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"(..).*(\1)").unwrap();
        static ref RE2: Regex = Regex::new(r"(.).(\1)").unwrap();
    }

    return RE1.is_match(line).unwrap() && RE2.is_match(line).unwrap();
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::{nice_part1, nice_part2};

    const SAMPLE: &str = indoc! {"
        ugknbfddgicrmopn
        aaa
        jchzalrnumimnmhp
        haegwjzuvuyypxyu
        dvszwmarrgswjxmb
    "};

    #[test]
    fn example_part1() {
        assert_eq!(SAMPLE.lines().filter(|l| nice_part1(l)).count(), 2);
    }

    const SAMPLE2: &str = indoc! {"
        qjhvhtzxzqqjkmpb
        xxyxx
        uurcxstgmygtbstg
        ieodomkazucvgmuy
    "};

    #[test]
    fn example_part2() {
        assert_eq!(SAMPLE2.lines().filter(|l| nice_part2(l)).count(), 2);
    }
}