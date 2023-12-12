use common::{Answer, Solution};

use std::collections::HashMap;
use itertools::Itertools;

pub struct Day12;

impl Solution for Day12 {
    fn name(&self) -> &'static str {
        "Hot Springs"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn count(mut springs: String, cfg: Vec<usize>, cache: &mut HashMap<(String, Vec<usize>), u64>) -> u64 {
    springs = springs.trim_start_matches('.').to_string();

    if springs.len() == 0 {
        if cfg.len() == 0 { return 1 } else { return 0 }
    }

    if cfg.len() == 0 {
        if springs.contains('#') { return 0 } else { return 1 }
    }

    if cache.contains_key(&(springs.clone(), cfg.clone())) {
        return *cache.get(&(springs, cfg)).unwrap()
    }

    if "#".contains(springs.chars().nth(0).unwrap()) {
        if springs.len() < cfg[0] {
            return 0;  // cfg does not fit in remaining string
        }
        else if springs[0..cfg[0]].contains('.') {
            return 0;  // started cfg does not fit in string ahead
        }
        else if cfg[0] == springs.len() {
            if cfg.len() == 1 { return 1 } else { return 0 };  // is cfg fits in string, must be last
        }
        else if springs.chars().nth(cfg[0]).unwrap() == '#' {
            return 0;  // Cluster to long, must be separated by . or ?
        }
        else {
            // Part of cfg found, look for next
            let result = count(springs[cfg[0]+1..].to_string(), Vec::from(&cfg[1..]), cache);
            cache.insert((springs.clone(), cfg.clone()), result);
            return result
        }
    };

    // Advance with both possible replacements for ?
    let result = count(format!("#{}", springs[1..].to_string()), cfg.clone(), cache)
                    + count(format!(".{}", springs[1..].to_string()), cfg.clone(), cache);
    cache.insert((springs, cfg), result);
    return result;
}

fn solve(input: &str, part2: bool) -> u64 {
    let mut result = 0;

    for line in input.lines() {
        let (mut springs, mut cfg) = line
            .split_once(" ")
            .map(|(s, c)|
                (
                    s.parse::<String>().unwrap(),
                    c.split(",")
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect_vec()
                )
            )
            .unwrap();

        if part2 {
            springs = std::iter::repeat(springs).take(5).join("?");
            cfg = std::iter::repeat(cfg).take(5).flatten().collect();
        }

        let mut cache: HashMap<(String, Vec<usize>), u64> = HashMap::new();
        result += count(springs, cfg, &mut cache);
    }

    return result as u64;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 21);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 525152);
    }
}