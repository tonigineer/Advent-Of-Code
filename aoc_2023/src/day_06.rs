use common::{Answer, Solution};

pub struct Day06;

impl Solution for Day06 {
    fn name(&self) -> &'static str {
        "Wait For It"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve2(input).into();
    }
}

fn solve(input: &str) -> u64 {
    let (time_str, distance_str) = input.split_once("\n").unwrap();
    let times: Vec<u32> = time_str
        .split_once(":")
        .unwrap().1
        .split_whitespace()
        .map(|t| t.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = distance_str
        .split_once(":")
        .unwrap().1
        .split_whitespace()
        .map(|t| t.parse::<u32>().unwrap())
        .collect();

    let mut result = 1;
    for i in 0..times.len() {
        let mut race_result = 0;
        for t in 0..=times[i] {
            if (times[i] - t) * t > distances[i] { race_result += 1 }
        }
        result *= race_result;
    }

    return result;
}

fn solve2(input: &str) -> u64 {
    let (time_str, distance_str) = input.split_once("\n").unwrap();
    let time = time_str
        .split_once(":").unwrap()
        .1
        .split_whitespace()
        .fold("".to_string(), |acc, t| format!("{acc}{t}"))
        .parse::<u64>().unwrap();
    let distance = distance_str
        .split_once(":").unwrap()
        .1
        .split_whitespace()
        .fold("".to_string(), |acc, t| format!("{acc}{t}"))
        .parse::<u64>().unwrap();

    return (0..time)
        .into_iter()
        .filter(|t| (time - t) * t > distance)
        .collect::<Vec<_>>()
        .len() as u64;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::{solve, solve2};

    const SAMPLE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE), 288);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve2(SAMPLE), 71503);
    }
}