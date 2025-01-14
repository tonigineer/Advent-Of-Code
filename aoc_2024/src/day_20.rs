use common::{grid::Grid, Answer, Solution};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub struct Day20;

impl Solution for Day20 {
    fn name(&self) -> &'static str {
        "Race Condition"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, false, 100).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, true, 100).into()
    }
}

fn solve(input: &str, part2: bool, min_save: isize) -> usize {
    let grid: Grid<char> = Grid::from(input);

    let (rs, cs) = (0..grid.rows)
        .cartesian_product(0..grid.cols)
        .find(|&(r, c)| grid.data[r][c] == 'S')
        .unwrap();

    let mut new_grid: VecDeque<VecDeque<isize>> = VecDeque::new();

    for _ in 0..grid.rows {
        let row: VecDeque<isize> = VecDeque::from(vec![-1; grid.cols]);
        new_grid.push_back(row);
    }

    new_grid[rs][cs] = 0;

    let mut q = VecDeque::new();
    q.push_back((rs, cs, 0));
    let mut seen = HashSet::new();

    while !q.is_empty() {
        let (r, c, d) = q.pop_front().unwrap();

        if !seen.insert((r, c)) {
            continue;
        }

        for (dr, dc) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let (nr, nc) = (dr + r as isize, dc + c as isize);

            if seen.contains(&(nr as usize, nc as usize)) {
                continue;
            }
            if nr < 0 || nr >= (grid.rows as isize) || nc < 0 || nc >= (grid.cols as isize) {
                continue;
            }

            if grid.data[nr as usize][nc as usize] == '#' {
                continue;
            }

            new_grid[nr as usize][nc as usize] = d + 1;
            q.push_back((nr as usize, nc as usize, d + 1));
        }
    }

    let mut ans = 0;
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if grid.data[r][c] == '#' {
                continue;
            }

            if !part2 {
                for (dr, dc) in [(2, 0), (1, 1), (0, 2), (-1, 1)] {
                    // check only half it adjacend tiles to avoid double counting
                    let (nr, nc) = (dr + r as isize, dc + c as isize);

                    if nr < 0 || nr >= (grid.rows as isize) || nc < 0 || nc >= (grid.cols as isize)
                    {
                        continue;
                    }

                    if grid.data[nr as usize][nc as usize] == '#' {
                        continue;
                    }

                    if (new_grid[r as usize][c as usize] - new_grid[nr as usize][nc as usize]).abs()
                        >= min_save + 2
                    {
                        dbg!(
                            new_grid[r as usize][c as usize],
                            new_grid[nr as usize][nc as usize]
                        );
                        ans += 1;
                    }
                }
                continue;
            }

            for radius in 2..=20 as isize {
                for dr in 0..=radius {
                    let dc = radius - dr;

                    for (nr, nc) in [
                        (r as isize + dr, c as isize - dc),
                        (r as isize + dr, c as isize + dc),
                        (r as isize - dr, c as isize - dc),
                        (r as isize - dr, c as isize + dc),
                    ] {
                        if nr < 0
                            || nr >= (grid.rows) as isize
                            || nc < 0
                            || nc >= (grid.cols) as isize
                        {
                            continue;
                        }

                        if grid.data[nr as usize][nc as usize] == '#' {
                            continue;
                        }

                        if new_grid[r as usize][c as usize] - new_grid[nr as usize][nc as usize]
                            >= (min_save + radius + 2)
                        {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false, 2), 44);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true, 50), 285);
    }
}
