use common::{Answer, Solution};
use std::collections::HashSet;

pub struct Day25;

impl Solution for Day25 {
    fn name(&self) -> &'static str {
        "Code Chronicle"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input).into()
    }

    fn part2(&self, _input: &str) -> Answer {
        "NO PUZZLE".into()
    }
}

fn solve(input: &str) -> usize {
    let mut keys = HashSet::new();
    let mut locks = HashSet::new();

    for item in input.split("\n\n") {
        let grid = item.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
        if grid[0][0] == b'#' {
            locks.insert(grid);
        } else {
            keys.insert(grid);
        }
    }

    let mut ans = 0;
    for lock in &locks {
        'loop_keys: for key in &keys {
            for r in 0..lock.len() {
                for c in 0..lock[0].len() {
                    if lock[r][c] == b'#' && key[r][c] == b'#' {
                        continue 'loop_keys;
                    }
                }
            }
            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE), 3);
    }
}
