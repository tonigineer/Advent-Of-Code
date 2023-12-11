use common::{Answer, Solution};

pub struct Day09;

impl Solution for Day09 {
    fn name(&self) -> &'static str {
        "Mirage Maintenance"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(input: &str, part2: bool) -> i64 {
    let input = input.strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input);
    let mut result: i64 = 0;

    for seq in input.split("\n") {
        let mut diffs: Vec<Vec<i64>> = Vec::new();
        let mut numbers: Vec<i64> = seq.split(" ").map(|n| n.parse::<i64>().unwrap()).collect();
        if part2 { numbers.reverse() }

        loop {
            diffs.push(numbers.clone());
            let diff: Vec<i64> = numbers.windows(2).map(|w| w[1] - w[0]).collect();
            if diff.iter().all(|&x| x == 0) { break }
            numbers = diff;
        }

        let mut new_value = 0;
        for i in (0..diffs.len()).rev() {
            new_value += diffs[i].last().unwrap();
        }

        result += new_value;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 114);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 2);
    }
}

