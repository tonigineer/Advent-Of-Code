//! # Laboratories
//!
//!

use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

use crate::common::grid::Grid;
use crate::common::position::Position;

type ParsedInput = (Grid<u8>, Position);

const DOWN: Position = Position::new(0, 1);
const DOWN_LEFT: Position = Position::new(-1, 1);
const DOWN_RIGHT: Position = Position::new(1, 1);

pub fn parse(input: &str) -> ParsedInput {
    let input = input.trim();

    let grid = Grid::from(input);
    let start = grid.search(b'S').unwrap();

    (grid, start)
}

pub fn part1(input: &ParsedInput) -> u32 {
    let (grid, start) = input;

    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back(*start);

    let mut result = 0;

    while let Some(position) = q.pop_front() {
        let position_below = position + DOWN;

        if !grid.contains(position_below) {
            continue;
        }

        if grid[position_below] == b'^' {
            result += 1;

            for delta in [DOWN_LEFT, DOWN_RIGHT] {
                let side_position = position + delta;

                if grid.contains(side_position) && seen.insert(side_position) {
                    q.push_back(side_position);
                }
            }

            continue;
        }

        if seen.insert(position_below) {
            q.push_back(position_below);
        }
    }

    result
}

fn solve(cache: &mut HashMap<Position, usize>, grid: &Grid<u8>, position: Position) -> usize {
    if let Some(v) = cache.get(&position) {
        return *v;
    }

    if !grid.contains(position) {
        return 1;
    }

    let result = match grid[position] {
        b'.' | b'S' => solve(cache, grid, position + DOWN),
        b'^' => {
            solve(cache, grid, position + DOWN_LEFT) + solve(cache, grid, position + DOWN_RIGHT)
        }
        _ => unreachable!(),
    };

    cache.insert(position, result);
    result
}

pub fn part2(input: &ParsedInput) -> usize {
    let (grid, start) = input;
    let mut cache = HashMap::<Position, usize>::new();

    solve(&mut cache, grid, *start)
}
