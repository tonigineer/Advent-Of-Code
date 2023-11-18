use fancy_regex::Regex;
use lazy_static::lazy_static;

fn nice_part1(line: &str) -> bool {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"([aeiou].*){3}").unwrap();
        static ref RE2: Regex = Regex::new(r"(.)\1").unwrap();
        static ref RE3: Regex = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    }

    return RE1.is_match(line).unwrap() &&
        RE2.is_match(line).unwrap() &&
        ! RE3.is_match(line).unwrap();
}

fn nice_part2(line: &str) -> bool {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"(..).*(\1)").unwrap();
        static ref RE2: Regex = Regex::new(r"(.).(\1)").unwrap();
    }

    return RE1.is_match(line).unwrap() &&
        RE2.is_match(line).unwrap();
}

fn main() {
    let input = include_str!("../../inputs/05.in").trim();

    print!("󰎤 {} ", input.lines().filter(|l| nice_part1(l)).count());
    print!("󰎧 {} ", input.lines().filter(|l| nice_part2(l)).count());
}
