use common::{Answer, Solution};

use std::collections::HashMap;

pub struct Day18;

impl Solution for Day18 {
    fn name(&self) -> &'static str {
        "No Such Thing as Too Much"
    }

    fn part1(&self, input: &str) -> Answer {
        let mut grid: Grid = input.into();
        for _ in 0..100 {
            grid.update(false);
        }
        return grid.lights_on_total().into();
    }

    fn part2(&self, input: &str) -> Answer {
        let mut grid: Grid = input.into();
        for _ in 0..100 {
            grid.update(true);
        }
        return grid.lights_on_total().into();
    }
}

#[derive(Clone)]
struct Grid {
    grid: HashMap<(isize, isize), bool>,
    rows: isize,
    cols: isize
}

impl From<&str> for Grid {
    fn from(s: &str) -> Grid {
        let mut grid: HashMap<(isize, isize), bool> = HashMap::new();

        for (r, line) in s.lines().enumerate() {
            for (c, state) in line.chars().enumerate() {
                grid.insert((r as isize, c as isize), state == '#');
            }
        }

        return Grid {
            grid: grid,
            rows: s.lines().count() as isize,
            cols: s.lines().next().unwrap().chars().count() as isize
        }
    }
}

impl Grid {
    // fn visualize(&self) {
    //     println!("-- Visualize");
    //     for r in 0..self.rows {
    //         for c in 0..self.cols {
    //             print!("{}", if *self.grid.get(&(r, c)).unwrap()  {'#'} else {'.'});
    //         }
    //         println!("");
    //     }
    // }

    fn lights_on_neighbors(&self, key: (isize, isize)) -> i32 {
        let mut num_lights_on = 0;
        for dr in -1..=1 as isize {
            for dc in -1..=1 as isize {
                if dr == 0 && dc == 0 { continue }
                if *self.grid.get(&(key.0 + dr, key.1 + dc)).unwrap_or(&false) {
                    num_lights_on += 1;
                }
            }
        }
        return num_lights_on;
    }

    fn update(&mut self, part2: bool) {
        let mut new_grid: HashMap<(isize, isize), bool> = HashMap::new();
        for (r, c) in self.grid.keys() {
            let current_state = self.grid.get(&(*r, *c)).unwrap();
            let num_neighbors_on = self.lights_on_neighbors((*r, *c));

            new_grid.insert(
                (*r, *c),
                if *current_state {[2, 3].contains(&num_neighbors_on)}
                    else {[3].contains(&num_neighbors_on)});
        }

        if part2 {
            new_grid.insert((0,             0), true);
            new_grid.insert((self.rows-1,   0), true);
            new_grid.insert((0,             self.cols-1), true);
            new_grid.insert((self.rows-1,   self.cols-1), true);
        }

        self.grid = new_grid;
    }

    fn lights_on_total(&self) -> i32 {
        return self.grid
            .values()
            .filter(|l| **l)
            .count() as i32;
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::Grid;

    const SAMPLE: &str = indoc! {"
        .#.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####..
    "};

    #[test]
    fn example_part1() {
        let mut grid: Grid = SAMPLE.into();
        for _ in 0..4 {
            grid.update(false);
        }
        assert_eq!(grid.lights_on_total(), 4);
    }

    const SAMPLE2: &str = indoc! {"
        ##.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####.#
    "};

    #[test]
    fn example_part2() {
        let mut grid: Grid = SAMPLE2.into();
        for _ in 0..5 {
            grid.update(true);
        }
        assert_eq!(grid.lights_on_total(), 17);
    }
}