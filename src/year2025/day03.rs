//! # Lobby
//!
//!

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let numbers = line.as_bytes().iter().map(|c| c - b'0').collect::<Vec<_>>();
        let (idx, first) =
            &numbers[..numbers.len() - 1].iter().enumerate().rev().max_by_key(|&(_, v)| v).unwrap();
        let second = &numbers[*idx + 1..numbers.len()].iter().max().unwrap();

        result += (**first * 10u8 + **second) as u32;
    }

    result
}

pub fn part2(input: &str) -> u64 {
    let mut result = 0;
    for line in input.lines() {
        let numbers = line.as_bytes().iter().map(|c| c - b'0').collect::<Vec<_>>();
        let mut voltage = 0;
        let mut idx = 0;
        for digit in 0..12 {
            let start = idx;
            let end = numbers.len() - 11 + digit;

            let (i, v) =
                &numbers[start..end].iter().enumerate().rev().max_by_key(|&(_, v)| v).unwrap();

            idx = idx + i + 1;
            voltage += 10u64.pow(11 - digit as u32) * **v as u64;
        }

        result += voltage;
    }

    result
}
