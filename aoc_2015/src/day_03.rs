use std::collections::HashMap;

use common::{Answer, Solution};

pub struct Day03;

impl Solution for Day03 {
    fn name(&self) -> &'static str {
        "Perfectly Spherical Houses in a Vacuum"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    let mut pos = (0, 0);
    let mut visited: HashMap<(i32, i32), u32> = HashMap::new();
    visited.insert(pos, 1);

    let mut pos_santa = (0, 0);
    let mut pos_robo = (0, 0);
    let mut current: &mut (i32, i32);
    let mut visited_part2: HashMap<(i32, i32), u32> = HashMap::new();
    visited_part2.insert(pos, 1);

    for (i, c) in input.chars().enumerate() {
        current = if i % 2 == 0 {
            &mut pos_santa
        } else {
            &mut pos_robo
        };

        match c {
            '^' => {
                pos.1 += 1;
                current.1 += 1;
            }
            'v' => {
                pos.1 -= 1;
                current.1 -= 1;
            }
            '>' => {
                pos.0 += 1;
                current.0 += 1;
            }
            '<' => {
                pos.0 -= 1;
                current.0 -= 1;
            }
            _ => panic!("Char {c} not implemented."),
        }

        visited.entry(pos).and_modify(|n| *n += 1).or_insert(1);
        visited_part2
            .entry(*current)
            .and_modify(|n| *n += 1)
            .or_insert(1);
    }

    if part2 {
        return visited_part2.len() as u32;
    };
    return visited.len() as u32;
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example_part1() {
        assert_eq!(solve(">", false), 2);
        assert_eq!(solve("^>v<", false), 4);
        assert_eq!(solve("^v^v^v^v^v", false), 2);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve("^v", true), 3);
        assert_eq!(solve("^>v<", true), 3);
        assert_eq!(solve("^v^v^v^v^v", true), 11);
    }
}