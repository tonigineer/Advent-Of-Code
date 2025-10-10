//! Reindeer Maze
//!
//! Summary: BFS with binary heap to find most efficient path. Second part going backwards and checking with
//! positions on the way have the expected score.

use crate::common::grid::*;
use crate::common::position::*;

use hashbrown::{HashMap, HashSet};
use std::collections::{BinaryHeap, VecDeque};

pub fn parse(input: &str) -> (i64, i64) {
    let grid = Grid::from(input.trim());
    let start = grid.search(b'S').unwrap();
    let end = grid.search(b'E').unwrap();

    let (mut part1, mut part2) = (0, HashSet::new());

    let mut q = BinaryHeap::from([(0, 0, start)]);
    let mut seen = HashMap::new();

    while let Some((score, dir, position)) = q.pop() {
        let score = -score;

        if position == end {
            part1 = score;
            break;
        }

        for (new_dir, delta) in CARDINALS.iter().enumerate() {
            let new_position = position + *delta;

            if grid[new_position] == b'#' {
                continue;
            }

            let new_score = score + if dir == new_dir { 1 } else { 1001 };
            let previous_score = seen.get(&(new_position, new_dir)).unwrap_or(&i64::MAX);

            if new_score < *previous_score {
                seen.insert((new_position, new_dir), new_score);
                q.push((-new_score, new_dir, new_position));
            }
        }
    }

    let mut q = VecDeque::new();
    for dir in 0..4 {
        if seen.get(&(end, dir)).copied().unwrap_or(i64::MAX) == part1 {
            q.push_back((end, dir, part1));
        }
    }

    while let Some((position, dir, score)) = q.pop_front() {
        part2.insert(position);

        for new_dir in 0..4 {
            let new_score = score - if dir == new_dir { 1 } else { 1001 };

            let delta = CARDINALS[dir];
            let new_position = position - delta;

            if *seen.get(&(new_position, new_dir)).unwrap_or(&i64::MAX) == new_score {
                q.push_back((new_position, new_dir, new_score));
            }
        }
    }

    (part1, (part2.len() + 1) as i64)
}

pub fn part1(input: &(i64, i64)) -> i64 {
    input.0
}

pub fn part2(input: &(i64, i64)) -> i64 {
    input.1
}
