//! Warehouse Woes
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use crate::common::grid_legacy::*;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> usize {
    solve(input, false)
}

pub fn part2(input: &str) -> usize {
    solve(input, true)
}

fn solve(input: &str, part2: bool) -> usize {
    let (mut grid_input, moves) = input.split_once("\n\n").unwrap();

    let mut changed_grid = grid_input.to_string();

    if part2 {
        changed_grid = changed_grid
            .replace("#", "##")
            .replace(".", "..")
            .replace("O", "[]")
            .replace("@", "@.");
        grid_input = changed_grid.as_str();
    }

    let mut grid: Grid<char> = Grid::from(grid_input);

    let (mut r, mut c) = (0..grid.rows)
        .cartesian_product(0..grid.cols)
        .find(|&(r, c)| grid.data[r][c] == '@')
        .unwrap();

    'moves_loop: for m in moves.trim().chars() {
        let dir: Direction = match m {
            '<' => Direction::Left,
            '^' => Direction::Top,
            '>' => Direction::Right,
            'v' => Direction::Bottom,
            _ => continue,
        };

        let mut q = VecDeque::from([(c, r)]);
        let mut pushes = HashSet::new();

        while !q.is_empty() {
            let (c, r) = q.pop_front().unwrap();
            let (nc, nr) = (
                (c as isize + dir.as_delta().0) as usize,
                (r as isize + dir.as_delta().1) as usize,
            );

            if !pushes.insert((c, r)) {
                continue;
            }

            match grid.data[nr][nc] {
                '#' => continue 'moves_loop,
                'O' => q.push_back((nc, nr)),
                '[' => q.extend([(nc, nr), (nc + 1, nr)]),
                ']' => q.extend([(nc, nr), (nc - 1, nr)]),
                _ => continue,
            }
        }

        while !pushes.is_empty() {
            for (c, r) in pushes.clone().iter() {
                let (nc, nr) = (
                    (*c as isize + dir.as_delta().0) as usize,
                    (*r as isize + dir.as_delta().1) as usize,
                );
                if !pushes.contains(&(nc, nr)) {
                    grid.data[nr][nc] = grid.data[*r][*c];
                    grid.data[*r][*c] = '.';
                    pushes.remove(&(*c, *r));
                }
            }
        }

        (c, r) =
            ((c as isize + dir.as_delta().0) as usize, (r as isize + dir.as_delta().1) as usize);
    }

    (0..grid.rows)
        .cartesian_product(0..grid.cols)
        .filter(|&(r, c)| (grid.data[r][c] == '[' && part2) || grid.data[r][c] == 'O')
        .map(|(r, c)| r * 100 + c)
        .sum()
}
