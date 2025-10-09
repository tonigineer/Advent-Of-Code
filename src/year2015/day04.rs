//! The Ideal Stocking Stuffer
//!
//! Summary:

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

        if format!("{:?}", digest)[0..num_zeros].chars().all(|c| c == '0') {
            return index;
        }

        index += 1
    }
}
