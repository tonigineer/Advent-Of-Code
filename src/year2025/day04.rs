//! # Printing Department
//!
//!

use crate::common::grid::*;
use crate::common::position::*;
use itertools::Itertools;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::from(input.trim())
}

fn can_be_moved(grid: &Grid<u8>, position: &Position) -> bool {
    CARDINALS
        .iter()
        .chain(DIAGONALS.iter())
        .filter(|&delta| {
            let new_position = *position + *delta;
            grid.contains(new_position) && grid[new_position] == b'@'
        })
        .count()
        < 4
}

pub fn part1(grid: &Grid<u8>) -> usize {
    (0..grid.height())
        .cartesian_product(0..grid.width())
        .filter(|(r, c)| {
            let position = Position::new(*c, *r);
            grid[position] == b'@' && can_be_moved(grid, &position)
        })
        .count()
}

pub fn part2(grid: &Grid<u8>) -> u32 {
    let mut current = grid.clone();
    let mut next = current.clone();
    let mut result = 0;

    loop {
        let mut has_changed = false;

        for (r, c) in (0..current.height()).cartesian_product(0..current.width()) {
            let position = Position::new(c, r);

            if current[position] == b'@' && can_be_moved(&current, &position) {
                result += 1;
                next[position] = b'.';
                has_changed = true;
            } else {
                next[position] = current[position];
            }
        }

        if !has_changed {
            break;
        }

        std::mem::swap(&mut current, &mut next);
    }

    result
}
