//! No Such Thing as Too Much
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use std::collections::HashMap;
pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> i32 {
    let mut grid: Grid = input.into();
    for _ in 0..100 {
        grid.update(false);
    }
    grid.lights_on_total()
}

pub fn part2(input: &str) -> i32 {
    let mut grid: Grid = input.into();
    for _ in 0..100 {
        grid.update(true);
    }
    grid.lights_on_total()
}

#[derive(Clone)]
struct Grid {
    grid: HashMap<(isize, isize), bool>,
    rows: isize,
    cols: isize,
}

impl From<&str> for Grid {
    fn from(s: &str) -> Grid {
        let mut grid: HashMap<(isize, isize), bool> = HashMap::new();

        for (r, line) in s.lines().enumerate() {
            for (c, state) in line.chars().enumerate() {
                grid.insert((r as isize, c as isize), state == '#');
            }
        }

        Grid {
            grid,
            rows: s.lines().count() as isize,
            cols: s.lines().next().unwrap().chars().count() as isize,
        }
    }
}

impl Grid {
    fn lights_on_neighbors(&self, key: (isize, isize)) -> i32 {
        let mut num_lights_on = 0;
        for dr in -1..=1_isize {
            for dc in -1..=1_isize {
                if dr == 0 && dc == 0 {
                    continue;
                }
                if *self.grid.get(&(key.0 + dr, key.1 + dc)).unwrap_or(&false) {
                    num_lights_on += 1;
                }
            }
        }
        num_lights_on
    }

    fn update(&mut self, part2: bool) {
        let mut new_grid: HashMap<(isize, isize), bool> = HashMap::new();
        for (r, c) in self.grid.keys() {
            let current_state = self.grid.get(&(*r, *c)).unwrap();
            let num_neighbors_on = self.lights_on_neighbors((*r, *c));

            new_grid.insert(
                (*r, *c),
                if *current_state {
                    [2, 3].contains(&num_neighbors_on)
                } else {
                    [3].contains(&num_neighbors_on)
                },
            );
        }

        if part2 {
            new_grid.insert((0, 0), true);
            new_grid.insert((self.rows - 1, 0), true);
            new_grid.insert((0, self.cols - 1), true);
            new_grid.insert((self.rows - 1, self.cols - 1), true);
        }

        self.grid = new_grid;
    }

    fn lights_on_total(&self) -> i32 {
        self.grid.values().filter(|l| **l).count() as i32
    }
}
