use common::{Answer, Solution};

use std::collections::{VecDeque, HashSet};

pub struct Day10;

impl Solution for Day10 {
    fn name(&self) -> &'static str {
        "Pipe Maze"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(input: &str, part2: bool) -> u64 {
    let grid = common::string_to_grid(input);
    let mut start: (usize, usize) = (0, 0);

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == 'S' {
                start = (r, c);
                // break out all for loops
            }
        }
    }

    let mut queue: VecDeque<((usize, usize), bool)> = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back((start, false));

    while let Some((pos, moving_vertical)) = queue.pop_front() {
        if seen.contains(&pos) { continue };

        seen.insert(pos);
        let (r, c) = pos;

        if moving_vertical {
            if r >= 1 {
                let dr = r - 1;
                if "S|F7".contains(grid[dr][c]) && "S|LJ".contains(grid[r][c]) {
                    queue.push_back(
                        (
                            (dr, c),
                            if grid[dr][c] == '|' {true} else {false}
                        )
                    );
                }
            }

            let dr = r + 1;
            if dr < grid.len() {
                if "S|LJ".contains(grid[dr][c]) && "S|F7".contains(grid[r][c]) {
                    queue.push_back(
                        (
                            (dr, c),
                            if grid[dr][c] == '|' {true} else {false}
                        )
                    );
                }
            }
        } else {
            if c >= 1 {
                let dc = c - 1;
                if "S-LF".contains(grid[r][dc]) && "S-J7".contains(grid[r][c]) {
                    queue.push_back(
                        (
                            (r, dc),
                            if grid[r][dc] == '-' {false} else {true}
                        )
                    );
                }
            }

            let dc = c + 1;
            if dc < grid[0].len() {
                if "S-7J".contains(grid[r][dc]) && "S-FL".contains(grid[r][c]) {
                    queue.push_back(
                        (
                            (r, dc),
                            if grid[r][dc] == '-' {false} else {true}
                        )
                    );
                }
            }
        }
    }

    if part2 {
        let mut p: HashSet<(usize, usize)> = HashSet::new();

        for r in 0..grid.len() {
            let mut is_inside = false;
            for c in 0..grid[r].len() {
                // if S represents a facing north char, replace it in input
                if seen.contains(&(r, c)) && "LJ|".contains(grid[r][c]) {
                    is_inside = !is_inside;
                } else if !seen.contains(&(r, c)) && is_inside {
                    p.insert((r, c));
                }
            }
        }

        return p.len() as u64;
    }

    return (seen.len() / 2) as u64;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {
        "-L|F7
        7S-7|
        L|7||
        -L-J|W
        L|-JF
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 4);
    }

    const SAMPLE2: &str = indoc! {
        ".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
    "};

    const SAMPLE3: &str = indoc! {
        "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "};

    #[test]
    fn example_part2a() {
        assert_eq!(solve(SAMPLE2, true), 8);
        assert_eq!(solve(SAMPLE3, true), 4);
    }
}