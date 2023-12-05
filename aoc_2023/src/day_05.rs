use common::{Answer, Solution};
use std::collections::VecDeque;

pub struct Day05;

impl Solution for Day05 {
    fn name(&self) -> &'static str {
        "If You Give A Seed A Fertilizer"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve2(input).into();
    }
}

fn solve(input: &str) -> u64 {
    let (seed_str, mappings) = input.split_once("\n\n").unwrap();
    let mut seeds = seed_str.split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    for map in mappings.split("\n\n") {
        let (_, ranges) = map.split_once("\n").unwrap();

        for i in 0..seeds.len() {
            for range in ranges.split("\n") {
                let tokens = range.split(" ").map(|i| i.parse::<u64>().unwrap()).collect::<Vec<_>>();
                let des = tokens[0];
                let src = tokens[1];
                let dim = tokens[2];

                if seeds[i] >= src && seeds[i] < src + dim {
                    seeds[i] = des + (seeds[i] - src);
                    break;
                }
            }
        }
    }
    return seeds.into_iter().min().unwrap();
}

fn solve2(input: &str) -> u64 {
    let (seed_str, mappings) = input.split_once("\n\n").unwrap();
    let seeds = seed_str.split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut new_seeds: VecDeque<_> = VecDeque::new();
    for i in (0..seeds.len()).step_by(2) {
        new_seeds.push_back((seeds[i], seeds[i] + seeds[i+1]));
    }

    let mut seeds = new_seeds;

    for map in mappings.split("\n\n") {
        let (_, ranges) = map.split_once("\n").unwrap();

        let mut new_seeds: VecDeque<_> = VecDeque::new();
        while ! seeds.is_empty() {
            let seed = seeds.pop_front().unwrap();
            let mut mapped = false;

            for range in ranges.split("\n") {
                let tokens = range.split(" ").map(|i| i.parse::<u64>().unwrap()).collect::<Vec<_>>();
                let des = tokens[0];
                let src = tokens[1];
                let dim = tokens[2];

                let start = std::cmp::max(seed.0, src);
                let end = std::cmp::min(seed.1, src + dim);

                if start < end {
                    mapped = true;
                    new_seeds.push_back((start - src + des, end - src + des));
                    if start > seed.0 {
                        seeds.push_back((seed.0, start));
                    }
                    if seed.1 > end {
                        seeds.push_back((end, seed.1))
                    }
                    break;
                }
            }
            if ! mapped { new_seeds.push_back((seed.0, seed.1)) };
        }
        seeds = new_seeds;
    }
    return seeds.iter().min().unwrap().0;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::{solve, solve2};

    const SAMPLE: &str = indoc! {"seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4"
    };

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE), 35);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve2(SAMPLE), 46);
    }
}