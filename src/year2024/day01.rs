//! # Historian Hysteria
//!
//! A summary line was not given :(

pub fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let num_lines = input.lines().count();

    let mut left: Vec<u32> = Vec::with_capacity(num_lines);
    let mut right: Vec<u32> = Vec::with_capacity(num_lines);

    for line in input.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse::<u32>().unwrap());
        right.push(r.parse::<u32>().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}

pub fn part1(sides: &(Vec<u32>, Vec<u32>)) -> u32 {
    sides
        .0
        .iter()
        .zip(&sides.1)
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum::<u32>()
}

pub fn part2(sides: &(Vec<u32>, Vec<u32>)) -> u32 {
    sides
        .0
        .iter()
        .map(|l| *l * sides.1.iter().filter(|v| **v == *l).count() as u32)
        .sum::<u32>()
}
