//! Guard Gallivant
//!
//! Summary:

use crate::common::grid_legacy::*;
use std::collections::HashSet;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u32 {
    solve_part1(input)
}

pub fn part2(input: &str) -> u32 {
    solve_part2(input)
}

fn find_guard(grid: &Grid<char>) -> (isize, isize) {
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if grid.data[r][c] == '^' {
                return (c as isize, r as isize);
            }
        }
    }
    panic!("Where is she?");
}

fn solve_part1(input: &str) -> u32 {
    let grid: Grid<char> = input.into();
    let mut guard = find_guard(&grid);
    let mut dc = 0;
    let mut dr = -1;

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert(guard);

    loop {
        let nc = guard.0 + dc;
        let nr = guard.1 + dr;

        if !grid.in_bounds(nc, nr) {
            break;
        }

        if grid.data[nr as usize][nc as usize] == '#' {
            (dc, dr) = (-dr, dc);
            continue;
        }

        guard = (nc, nr);
        visited.insert(guard);
    }

    visited.len() as u32
}

fn is_loop(grid: &Grid<char>, start: &(isize, isize)) -> bool {
    let mut guard = (start.0, start.1);
    let mut dc = 0;
    let mut dr = -1;

    let mut visited: HashSet<(isize, isize, isize, isize)> = HashSet::new();
    visited.insert((guard.0, guard.1, dc, dr));

    loop {
        let nc = guard.0 + dc;
        let nr = guard.1 + dr;

        if !grid.in_bounds(nc, nr) {
            return false;
        }

        if grid.data[nr as usize][nc as usize] == '#' {
            (dc, dr) = (-dr, dc);
            continue;
        }

        if visited.contains(&(nc, nr, dc, dr)) {
            return true;
        }

        guard = (nc, nr);
        visited.insert((nc, nr, dc, dr));
    }
}

fn solve_part2(input: &str) -> u32 {
    let grid: Grid<char> = input.into();
    let guard: (isize, isize) = find_guard(&grid);

    let mut ans = 0;

    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if (c as isize, r as isize) == guard {
                continue;
            }

            let mut obstructed_grid: Grid<char> = grid.clone();
            obstructed_grid.data[r][c] = '#';

            if is_loop(&obstructed_grid, &guard) {
                ans += 1;
            }
        }
    }

    ans
}
