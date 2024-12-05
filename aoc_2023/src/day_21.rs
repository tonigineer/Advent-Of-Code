use common::{Answer, Solution};
use std::collections::{HashSet, VecDeque};

pub struct Day21;

impl Solution for Day21 {
    fn name(&self) -> &'static str {
        "Step Counter"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, 64).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve2(input, 26501365).into();
    }
}

fn solve2(input: &str, steps: u64) -> u64 {
    let g: common::grid::Grid<char> = input.into();
    let mut sr: isize = 0;
    let mut sc: isize = 0;

    for r in 0..g.data.len() {
        for c in 0..g.data[0].len() {
            if g.data[r][c] == 'S' {
                sr = r as isize;
                sc = c as isize;
            }
        }
    }

    // Assumptions
    let size = g.data.len();
    assert_eq!(g.data.len(), g.data[0].len());
    assert_eq!(sr, sc);
    assert_eq!(sr as usize, g.data.len() / 2);

    let grid_width = steps / size as u64 - 1;
    // println!("{} {}", size, grid_width);

    let num_odd_grids = (grid_width / 2 * 2 + 1).pow(2);
    let num_even_grids = ((grid_width + 1) / 2 * 2).pow(2);
    // println!("{} {}", num_odd_grids, num_even_grids);

    let odd_points = fill(&g, sr, sc, (size * 2 + 1) as u64);
    let even_points = fill(&g, sr, sc, (size * 2) as u64);
    // println!("{} {}", odd_points, even_points);

    let corner_top = fill(&g, size as isize - 1, sc, size as u64 - 1);
    let corner_right = fill(&g, sr, 0, size as u64 - 1);
    let corner_bottom = fill(&g, 0, sc, size as u64 - 1);
    let corner_left = fill(&g, sr, size as isize - 1, size as u64 - 1);
    // println!("{} {} {} {}", corner_top, corner_right, corner_bottom, corner_left);

    let small_top_right = fill(&g, (size - 1) as isize, 0, (size / 2 - 1) as u64);
    let small_top_left = fill(
        &g,
        (size - 1) as isize,
        (size - 1) as isize,
        (size / 2 - 1) as u64,
    );
    let small_bottom_right = fill(&g, 0, 0, (size / 2 - 1) as u64);
    let small_bottom_left = fill(&g, 0, (size - 1) as isize, (size / 2 - 1) as u64);
    // println!("{} {} {} {}", small_top_right, small_top_left, small_bottom_right, small_bottom_left);

    let large_top_right = fill(&g, (size - 1) as isize, 0, (size * 3 / 2 - 1) as u64);
    let large_top_left = fill(
        &g,
        (size - 1) as isize,
        (size - 1) as isize,
        (size * 3 / 2 - 1) as u64,
    );
    let large_bottom_right = fill(&g, 0, 0, (size * 3 / 2 - 1) as u64);
    let large_bottom_left = fill(&g, 0, (size - 1) as isize, (size * 3 / 2 - 1) as u64);
    // println!("{} {} {} {}", large_top_right, large_top_left, large_bottom_right, large_bottom_left);

    return (num_odd_grids * odd_points as u64
        + num_even_grids * even_points as u64
        + (corner_top + corner_right + corner_bottom + corner_left) as u64
        + (grid_width + 1)
            * (small_top_right + small_top_left + small_bottom_right + small_bottom_left) as u64
        + grid_width
            * (large_top_right + large_top_left + large_bottom_right + large_bottom_left) as u64)
        as u64;
}

fn solve(input: &str, steps: u64) -> u64 {
    let g: common::grid::Grid<char> = input.into();
    let mut sr: isize = 0;
    let mut sc: isize = 0;

    for r in 0..g.data.len() {
        for c in 0..g.data[0].len() {
            if g.data[r][c] == 'S' {
                sr = r as isize;
                sc = c as isize;
            }
        }
    }

    return fill(&g, sr, sc, steps);
}

fn fill(g: &common::grid::Grid<char>, sr: isize, sc: isize, steps: u64) -> u64 {
    let mut ans = HashSet::<(isize, isize)>::new();
    let mut seen = HashSet::<(isize, isize)>::new();
    seen.insert((sr, sc));

    let mut q = VecDeque::<(isize, isize, u64)>::new(); // r, c, step remaining
    q.push_back((sr, sc, steps));

    while !q.is_empty() {
        let (r, c, s) = q.pop_front().unwrap();

        if s % 2 == 0 {
            ans.insert((r, c));
        }
        if s == 0 {
            continue;
        }

        for (dr, dc) in [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)] {
            let nr = r + dr;
            let nc = c + dc;

            if nr < 0 || nr >= g.data.len() as isize {
                continue;
            }
            if nc < 0 || nc >= g.data[0].len() as isize {
                continue;
            }

            if g.data[nr as usize][nc as usize] == '#' {
                continue;
            }
            if seen.contains(&(nr, nc)) {
                continue;
            }

            seen.insert((nr, nc));
            q.push_back((nr, nc, s - 1));
        }
    }
    return ans.len() as u64;
}

#[cfg(test)]
mod tests {
    use super::solve;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ..........."};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, 6), 16);
    }

    // #[test]
    // fn example_part2() {
    //     assert_eq!(solve(SAMPLE, true), 94);
    // }
}
