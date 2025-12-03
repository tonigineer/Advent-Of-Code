//! # Lobby
//!
//!

type InputParsed = Vec<Vec<u8>>;

pub fn parse(input: &str) -> InputParsed {
    input
        .trim()
        .lines()
        .map(|l| l.as_bytes().iter().map(|&c| c - b'0').collect::<Vec<u8>>())
        .collect::<InputParsed>()
}

pub fn part1(input: &InputParsed) -> u32 {
    input
        .iter()
        .map(|numbers| {
            let n = numbers.len();

            let (idx, first) =
                &numbers[..n - 1].iter().enumerate().rev().max_by_key(|&(_, v)| v).unwrap();
            let second = &numbers[*idx + 1..n].iter().max().unwrap();

            **first as u32 * 10 + **second as u32
        })
        .sum()
}

pub fn part2(input: &InputParsed) -> u64 {
    input
        .iter()
        .map(|numbers| {
            let mut voltage = 0;
            let mut start = 0;

            for digit in 0..12 {
                let end = numbers.len() - 11 + digit;

                let (idx, val) = &numbers[start..end]
                    .iter()
                    .enumerate()
                    .rev()
                    .max_by_key(|&(_, val)| val)
                    .unwrap();

                start = start + idx + 1;
                voltage += 10u64.pow(11 - digit as u32) * **val as u64
            }
            voltage
        })
        .sum()
}
