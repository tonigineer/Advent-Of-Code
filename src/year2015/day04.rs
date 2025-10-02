//! The Ideal Stocking Stuffer
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u32 {
    solve(input, 5)
}

pub fn part2(input: &str) -> u32 {
    solve(input, 6)
}

fn solve(input: &str, num_zeros: usize) -> u32 {
    let mut index = 0;

    loop {
        let digest = md5::compute(format!("{input}{index}"));

        if &format!("{:?}", digest)[0..num_zeros] == format!("{:0num_zeros$}", 0) {
            return index;
        }

        index += 1
    }
}
