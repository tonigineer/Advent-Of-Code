//! # Gift Shop
//!
//!

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u64 {
    let mut result: u64 = 0;

    for range in input.split(',') {
        let (s_start, s_end) = range.split_once('-').unwrap();
        let start = s_start.parse::<u64>().unwrap();
        let end = s_end.parse::<u64>().unwrap();

        for num in start..=end {
            let s = num.to_string();
            let (left, right) = s.split_at(s.len() / 2);
            if left == right {
                result += num;
            }
        }
    }

    result
}

pub fn part2(input: &str) -> u64 {
    let mut result: u64 = 0;

    for range in input.split(',') {
        let (s_start, s_end) = range.split_once('-').unwrap();
        let start = s_start.parse::<u64>().unwrap();
        let end = s_end.parse::<u64>().unwrap();

        for num in start..=end {
            let s = num.to_string();
            let n = s.len();

            for idx in 0..n / 2 {
                let substring = &s[0..=idx];
                let repeated = substring.repeat(n / substring.len());

                if repeated == s {
                    result += num;
                    break;
                }
            }
        }
    }

    result
}
