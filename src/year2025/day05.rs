//! # Cafeteria
//!
//!

use crate::common::parse::ParseInteger;

type InputParsed<'a> = (Vec<(u64, u64)>, &'a str);

pub fn parse<'a>(input: &'a str) -> InputParsed<'a> {
    let (top, bottom) = input.trim().split_once("\n\n").unwrap();

    let ranges = top
        .lines()
        .map(|l| {
            let (s, e) = l.split_once('-').unwrap();
            (s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap())
        })
        .collect();

    (ranges, bottom)
}

pub fn part1(input: &InputParsed) -> usize {
    let (ranges, bottom_str) = input;
    let ingredients = bottom_str.parse_uint_iter::<u64>();

    ingredients.filter(|&ing| ranges.iter().any(|&(start, end)| ing >= start && ing <= end)).count()
}

pub fn part2(input: &InputParsed) -> u64 {
    let mut ranges = input.0.clone();
    ranges.sort_unstable();

    let last = (ranges[0].0, ranges[0].1);

    let (result, last) = ranges.iter().skip(1).fold((0, last), |acc, &range| {
        let mut result = acc.0;
        let mut last = acc.1;

        if last.1 < range.0 {
            result += last.1 - last.0 + 1;
            last = range;
        } else {
            last.1 = last.1.max(range.1);
        }

        (result, last)
    });

    result + last.1 - last.0 + 1
}
