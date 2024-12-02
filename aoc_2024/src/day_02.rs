use common::{Answer, Solution};

pub struct Day02;

impl Solution for Day02 {
    fn name(&self) -> &'static str {
        "Red-Nosed Reports"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    let mut ans = 0;

    for line in input.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        if is_safe(&report) {
            ans += 1;
            continue;
        }

        if !part2 {
            continue;
        }

        for i in 0..report.len() {
            let mut report_tol = report.clone();
            report_tol.remove(i);

            if is_safe(&report_tol) {
                ans += 1;
                break;
            }
        }
    }

    return ans;
}

fn is_safe(report: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = report.windows(2).map(|pair| pair[0] - pair[1]).collect();

    if diffs.iter().all(|v| v > &0) || diffs.iter().all(|v| v < &0) {
        if diffs.iter().map(|&x| x.abs()).max().unwrap() <= 3 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::solve;

    const SAMPLE: &str = indoc::indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 2);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 4);
    }
}
