//! Disk Fragmenter
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use std::collections::HashMap;
use std::collections::VecDeque;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> i64 {
    solve_part1(input)
}

pub fn part2(input: &str) -> usize {
    solve_part2(input)
}

fn solve_part1(input: &str) -> i64 {
    let mut disc: VecDeque<i64> = VecDeque::new();

    for (id, num) in input.chars().map(|v| v.to_digit(10).unwrap()).enumerate() {
        if id % 2 == 0 {
            for _ in 0..num {
                disc.push_back((id / 2) as i64);
            }
        } else {
            for _ in 0..num {
                disc.push_back(-1_i64);
            }
        }
    }

    let mut disc_arranged: VecDeque<i64> = VecDeque::new();

    while !disc.is_empty() {
        let left = disc.pop_front().unwrap();
        if left != -1 {
            disc_arranged.push_back(left);
        } else {
            while !disc.is_empty() {
                let right = disc.pop_back().unwrap();
                if right != -1 {
                    disc_arranged.push_back(right);
                    break;
                }
            }
        }
    }

    disc_arranged
        .iter()
        .enumerate()
        .map(|(id, v)| id as i64 * v)
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let mut files: HashMap<usize, (usize, u32)> = HashMap::new();
    let mut spaces: Vec<(usize, u32)> = Vec::new();

    let mut pos = 0;
    for (id, num) in input.chars().map(|v| v.to_digit(10).unwrap()).enumerate() {
        if id % 2 == 0 {
            files.insert(id / 2, (pos, num));
        } else {
            spaces.push((pos, num));
        }
        pos += num as usize;
    }

    for id in (0..files.len()).rev() {
        let (pos, length) = files[&id];

        for (id_space, (pos_space, length_space)) in spaces.clone().into_iter().enumerate() {
            if pos < pos_space {
                break;
            }

            if length <= length_space {
                files.insert(id, (pos_space, length));
                spaces[id_space] = (pos_space + length as usize, length_space - length);
                break;
            }
        }
    }

    files
        .keys()
        .zip(files.values())
        .map(|(id, (pos, len))| (*pos..*pos + *len as usize).map(|p| p * id).sum::<usize>())
        .sum()
}
