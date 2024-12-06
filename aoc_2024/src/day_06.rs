use common::grid::Grid;
use common::{Answer, Solution};
use std::collections::HashSet;

pub struct Day06;

impl Solution for Day06 {
    fn name(&self) -> &'static str {
        "Guard Gallivant"
    }

    fn part1(&self, input: &str) -> Answer {
        solve_part1(input).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve_part2(input).into()
    }
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
    let mut dc = 0;
    let mut dr = -1;
    let mut guard = find_guard(&grid);

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert(guard);

    loop {
        let nc = guard.0 + dc;
        let nr = guard.1 + dr;

        if !grid.in_bounds(nc, nr) {
            break;
        }

        if grid.data[nr as usize][nc as usize] == '#' {
            let dc_temp = dc;
            dc = -dr;
            dr = dc_temp;

            continue;
        }

        guard = (nc, nr);
        visited.insert(guard);
    }

    visited.len() as u32
}

fn is_loop(grid: &Grid<char>, start: &(isize, isize)) -> bool {
    let mut visited: HashSet<(isize, isize, isize, isize)> = HashSet::new();
    let mut dc = 0;
    let mut dr = -1;
    let mut guard = start.clone();
    visited.insert((guard.0 as isize, guard.1 as isize, dc, dr));

    loop {
        let nc = guard.0 + dc;
        let nr = guard.1 + dr;

        if !grid.in_bounds(nc, nr) {
            return false;
        }

        if grid.data[nr as usize][nc as usize] == '#' {
            let dc_temp = dc;
            dc = -dr;
            dr = dc_temp;

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

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const SAMPLE: &str = indoc::indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(SAMPLE), 41);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(SAMPLE), 6);
    }
}
