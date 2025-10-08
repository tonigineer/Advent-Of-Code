//! Red-Nosed Reports
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u32 {
    solve(input, false)
}

pub fn part2(input: &str) -> u32 {
    solve(input, true)
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

    ans
}

fn is_safe(report: &[i32]) -> bool {
    let diffs: Vec<i32> = report.windows(2).map(|pair| pair[0] - pair[1]).collect();

    (diffs.iter().all(|v| v > &0) || diffs.iter().all(|v| v < &0))
        && diffs.iter().map(|&x| x.abs()).max().unwrap() <= 3
}
