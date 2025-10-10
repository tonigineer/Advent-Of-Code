//! Race Condition
//!
//! Summary:

use crate::common::grid::*;
use crate::common::position::*;

use hashbrown::HashMap;
use itertools::Itertools;
use std::collections::VecDeque;

pub fn parse(input: &str) -> (usize, usize) {
    let grid = Grid::from(input.trim());
    let start = grid.search(b'S').unwrap();
    let end = grid.search(b'E').unwrap();

    // BFS all positions to get distance to start
    let mut distances = HashMap::new();
    let mut q = VecDeque::from([(start, 0)]);

    while let Some((position, dist)) = q.pop_front() {
        if distances.contains_key(&position) {
            continue;
        }

        distances.insert(position, dist);

        if position == end {
            continue;
        }

        for delta in CARDINALS {
            if grid[position + delta] != b'#' {
                q.push_back((position + delta, dist + 1));
            }
        }
    }

    // Iterate only once and check both puzzles
    distances.iter().tuple_combinations().fold(
        (0usize, 0usize),
        |(mut result1, mut result2), ((position1, distance1), (position2, distance2))| {
            let manhattan_dist = position1.manhattan(position2);

            let d1: usize = *distance1;
            let d2: usize = *distance2;

            if manhattan_dist <= 20 && d2.abs_diff(d1) >= manhattan_dist + 100 {
                if manhattan_dist <= 2 {
                    result1 += 1;
                }
                result2 += 1;
            }

            (result1, result2)
        },
    )
}

pub fn part1(results: &(usize, usize)) -> usize {
    results.0
}

pub fn part2(results: &(usize, usize)) -> usize {
    results.1
}
