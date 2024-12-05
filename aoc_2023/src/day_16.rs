use common::{grid::Grid, Answer, Solution};

use std::collections::{HashSet, VecDeque};

pub struct Day16;

impl Solution for Day16 {
    fn name(&self) -> &'static str {
        "The Floor Will Be Lava"
    }

    fn part1(&self, input: &str) -> Answer {
        let g: Grid<char> = input.into();
        let grid = g.data;

        return solve(&grid, (0, -1, 0, 1)).into();
    }

    fn part2(&self, input: &str) -> Answer {
        let g: Grid<char> = input.into();
        let grid = g.data;
        let mut ans = 0;

        for r in 0..grid.len() {
            ans = ans.max(solve(&grid, (r as isize, -1, 0, 1)));
            ans = ans.max(solve(&grid, (r as isize, grid[0].len() as isize, 0, -1)));
        }

        for c in 0..grid[0].len() {
            ans = ans.max(solve(&grid, (-1, c as isize, 1, 0)));
            ans = ans.max(solve(&grid, (grid.len() as isize, c as isize, -1, 0)));
        }

        return ans.into();
    }
}

fn solve(grid: &VecDeque<VecDeque<char>>, initial: (isize, isize, isize, isize)) -> usize {
    let mut seen: HashSet<(isize, isize, isize, isize)> = HashSet::new();
    let mut queue: VecDeque<(isize, isize, isize, isize)> = VecDeque::new();

    queue.push_back(initial);

    while !queue.is_empty() {
        let (mut r, mut c, mut dr, mut dc) = queue.pop_front().unwrap();

        r = r + dr;
        c = c + dc;

        if r < 0 || r >= grid.len() as isize || c < 0 || c >= grid[0].len() as isize {
            continue;
        };

        let ch: char = grid[r as usize][c as usize];
        if (dr == 0 && ch == '-') || (dc == 0 && ch == '|') || ch == '.' {
            if !seen.contains(&(r, c, dr, dc)) {
                queue.push_back((r, c, dr, dc));
                seen.insert((r, c, dr, dc));
            }
        } else if ch == '/' {
            let t = dr;
            dr = -dc;
            dc = -t;
            if !seen.contains(&(r, c, dr, dc)) {
                queue.push_back((r, c, dr, dc));
                seen.insert((r, c, dr, dc));
            }
        } else if ch == '\\' {
            let t = dr;
            dr = dc;
            dc = t;
            if !seen.contains(&(r, c, dr, dc)) {
                queue.push_back((r, c, dr, dc));
                seen.insert((r, c, dr, dc));
            }
        } else {
            match ch {
                '-' => {
                    for dir in [(0, 1), (0, -1)] {
                        dr = dir.0;
                        dc = dir.1;
                        if !seen.contains(&(r, c, dr, dc)) {
                            queue.push_back((r, c, dr, dc));
                            seen.insert((r, c, dr, dc));
                        }
                    }
                }
                '|' => {
                    for dir in [(1, 0), (-1, 0)] {
                        dr = dir.0;
                        dc = dir.1;
                        if !seen.contains(&(r, c, dr, dc)) {
                            queue.push_back((r, c, dr, dc));
                            seen.insert((r, c, dr, dc));
                        }
                    }
                }
                _ => panic! {"Not supposed to happen, ffs!"},
            }
        }
    }

    let mut ans: HashSet<(isize, isize)> = HashSet::new();
    for item in seen.iter() {
        ans.insert((item.0, item.1));
    }
    ans.len()
}
