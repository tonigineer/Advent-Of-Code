//! Hoof It
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use crate::common::grid_legacy::*;
use std::collections::VecDeque;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u32 {
    solve(input, false)
}

pub fn part2(input: &str) -> u32 {
    solve(input, true)
}

fn solve(input: &str, part2: bool) -> u32 {
    let mut ans = 0;
    let grid: Grid<usize> = Grid::from(input);

    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if grid.data[r][c] == 0 {
                ans += find_trails(&grid, c, r, part2);
            }
        }
    }

    ans
}

fn find_trails(grid: &Grid<usize>, c: usize, r: usize, part2: bool) -> u32 {
    let mut trails = 0;

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((c, r));

    while !q.is_empty() {
        let (c, r) = q.pop_front().unwrap();

        if grid.data[r][c] == 9 {
            trails += 1;
            continue;
        }

        for (nc, nr) in grid.adjacent_cardinals(c, r) {
            if grid.data[r][c] + 1 == grid.data[nr][nc] && (!q.contains(&(nc, nr)) || part2) {
                q.push_back((nc, nr));
            }
        }
    }

    trails
}
