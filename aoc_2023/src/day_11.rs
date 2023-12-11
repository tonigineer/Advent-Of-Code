use common::{Answer, Solution};

use std::collections::HashSet;
use itertools::Itertools;

pub struct Day11;

impl Solution for Day11 {
    fn name(&self) -> &'static str {
        "Cosmic Expansion"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, 2).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, 1000000).into();
    }
}

fn solve(input: &str, expansion_dist: u64) -> u64 {
    let mut result = 0;

    let mut empty_rows: HashSet<u64> = HashSet::new();
    let mut empty_cols: HashSet<u64> = HashSet::new();

    let mut grid: Vec<Vec<char>> = Vec::new();
    for (r, line) in input.lines().enumerate() {
        let l: Vec<char> = line.chars().into_iter().collect();

        if ! line.contains("#") {
            empty_rows.insert(r as u64);
        }

        grid.push(l);
    }

    for c in 0..grid[0].len() {
        let mut contains = false;

        for r in 0..grid.len() {
            if grid[r][c] == '#' {
                contains = true;
                break;
            }
        }

        if ! contains { 
            empty_cols.insert(c as u64);
        }
    }

    let mut galaxies: HashSet<(usize, usize)> = HashSet::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '#' {
                galaxies.insert((r, c));
            }
        }
    }

    for pair in galaxies.iter().combinations(2) {
        for r in pair[0].0.min(pair[1].0)..pair[0].0.max(pair[1].0) {
            if empty_rows.contains(&(r as u64)) {
                result += expansion_dist
            }
            else {
                result += 1;
            }
        }

        for c in pair[0].1.min(pair[1].1)..pair[0].1.max(pair[1].1) {
            if empty_cols.contains(&(c as u64)) {
                result += expansion_dist
            }
            else {
                result += 1;
            }
        }
    }
    return result as u64;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {
        "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."
    };

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, 2), 374);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, 100), 8410);
    }
}