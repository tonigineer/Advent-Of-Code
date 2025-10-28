//! The Ideal Stocking Stuffer
//!
//! Summary: The `to_string()` in creating the bytes for `compute` is
//! really slow.

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
    let mut idx = 0;

    let full_bytes = num_zeros / 2;
    let half_bytes = num_zeros % 2;

    let prefix = input.as_bytes();
    let mut buf: Vec<u8> = Vec::with_capacity(prefix.len() + 20);

    loop {
        buf.clear();
        buf.extend_from_slice(prefix);
        buf.extend_from_slice(&idx.to_string().into_bytes());

        let digest = md5::compute(&buf);
        let bytes = digest.0;

        if bytes[..full_bytes].iter().all(|&b| b == 0)
            && (half_bytes == 0 || bytes[full_bytes] & 0xF0 == 0)
        {
            return idx;
        }

        idx += 1
    }
}
