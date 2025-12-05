//! # Printing Department
//!
//!

use crate::common::grid::*;
use crate::common::position::*;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u32 {
    let grid = Grid::from(input);
    let mut result = 0;

    for r in 0..grid.height() {
        for c in 0..grid.width() {
            let position = Position::new(c, r);
            if grid[position] != b'@' {
                continue;
            }

            let mut neighbors = 0;
            for delta in CARDINALS.iter().chain(DIAGONALS.iter()) {
                let new_position = position + *delta;

                if !grid.contains(new_position) {
                    continue;
                }

                if grid[position + *delta] == b'@' {
                    neighbors += 1;
                }
            }

            if neighbors < 4 {
                dbg!(position);
                dbg!(neighbors);
                result += 1;
            }
        }
    }

    result
}

pub fn part2(input: &str) -> u32 {
    let mut grid = Grid::from(input);
    let mut result = 0;

    loop {
        let mut changes = false;
        let mut new_grid = grid.clone();

        for r in 0..grid.height() {
            for c in 0..grid.width() {
                let position = Position::new(c, r);

                if grid[position] != b'@' {
                    continue;
                }

                let mut neighbors = 0;
                for delta in CARDINALS.iter().chain(DIAGONALS.iter()) {
                    let new_position = position + *delta;

                    if !grid.contains(new_position) {
                        continue;
                    }

                    if grid[position + *delta] == b'@' {
                        neighbors += 1;
                    }
                }

                if neighbors < 4 {
                    dbg!(position);
                    dbg!(neighbors);
                    result += 1;
                    new_grid[position] = b'.';
                    changes = true;
                }
            }
        }

        grid = new_grid;
        if !changes {
            break;
        }
    }

    // result
    result
}
